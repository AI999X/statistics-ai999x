//! Statistics' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! densities from the model's own quantiles or cumulative probabilities, stems from its probability runs, box plots
//! from its five-number summaries, intervals from its runs at five confidence levels, control charts, survival
//! curves. A figure that cannot be drawn for the inputs is left out (docs/FIGURE_PARITY.md).

use crate::figures_more as more;
use crate::plot::{self as pl, list, Plot, BAND, X0, X1, Y0};
use ax_engine::figkit::{num, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input, Val};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // a test statistic on its null density
        "bartlett_test" => vec![more::boxes(s), null_density(s, Dist::Chi(s.out("df").unwrap_or(f64::NAN)), "statistic", "B", Alt::Upper)],
        "chi_square_goodness_of_fit" => vec![more::gof_bars(s), chi_null(s, "chi_square")],
        "chi_square_independence" => vec![chi_null(s, "chi_square")],
        "chi_square_variance_test" => vec![null_density(s, Dist::Chi(s.out("df").unwrap_or(f64::NAN)), "chi_square", "χ²", alt(s))],
        "correlation_t_test" => vec![t_null(s)],
        "f_test_variances" => vec![more::boxes(s), f_null(s, "f", "df1", "df2", "F", alt(s))],
        "friedman_test" => vec![more::matrix(s, "data", "Friedman test: the blocks"), chi_null(s, "chi_square")],
        "kruskal_wallis" => vec![more::boxes(s), null_density(s, Dist::Chi(s.out("df").unwrap_or(f64::NAN)), "h", "H", Alt::Upper)],
        "levene_test" => vec![more::boxes(s), f_null(s, "w", "df1", "df2", "W", Alt::Upper)],
        "log_rank_test" => vec![chi1_null(s), more::survival_two(s)],
        "mcnemar_test" => vec![more::matrix(s, "table", "McNemar's test: the paired table"), chi1_null(s)],
        "one_way_anova" => vec![more::boxes(s), f_null(s, "f", "df_between", "df_within", "F", Alt::Upper)],
        "paired_t_test" => vec![t_null(s), more::paired(s)],
        "proportion_z_test" => vec![z_null(s)],
        "randomized_block_anova" => vec![more::matrix(s, "data", "Randomized block design: the data"),
                                         f_null(s, "f_treatment", "df_treatment", "df_error", "F", Alt::Upper)],
        "t_test_summary_statistics" | "t_test_two_sample_pooled" | "welch_t_test" => {
            let b = if s.key == "t_test_summary_statistics" { None } else { more::boxes(s) };
            vec![b, t_null(s)]
        }
        "two_proportion_z_test" | "z_test_mean" => vec![z_null(s)],
        "t_test_one_sample" => vec![t_null(s), dots_t_one(s)],
        // densities
        "chi_square_critical_value" => vec![crit_chi(s)],
        "f_critical_value" => vec![crit_f(s)],
        "t_critical_value" => vec![crit_sym(s, Dist::T(s.inp("df").unwrap_or(f64::NAN)))],
        "z_critical_value" => vec![crit_sym(s, Dist::Z)],
        "exponential_probability_above" | "exponential_probability_below" => vec![exp_density(s)],
        "inverse_normal" | "normal_probability_above" | "normal_probability_below" | "normal_probability_between"
        | "z_score" | "process_capability" => vec![normal_density(s)],
        "sample_mean_probability_above" | "sample_mean_probability_below" => vec![sampling_density(s)],
        // stems
        "binomial_at_least" | "binomial_at_most" | "binomial_exactly" | "binomial_mean_sd" => vec![binomial_stems(s)],
        "poisson_at_least" | "poisson_at_most" | "poisson_exactly" => vec![poisson_stems(s)],
        "geometric_probability" => vec![geometric_stems(s)],
        "hypergeometric_probability" => vec![hyper_stems(s)],
        "negative_binomial_probability" => vec![negbin_stems(s)],
        "discrete_random_variable" => vec![discrete_stems(s)],
        "weighted_mean" => vec![weighted_stems(s)],
        "fisher_exact_test" => vec![more::matrix(s, "table", "Fisher's exact test: the 2 × 2 table"), fisher_stems(s)],
        "sign_test" => vec![more::differences(s), sign_stems(s)],
        // number lines
        "mean" | "median" | "range" | "coefficient_of_variation" | "population_standard_deviation"
        | "sample_standard_deviation" | "standard_error_of_mean" | "mean_absolute_deviation"
        | "median_absolute_deviation" | "geometric_mean" | "harmonic_mean" | "skewness_kurtosis" | "trimmed_mean" => {
            vec![dots(s)]
        }
        "ci_mean_t" => vec![dots_ci(s), more::intervals(s)],
        _ => more::live(s),
    };
    v.into_iter().flatten().map(|f| with_answer(s, f)).collect()
}

/// The answer's first number (its value, else the asked output's first entry, else its first number), stated in every
/// figure: a figure whose text does not hold it gets "Returned: ..." at the end of its caption.
fn with_answer(s: &Solved, mut f: Figure) -> Figure {
    let target = s.a.target.clone().or(s.a.solved_for.clone()).unwrap_or_default();
    let first = |v: &serde_json::Value| -> Option<f64> {
        match v {
            serde_json::Value::Number(n) => n.as_f64(),
            serde_json::Value::Array(a) => a.first().and_then(|x| x.as_f64()),
            _ => None,
        }
    };
    let (name, value, shown) = if let Some(v) = s.a.value.filter(|v| v.is_finite()) {
        (target.clone(), v, num(v))
    } else if let Some(e) = s.a.values.iter().find(|e| e.0 == target && first(&e.1).is_some()) {
        let v = first(&e.1).unwrap();
        (e.0.clone(), v, match &e.1 { serde_json::Value::Array(a) => format!("({})", a.iter().take(8)
            .map(|x| x.as_f64().map(num).unwrap_or_default()).collect::<Vec<_>>().join(", ")), _ => num(v) })
    } else if let Some(e) = s.a.values.iter().find(|e| e.1.as_f64().is_some()) {
        let v = e.1.as_f64().unwrap();
        (e.0.clone(), v, num(v))
    } else if let Some(e) = s.a.values.iter().find(|e| e.1.is_array() && first(&e.1).is_some()) {
        let v = first(&e.1).unwrap();
        let a = e.1.as_array().unwrap();
        (e.0.clone(), v, format!("({})", a.iter().take(8).map(|x| x.as_f64().map(num).unwrap_or_default()).collect::<Vec<_>>().join(", ")))
    } else {
        return f;
    };
    if !value.is_finite() || f.svg.contains(&num(value)) {
        return f;
    }
    let add = format!(" Returned: {} = {}.", name.replace('_', " "), shown);
    let esc = add.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    // before the visitor's inputs, which end every caption
    if f.caption.contains(" Your inputs:") && f.svg.contains(" Your inputs:") {
        f.caption = f.caption.replacen(" Your inputs:", &format!("{add} Your inputs:"), 1);
        f.svg = f.svg.replacen(" Your inputs:", &format!("{esc} Your inputs:"), 1);
    } else {
        f.caption.push_str(&add);
        f.svg = f.svg.replacen("</desc>", &format!("{esc}</desc>"), 1);
    }
    f
}

// ------------------------------------------------------------------------------------------------ small helpers
pub fn fl(x: f64) -> Input {
    Input::Float(x)
}
pub fn tx(x: &str) -> Input {
    Input::Text(x.to_string())
}
/// One number output of another capability solved with these inputs.
pub fn o1(s: &Solved, key: &str, set: &[(&str, Input)], out: &str) -> Option<f64> {
    let a = s.solve_other(key, set)?;
    ax_engine::figkit::num_of(&a, out)
}
/// A number input (a real or a whole number) as read.
pub fn nin(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        Val::Int(i) => Some(i as f64),
        Val::Num(x) if x.is_finite() => Some(x),
        _ => None,
    })
}
/// A number of an answer.
pub fn ans(a: &Answer, sym: &str) -> Option<f64> {
    ax_engine::figkit::num_of(a, sym)
}
/// The alternative of a test as given (two-sided when not given).
fn alt(s: &Solved) -> Alt {
    match s.txt("alternative").as_deref() {
        Some("greater") => Alt::Upper,
        Some("less") => Alt::Lower,
        _ => Alt::Two,
    }
}

// ------------------------------------------------------------------------------------------------ distributions
#[derive(Clone, Copy, PartialEq)]
pub enum Dist {
    Z,
    T(f64),
    Chi(f64),
    F(f64, f64),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Alt {
    Upper,
    Lower,
    Two,
}

impl Dist {
    fn ok(&self) -> bool {
        match self {
            Dist::Z => true,
            Dist::T(d) | Dist::Chi(d) => d.is_finite() && *d > 0.0,
            Dist::F(a, b) => a.is_finite() && b.is_finite() && *a > 0.0 && *b > 0.0,
        }
    }
    fn symmetric(&self) -> bool {
        matches!(self, Dist::Z | Dist::T(_))
    }
    pub fn words(&self) -> String {
        match self {
            Dist::Z => "the standard normal density".into(),
            Dist::T(d) => format!("the t density with {} degrees of freedom", num(*d)),
            Dist::Chi(d) => format!("the chi-square density with {} degrees of freedom", num(*d)),
            Dist::F(a, b) => format!("the F density with {} and {} degrees of freedom", num(*a), num(*b)),
        }
    }
}

thread_local! {
    /// Quantiles already solved (a distribution's quantile never changes): the figures of one request, and the next
    /// request's, reuse them instead of solving again.
    static QC: std::cell::RefCell<Vec<(Dist, f64, f64)>> = std::cell::RefCell::new(vec![]);
}

fn cached(d: Dist, p: f64) -> Option<f64> {
    QC.with(|c| c.borrow().iter().find(|e| e.0 == d && (e.1 - p).abs() < 1e-13).map(|e| e.2))
}

fn remember(d: Dist, p: f64, x: f64) {
    QC.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 4000 {
            c.clear();
        }
        c.push((d, p, x));
    });
}

/// The p-quantile of a distribution, from the model's own critical-value capability (upper tail area 1 - p; the
/// chi-square capability returns both tails of one area at once).
pub fn quantile(s: &Solved, d: Dist, p: f64) -> Option<f64> {
    if let Some(x) = cached(d, p) {
        return Some(x);
    }
    let a = 1.0 - p;
    let x = match d {
        Dist::Z => o1(s, "z_critical_value", &[("alpha", fl(a)), ("tails", tx("right-tailed"))], "critical_value")?,
        Dist::T(df) => o1(s, "t_critical_value", &[("df", fl(df)), ("alpha", fl(a)), ("tails", tx("right-tailed"))],
                          "critical_value")?,
        Dist::Chi(df) => {
            let al = p.min(1.0 - p);
            let r = s.solve_other("chi_square_critical_value", &[("df", fl(df)), ("alpha", fl(al))])?;
            let (up, lo) = (ans(&r, "critical_value")?, ans(&r, "left_critical_value")?);
            remember(d, 1.0 - al, up);
            remember(d, al, lo);
            return Some(if p <= 0.5 { lo } else { up });
        }
        Dist::F(d1, d2) => o1(s, "f_critical_value", &[("df1", fl(d1)), ("df2", fl(d2)), ("alpha", fl(a))],
                              "critical_value")?,
    };
    remember(d, p, x);
    Some(x)
}

/// The cumulative probabilities at which a density's quantiles are solved (symmetric pairs: the chi-square
/// capability gives both of a pair in one solve; 0.025, 0.05, 0.95, 0.975 serve the critical values too).
const NODES: [f64; 17] = [0.001, 0.005, 0.025, 0.05, 0.1, 0.17, 0.25, 0.35, 0.5, 0.65, 0.75, 0.83, 0.9, 0.95, 0.975, 0.995,
                          0.999];

// the number of quantiles the last density was drawn from
thread_local!(static NQ: std::cell::Cell<usize> = const { std::cell::Cell::new(0) });

/// A density from the model's quantiles at a few cumulative probabilities: the cumulative probability joined through
/// them by a shape-preserving (monotone) cubic and differentiated; (x, density) points. More tail quantiles while
/// the curve does not reach `reach` (a statistic far out).
pub fn quantile_density(s: &Solved, d: Dist, reach: &[f64]) -> Option<Vec<(f64, f64)>> {
    if !d.ok() {
        return None;
    }
    let mut pts: Vec<(f64, f64)> = vec![];
    for p in NODES {
        if let Some(x) = quantile(s, d, p) {
            pts.push((p, x));
        }
    }
    let hi = reach.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let lo = reach.iter().cloned().fold(f64::INFINITY, f64::min);
    for t in [0.9999, 0.99999, 0.999999] {
        if pts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max) >= hi {
            break;
        }
        if let Some(x) = quantile(s, d, t) {
            pts.push((t, x));
        }
    }
    if d.symmetric() {
        for t in [0.0001, 0.00001, 0.000001] {
            if pts.iter().map(|p| p.1).fold(f64::INFINITY, f64::min) <= lo {
                break;
            }
            if let Some(x) = quantile(s, d, t) {
                pts.push((t, x));
            }
        }
    }
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    pts.dedup_by(|a, b| a.1 <= b.1);                  // strictly increasing x
    NQ.with(|n| n.set(pts.len()));
    pchip_density(&pts)
}

/// The density through the model's quantiles: at each quantile the slope of the cumulative probability (the
/// three-point derivative over its neighbours), joined by a monotone cubic through those densities.
fn pchip_density(pts: &[(f64, f64)]) -> Option<Vec<(f64, f64)>> {
    let n = pts.len();
    if n < 8 {
        return None;
    }
    let (x, y): (Vec<f64>, Vec<f64>) = (pts.iter().map(|p| p.1).collect(), pts.iter().map(|p| p.0).collect());
    let h: Vec<f64> = (0..n - 1).map(|k| x[k + 1] - x[k]).collect();
    let dl: Vec<f64> = (0..n - 1).map(|k| (y[k + 1] - y[k]) / h[k]).collect();
    if h.iter().any(|v| !(*v > 0.0)) || dl.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let mut dens = vec![0.0; n];
    dens[0] = dl[0];
    dens[n - 1] = dl[n - 2];
    for k in 1..n - 1 {
        dens[k] = (h[k - 1] * dl[k] + h[k] * dl[k - 1]) / (h[k - 1] + h[k]);
    }
    let dens: Vec<f64> = dens.iter().map(|v| v.max(0.0)).collect();
    let out = monotone_cubic(&x, &dens, 10);
    out.iter().all(|p| p.1.is_finite()).then_some(out)
}

/// Points of the monotone (Fritsch-Carlson) cubic through (x, y), `per` samples in each interval.
fn monotone_cubic(x: &[f64], y: &[f64], per: usize) -> Vec<(f64, f64)> {
    let n = x.len();
    let h: Vec<f64> = (0..n - 1).map(|k| x[k + 1] - x[k]).collect();
    let dl: Vec<f64> = (0..n - 1).map(|k| (y[k + 1] - y[k]) / h[k]).collect();
    let mut m = vec![0.0; n];
    m[0] = dl[0];
    m[n - 1] = dl[n - 2];
    for k in 1..n - 1 {
        if dl[k - 1] * dl[k] > 0.0 {
            let (w1, w2) = (2.0 * h[k] + h[k - 1], h[k] + 2.0 * h[k - 1]);
            m[k] = (w1 + w2) / (w1 / dl[k - 1] + w2 / dl[k]);
        }
    }
    let mut out = vec![];
    for k in 0..n - 1 {
        for i in 0..per {
            let t = i as f64 / per as f64;
            let (t2, t3) = (t * t, t * t * t);
            let v = (2.0 * t3 - 3.0 * t2 + 1.0) * y[k] + (t3 - 2.0 * t2 + t) * h[k] * m[k] + (-2.0 * t3 + 3.0 * t2) * y[k + 1]
                + (t3 - t2) * h[k] * m[k + 1];
            out.push((x[k] + t * h[k], v.max(0.0)));
        }
    }
    out.push((x[n - 1], y[n - 1]));
    out
}

/// How many quantiles the last density was drawn from (for its caption).
fn nq() -> usize {
    NQ.with(|n| n.get())
}

/// (cumulative probability, x) pairs -> (x, density) at midpoints, by differencing.
pub fn density_from_cdf(mut pts: Vec<(f64, f64)>) -> Option<Vec<(f64, f64)>> {
    pts.retain(|p| p.0.is_finite() && p.1.is_finite());
    pts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut out = vec![];
    for w in pts.windows(2) {
        let (dp, dx) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
        if dx > 0.0 && dp >= 0.0 {
            out.push(((w[0].1 + w[1].1) / 2.0, dp / dx));
        }
    }
    (out.len() >= 20).then_some(out)
}

/// A density from the model's cumulative probabilities on a grid of x: `key` solved with `fixed` and each x.
pub fn cdf_density(s: &Solved, key: &str, fixed: &[(&str, Input)], xs: &[f64]) -> Option<Vec<(f64, f64)>> {
    let mut pts = vec![];
    for x in xs {
        let mut set: Vec<(&str, Input)> = fixed.to_vec();
        set.push(("x", fl(*x)));
        if let Some(p) = o1(s, key, &set, "probability") {
            pts.push((p, *x));
        }
    }
    density_from_cdf(pts)
}

/// A shaded area under the density between a and b.
pub struct Area {
    pub a: f64,
    pub b: f64,
    pub class: &'static str,
}

/// A marked value: a vertical line with its label at the top of the plot.
pub struct Mark {
    pub x: f64,
    pub label: String,
    pub accent: bool,
}

/// The vertical range of a density: its bulk (a density that rises without bound at 0 is clipped at the frame).
fn dens_top(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len();
    let core: Vec<f64> = pts[n / 40..n - n / 40].iter().map(|p| p.1).collect();
    let (_, m) = pl::minmax(&core);
    // a density that is largest at its left end (rising without bound at 0): its height 2 % of the way across
    let at = core.iter().position(|v| *v == m).unwrap_or(0);
    let (x0, x1) = (pts[0].0, pts[n - 1].0);
    let m = if at < 3 && core.len() > 10 {
        let past: Vec<f64> = pts.iter().filter(|p| p.0 >= x0 + 0.02 * (x1 - x0)).map(|p| p.1).collect();
        pl::minmax(&past).1 * 1.3
    } else {
        m
    };
    if m.is_finite() && m > 0.0 { m * 1.12 } else { 1.0 }
}

/// A density chart: the curve, shaded areas, marked values (labels staggered), an optional second dashed curve.
pub fn density_chart(pts: &[(f64, f64)], areas: &[Area], marks: &[Mark], other: Option<&[(f64, f64)]>, xlab: &str,
                     header: &str) -> String {
    density_chart_l(pts, areas, marks, other, xlab, header, None)
}

/// `density_chart` with a legend row above the plot.
pub fn density_chart_l(pts: &[(f64, f64)], areas: &[Area], marks: &[Mark], other: Option<&[(f64, f64)]>, xlab: &str,
                       header: &str, legend: Option<&[(String, &str)]>) -> String {
    let mut xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    xs.extend(marks.iter().map(|m| m.x));
    let (x0, x1) = pl::minmax(&xs);
    let mut p = Plot::from_zero((x0, x1), dens_top(pts), true);
    p.clip_on();
    for a in areas {
        let (lo, hi) = (a.a.max(pts[0].0), a.b.min(pts[pts.len() - 1].0));
        if hi > lo {
            p.under(pts, lo, hi, 0.0, a.class);
        }
    }
    if let Some(o) = other {
        p.dpath(o, "axp-c4");
    }
    p.path(pts, "axp-c");
    p.unclip();
    let mut sorted: Vec<&Mark> = marks.iter().collect();
    sorted.sort_by_key(|m| !m.accent);
    for m in &sorted {
        p.vline(m.x, if m.accent { "axp-c3" } else { "axp-c4" }, !m.accent);
    }
    for m in sorted {
        let px = p.x(m.x);
        p.place(px, Y0 + 16.0, 17.0, 5, if m.accent { "axp-mt" } else { "axp-vt" }, &m.label);
    }
    if let Some(l) = legend {
        p.legend_row(l);
    }
    p.frame(xlab, "density", header);
    p.body()
}

/// A test statistic on its null density: the rejection region for α = 0.05 (the model's critical values) and, paler,
/// the area beyond the statistic that the p-value measures.
fn null_density(s: &Solved, d: Dist, stat_sym: &str, name: &str, side: Alt) -> Option<Figure> {
    null_density_k(s, d, stat_sym, name, side, "null-density")
}

fn null_density_k(s: &Solved, d: Dist, stat_sym: &str, name: &str, side: Alt, kind: &str) -> Option<Figure> {
    let stat = s.out(stat_sym)?;
    let pv = s.out("p_value");
    if !d.ok() {
        return None;
    }
    let pts = quantile_density(s, d, &[stat, -stat.abs()])?;
    let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
    let (mut areas, mut marks) = (vec![], vec![]);
    let region: String;
    match side {
        Alt::Upper => {
            let c = quantile(s, d, 0.95)?;
            areas.push(Area { a: c, b: hi, class: "axp-s" });
            areas.push(Area { a: stat, b: hi, class: "axp-s3" });
            marks.push(Mark { x: c, label: num(c), accent: false });
            region = format!("the upper rejection region above {} for α = 0.05", num(c));
        }
        Alt::Lower => {
            let c = quantile(s, d, 0.05)?;
            areas.push(Area { a: lo, b: c, class: "axp-s" });
            areas.push(Area { a: lo, b: stat, class: "axp-s3" });
            marks.push(Mark { x: c, label: num(c), accent: false });
            region = format!("the lower rejection region below {} for α = 0.05", num(c));
        }
        Alt::Two => {
            let (c0, c1) = (quantile(s, d, 0.025)?, quantile(s, d, 0.975)?);
            areas.push(Area { a: lo, b: c0, class: "axp-s" });
            areas.push(Area { a: c1, b: hi, class: "axp-s" });
            if d.symmetric() {
                areas.push(Area { a: stat.abs(), b: hi, class: "axp-s3" });
                areas.push(Area { a: lo, b: -stat.abs(), class: "axp-s3" });
            } else {
                let med = quantile(s, d, 0.5)?;
                if stat >= med {
                    areas.push(Area { a: stat, b: hi, class: "axp-s3" });
                } else {
                    areas.push(Area { a: lo, b: stat, class: "axp-s3" });
                }
            }
            marks.push(Mark { x: c0, label: num(c0), accent: false });
            marks.push(Mark { x: c1, label: num(c1), accent: false });
            region = format!("the two-sided rejection region below {} and above {} for α = 0.05", num(c0), num(c1));
        }
    }
    marks.push(Mark { x: stat, label: format!("{name} = {}", num(stat)), accent: true });
    let header = match pv {
        Some(p) => format!("{name} = {}, p-value = {}", num(stat), num(p)),
        None => format!("{name} = {}", num(stat)),
    };
    let legend = [("rejection region, α = 0.05".to_string(), "axp-s"), ("p-value area".to_string(), "axp-s3")];
    let body = density_chart_l(&pts, &areas, &marks, None, &format!("{name} under the null hypothesis"), &header,
                               Some(&legend));
    let title = format!("{name} on {}", d.words().trim_start_matches("the "));
    let caption = format!("Drawn: {} (the model's own quantiles at {} cumulative probabilities, differentiated and joined by a monotone cubic); shaded: \
                           {region} (critical values from the model) and, paler, the area beyond the statistic that the \
                           p-value measures. Marked: {name} = {}{}.", d.words(), nq(), num(stat),
                          pv.map(|p| format!(", p-value = {}", num(p))).unwrap_or_default());
    Some(s.chart(kind, &title, &caption, &body))
}

fn chi_null(s: &Solved, sym: &str) -> Option<Figure> {
    null_density(s, Dist::Chi(s.out("df")?), sym, "χ²", Alt::Upper)
}

fn t_null(s: &Solved) -> Option<Figure> {
    let kind = if s.key == "t_test_one_sample" { "density" } else { "null-density" };
    null_density_k(s, Dist::T(s.out("df")?), "t", "t", alt(s), kind)
}

fn z_null(s: &Solved) -> Option<Figure> {
    null_density(s, Dist::Z, "z", "z", alt(s))
}

fn f_null(s: &Solved, sym: &str, d1: &str, d2: &str, name: &str, side: Alt) -> Option<Figure> {
    null_density(s, Dist::F(s.out(d1)?, s.out(d2)?), sym, name, side)
}

/// A chi-square statistic whose degrees of freedom are not returned (two groups, a 2 x 2 table): drawn on one degree
/// of freedom only when the model's own chi-square quantile at the returned p-value gives back the statistic.
fn chi1_null(s: &Solved) -> Option<Figure> {
    let (x, p) = (s.out("chi_square")?, s.out("p_value")?);
    if !(p > 1e-12 && p < 1.0) {
        return None;
    }
    let back = quantile(s, Dist::Chi(1.0), 1.0 - p)?;
    if (back - x).abs() > 1e-6 * x.abs().max(1.0) {
        return None;
    }
    null_density(s, Dist::Chi(1.0), "chi_square", "χ²", Alt::Upper)
}

// ------------------------------------------------------------------------------------------------ density kinds
fn crit_chi(s: &Solved) -> Option<Figure> {
    let (df, a) = (s.inp("df")?, s.inp("alpha")?);
    let (c, l) = (s.out("critical_value")?, s.out("left_critical_value")?);
    let d = Dist::Chi(df);
    let pts = quantile_density(s, d, &[c])?;
    let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
    let areas = [Area { a: c, b: hi, class: "axp-s" }, Area { a: lo, b: l, class: "axp-s4" }];
    let marks = [Mark { x: c, label: format!("upper {}", num(c)), accent: true },
                 Mark { x: l, label: format!("lower {}", num(l)), accent: false }];
    let body = density_chart(&pts, &areas, &marks, None, "χ²", &format!("df = {}, α = {}", num(df), num(a)));
    Some(s.chart("density", "Chi-square critical values",
                 &format!("{} (from the model's quantiles, differentiated and joined by a monotone cubic); shaded: the upper area α = {} beyond the \
                           returned critical value {} and, paler, the lower area α below the returned left critical \
                           value {}.", cap1(&d.words()), num(a), num(c), num(l)), &body))
}

fn crit_f(s: &Solved) -> Option<Figure> {
    let (d1, d2, a) = (s.inp("df1")?, s.inp("df2")?, s.inp("alpha")?);
    let c = s.out("critical_value")?;
    let d = Dist::F(d1, d2);
    let pts = quantile_density(s, d, &[c])?;
    let hi = pts[pts.len() - 1].0;
    let body = density_chart(&pts, &[Area { a: c, b: hi, class: "axp-s" }],
                             &[Mark { x: c, label: format!("F = {}", num(c)), accent: true }], None, "F",
                             &format!("upper area α = {}", num(a)));
    Some(s.chart("density", "F critical value",
                 &format!("{} (from the model's quantiles, differentiated and joined by a monotone cubic); shaded: the upper area α = {} beyond the \
                           returned critical value {}.", cap1(&d.words()), num(a), num(c)), &body))
}

/// The t or z critical value and its tail areas.
fn crit_sym(s: &Solved, d: Dist) -> Option<Figure> {
    let a = s.inp("alpha")?;
    let c = s.out("critical_value")?;
    let tails = s.txt("tails")?;
    let pts = quantile_density(s, d, &[c.abs(), -c.abs()])?;
    let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
    let (areas, marks, what) = match tails.as_str() {
        "two-tailed" => (vec![Area { a: c.abs(), b: hi, class: "axp-s" }, Area { a: lo, b: -c.abs(), class: "axp-s" }],
                         vec![Mark { x: c.abs(), label: format!("+{}", num(c.abs())), accent: true },
                              Mark { x: -c.abs(), label: format!("-{}", num(c.abs())), accent: true }],
                         format!("the two-tailed area α = {} beyond ±{}", num(a), num(c.abs()))),
        "left-tailed" => (vec![Area { a: lo, b: c, class: "axp-s" }],
                          vec![Mark { x: c, label: num(c), accent: true }],
                          format!("the lower area α = {} below {}", num(a), num(c))),
        _ => (vec![Area { a: c, b: hi, class: "axp-s" }], vec![Mark { x: c, label: num(c), accent: true }],
              format!("the upper area α = {} beyond {}", num(a), num(c))),
    };
    let name = if d == Dist::Z { "z" } else { "t" };
    let body = density_chart(&pts, &areas, &marks, None, name, &format!("{tails}, α = {}", num(a)));
    Some(s.chart("density", &format!("{name} critical value and its tail area"),
                 &format!("{} (from the model's quantiles, differentiated and joined by a monotone cubic); shaded: {what}, the returned critical value.",
                          cap1(&d.words())), &body))
}

fn cap1(t: &str) -> String {
    let mut c = t.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

fn exp_density(s: &Solved) -> Option<Figure> {
    let x = s.inp("x")?;
    let p = s.out("probability")?;
    let (rate, mean) = (s.inp("rate"), s.inp("mean"));
    let scale = match (rate, mean) {
        (Some(r), _) if r > 0.0 => 1.0 / r,
        (_, Some(m)) if m > 0.0 => m,
        _ => return None,
    };
    let top = (5.0 * scale).max(x * 1.25);
    let xs = pl::nice_ticks(0.0, 1.0, 1);
    let _ = xs;
    let grid: Vec<f64> = (0..=90).map(|i| top * i as f64 / 90.0).collect();
    let mut fixed: Vec<(&str, Input)> = vec![];
    if let Some(r) = rate {
        fixed.push(("rate", fl(r)));
    } else if let Some(m) = mean {
        fixed.push(("mean", fl(m)));
    }
    let pts = cdf_density(s, "exponential_probability_below", &fixed, &grid)?;
    let above = s.key.ends_with("above");
    let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
    let area = if above { Area { a: x, b: hi, class: "axp-s" } } else { Area { a: lo, b: x, class: "axp-s" } };
    let what = if above { format!("P(X > {}) = {}", num(x), num(p)) } else { format!("P(X < {}) = {}", num(x), num(p)) };
    let body = density_chart(&pts, &[area], &[Mark { x, label: format!("x = {}", num(x)), accent: true }], None, "x", &what);
    let param = match rate { Some(r) => format!("rate {}", num(r)), None => format!("mean {}", num(scale)) };
    Some(s.chart("density", &format!("Exponential probability {} a value", if above { "above" } else { "below" }),
                 &format!("The exponential density with {param} (from the model's cumulative probabilities on 91 points, \
                           differenced); shaded: {what} as returned."), &body))
}

/// Normal densities: probabilities above, below, between; the inverse; a z-score; process capability.
fn normal_density(s: &Solved) -> Option<Figure> {
    let (mu, sigma) = (s.inp("mu")?, s.inp("sigma")?);
    if !(sigma > 0.0) {
        return None;
    }
    let mut keep: Vec<f64> = vec![mu - 4.0 * sigma, mu + 4.0 * sigma];
    for k in ["x", "a", "b", "lsl", "usl"] {
        if let Some(v) = s.get(k) {
            keep.push(v);
        }
    }
    let (lo, hi) = pl::minmax(&keep);
    let (lo, hi) = (lo.min(mu - 4.0 * sigma), hi.max(mu + 4.0 * sigma));
    let grid: Vec<f64> = (0..=100).map(|i| lo + (hi - lo) * i as f64 / 100.0).collect();
    let pts = cdf_density(s, "normal_probability_below", &[("mu", fl(mu)), ("sigma", fl(sigma))], &grid)?;
    let (plo, phi) = (pts[0].0, pts[pts.len() - 1].0);
    let src = format!("The normal density with μ = {}, σ = {} (from the model's cumulative probabilities on 101 points, \
                       differenced)", num(mu), num(sigma));
    let (areas, marks, header, title, caption): (Vec<Area>, Vec<Mark>, String, &str, String) = match s.key {
        "normal_probability_above" => {
            let (x, p, z) = (s.inp("x")?, s.out("probability")?, s.out("z")?);
            (vec![Area { a: x, b: phi, class: "axp-s" }], vec![Mark { x, label: format!("x = {}, z = {}", num(x), num(z)), accent: true }],
             format!("P(X > {}) = {}", num(x), num(p)), "Normal probability above a value",
             format!("{src}; shaded: P(X > {}) = {} as returned; marked: x = {}, z = {}.", num(x), num(p), num(x), num(z)))
        }
        "normal_probability_below" => {
            let (x, p, z) = (s.inp("x")?, s.out("probability")?, s.out("z")?);
            (vec![Area { a: plo, b: x, class: "axp-s" }], vec![Mark { x, label: format!("x = {}, z = {}", num(x), num(z)), accent: true }],
             format!("P(X < {}) = {}", num(x), num(p)), "Normal probability below a value",
             format!("{src}; shaded: P(X < {}) = {} as returned; marked: x = {}, z = {}.", num(x), num(p), num(x), num(z)))
        }
        "normal_probability_between" => {
            let (a, b, p) = (s.inp("a")?, s.inp("b")?, s.out("probability")?);
            let (za, zb) = (s.out("z_a")?, s.out("z_b")?);
            (vec![Area { a, b, class: "axp-s" }],
             vec![Mark { x: a, label: format!("z = {}", num(za)), accent: true }, Mark { x: b, label: format!("z = {}", num(zb)), accent: true }],
             format!("P({} < X < {}) = {}", num(a), num(b), num(p)), "Normal probability between two values",
             format!("{src}; shaded: P({} < X < {}) = {} as returned, between z = {} and z = {}.", num(a), num(b), num(p),
                     num(za), num(zb)))
        }
        "inverse_normal" => {
            let (pp, x, z) = (s.inp("p")?, s.out("x")?, s.out("z")?);
            (vec![Area { a: plo, b: x, class: "axp-s" }], vec![Mark { x, label: format!("x = {}", num(x)), accent: true }],
             format!("p = {}: x = {}, z = {}", num(pp), num(x), num(z)), "The value with a given lower-tail probability",
             format!("{src}; shaded: the area p = {} below the returned x = {} (z = {}).", num(pp), num(x), num(z)))
        }
        "z_score" => {
            let (x, z) = (s.inp("x")?, s.out("z")?);
            let mut m = vec![Mark { x, label: format!("x = {}, z = {}", num(x), num(z)), accent: true }];
            for k in [-2.0, -1.0, 1.0, 2.0] {
                m.push(Mark { x: mu + k * sigma, label: format!("{}σ", if k > 0.0 { format!("+{}", k) } else { num(k) }), accent: false });
            }
            (vec![], m, format!("z = {}", num(z)), "A z-score on the normal curve",
             format!("{src}, with lines one and two standard deviations from the mean; marked: x = {} and the returned \
                      z = {}, its distance from μ in standard deviations.", num(x), num(z)))
        }
        "process_capability" => {
            let (lsl, usl) = (s.inp("lsl")?, s.inp("usl")?);
            let (cp, cpk, ppm) = (s.out("cp")?, s.out("cpk")?, s.out("ppm_out")?);
            (vec![Area { a: plo, b: lsl, class: "axp-s" }, Area { a: usl, b: phi, class: "axp-s" }],
             vec![Mark { x: lsl, label: format!("LSL {}", num(lsl)), accent: true }, Mark { x: usl, label: format!("USL {}", num(usl)), accent: true }],
             format!("Cp = {}, Cpk = {}", num(cp), num(cpk)), "Process spread against specification limits",
             format!("{src}, with the out-of-specification tails beyond LSL = {} and USL = {} shaded. Returned: Cp = {}, \
                      Cpk = {}, {} parts per million out.", num(lsl), num(usl), num(cp), num(cpk), num(ppm)))
        }
        _ => return None,
    };
    let body = density_chart(&pts, &areas, &marks, None, "x", &header);
    Some(s.chart("density", title, &caption, &body))
}

/// The sampling distribution of the mean beside the density of one observation.
fn sampling_density(s: &Solved) -> Option<Figure> {
    let (x, mu, sigma) = (s.inp("x")?, s.inp("mu")?, s.inp("sigma")?);
    let (p, z, se) = (s.out("probability")?, s.out("z")?, s.out("standard_error")?);
    if !(sigma > 0.0 && se > 0.0) {
        return None;
    }
    let lo = (mu - 4.0 * sigma).min(x);
    let hi = (mu + 4.0 * sigma).max(x);
    let wide: Vec<f64> = (0..=60).map(|i| lo + (hi - lo) * i as f64 / 60.0).collect();
    let (nlo, nhi) = ((mu - 4.0 * se).max(lo), (mu + 4.0 * se).min(hi));
    let narrow: Vec<f64> = (0..=80).map(|i| nlo + (nhi - nlo) * i as f64 / 80.0).collect();
    let one = cdf_density(s, "normal_probability_below", &[("mu", fl(mu)), ("sigma", fl(sigma))], &wide)?;
    let mean = cdf_density(s, "normal_probability_below", &[("mu", fl(mu)), ("sigma", fl(se))], &narrow)?;
    let above = s.key.ends_with("above");
    let (mlo, mhi) = (mean[0].0, mean[mean.len() - 1].0);
    let area = if above { Area { a: x, b: mhi, class: "axp-s" } } else { Area { a: mlo, b: x, class: "axp-s" } };
    let what = format!("P(x̄ {} {}) = {}", if above { ">" } else { "<" }, num(x), num(p));
    let body = density_chart(&mean, &[area], &[Mark { x, label: format!("z = {}", num(z)), accent: true }], Some(&one), "x̄", &what);
    Some(s.chart("density", "Sampling distribution of the mean",
                 &format!("The distribution of the mean (μ = {}, standard error {} as returned), from the model's normal \
                           cumulative probabilities, differenced, beside the wider density of one observation (σ = {}, \
                           dashed). Shaded: {what}; marked: z = {}.", num(mu), num(se), num(sigma), num(z)), &body))
}

// ------------------------------------------------------------------------------------------------ stems
/// Stems: (k, probability) with the highlighted ones in the first colour; `mark`: a vertical line and its label.
pub fn stems_chart(pts: &[(f64, f64)], hl: &dyn Fn(f64) -> bool, marks: &[(f64, String)], xlab: &str, header: &str,
                   dashed: &[f64]) -> String {
    let xs: Vec<f64> = pts.iter().map(|p| p.0).chain(marks.iter().map(|m| m.0)).collect();
    let (x0, x1) = pl::minmax(&xs);
    let ymax = pts.iter().map(|p| p.1).fold(0.0, f64::max);
    let mut p = Plot::from_zero((x0 - 0.7, x1 + 0.7), ymax, false);
    for d in dashed {
        p.vline(*d, "axp-c4", true);
    }
    let nh = pts.iter().filter(|q| hl(q.0)).count();
    let few = pts.len() <= 16 && nh <= 6;
    for (x, l) in marks {
        p.vline(*x, "axp-c3", false);
        let px = p.x(*x);
        p.place(px, Y0 + 16.0, 17.0, 4, "axp-mt", l);
    }
    for (k, v) in pts {
        let on = hl(*k);
        p.seg((*k, 0.0), (*k, *v), if on { "axp-c" } else { "axp-c4" });
        p.dot(*k, *v, 4.0, if on { "axp-d" } else { "axp-d4" });
        if few && on && *v > 0.03 * ymax {
            let (px, py) = (p.x(*k), p.y(*v));
            p.place_mid(px, py - 8.0, -14.0, 2, "axp-lt", &num(*v));
        }
    }
    p.frame(xlab, "probability", header);
    p.body()
}

/// A run of the model over a range of whole numbers: (k, probability).
fn runs(s: &Solved, key: &str, fixed: &[(&str, Input)], var: &str, ks: impl Iterator<Item = i64>) -> Vec<(f64, f64)> {
    let mut out = vec![];
    for k in ks {
        let mut set = fixed.to_vec();
        set.push((var, Input::Int(k)));
        if let Some(v) = o1(s, key, &set, "probability") {
            out.push((k as f64, v));
        }
    }
    out
}

fn binomial_stems(s: &Solved) -> Option<Figure> {
    let (n, pr) = (nin(s, "n")?, s.inp("p")?);
    if !(n >= 1.0) || !(0.0..=1.0).contains(&pr) {
        return None;
    }
    let n = n as i64;
    // the window: every k when n is small, else the mean ± 5 standard deviations from the model's own moments
    let (a, b) = if n <= 40 { (0, n) } else {
        let m = s.solve_other("binomial_mean_sd", &[("n", Input::Int(n)), ("p", fl(pr))])?;
        let (mean, sd) = (ans(&m, "mean")?, ans(&m, "standard_deviation")?);
        (((mean - 5.0 * sd).floor() as i64).max(0), ((mean + 5.0 * sd).ceil() as i64).min(n))
    };
    let k0 = nin(s, "k");
    if let Some(k) = k0 {
        if (k as i64) < a || (k as i64) > b {
            return None;
        }
    }
    let pts = runs(s, "binomial_exactly", &[("n", Input::Int(n)), ("p", fl(pr))], "k", a..=b);
    if pts.len() < 2 {
        return None;
    }
    let base = format!("n = {}, p = {}", n, num(pr));
    let (hl, marks, header, title, what, dashed): (Box<dyn Fn(f64) -> bool>, Vec<(f64, String)>, String, String, String, Vec<f64>) =
        match s.key {
            "binomial_mean_sd" => {
                let (m, sd, var) = (s.out("mean")?, s.out("standard_deviation")?, s.out("variance")?);
                (Box::new(|_| true), vec![(m, format!("mean {}", num(m)))], format!("{base}: mean {}, sd {}", num(m), num(sd)),
                 "Binomial mean and spread".into(),
                 format!("marked: the returned mean {} and, dashed, one returned standard deviation {} either side \
                          (variance {})", num(m), num(sd), num(var)), vec![m - sd, m + sd])
            }
            key => {
                let k = k0?;
                let pv = s.out("probability")?;
                let (rel, f): (&str, Box<dyn Fn(f64) -> bool>) = match key {
                    "binomial_at_least" => ("≥", Box::new(move |x| x >= k)),
                    "binomial_at_most" => ("≤", Box::new(move |x| x <= k)),
                    _ => ("=", Box::new(move |x| x == k)),
                };
                (f, vec![], format!("{base}: P(X {rel} {}) = {}", num(k), num(pv)), format!("Binomial distribution: k {rel} {}", num(k)),
                 format!("the stems with k {rel} {} are in the first colour and carry the returned probability {}", num(k), num(pv)),
                 vec![])
            }
        };
    let body = stems_chart(&pts, &*hl, &marks, "k (successes)", &header, &dashed);
    Some(s.chart("stems", &title,
                 &format!("The binomial distribution with n = {n} trials and p = {}: P(X = k) for k = {} to {}, each from \
                           the model's binomial run; {what}.", num(pr), a, b), &body))
}

fn poisson_stems(s: &Solved) -> Option<Figure> {
    let (lam, k) = (s.inp("lam")?, nin(s, "k")?);
    let pv = s.out("probability")?;
    if !(lam > 0.0) {
        return None;
    }
    let hi = ((lam + 4.5 * lam.sqrt() + 3.0).ceil() as i64).max(k as i64 + 2);
    let lo = if hi > 60 { ((lam - 4.5 * lam.sqrt()).floor() as i64).max(0).min(k as i64) } else { 0 };
    if hi - lo > 80 {
        return None;
    }
    let pts = runs(s, "poisson_exactly", &[("lam", fl(lam))], "k", lo..=hi);
    if pts.len() < 2 {
        return None;
    }
    let (rel, hl): (&str, Box<dyn Fn(f64) -> bool>) = match s.key {
        "poisson_at_least" => ("≥", Box::new(move |x| x >= k)),
        "poisson_at_most" => ("≤", Box::new(move |x| x <= k)),
        _ => ("=", Box::new(move |x| x == k)),
    };
    let body = stems_chart(&pts, &*hl, &[], "k (count)", &format!("λ = {}: P(X {rel} {}) = {}", num(lam), num(k), num(pv)), &[]);
    Some(s.chart("stems", &format!("Poisson distribution: k {rel} {}", num(k)),
                 &format!("The Poisson distribution with mean λ = {}: P(X = k) for k = {lo} to {hi}, each from the model's \
                           Poisson run; the stems with k {rel} {} are in the first colour and carry the returned probability \
                           {}.", num(lam), num(k), num(pv)), &body))
}

fn geometric_stems(s: &Solved) -> Option<Figure> {
    let (pr, k) = (s.inp("p")?, nin(s, "k")?);
    let (pv, cum) = (s.out("probability")?, s.out("cumulative")?);
    if !(pr > 0.0 && pr <= 1.0) || k < 1.0 {
        return None;
    }
    // trials until 99 % of the probability is shown (at most 40), and past the visitor's k
    let mut pts = vec![];
    let mut total = 0.0;
    let mut j = 1;
    while j <= 40 && (total < 0.99 || (j as f64) <= k + 1.0) {
        if let Some(v) = o1(s, "geometric_probability", &[("p", fl(pr)), ("k", Input::Int(j))], "probability") {
            total += v;
            pts.push((j as f64, v));
        }
        j += 1;
    }
    if pts.len() < 2 || k > 40.0 {
        return None;
    }
    let body = stems_chart(&pts, &|x| x <= k, &[(k, format!("k = {}: {}", num(k), num(pv)))], "trial of the first success",
                           &format!("p = {}: P(X ≤ {}) = {}", num(pr), num(k), num(cum)), &[]);
    Some(s.chart("stems", "Geometric distribution: waiting for a success",
                 &format!("The geometric distribution with success probability p = {}: P(first success on trial k) for \
                           k = 1 to {}, each from the model; marked: k = {}, P = {}; the stems k ≤ {} carry the returned \
                           cumulative probability {}.", num(pr), pts.len(), num(k), num(pv), num(k), num(cum)), &body))
}

fn hyper_stems(s: &Solved) -> Option<Figure> {
    let (nn, kk, n, k) = (nin(s, "N")? as i64, nin(s, "K")? as i64, nin(s, "n")? as i64, nin(s, "k")?);
    let (pv, cum) = (s.out("probability")?, s.out("cumulative")?);
    let (a, b) = ((n - (nn - kk)).max(0), n.min(kk));
    if b < a || b - a > 80 {
        return None;
    }
    let pts = runs(s, "hypergeometric_probability", &[("N", Input::Int(nn)), ("K", Input::Int(kk)), ("n", Input::Int(n))], "k", a..=b);
    if pts.len() < 2 {
        return None;
    }
    let body = stems_chart(&pts, &|x| x <= k, &[(k, format!("k = {}: {}", num(k), num(pv)))], "k (successes in the sample)",
                           &format!("P(X ≤ {}) = {}", num(k), num(cum)), &[]);
    Some(s.chart("stems", "Hypergeometric distribution",
                 &format!("A sample of n = {n} drawn without replacement from N = {nn} items of which K = {kk} are \
                           successes: P(X = k) for every possible k ({a} to {b}), each from the model. Marked: k = {}, P = {}; \
                           the stems k ≤ {} carry the returned cumulative {}.", num(k), num(pv), num(k), num(cum)), &body))
}

fn negbin_stems(s: &Solved) -> Option<Figure> {
    let (r, pr, n) = (nin(s, "r")? as i64, s.inp("p")?, nin(s, "n")?);
    let pv = s.out("probability")?;
    if r < 1 || !(pr > 0.0 && pr <= 1.0) {
        return None;
    }
    let mut pts = vec![];
    let mut total = 0.0;
    let mut j = r;
    while j < r + 60 && (total < 0.99 || (j as f64) <= n + 1.0) {
        if let Some(v) = o1(s, "negative_binomial_probability", &[("r", Input::Int(r)), ("p", fl(pr)), ("n", Input::Int(j))], "probability") {
            total += v;
            pts.push((j as f64, v));
        }
        j += 1;
    }
    if pts.len() < 2 || n > pts[pts.len() - 1].0 {
        return None;
    }
    let body = stems_chart(&pts, &|x| x == n, &[], "trial of the r-th success", &format!("r = {r}, p = {}: P(n = {}) = {}", num(pr), num(n), num(pv)), &[]);
    Some(s.chart("stems", "Negative binomial: waiting for the r-th success",
                 &format!("The negative binomial distribution of the trial on which success number r = {r} occurs (p = {}), \
                           for n = {r} to {}, each from the model; the visitor's n = {} has the returned probability {}.",
                          num(pr), pts[pts.len() - 1].0, num(n), num(pv)), &body))
}

fn discrete_stems(s: &Solved) -> Option<Figure> {
    let (x, pr) = (s.vec("x")?, s.vec("p")?);
    if x.len() != pr.len() || x.len() > 60 {
        return None;
    }
    let (m, sd, var) = (s.out("expected_value")?, s.out("standard_deviation")?, s.out("variance")?);
    let pts: Vec<(f64, f64)> = x.iter().cloned().zip(pr.iter().cloned()).collect();
    let body = stems_chart(&pts, &|_| true, &[(m, format!("E[X] = {}", num(m)))], "x", &format!("E[X] = {}, sd = {}", num(m), num(sd)),
                           &[m - sd, m + sd]);
    Some(s.chart("stems", "A discrete distribution, its mean and spread",
                 &format!("The distribution x = {} with probabilities {}; marked: the returned expected value {} (the \
                           balance point) and, dashed, one returned standard deviation {} either side (variance {}).",
                          list(&x, 12), list(&pr, 12), num(m), num(sd), num(var)), &body))
}

fn weighted_stems(s: &Solved) -> Option<Figure> {
    let (x, w) = (s.vec("x")?, s.vec("w")?);
    if x.len() != w.len() || x.len() > 60 {
        return None;
    }
    let (wm, tw) = (s.out("weighted_mean")?, s.out("total_weight")?);
    let plain = o1(s, "mean", &[("x", tx(&pl::text_list(&x)))], "mean")?;
    let pts: Vec<(f64, f64)> = x.iter().cloned().zip(w.iter().cloned()).collect();
    let xs: Vec<f64> = x.iter().cloned().chain([wm, plain]).collect();
    let (x0, x1) = pl::minmax(&xs);
    let ymax = w.iter().cloned().fold(0.0, f64::max);
    let mut p = Plot::from_zero((x0, x1), ymax, true);
    for (xv, wv) in &pts {
        p.seg((*xv, 0.0), (*xv, *wv), "axp-c");
        p.dot(*xv, *wv, 4.0, "axp-d");
    }
    p.vline(wm, "axp-c3", false);
    p.vline(plain, "axp-c4", true);
    let (a, b) = (p.x(wm), p.x(plain));
    let wr = a >= b;
    p.text(if wr { a + 5.0 } else { a - 5.0 }, Y0 + 16.0, if wr { "start" } else { "end" }, "axp-mt", &format!("weighted {}", num(wm)));
    p.text(if wr { b - 5.0 } else { b + 5.0 }, Y0 + 34.0, if wr { "end" } else { "start" }, "axp-vt", &format!("unweighted {}", num(plain)));
    p.frame("value", "weight", &format!("total weight {}", num(tw)));
    Some(s.chart("stems", "Weighted mean pulled by the weights",
                 &format!("The values {} with weights {} (total weight {}) as stems; marked: the returned weighted mean {}, \
                           pulled towards the heavily weighted values, and the unweighted mean {} from the model's mean.",
                          list(&x, 12), list(&w, 12), num(tw), num(wm), num(plain)), &p.body()))
}

fn fisher_stems(s: &Solved) -> Option<Figure> {
    let t = s.mat("table")?;
    if t.len() != 2 || t[0].len() != 2 || t.iter().flatten().any(|v| *v < 0.0 || v.fract() != 0.0) {
        return None;
    }
    let (a, b, c, d) = (t[0][0] as i64, t[0][1] as i64, t[1][0] as i64, t[1][1] as i64);
    let (r1, c1, nn) = (a + b, a + c, a + b + c + d);
    let (pv, or) = (s.out("p_value")?, s.out("odds_ratio"));
    let (lo, hi) = ((r1 + c1 - nn).max(0), r1.min(c1));
    if hi - lo > 80 || hi < lo {
        return None;
    }
    let pts = runs(s, "hypergeometric_probability", &[("N", Input::Int(nn)), ("K", Input::Int(c1)), ("n", Input::Int(r1))], "k", lo..=hi);
    let pobs = pts.iter().find(|p| p.0 == a as f64)?.1;
    let alt = s.txt("alternative").unwrap_or_else(|| "two-sided".into());
    let af = a as f64;
    let hl: Box<dyn Fn(f64) -> bool> = match alt.as_str() {
        "greater" => Box::new(move |k| k >= af),
        "less" => Box::new(move |k| k <= af),
        _ => {
            let probs = pts.clone();
            Box::new(move |k| probs.iter().any(|p| p.0 == k && p.1 <= pobs * (1.0 + 1e-7)))
        }
    };
    let body = stems_chart(&pts, &*hl, &[(af, format!("observed {a}"))], "top-left count",
                           &format!("p-value = {}", num(pv)), &[]);
    Some(s.chart("stems", "Fisher's exact test: every table with these margins",
                 &format!("With the margins of the table fixed (first row {r1}, first column {c1}, {nn} in all, from the \
                           inputs), the top-left count is hypergeometric: each probability from the model's hypergeometric \
                           run. The tables counted by the {alt} p-value are in the first colour; the returned p-value is {}{}.",
                          num(pv), or.map(|o| format!(", odds ratio {}", num(o))).unwrap_or_default()), &body))
}

fn sign_stems(s: &Solved) -> Option<Figure> {
    let (pos, n, pv) = (s.out("positives")?, s.out("n")?, s.out("p_value")?);
    if !(n >= 1.0) || n > 80.0 {
        return None;
    }
    let nn = n as i64;
    let pts = runs(s, "binomial_exactly", &[("n", Input::Int(nn)), ("p", fl(0.5))], "k", 0..=nn);
    let alt = s.txt("alternative").unwrap_or_else(|| "two-sided".into());
    let half = n / 2.0;
    let hl: Box<dyn Fn(f64) -> bool> = match alt.as_str() {
        "greater" => Box::new(move |k| k >= pos),
        "less" => Box::new(move |k| k <= pos),
        _ => Box::new(move |k| (k - half).abs() >= (pos - half).abs() - 1e-9),
    };
    let body = stems_chart(&pts, &*hl, &[(pos, format!("observed {}", num(pos)))], "count of positive signs",
                           &format!("p-value = {}", num(pv)), &[]);
    Some(s.chart("stems", "Sign test: the null distribution of the count",
                 &format!("Under the null hypothesis the count of positive signs among the {nn} non-zero differences is \
                           binomial (n = {nn}, p = 1/2): each probability from the model's binomial run. The counts the \
                           {alt} p-value adds up are in the first colour; observed {}, returned p-value {}.", num(pos), num(pv)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ number lines
/// A dot plot on a number line: equal values stacked; returns the plot (vertical range 0..1) and each dot's height.
pub fn numline(data: &[f64], extra: &[f64]) -> (Plot, Vec<f64>) {
    let xs: Vec<f64> = data.iter().chain(extra.iter()).cloned().collect();
    let (x0, x1) = pl::minmax(&xs);
    let mut p = Plot::axes(pl::Ax::new(x0, x1, X0, X1, true), pl::Ax::new(0.0, 1.0, pl::Y1, Y0, false));
    let mut seen: Vec<(f64, usize)> = vec![];
    let tol = (x1 - x0).abs().max(1e-12) * 1e-9;
    let mut ys = vec![];
    let maxc = {
        let mut c: Vec<(f64, usize)> = vec![];
        for v in data {
            match c.iter_mut().find(|e| (e.0 - v).abs() <= tol) {
                Some(e) => e.1 += 1,
                None => c.push((*v, 1)),
            }
        }
        c.iter().map(|e| e.1).max().unwrap_or(1)
    };
    let step = (0.5 / maxc as f64).min(0.09);
    for v in data {
        let k = match seen.iter_mut().find(|e| (e.0 - v).abs() <= tol) {
            Some(e) => {
                e.1 += 1;
                e.1 - 1
            }
            None => {
                seen.push((*v, 1));
                0
            }
        };
        ys.push(0.22 + step * k as f64);
    }
    let _ = &mut p;
    (p, ys)
}

/// A marked value on a number line: the line and its label at the top (row k).
pub fn mark(p: &mut Plot, x: f64, label: &str, class: &str, dashed: bool, row: usize) {
    p.vline(x, class, dashed);
    let px = p.x(x);
    let _ = row;
    p.place(px, Y0 + 16.0, 17.0, 5, if class == "axp-c3" { "axp-mt" } else { "axp-vt" }, label);
}

/// Dashed lines either side of a centre, with a bracket label below.
fn spread(p: &mut Plot, c: f64, d: f64, label: &str, row: usize) {
    p.vline(c - d, "axp-c4", true);
    p.vline(c + d, "axp-c4", true);
    let y = 0.12 - 0.05 * row as f64;
    p.seg((c - d, y), (c + d, y), "axp-c4");
    let (px, py) = (p.x(c), p.y(y));
    p.place_mid(px, py + 15.0, 14.0, 1, "axp-lt", label);
}

fn data_dots(p: &mut Plot, data: &[f64], ys: &[f64], grey: &dyn Fn(usize) -> bool) {
    for (i, (v, y)) in data.iter().zip(ys).enumerate() {
        p.dot(*v, *y, 5.0, if grey(i) { "axp-d4" } else { "axp-d" });
    }
}

/// The descriptive statistics of one list on a number line.
fn dots(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    if x.is_empty() || x.len() > 400 {
        return None;
    }
    let xt = tx(&pl::text_list(&x));
    let mean_of = || o1(s, "mean", &[("x", xt.clone())], "mean");
    let shown = format!("x = {}", list(&x, 12));
    let (title, caption, header, body): (&str, String, String, String) = match s.key {
        "mean" => {
            let (m, n, sum) = (s.out("mean")?, s.out("n")?, s.out("sum")?);
            let (mut p, ys) = numline(&x, &[m]);
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
            p.frame_with("x", "", &format!("mean = {} / {} = {}", num(sum), num(n), num(m)), true, false);
            ("The mean as the balance point of the data",
             format!("The data {shown} as dots on a number line (equal values stacked); marked: the returned mean {} = sum {} / \
                      n {}, the point the dots balance on.", num(m), num(sum), num(n)), String::new(), p.body())
        }
        "median" => {
            let (m, n) = (s.out("median")?, s.out("n")?);
            let (mut p, ys) = numline(&x, &[m]);
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, m, &format!("median {}", num(m)), "axp-c3", false, 0);
            let below = x.iter().filter(|v| **v < m).count();
            let above = x.iter().filter(|v| **v > m).count();
            p.frame_with("x", "", &format!("{below} below, {above} above"), true, false);
            ("The median splits the data in half",
             format!("The data {shown} as dots on a number line; marked: the returned median {} of the n = {} values, with \
                      {below} values below and {above} above.", num(m), num(n)), String::new(), p.body())
        }
        "range" => {
            let (r, lo, hi) = (s.out("range")?, s.out("minimum")?, s.out("maximum")?);
            let (mut p, ys) = numline(&x, &[lo, hi]);
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, lo, &format!("min {}", num(lo)), "axp-c3", false, 0);
            mark(&mut p, hi, &format!("max {}", num(hi)), "axp-c3", false, 1);
            p.arrow(((lo + hi) / 2.0, 0.7), (hi, 0.7), "axp-k");
            p.arrow(((lo + hi) / 2.0, 0.7), (lo, 0.7), "axp-k");
            p.label((lo + hi) / 2.0, 0.7, 0.0, -8.0, "middle", "axp-bt", &format!("range {}", num(r)));
            p.frame_with("x", "", &format!("range = {}", num(r)), true, false);
            ("Range: from the smallest to the largest value",
             format!("The data {shown}; marked: the returned minimum {}, maximum {} and the range {} between them.", num(lo),
                     num(hi), num(r)), String::new(), p.body())
        }
        "coefficient_of_variation" | "population_standard_deviation" | "sample_standard_deviation" => {
            let (m, sd) = (s.out("mean")?, s.out("standard_deviation")?);
            let (mut p, ys) = numline(&x, &[m - sd, m + sd]);
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
            spread(&mut p, m, sd, &format!("± sd {}", num(sd)), 0);
            let (title, extra, head) = match s.key {
                "coefficient_of_variation" => {
                    let (cv, pc) = (s.out("cv")?, s.out("cv_percent")?);
                    ("Spread relative to the mean",
                     format!(" The coefficient of variation is their ratio: {} ({} %).", num(cv), num(pc)),
                     format!("cv = {} ({} %)", num(cv), num(pc)))
                }
                "population_standard_deviation" => ("Population standard deviation around the mean",
                                                    format!(" (population variance {})", num(s.out("variance")?)),
                                                    format!("population sd = {}", num(sd))),
                _ => ("Sample standard deviation around the mean", format!(" (sample variance {})", num(s.out("variance")?)),
                      format!("sample sd = {}", num(sd))),
            };
            p.frame_with("x", "", &head, true, false);
            (title, format!("The data {shown} (n = {}); marked: the returned mean {} and, dashed, one returned standard \
                             deviation {} either side.{extra}", x.len(), num(m), num(sd)), String::new(), p.body())
        }
        "standard_error_of_mean" => {
            let (se, sd) = (s.out("standard_error")?, s.out("standard_deviation")?);
            let m = mean_of()?;
            let (mut p, ys) = numline(&x, &[m - sd, m + sd]);
            p.rect((m - se, 0.0), (m + se, 1.0), "axp-s2");
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
            spread(&mut p, m, sd, &format!("± sd {}", num(sd)), 0);
            p.frame_with("x", "", &format!("standard error {}", num(se)), true, false);
            ("Standard error of the mean against the spread",
             format!("The data {shown} (n = {}); marked: the mean {} (from the model's mean); dashed: one returned standard \
                      deviation {} either side; shaded: one returned standard error {} either side, the uncertainty of the \
                      mean itself.", x.len(), num(m), num(sd), num(se)), String::new(), p.body())
        }
        "mean_absolute_deviation" | "median_absolute_deviation" => {
            let mean = s.key == "mean_absolute_deviation";
            let (c, d) = if mean { (s.out("mean")?, s.out("mean_absolute_deviation")?) } else {
                (s.out("median")?, s.out("median_absolute_deviation")?)
            };
            let (mut p, ys) = numline(&x, &[c - d, c + d]);
            for (v, y) in x.iter().zip(&ys) {
                p.seg((*v, *y), (c, *y), "axp-c4");
            }
            data_dots(&mut p, &x, &ys, &|_| false);
            let cn = if mean { "mean" } else { "median" };
            mark(&mut p, c, &format!("{cn} {}", num(c)), "axp-c3", false, 0);
            spread(&mut p, c, d, &format!("± {}", num(d)), 0);
            p.frame_with("x", "", &format!("{} absolute deviation = {}", if mean { "mean" } else { "median" }, num(d)), true, false);
            (if mean { "Mean absolute deviation: the average distance to the mean" } else {
                "Median absolute deviation: the median distance to the median"
            }, format!("The data {shown} with each value's distance to the returned {cn} {} drawn as a segment; the returned \
                        {} absolute deviation {} (dashed either side) is the {} of those distances.", num(c),
                       if mean { "mean" } else { "median" }, num(d), if mean { "average" } else { "median" }), String::new(), p.body())
        }
        "geometric_mean" | "harmonic_mean" => {
            let am = mean_of()?;
            let gm = if s.key == "geometric_mean" { s.out("geometric_mean")? } else { o1(s, "geometric_mean", &[("x", xt.clone())], "geometric_mean")? };
            let hm = if s.key == "harmonic_mean" { s.out("harmonic_mean")? } else { o1(s, "harmonic_mean", &[("x", xt.clone())], "harmonic_mean")? };
            let (mut p, ys) = numline(&x, &[am, gm, hm]);
            data_dots(&mut p, &x, &ys, &|_| false);
            let own_g = s.key == "geometric_mean";
            let marks = [(hm, "harmonic", !own_g), (gm, "geometric", own_g), (am, "arithmetic", false)];
            for (row, (v, name, own)) in marks.iter().enumerate() {
                mark(&mut p, *v, &format!("{name} {}", num(*v)), if *own { "axp-c3" } else { "axp-c4" }, !*own, row);
            }
            p.frame_with("x", "", "harmonic ≤ geometric ≤ arithmetic", true, false);
            (if own_g { "Geometric mean beside the other means" } else { "Harmonic mean beside the other means" },
             format!("The data {shown} with three means from the model: arithmetic {}, geometric {}, harmonic {}. The {} mean of \
                      this capability is the solid line: {}.", num(am), num(gm), num(hm), if own_g { "geometric" } else { "harmonic" },
                     num(if own_g { gm } else { hm })), String::new(), p.body())
        }
        "skewness_kurtosis" => {
            let (sk, ek) = (s.out("skewness")?, s.out("excess_kurtosis")?);
            let m = mean_of()?;
            let md = o1(s, "median", &[("x", xt.clone())], "median")?;
            let (mut p, ys) = numline(&x, &[m, md]);
            data_dots(&mut p, &x, &ys, &|_| false);
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
            mark(&mut p, md, &format!("median {}", num(md)), "axp-c4", true, 1);
            p.frame_with("x", "", &format!("skewness {}, excess kurtosis {}", num(sk), num(ek)), true, false);
            (if sk > 0.0 { "A right-skewed sample: mean beyond median" } else if sk < 0.0 { "A left-skewed sample: mean below median" } else { "A symmetric sample" },
             format!("The data {shown} with its mean {} and median {} (from the model). Returned: skewness {} (biased {}), \
                      excess kurtosis {} (biased {}).", num(m), num(md), num(sk), num(s.out("skewness_biased")?), num(ek),
                     num(s.out("excess_kurtosis_biased")?)), String::new(), p.body())
        }
        "trimmed_mean" => {
            let (tm, k) = (s.out("trimmed_mean")?, s.out("removed_each_end")?);
            let m = mean_of()?;
            let k = k as usize;
            if 2 * k >= x.len() + 1 {
                return None;
            }
            let mut order: Vec<usize> = (0..x.len()).collect();
            order.sort_by(|a, b| x[*a].partial_cmp(&x[*b]).unwrap_or(std::cmp::Ordering::Equal));
            let cut: Vec<usize> = order[..k].iter().chain(order[x.len() - k..].iter()).cloned().collect();
            let (mut p, ys) = numline(&x, &[m, tm]);
            data_dots(&mut p, &x, &ys, &|i| cut.contains(&i));
            mark(&mut p, tm, &format!("trimmed {}", num(tm)), "axp-c3", false, 0);
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c4", true, 1);
            p.frame_with("x", "", &format!("{k} value(s) cut at each end"), true, false);
            ("Trimmed mean ignores the extremes",
             format!("The data {shown} with the proportion {} cut from each end: the returned {k} value(s) at each end are \
                      grey. Marked: the returned trimmed mean {} and, dashed, the untrimmed mean {} from the model.",
                     num(s.inp("proportion")?), num(tm), num(m)), String::new(), p.body())
        }
        _ => return None,
    };
    let _ = header;
    Some(s.chart("dots", title, &caption, &body))
}

/// A one-sample t test: the sample against the hypothesised mean.
fn dots_t_one(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (mu0, m, sd, t, df, pv) = (s.inp("mu0")?, s.out("mean")?, s.out("standard_deviation")?, s.out("t")?, s.out("df")?, s.out("p_value")?);
    if x.len() > 400 {
        return None;
    }
    let (mut p, ys) = numline(&x, &[mu0, m]);
    data_dots(&mut p, &x, &ys, &|_| false);
    mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
    mark(&mut p, mu0, &format!("μ₀ = {}", num(mu0)), "axp-c4", true, 1);
    p.frame_with("x", "", &format!("t = {}, p-value = {}", num(t), num(pv)), true, false);
    Some(s.chart("dots", "A sample against its hypothesised mean",
                 &format!("The sample x = {} against the hypothesised mean μ₀ = {}: returned mean {}, standard deviation {}, \
                           t = {} with {} degrees of freedom, p-value {}.", list(&x, 12), num(mu0), num(m), num(sd), num(t),
                          num(df), num(pv)), &p.body()))
}

/// A t interval for the mean against the data.
fn dots_ci(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (lo, hi, m, me, cv, df) = (s.out("lower")?, s.out("upper")?, s.out("mean")?, s.out("margin_of_error")?,
                                   s.out("critical_value")?, s.out("df")?);
    let c = s.inp("confidence")?;
    if x.len() > 400 {
        return None;
    }
    let (mut p, ys) = numline(&x, &[lo, hi]);
    p.rect((lo, 0.0), (hi, 1.0), BAND[0]);
    data_dots(&mut p, &x, &ys, &|_| false);
    mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
    p.label(lo, 0.75, 4.0, 0.0, "start", "axp-vt", &num(lo));
    p.label(hi, 0.62, -4.0, 0.0, "end", "axp-vt", &num(hi));
    p.frame_with("x", "", &format!("{} % interval [{}, {}]", num(c * 100.0), num(lo), num(hi)), true, false);
    Some(s.chart("dots", "A t interval against the data",
                 &format!("The sample x = {} with the returned {} % t interval for the population mean shaded: [{}, {}] around \
                           the mean {} (margin {}, t critical value {}, {} degrees of freedom). The interval covers where μ \
                           plausibly is, not where the data are.", list(&x, 12), num(c * 100.0), num(lo), num(hi), num(m),
                          num(me), num(cv), num(df)), &p.body()))
}
