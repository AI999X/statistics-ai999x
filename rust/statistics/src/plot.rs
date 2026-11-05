#![allow(dead_code)]
//! A plotting frame for this model's own figures: axes with ticks in the generic 520 x 340 chart frame, on which a
//! figure draws its own marks (shaded areas under a density, stems, box plots, intervals, dots, segments). Only the
//! website's figure classes (axp-*) are used, so the page's light and dark themes apply. Every text is kept inside
//! the canvas.

use ax_engine::figkit::num;

pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const Y0: f64 = 44.0;
pub const Y1: f64 = 278.0;

/// The first four colours of the theme: lines, dots, bars, shaded bands.
pub const LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
pub const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
pub const BAR: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
pub const BAND: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Estimated width of a text in a class (a little wider than drawn, so a clamped text never leaves the canvas).
pub fn width(t: &str, class: &str) -> f64 {
    let per = match class {
        "axp-mt" => 9.4,
        "axp-vt" | "axp-t" | "axp-ht" => 8.3,
        "axp-l" => 8.6,
        _ => 7.8,
    };
    t.chars().count() as f64 * per
}

/// Shortened to n characters ("..." at the end).
pub fn clip(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}...", c[..n.saturating_sub(3)].iter().collect::<String>().trim_end()) }
}

/// A linear (or logarithmic) axis: values lo..hi onto pixels a..b.
#[derive(Clone, Copy, Debug)]
pub struct Ax {
    pub lo: f64,
    pub hi: f64,
    a: f64,
    b: f64,
    log: bool,
}

impl Ax {
    /// `pad`: 6 % of the span added at both ends.
    pub fn new(lo: f64, hi: f64, a: f64, b: f64, pad: bool) -> Ax {
        let (mut lo, mut hi) = if lo.is_finite() && hi.is_finite() { (lo.min(hi), hi.max(lo)) } else { (0.0, 1.0) };
        if hi - lo <= 1e-12 * (lo.abs() + hi.abs()).max(1e-300) {
            let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
            lo -= d;
            hi += d;
        }
        if pad {
            let d = (hi - lo) * 0.06;
            lo -= d;
            hi += d;
        }
        Ax { lo, hi, a, b, log: false }
    }
    /// A logarithmic axis (both ends positive).
    pub fn log(lo: f64, hi: f64, a: f64, b: f64) -> Ax {
        let lo = if lo > 0.0 { lo } else { hi / 10.0 };
        let hi = if hi > lo { hi } else { lo * 10.0 };
        Ax { lo: lo / 1.25, hi: hi * 1.25, a, b, log: true }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let step = (((e1 - e0 + 1).max(1) as f64) / 6.0).ceil().max(1.0) as usize;
            return (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
        }
        nice_ticks(self.lo, self.hi, 5)
    }
}

pub fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

/// The smallest and largest finite value.
pub fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().filter(|x| x.is_finite()).fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

/// A chart: the frame (axes, ticks, labels, heading) and the marks a figure adds, in data coordinates.
pub struct Plot {
    pub xa: Ax,
    pub ya: Ax,
    pub s: String,
    pub h: f64,
    /// the labels placed so far: (baseline, left, right) in pixels
    occ: Vec<(f64, f64, f64)>,
}

impl Plot {
    /// The generic frame (96..504 across, 44..278 down) over the value ranges given, padded.
    pub fn new(xr: (f64, f64), yr: (f64, f64)) -> Plot {
        Plot::axes(Ax::new(xr.0, xr.1, X0, X1, true), Ax::new(yr.0, yr.1, Y1, Y0, true))
    }
    /// The frame with a vertical range from 0 (not padded below): densities, bars, probabilities.
    pub fn from_zero(xr: (f64, f64), ymax: f64, pad_x: bool) -> Plot {
        let ya = Ax::new(0.0, if ymax > 0.0 { ymax * 1.08 } else { 1.0 }, Y1, Y0, false);
        Plot::axes(Ax::new(xr.0, xr.1, X0, X1, pad_x), ya)
    }
    pub fn axes(xa: Ax, ya: Ax) -> Plot {
        Plot { xa, ya, s: String::new(), h: 340.0, occ: vec![] }
    }
    /// Equal scales across and up (a plane: points, distances, circles); the data centred.
    pub fn equal(xs: &[f64], ys: &[f64]) -> Plot {
        let (x0, x1) = minmax(xs);
        let (y0, y1) = minmax(ys);
        let (mut w, mut h) = ((x1 - x0).max(1e-9), (y1 - y0).max(1e-9));
        // pixels per unit: the same on both axes
        let (pw, ph) = (X1 - X0, Y1 - Y0);
        let k = (pw / (w * 1.15)).min(ph / (h * 1.15));
        w = pw / k;
        h = ph / k;
        let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        Plot::axes(Ax::new(cx - w / 2.0, cx + w / 2.0, X0, X1, false), Ax::new(cy - h / 2.0, cy + h / 2.0, Y1, Y0, false))
    }
    pub fn x(&self, v: f64) -> f64 {
        self.xa.px(v)
    }
    pub fn y(&self, v: f64) -> f64 {
        self.ya.px(v)
    }
    /// Grid, ticks, axis names and the heading; `xt`/`yt`: draw that axis's ticks.
    pub fn frame_with(&mut self, xlab: &str, ylab: &str, header: &str, xt: bool, yt: bool) {
        let mut o = String::new();
        if xt {
            for t in self.xa.ticks() {
                let x = self.x(t);
                if !(X0 - 0.5..=X1 + 0.5).contains(&x) {
                    continue;
                }
                o.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{Y0}\" x2=\"{x:.1}\" y2=\"{Y1}\"/>"));
                self.tick_text(&mut o, x, Y1 + 20.0, "middle", &num(t));
            }
        }
        if yt {
            for t in self.ya.ticks() {
                let y = self.y(t);
                if !(Y0 - 0.5..=Y1 + 0.5).contains(&y) {
                    continue;
                }
                o.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>"));
                self.tick_text(&mut o, X0 - 6.0, y + 5.0, "end", &num(t));
            }
        }
        o.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
        o.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/></clipPath>",
                            X1 - X0, Y1 - Y0));
        self.s.insert_str(0, &o);
        if !xlab.is_empty() {
            self.text((X0 + X1) / 2.0, 326.0, "middle", "axp-l", &clip(xlab, 54));
        }
        if !ylab.is_empty() {
            self.s.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" \
                                      text-anchor=\"middle\">{}</text>", (Y0 + Y1) / 2.0, esc(&clip(ylab, 28))));
        }
        if !header.is_empty() {
            self.text(X0, 20.0, "start", "axp-mt", header);
        }
    }
    pub fn frame(&mut self, xlab: &str, ylab: &str, header: &str) {
        self.frame_with(xlab, ylab, header, true, true);
    }
    fn tick_text(&self, o: &mut String, x: f64, y: f64, anchor: &str, t: &str) {
        o.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>", esc(t)));
    }
    /// Marks drawn after this are clipped to the plot area, until `unclip`.
    pub fn clip_on(&mut self) {
        self.s.push_str("<g clip-path=\"url(#CLIPID)\">");
    }
    pub fn unclip(&mut self) {
        self.s.push_str("</g>");
    }
    fn d(&self, pts: &[(f64, f64)], close: bool) -> String {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in pts {
            let (px, py) = (self.x(*x), self.y(*y));
            if !px.is_finite() || !py.is_finite() {
                pen = false;
                continue;
            }
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        if close && !d.is_empty() {
            d.push('Z');
        }
        d.trim_end().to_string()
    }
    /// A polyline through data points (a non-finite point breaks it).
    pub fn path(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts, false);
        if !d.is_empty() {
            self.s.push_str(&format!("<path class=\"{class}\" fill=\"none\" d=\"{d}\"/>"));
        }
    }
    pub fn dpath(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts, false);
        if !d.is_empty() {
            self.s.push_str(&format!("<path class=\"{class}\" fill=\"none\" stroke-dasharray=\"5 4\" d=\"{d}\"/>"));
        }
    }
    /// A filled polygon.
    pub fn fill(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts, true);
        if !d.is_empty() {
            self.s.push_str(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
        }
    }
    /// The area between a curve (points in increasing x) and the level `base`, between x = a and x = b (the curve
    /// interpolated at both ends).
    pub fn under(&mut self, pts: &[(f64, f64)], a: f64, b: f64, base: f64, class: &str) {
        let (a, b) = (a.min(b), a.max(b));
        let mut poly = vec![];
        if let Some(ya) = interp(pts, a) {
            poly.push((a, base));
            poly.push((a, ya));
        }
        for p in pts.iter().filter(|p| p.0 > a && p.0 < b) {
            poly.push(*p);
        }
        if let Some(yb) = interp(pts, b) {
            poly.push((b, yb));
            poly.push((b, base));
        }
        if poly.len() >= 3 {
            self.fill(&poly, class);
        }
    }
    pub fn dot(&mut self, x: f64, y: f64, r: f64, class: &str) {
        let (px, py) = (self.x(x), self.y(y));
        if px.is_finite() && py.is_finite() {
            self.s.push_str(&format!("<circle class=\"{class}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r:.1}\"/>"));
        }
    }
    /// A circle of a radius in data units across (an equal-scale plot).
    pub fn ring(&mut self, x: f64, y: f64, r: f64, class: &str, dashed: bool) {
        let (px, py) = (self.x(x), self.y(y));
        let rp = (self.x(x + r) - px).abs();
        if px.is_finite() && py.is_finite() && rp.is_finite() {
            self.s.push_str(&format!("<circle class=\"{class}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{rp:.1}\"{}/>",
                                     if dashed { " stroke-dasharray=\"5 4\"" } else { "" }));
        }
    }
    pub fn seg(&mut self, a: (f64, f64), b: (f64, f64), class: &str) {
        self.path(&[a, b], class);
    }
    pub fn dseg(&mut self, a: (f64, f64), b: (f64, f64), class: &str) {
        self.dpath(&[a, b], class);
    }
    /// A vertical line across the plot at x.
    pub fn vline(&mut self, x: f64, class: &str, dashed: bool) {
        let px = self.x(x);
        if px.is_finite() && (X0 - 0.5..=X1 + 0.5).contains(&px) {
            self.s.push_str(&format!("<path class=\"{class}\"{} d=\"M{px:.1},{Y0} L{px:.1},{Y1}\"/>",
                                     if dashed { " stroke-dasharray=\"5 4\"" } else { "" }));
        }
    }
    /// A horizontal line across the plot at y.
    pub fn hline(&mut self, y: f64, class: &str, dashed: bool) {
        let py = self.y(y);
        if py.is_finite() && (Y0 - 0.5..=Y1 + 0.5).contains(&py) {
            self.s.push_str(&format!("<path class=\"{class}\"{} d=\"M{X0},{py:.1} L{X1},{py:.1}\"/>",
                                     if dashed { " stroke-dasharray=\"5 4\"" } else { "" }));
        }
    }
    /// A rectangle between two data corners.
    pub fn rect(&mut self, a: (f64, f64), b: (f64, f64), class: &str) {
        let (x0, x1) = (self.x(a.0), self.x(b.0));
        let (y0, y1) = (self.y(a.1), self.y(b.1));
        if [x0, x1, y0, y1].iter().all(|v| v.is_finite()) {
            self.s.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                     x0.min(x1), y0.min(y1), (x1 - x0).abs().max(0.8), (y1 - y0).abs().max(0.8)));
        }
    }
    /// An arrow from a to b (data coordinates).
    pub fn arrow(&mut self, a: (f64, f64), b: (f64, f64), class: &str) {
        let (ax, ay, bx, by) = (self.x(a.0), self.y(a.1), self.x(b.0), self.y(b.1));
        let l = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
        if !(l > 2.0) || !l.is_finite() {
            return;
        }
        let (ux, uy) = ((bx - ax) / l, (by - ay) / l);
        let (nx, ny) = (-uy, ux);
        let (hx, hy) = (bx - ux * 9.0, by - uy * 9.0);
        self.s.push_str(&format!("<path class=\"{class}\" d=\"M{ax:.1},{ay:.1} L{hx:.1},{hy:.1}\"/>\
                                  <path class=\"axp-kd\" d=\"M{bx:.1},{by:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                                 hx + nx * 4.5, hy + ny * 4.5, hx - nx * 4.5, hy - ny * 4.5));
    }
    /// Text at pixel coordinates, moved inside the canvas when it would leave it.
    pub fn text(&mut self, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
        if t.is_empty() || !x.is_finite() || !y.is_finite() {
            return;
        }
        let w = width(t, class);
        let left = match anchor {
            "middle" => x - w / 2.0,
            "end" => x - w,
            _ => x,
        };
        let shift = if left < 2.0 { 2.0 - left } else if left + w > 518.0 { 518.0 - (left + w) } else { 0.0 };
        let y = y.clamp(14.0, self.h - 4.0);
        self.s.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                 x + shift, esc(t)));
    }
    /// Text at a data point, offset by (dx, dy) pixels.
    pub fn label(&mut self, x: f64, y: f64, dx: f64, dy: f64, anchor: &str, class: &str, t: &str) {
        let (px, py) = (self.x(x), self.y(y));
        self.text(px + dx, py + dy, anchor, class, t);
    }
    /// A label beside the pixel x, on the first of `rows` lines (y0, y0 + step, ...) where it overlaps no label placed
    /// before: right of x when it fits, else left of it.
    pub fn place(&mut self, px: f64, y0: f64, step: f64, rows: usize, class: &str, t: &str) {
        if t.is_empty() || !px.is_finite() {
            return;
        }
        let w = width(t, class);
        let mut first: Option<(f64, f64, &str)> = None;
        for r in 0..rows.max(1) {
            let y = y0 + step * r as f64;
            for (l, anchor) in [(px + 5.0, "start"), (px - 5.0 - w, "end")] {
                if l < 2.0 || l + w > 518.0 {
                    continue;
                }
                let free = self.occ.iter().all(|o| (o.0 - y).abs() > 12.0 || l + w + 12.0 < o.1 || l > o.2 + 12.0);
                if first.is_none() {
                    first = Some((y, l, anchor));
                }
                if free {
                    self.occ.push((y, l, l + w));
                    let x = if anchor == "start" { l } else { l + w };
                    self.text(x, y, anchor, class, t);
                    return;
                }
            }
        }
        let (y, l, anchor) = first.unwrap_or((y0, px + 5.0, "start"));
        self.occ.push((y, l, l + w));
        self.text(if anchor == "start" { l } else { l + w }, y, anchor, class, t);
    }
    /// A label centred on the pixel x on the first free line of `rows` (as `place`).
    pub fn place_mid(&mut self, px: f64, y0: f64, step: f64, rows: usize, class: &str, t: &str) {
        let w = width(t, class);
        let l = (px - w / 2.0).clamp(2.0, 518.0 - w);
        for r in 0..rows.max(1) {
            let y = y0 + step * r as f64;
            if self.occ.iter().all(|o| (o.0 - y).abs() > 12.0 || l + w + 12.0 < o.1 || l > o.2 + 12.0) || r + 1 == rows.max(1) {
                self.occ.push((y, l, l + w));
                self.text(l + w / 2.0, y, "middle", class, t);
                return;
            }
        }
    }
    /// A legend in one row above the plot, at its right end: (name, fill class).
    pub fn legend_row(&mut self, items: &[(String, &str)]) {
        let mut x = X1;
        for (name, class) in items.iter().rev() {
            let w = width(name, "axp-lt");
            x -= w;
            self.text(x, 39.0, "start", "axp-lt", name);
            x -= 18.0;
            self.s.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"30\" width=\"13\" height=\"9\"/>"));
            x -= 12.0;
        }
    }
    /// A legend in the top right corner of the plot: (name, fill class).
    pub fn legend(&mut self, items: &[(String, &str)]) {
        self.legend_at(items, false);
    }
    /// The legend in the top left corner when `left`.
    pub fn legend_at(&mut self, items: &[(String, &str)], left: bool) {
        let items: Vec<&(String, &str)> = items.iter().filter(|i| !i.0.is_empty()).take(6).collect();
        if items.is_empty() {
            return;
        }
        let w = items.iter().map(|i| width(&clip(&i.0, 34), "axp-lt")).fold(0.0, f64::max) + 30.0;
        let x = if left { X0 + 6.0 } else { X1 - 6.0 - w };
        let h = 8.0 + 17.0 * items.len() as f64;
        self.s.push_str(&format!("<rect class=\"axp-ka\" x=\"{x:.1}\" y=\"{:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>",
                                 Y0 + 6.0));
        for (k, (name, class)) in items.iter().enumerate() {
            let y = Y0 + 22.0 + 17.0 * k as f64;
            self.s.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"9\"/>",
                                     x + 7.0, y - 9.0));
            self.text(x + 26.0, y, "start", "axp-lt", &clip(name, 34));
        }
    }
    pub fn body(self) -> String {
        self.s
    }
}

/// A curve's value at x (linear between its points; None outside them).
pub fn interp(pts: &[(f64, f64)], x: f64) -> Option<f64> {
    for w in pts.windows(2) {
        let ((x0, y0), (x1, y1)) = (w[0], w[1]);
        if x >= x0.min(x1) && x <= x0.max(x1) {
            if (x1 - x0).abs() < 1e-300 {
                return Some(y0);
            }
            return Some(y0 + (y1 - y0) * (x - x0) / (x1 - x0));
        }
    }
    None
}

/// "a, b, c" of numbers at 4 digits (at most `n`, then "...").
pub fn list(v: &[f64], n: usize) -> String {
    let mut s: Vec<String> = v.iter().take(n).map(|x| num(*x)).collect();
    if v.len() > n {
        s.push("...".into());
    }
    s.join(", ")
}

/// A list as the inputs read it ("1, 2, 3"), every digit kept: for solving the model again.
pub fn text_list(v: &[f64]) -> String {
    v.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(", ")
}

/// A matrix as text ("[[1, 2], [3, 4]]"), every digit kept.
pub fn text_mat(m: &[Vec<f64>]) -> String {
    format!("[{}]", m.iter().map(|r| format!("[{}]", text_list(r))).collect::<Vec<_>>().join(", "))
}
