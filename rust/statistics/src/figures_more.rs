//! Statistics' own live figures, continued: box plots, intervals at five confidence levels, bars, control charts,
//! regression and correlation plots, ranks, survival curves, reliability block diagrams.

use crate::figures::{fl, nin, mark, o1, tx};
use crate::plot::{self as pl, list, text_list, text_mat, Plot, X0, X1, Y0, Y1};
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "cohens_d" => vec![boxes(s)],
        "ci_mean_known_sigma" | "ci_mean_t_summary" | "ci_proportion" | "ci_variance" => vec![intervals(s)],
        "ci_two_means" | "correlation_confidence_interval" => vec![intervals(s), xy(s)],
        "wilson_interval" => vec![wilson(s)],
        "spearman_correlation" => vec![scatter(s), xy(s)],
        "covariance" | "pearson_correlation" => vec![scatter(s)],
        "simple_linear_regression" => vec![fit(s), residuals(s)],
        "regression_prediction" | "multiple_linear_regression" => vec![fit(s)],
        "kendall_tau" => vec![kendall(s)],
        "mann_whitney_exact" | "mann_whitney_u" => vec![ranks(s)],
        "wilcoxon_signed_rank" | "wilcoxon_signed_rank_exact" => vec![differences(s)],
        "kaplan_meier" => vec![kaplan(s)],
        "c_chart" | "p_chart" | "individuals_moving_range_chart" | "xbar_r_chart" | "xbar_s_chart" => vec![control(s)],
        "series_system_reliability" | "parallel_system_reliability" => vec![blocks(s)],
        "weibull_reliability" => vec![weibull(s)],
        "factorial_2x2_effects" => vec![interaction(s)],
        "factorial_2x2x2_effects" | "frequency_table_statistics" | "mode" | "population_variance" | "sample_variance" => {
            vec![bars(s)]
        }
        "durbin_watson" => vec![durbin(s)],
        "five_number_summary" | "iqr_outliers" => vec![one_box(s)],
        "cronbach_alpha" => vec![matrix(s, "data", "Cronbach's alpha: the responses")],
        _ => vec![],
    }
}

/// Ranks of values (ties share their average rank), 1-based.
pub fn rank(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|a, b| v[*a].partial_cmp(&v[*b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for t in i..=j {
            r[idx[t]] = avg;
        }
        i = j + 1;
    }
    r
}

/// The groups of a comparison: g1 ... g5, or the samples x and y.
fn groups(s: &Solved) -> Vec<(String, Vec<f64>)> {
    let mut g = vec![];
    for n in ["g1", "g2", "g3", "g4", "g5"] {
        if let Some(v) = s.vec(n) {
            g.push((n.to_string(), v));
        }
    }
    if g.is_empty() {
        for n in ["x", "y"] {
            if let Some(v) = s.vec(n) {
                g.push((n.to_string(), v));
            }
        }
    }
    g
}

/// Box plots of the groups (quartiles from the model's five-number summaries) with their data, each labelled with a
/// statistic the model computes for it.
pub fn boxes(s: &Solved) -> Option<Figure> {
    let g = groups(s);
    if g.len() < 2 || g.iter().any(|x| x.1.len() > 400) {
        return None;
    }
    let (stat_key, stat_out, stat_name) = match s.key {
        "bartlett_test" | "f_test_variances" => ("sample_variance", "variance", "variance"),
        "levene_test" | "kruskal_wallis" | "one_way_anova" => ("median", "median", "median"),
        _ => ("mean", "mean", "mean"),
    };
    let mut rows = vec![];
    for (name, v) in &g {
        let t = tx(&text_list(v));
        let a = s.solve_other("five_number_summary", &[("x", t.clone())])?;
        let f5 = ["minimum", "q1", "median", "q3", "maximum"].map(|k| k::num_of(&a, k));
        if f5.iter().any(|x| x.is_none()) {
            return None;
        }
        let st = o1(s, stat_key, &[("x", t)], stat_out)?;
        rows.push((name.clone(), v.clone(), f5.map(|x| x.unwrap()), st));
    }
    let all: Vec<f64> = rows.iter().flat_map(|r| r.1.iter().cloned()).collect();
    let n = rows.len() as f64;
    let mut p = Plot::axes(pl::Ax::new(0.4, n + 0.6, X0, X1, false), {
        let (lo, hi) = pl::minmax(&all);
        pl::Ax::new(lo, hi, Y1, Y0 + 20.0, true)
    });
    for (i, (name, v, f5, st)) in rows.iter().enumerate() {
        let c = i as f64 + 1.0;
        let [mn, q1, md, q3, mx] = *f5;
        p.seg((c, mn), (c, q1), "axp-k");
        p.seg((c, q3), (c, mx), "axp-k");
        p.seg((c - 0.08, mn), (c + 0.08, mn), "axp-k");
        p.seg((c - 0.08, mx), (c + 0.08, mx), "axp-k");
        p.rect((c - 0.17, q1), (c + 0.17, q3), "axp-ka");
        p.rect((c - 0.17, q1), (c + 0.17, q3), "axp-f");
        p.seg((c - 0.17, md), (c + 0.17, md), "axp-c");
        for (j, x) in v.iter().enumerate() {
            p.dot(c + 0.26 + 0.035 * (j % 3) as f64, *x, 3.0, "axp-d3");
        }
        if stat_name == "mean" {
            p.dot(c, *st, 5.0, "axp-m");
        }
        let px = p.x(c);
        p.text(px, Y1 + 20.0, "middle", "axp-bt", name);
        p.text(px, Y0 + 12.0, "middle", "axp-vt", &format!("{stat_name} {}", num(*st)));
    }
    let res = test_words(s);
    p.frame_with("", "value", "", false, true);
    let shown = rows.iter().map(|r| format!("{} = {}", r.0, list(&r.1, 8))).collect::<Vec<_>>().join("; ");
    let stats = rows.iter().map(|r| format!("{} {}", r.0, num(r.3))).collect::<Vec<_>>().join(", ");
    let title = match stat_name {
        "variance" => "Groups and their spreads",
        "median" => "Groups compared by box plots",
        _ => "Samples and their means",
    };
    Some(s.chart("boxes", title,
                 &format!("Box plots (quartiles from the model's five-number summaries) and data of {} groups ({shown}), each \
                           labelled with its {stat_name} from the model: {stats}.{res}", rows.len()), &p.body()))
}

/// The test's returned statistic and p-value in words.
fn test_words(s: &Solved) -> String {
    let mut parts = vec![];
    for (k, name) in [("statistic", "statistic"), ("f", "F"), ("w", "W"), ("h", "H"), ("t", "t"), ("d", "Cohen's d"),
                      ("hedges_g", "Hedges' g"), ("df", "degrees of freedom"), ("df1", "df1"), ("df2", "df2"),
                      ("df_between", "df between"), ("df_within", "df within"), ("p_value", "p-value"),
                      ("eta_squared", "η²"), ("pooled_sd", "pooled sd")] {
        if let Some(v) = s.out(k) {
            parts.push(format!("{name} {}", num(v)));
        }
    }
    if parts.is_empty() { String::new() } else { format!(" Returned: {}.", parts.join(", ")) }
}

/// Observed against expected counts.
pub fn gof_bars(s: &Solved) -> Option<Figure> {
    let o = s.vec("observed")?;
    let e = s.out_list("expected")?;
    if o.len() != e.len() || o.len() > 30 {
        return None;
    }
    let labels: Vec<String> = (1..=o.len()).map(|i| i.to_string()).collect();
    let body = k::bar_groups(&labels, &[("observed".into(), o.iter().map(|v| Some(*v)).collect()),
                                         ("expected".into(), e.iter().map(|v| Some(*v)).collect())],
                             "category", "count", &format!("χ² = {}, p-value = {}", num(s.out("chi_square")?), num(s.out("p_value")?)));
    Some(s.chart("bars", "Observed against expected counts",
                 &format!("Observed counts {} beside the expected counts returned by the model ({}). χ² = {} with {} degrees of \
                           freedom, p-value {}.", list(&o, 12), list(&e, 12), num(s.out("chi_square")?), num(s.out("df")?),
                          num(s.out("p_value")?)), &body))
}

/// A matrix input as a heat map with its rows and columns numbered.
pub fn matrix(s: &Solved, sym: &str, title: &str) -> Option<Figure> {
    let m = s.mat(sym)?;
    if m.len() > 30 || m[0].len() > 30 || m.iter().any(|r| r.len() != m[0].len()) {
        return None;
    }
    let rows: Vec<String> = (1..=m.len()).map(|i| format!("row {i}")).collect();
    let cols: Vec<String> = (1..=m[0].len()).map(|i| format!("col {i}")).collect();
    let body = k::heat(&m, &rows, &cols, &format!("{} × {}", m.len(), m[0].len()));
    Some(s.chart("matrix", title,
                 &format!("The stated {sym} = {} as a heat map (each cell shaded by its value).{}", text_mat(&m), test_words(s)),
                 &body))
}

/// Confidence intervals at five levels (and the visitor's), each a run of the model.
pub fn intervals(s: &Solved) -> Option<Figure> {
    let c0 = s.inp("confidence")?;
    let mut levels = vec![0.5, 0.8, 0.9, 0.95, 0.99];
    if !levels.iter().any(|l| (l - c0).abs() < 1e-9) {
        levels.push(c0);
        levels.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    let mut rows = vec![];
    for l in &levels {
        if let Some(a) = s.with(&[("confidence", *l)]) {
            if let (Some(lo), Some(hi)) = (k::num_of(&a, "lower"), k::num_of(&a, "upper")) {
                rows.push((*l, lo, hi));
            }
        }
    }
    if rows.len() < 2 {
        return None;
    }
    let (centre, cname) = match s.key {
        "ci_mean_known_sigma" | "ci_mean_t_summary" => (s.inp("xbar")?, "x̄"),
        "ci_mean_t" => (s.out("mean")?, "mean"),
        "ci_proportion" => (s.out("proportion")?, "p̂"),
        "ci_two_means" => (s.out("difference")?, "difference"),
        "ci_variance" => (s.out("variance")?, "s²"),
        _ => (s.out("r")?, "r"),
    };
    let mut xs: Vec<f64> = rows.iter().flat_map(|r| [r.1, r.2]).collect();
    xs.push(centre);
    if s.key == "ci_two_means" {
        xs.push(0.0);
    }
    let (lo, hi) = pl::minmax(&xs);
    let nr = rows.len() as f64;
    let mut p = Plot::axes(pl::Ax::new(lo, hi, X0, X1, true), pl::Ax::new(0.4, nr + 0.6, Y1, Y0 + 14.0, false));
    p.vline(centre, "axp-c4", true);
    if s.key == "ci_two_means" {
        p.vline(0.0, "axp-c3", true);
    }
    for (i, (l, a, b)) in rows.iter().enumerate() {
        let y = i as f64 + 1.0;
        let mine = (l - c0).abs() < 1e-9;
        p.seg((*a, y), (*b, y), if mine { "axp-c" } else { "axp-c4" });
        p.seg((*a, y - 0.12), (*a, y + 0.12), if mine { "axp-c" } else { "axp-c4" });
        p.seg((*b, y - 0.12), (*b, y + 0.12), if mine { "axp-c" } else { "axp-c4" });
        p.dot(centre, y, 3.5, if mine { "axp-d" } else { "axp-d4" });
        p.text(X0 - 6.0, p.y(y) + 5.0, "end", if mine { "axp-bt" } else { "axp-lt" }, &format!("{} %", num(l * 100.0)));
        let t = format!("[{}, {}]", num(*a), num(*b));
        let cls = if mine { "axp-vt" } else { "axp-lt" };
        let (pa, pb, py) = (p.x(*a), p.x(*b), p.y(y));
        if pb + 8.0 + pl::width(&t, cls) <= 516.0 {
            p.text(pb + 8.0, py + 5.0, "start", cls, &t);
        } else if pa - 8.0 - pl::width(&t, cls) >= X0 {
            p.text(pa - 8.0, py + 5.0, "end", cls, &t);
        } else {
            p.text((pa + pb) / 2.0, py - 9.0, "middle", cls, &t);
        }
    }
    p.frame_with("value", "", &format!("{cname} = {}", num(centre)), true, false);
    let shown = rows.iter().map(|r| format!("{} % [{}, {}]", num(r.0 * 100.0), num(r.1), num(r.2))).collect::<Vec<_>>().join("; ");
    let (title, extra) = match s.key {
        "ci_variance" => ("Variance intervals are asymmetric", " They are not symmetric about s²: the chi-square distribution is skewed."),
        "ci_two_means" => ("Difference of two means: intervals against zero", " An interval that excludes 0 (dashed) means a difference at that level."),
        "correlation_confidence_interval" => ("Correlation intervals at several levels", " They are asymmetric about r and stay inside [-1, 1]."),
        "ci_proportion" => ("Proportion intervals at several levels", ""),
        _ => ("Confidence intervals at several levels", " Higher confidence, wider interval."),
    };
    Some(s.chart("intervals", title,
                 &format!("Confidence intervals at {} levels, each a run of the model: {shown}; centre {cname} = {}; the \
                           visitor's level {} % in the first colour.{extra}", rows.len(), num(centre), num(c0 * 100.0)), &p.body()))
}

/// Wilson intervals for every count beside the model's normal-approximation (Wald) interval.
fn wilson(s: &Solved) -> Option<Figure> {
    let (x0, n, c) = (nin(s, "x")?, nin(s, "n")?, s.inp("confidence")?);
    let (lo0, hi0) = (s.out("lower")?, s.out("upper")?);
    if !(n >= 1.0) || n > 60.0 {
        return None;
    }
    let nn = n as i64;
    let mut w = vec![];
    let mut wald = vec![];
    for x in 0..=nn {
        if let Some(a) = s.with_input(&[("x", Input::Int(x))]) {
            if let (Some(a1), Some(b1)) = (k::num_of(&a, "lower"), k::num_of(&a, "upper")) {
                w.push((x as f64, a1, b1));
            }
        }
        if let Some(a) = s.solve_other("ci_proportion", &[("x", Input::Int(x)), ("n", Input::Int(nn)), ("confidence", fl(c))]) {
            if let (Some(a1), Some(b1)) = (k::num_of(&a, "lower"), k::num_of(&a, "upper")) {
                wald.push((x as f64, a1, b1));
            }
        }
    }
    if w.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = w.iter().chain(wald.iter()).flat_map(|r| [r.1, r.2]).collect();
    let (ylo, yhi) = pl::minmax(&ys);
    let mut p = Plot::new((-0.5, n + 0.5), (ylo.min(0.0), yhi.max(1.0)));
    p.hline(0.0, "axp-c4", true);
    p.hline(1.0, "axp-c4", true);
    let off = (n / 120.0).max(0.08);
    for (x, a, b) in &wald {
        p.seg((x + off, *a), (x + off, *b), "axp-c4");
    }
    for (x, a, b) in &w {
        p.seg((x - off, *a), (x - off, *b), if *x == x0 { "axp-c3" } else { "axp-c" });
    }
    p.dot(x0 - off, lo0, 4.0, "axp-m");
    p.dot(x0 - off, hi0, 4.0, "axp-m");
    p.legend_at(&[("Wilson".into(), "axp-b"), ("normal approximation".into(), "axp-b4")], true);
    p.frame("count x", "proportion", &format!("x = {}: [{}, {}]", num(x0), num(lo0), num(hi0)));
    Some(s.chart("intervals", "Wilson against normal-approximation intervals for every count",
                 &format!("{} % intervals for a proportion for every count x = 0 to {nn} out of n = {nn}: Wilson (this capability) \
                           beside the normal-approximation interval of the model's ci_proportion. Near 0 and n the normal \
                           interval collapses or leaves [0, 1]; Wilson does not. Marked: the visitor's x = {}, [{}, {}].",
                          num(c * 100.0), num(x0), num(lo0), num(hi0)), &p.body()))
}

/// Paired data as given: y against x.
fn xy(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.len() < 2 || x.len() > 400 {
        return None;
    }
    let mut p = Plot::new(pl::minmax(&x), pl::minmax(&y));
    for (a, b) in x.iter().zip(&y) {
        p.dot(*a, *b, 4.5, "axp-d");
    }
    let mut ret = vec![];
    for k_ in ["difference", "lower", "upper", "r", "rho", "p_value", "df", "standard_error", "n"] {
        if let Some(v) = s.out(k_) {
            ret.push(format!("{k_} = {}", num(v)));
        }
    }
    p.frame("x", "y", &pl::clip(&ret.iter().take(2).cloned().collect::<Vec<_>>().join(", "), 46));
    Some(s.chart("xy", "The data and what was returned",
                 &format!("The data y against x as given ({} points). Returned: {}.", x.len(), ret.join(", ")), &p.body()))
}

/// Scatter plots: covariance as signed rectangles, Pearson with the least-squares line, Spearman on ranks.
fn scatter(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.len() < 2 || x.len() > 400 {
        return None;
    }
    match s.key {
        "covariance" => {
            let (sc, pc) = (s.out("sample_covariance")?, s.out("population_covariance")?);
            let mx = o1(s, "mean", &[("x", tx(&text_list(&x)))], "mean")?;
            let my = o1(s, "mean", &[("x", tx(&text_list(&y)))], "mean")?;
            let mut p = Plot::new(pl::minmax(&x), pl::minmax(&y));
            for (a, b) in x.iter().zip(&y) {
                let pos = (a - mx) * (b - my) >= 0.0;
                p.rect((mx, my), (*a, *b), if pos { "axp-s" } else { "axp-s3" });
            }
            p.vline(mx, "axp-c4", true);
            p.hline(my, "axp-c4", true);
            for (a, b) in x.iter().zip(&y) {
                p.dot(*a, *b, 4.5, "axp-d");
            }
            p.legend_at(&[("positive product".into(), "axp-s"), ("negative product".into(), "axp-s3")], true);
            p.frame("x", "y", &format!("sample covariance {}", num(sc)));
            Some(s.chart("scatter", "Covariance as signed rectangles",
                         &format!("The pairs x = {}, y = {} with the means of x and y (from the model's mean: {}, {}) as dashed \
                                   lines; each rectangle is a product (x - x̄)(y - ȳ), positive in the first colour, negative in \
                                   the second. Their average is the covariance: sample {}, population {}.", list(&x, 10),
                                  list(&y, 10), num(mx), num(my), num(sc), num(pc)), &p.body()))
        }
        "pearson_correlation" => {
            let (r, r2, pv) = (s.out("r")?, s.out("r_squared")?, s.out("p_value")?);
            let a = s.solve_other("simple_linear_regression", &[("x", tx(&text_list(&x))), ("y", tx(&text_list(&y)))])?;
            let (b0, b1) = (k::num_of(&a, "intercept")?, k::num_of(&a, "slope")?);
            let (x0, x1) = pl::minmax(&x);
            let mut p = Plot::new((x0, x1), pl::minmax(&y));
            p.clip_on();
            p.path(&[(x0, b0 + b1 * x0), (x1, b0 + b1 * x1)], "axp-c3");
            p.unclip();
            for (a, b) in x.iter().zip(&y) {
                p.dot(*a, *b, 4.5, "axp-d");
            }
            p.frame("x", "y", &format!("r = {}, R² = {}", num(r), num(r2)));
            Some(s.chart("scatter", "Linear association and its correlation",
                         &format!("The pairs x = {}, y = {} with the least-squares line from the model's regression run \
                                   (y = {} + {}x); returned: r = {}, R² = {}, p-value {} (n = {}).", list(&x, 10), list(&y, 10),
                                  num(b0), num(b1), num(r), num(r2), num(pv), x.len()), &p.body()))
        }
        _ => {
            let (rho, pv) = (s.out("rho")?, s.out("p_value")?);
            let (rx, ry) = (rank(&x), rank(&y));
            let n = x.len() as f64;
            let mut p = Plot::new((1.0, n), (1.0, n));
            for (a, b) in rx.iter().zip(&ry) {
                p.dot(*a, *b, 4.5, "axp-d");
            }
            p.frame("rank of x", "rank of y", &format!("ρ = {}, p-value = {}", num(rho), num(pv)));
            Some(s.chart("scatter", "Rank correlation",
                         &format!("The ranks of x = {} against the ranks of y = {} (ranks from the inputs, ties averaged); \
                                   Spearman's ρ is the correlation of these ranks: returned ρ = {}, p-value {}.", list(&x, 10),
                                  list(&y, 10), num(rho), num(pv)), &p.body()))
        }
    }
}

/// Regression fits.
fn fit(s: &Solved) -> Option<Figure> {
    if s.key == "multiple_linear_regression" {
        let xm = s.mat("X")?;
        let y = s.vec("y")?;
        let c = s.out_list("coefficients")?;
        if xm.len() != y.len() || c.len() != xm[0].len() + 1 || y.len() > 400 {
            return None;
        }
        let fitted: Vec<f64> = xm.iter().map(|r| c[0] + r.iter().zip(&c[1..]).map(|(a, b)| a * b).sum::<f64>()).collect();
        let all: Vec<f64> = y.iter().chain(fitted.iter()).cloned().collect();
        let (lo, hi) = pl::minmax(&all);
        let mut p = Plot::new((lo, hi), (lo, hi));
        p.dpath(&[(lo, lo), (hi, hi)], "axp-c4");
        for (f, o) in fitted.iter().zip(&y) {
            p.seg((*f, *f), (*f, *o), "axp-c3");
            p.dot(*f, *o, 4.5, "axp-d");
        }
        let (r2, ar2, fv, se) = (s.out("r_squared")?, s.out("adj_r_squared")?, s.out("f")?, s.out("residual_se")?);
        p.frame("fitted value", "observed y", &format!("R² = {}, F = {}", num(r2), num(fv)));
        return Some(s.chart("fit", "Observed against fitted values",
                            &format!("Observed y = {} against the fitted values from the returned coefficients {} (intercept \
                                      first, then one per column of X); vertical segments are the residuals, the dashed line \
                                      is a perfect fit. Returned: R² = {} (adjusted {}), F = {}, residual SE {}.", list(&y, 10),
                                     list(&c, 8), num(r2), num(ar2), num(fv), num(se)), &p.body()));
    }
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.len() < 2 || x.len() > 400 {
        return None;
    }
    let (b0, b1) = (s.out("intercept")?, s.out("slope")?);
    let mut xs = x.clone();
    let x0v = s.inp("x0");
    if let Some(v) = x0v {
        xs.push(v);
    }
    let (lo, hi) = pl::minmax(&xs);
    let mut ys = y.clone();
    ys.extend([b0 + b1 * lo, b0 + b1 * hi]);
    let mut p = Plot::new((lo, hi), pl::minmax(&ys));
    p.path(&[(lo, b0 + b1 * lo), (hi, b0 + b1 * hi)], "axp-c3");
    for (a, b) in x.iter().zip(&y) {
        p.dot(*a, *b, 4.5, "axp-d");
    }
    if s.key == "regression_prediction" {
        let (x0, yh) = (x0v?, s.out("y_hat")?);
        p.dot(x0, yh, 6.0, "axp-m");
        p.label(x0, yh, 10.0, 18.0, "start", "axp-mt", &format!("ŷ({}) = {}", num(x0), num(yh)));
        p.frame("x", "y", &format!("ŷ = {} + {}x", num(b0), num(b1)));
        return Some(s.chart("fit", "Predicting from a fitted line",
                            &format!("The data x = {}, y = {} with the returned least-squares line (intercept {}, slope {}); \
                                      marked: the prediction at x₀ = {}, ŷ = {}.", list(&x, 10), list(&y, 10), num(b0), num(b1),
                                     num(x0), num(yh)), &p.body()));
    }
    let (r2, pv) = (s.out("r_squared")?, s.out("p_value")?);
    p.frame("x", "y", &format!("ŷ = {} + {}x, R² = {}", num(b0), num(b1), num(r2)));
    Some(s.chart("fit", "Simple linear regression: data and fitted line",
                 &format!("The least-squares line through x = {}, y = {}: intercept {} (standard error {}), slope {} (standard \
                           error {}), r = {}, R² = {}, p-value of the slope {}, as returned by the model.", list(&x, 10),
                          list(&y, 10), num(b0), num(s.out("intercept_se")?), num(b1), num(s.out("slope_se")?),
                          num(s.out("r")?), num(r2), num(pv)), &p.body()))
}

fn residuals(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.len() < 3 || x.len() > 400 {
        return None;
    }
    let (b0, b1, se) = (s.out("intercept")?, s.out("slope")?, s.out("residual_se")?);
    let res: Vec<f64> = x.iter().zip(&y).map(|(a, b)| b - (b0 + b1 * a)).collect();
    let mut ys = res.clone();
    ys.extend([se, -se, 0.0]);
    let mut p = Plot::new(pl::minmax(&x), pl::minmax(&ys));
    p.hline(0.0, "axp-c4", false);
    p.hline(se, "axp-c4", true);
    p.hline(-se, "axp-c4", true);
    for (a, r) in x.iter().zip(&res) {
        p.seg((*a, 0.0), (*a, *r), "axp-c3");
        p.dot(*a, *r, 4.5, "axp-d");
    }
    p.frame("x", "residual", &format!("residual SE ± {}", num(se)));
    Some(s.chart("residuals", "Residuals of a straight-line fit",
                 &format!("Residuals of y = {} from the returned line ŷ = {} + {}x (from the inputs and the returned coefficients), \
                           with the returned residual standard error {} either side of zero (dashed; R² = {}): no pattern \
                           means the straight line fits.", list(&y, 10), num(b0), num(b1), num(se), num(s.out("r_squared")?)),
                 &p.body()))
}

/// Every pair of points joined: concordant rising, discordant falling.
fn kendall(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.len() < 2 || x.len() > 30 {
        return None;
    }
    let (tau, pv, c, d) = (s.out("tau")?, s.out("p_value")?, s.out("concordant")?, s.out("discordant")?);
    let mut p = Plot::new(pl::minmax(&x), pl::minmax(&y));
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            let sgn = (x[j] - x[i]) * (y[j] - y[i]);
            if sgn == 0.0 {
                continue;
            }
            p.seg((x[i], y[i]), (x[j], y[j]), if sgn > 0.0 { "axp-c" } else { "axp-c3" });
        }
    }
    for (a, b) in x.iter().zip(&y) {
        p.dot(*a, *b, 5.0, "axp-kd");
    }
    p.legend_at(&[(format!("concordant {}", num(c)), "axp-b"), (format!("discordant {}", num(d)), "axp-b3")], true);
    p.frame("x", "y", &format!("τ = {}, p-value = {}", num(tau), num(pv)));
    Some(s.chart("pairs", "Concordant and discordant pairs",
                 &format!("Every pair of the points x = {}, y = {} joined: rising segments (concordant) in the first colour, \
                           falling (discordant) in the second (classified from the inputs). Returned: {} concordant, {} \
                           discordant, τ = {}, p-value {}.", list(&x, 10), list(&y, 10), num(c), num(d), num(tau), num(pv)), &p.body()))
}

/// Paired measurements: each pair joined from x to y.
pub fn paired(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() != y.len() || x.is_empty() || x.len() > 200 {
        return None;
    }
    let (md, t, df, pv) = (s.out("mean_difference")?, s.out("t")?, s.out("df")?, s.out("p_value")?);
    let all: Vec<f64> = x.iter().chain(&y).cloned().collect();
    let mut p = Plot::axes(pl::Ax::new(-0.25, 1.25, X0, X1, false), pl::Ax::new(pl::minmax(&all).0, pl::minmax(&all).1, Y1, Y0 + 14.0, true));
    for (a, b) in x.iter().zip(&y) {
        p.seg((0.0, *a), (1.0, *b), if b < a { "axp-c" } else { "axp-c3" });
        p.dot(0.0, *a, 4.0, "axp-kd");
        p.dot(1.0, *b, 4.0, "axp-kd");
    }
    p.text(p.x(0.0), Y1 + 20.0, "middle", "axp-bt", "x");
    p.text(p.x(1.0), Y1 + 20.0, "middle", "axp-bt", "y");
    p.frame_with("", "value", &format!("mean difference {}, t = {}", num(md), num(t)), false, true);
    Some(s.chart("pairs", "Paired measurements",
                 &format!("Each pair (x, y) = ({}) to ({}) joined by a line, falling lines in the first colour. Returned: mean \
                           difference {}, t = {}, {} degrees of freedom, p-value {}.", list(&x, 10), list(&y, 10), num(md),
                          num(t), num(df), num(pv)), &p.body()))
}

/// Two samples ranked together on one axis.
fn ranks(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if x.len() + y.len() > 40 {
        return None;
    }
    let (u, pv) = (s.out("u")?, s.out("p_value")?);
    let all: Vec<f64> = x.iter().chain(&y).cloned().collect();
    let r = rank(&all);
    let mut p = Plot::axes(pl::Ax::new(pl::minmax(&all).0, pl::minmax(&all).1, X0, X1, true), pl::Ax::new(0.0, 1.0, Y1, Y0, false));
    p.hline(0.68, "axp-c4", true);
    p.hline(0.3, "axp-c4", true);
    for (i, v) in all.iter().enumerate() {
        let top = i < x.len();
        let yv = if top { 0.68 } else { 0.3 };
        p.dot(*v, yv, 5.5, if top { "axp-d" } else { "axp-d3" });
        let up = if i % 2 == 0 { -10.0 } else { 20.0 };
        p.label(*v, yv, 0.0, up, "middle", "axp-vt", &num(r[i]));
    }
    p.text(X0 + 4.0, p.y(0.68) - 24.0, "start", "axp-bt", "x");
    p.text(X0 + 4.0, p.y(0.3) - 24.0, "start", "axp-bt", "y");
    p.frame_with("value (labels: rank in the combined sample)", "", &format!("U = {}, p-value = {}", num(u), num(pv)), true, false);
    Some(s.chart("ranks", "Two samples ranked together",
                 &format!("The samples x = {} (upper row) and y = {} (lower row) on one axis, each labelled with its rank in the \
                           combined sample (ranks from the inputs, ties averaged). Returned: U = {}, p-value {}.", list(&x, 10),
                          list(&y, 10), num(u), num(pv)), &p.body()))
}

/// Paired differences as bars, positive in the first colour (Wilcoxon: labelled with the rank of |d|).
pub fn differences(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let d: Vec<f64> = match s.vec("y") {
        Some(y) if y.len() == x.len() => x.iter().zip(&y).map(|(a, b)| a - b).collect(),
        Some(_) => return None,
        None => x.clone(),
    };
    if d.is_empty() || d.len() > 60 {
        return None;
    }
    let wil = s.key.starts_with("wilcoxon");
    let nz: Vec<f64> = d.iter().filter(|v| **v != 0.0).map(|v| v.abs()).collect();
    let r = rank(&nz);
    let mut ri = 0;
    let n = d.len() as f64;
    let mut p = Plot::new((0.5, n + 0.5), (pl::minmax(&d).0.min(0.0), pl::minmax(&d).1.max(0.0)));
    p.hline(0.0, "axp-c4", false);
    for (i, v) in d.iter().enumerate() {
        let c = i as f64 + 1.0;
        p.rect((c - 0.33, 0.0), (c + 0.33, *v), if *v > 0.0 { "axp-b" } else { "axp-b4" });
        if *v != 0.0 {
            if wil && d.len() <= 24 {
                p.label(c, *v, 0.0, if *v > 0.0 { -6.0 } else { 15.0 }, "middle", "axp-vt", &num(r[ri]));
            }
            ri += 1;
        }
    }
    let pv = s.out("p_value")?;
    let (header, ret) = if wil {
        (format!("R+ = {}, p-value = {}", num(s.out("r_plus")?), num(pv)),
         format!("R+ = {}, statistic {} (n = {}), p-value {}", num(s.out("r_plus")?), num(s.out("statistic")?),
                 num(s.out("n")?), num(pv)))
    } else {
        (format!("{} positive of {}", num(s.out("positives")?), num(s.out("n")?)),
         format!("{} positive of {} non-zero differences, p-value {}", num(s.out("positives")?), num(s.out("n")?), num(pv)))
    };
    p.frame("pair", "difference x - y", &header);
    Some(s.chart("differences", if wil { "Signed ranks of paired differences" } else { "Signs of paired differences" },
                 &format!("The differences x - y of the pairs (from the inputs: {}), positive in the first colour{}. Returned: {ret}.",
                          list(&d, 12), if wil { "; each labelled with the rank of its absolute value" } else { "" }), &p.body()))
}

/// A Kaplan-Meier step curve from (event time, survival) pairs, starting at 1.
fn steps(times: &[f64], surv: &[f64], end: f64) -> Vec<(f64, f64)> {
    let mut pts = vec![(0.0, 1.0)];
    let mut cur = 1.0;
    for (t, sv) in times.iter().zip(surv) {
        pts.push((*t, cur));
        pts.push((*t, *sv));
        cur = *sv;
    }
    pts.push((end.max(times.last().copied().unwrap_or(0.0)), cur));
    pts
}

fn kaplan(s: &Solved) -> Option<Figure> {
    let (et, sc, t, sv) = (s.out_list("event_times")?, s.out_list("survival_curve")?, s.inp("t")?, s.out("survival")?);
    let times = s.vec("times")?;
    if et.len() != sc.len() {
        return None;
    }
    let end = pl::minmax(&times).1.max(t);
    let pts = steps(&et, &sc, end);
    let mut p = Plot::new((0.0, end), (0.0, 1.0));
    p.path(&pts, "axp-c");
    p.vline(t, "axp-c4", true);
    p.dot(t, sv, 6.0, "axp-m");
    p.label(t, sv, 8.0, -10.0, "start", "axp-mt", &format!("S({}) = {}", num(t), num(sv)));
    p.frame("time", "survival probability", "Kaplan-Meier estimate");
    Some(s.chart("survival", "Kaplan-Meier survival curve",
                 &format!("The Kaplan-Meier survival curve returned by the model for times {} with event indicators {} (0 = \
                           censored): it steps down at each event time {}; marked: t = {}, S = {}.", list(&times, 12),
                          list(&s.vec("events")?, 12), list(&et, 12), num(t), num(sv)), &p.body()))
}

/// Two groups' Kaplan-Meier curves, each from the model's Kaplan-Meier run.
pub fn survival_two(s: &Solved) -> Option<Figure> {
    let mut series = vec![];
    let mut end: f64 = 0.0;
    let mut words = vec![];
    for g in ["1", "2"] {
        let (t, e) = (s.vec(&format!("times{g}"))?, s.vec(&format!("events{g}"))?);
        let top = pl::minmax(&t).1;
        end = end.max(top);
        let a = s.solve_other("kaplan_meier", &[("times", tx(&text_list(&t))), ("events", tx(&text_list(&e))), ("t", fl(top))])?;
        let (et, sc) = (k::list_of(&a, "event_times")?, k::list_of(&a, "survival_curve")?);
        words.push(format!("group {g} (times {}, events {})", list(&t, 10), list(&e, 10)));
        series.push((et, sc));
    }
    let mut p = Plot::new((0.0, end), (0.0, 1.0));
    for (i, (et, sc)) in series.iter().enumerate() {
        p.path(&steps(et, sc, end), if i == 0 { "axp-c" } else { "axp-c3" });
    }
    p.legend(&[("group 1".into(), "axp-b"), ("group 2".into(), "axp-b3")]);
    let (x2, pv) = (s.out("chi_square")?, s.out("p_value")?);
    p.frame("time", "survival probability", &format!("log-rank χ² = {}, p-value = {}", num(x2), num(pv)));
    Some(s.chart("survival", "Two survival curves compared",
                 &format!("Kaplan-Meier curves from the model's Kaplan-Meier runs for {}. Log-rank test: χ² = {}, p-value {}; \
                           group 1 had {} events against {} expected.", words.join(" and "), num(x2), num(pv),
                          num(s.out("observed_1")?), num(s.out("expected_1")?)), &p.body()))
}

/// One control chart panel: the points in time order, the centre line and the limits.
fn cc_panel(v: &[f64], centre: f64, ucl: f64, lcl: f64, ylab: &str, header: &str) -> String {
    let n = v.len() as f64;
    let mut ys = v.to_vec();
    ys.extend([centre, ucl, lcl]);
    let mut p = Plot::new((1.0, n.max(2.0) + (n.max(2.0) - 1.0) * 0.32), pl::minmax(&ys));
    p.hline(centre, "axp-c2", false);
    p.hline(ucl, "axp-c", true);
    p.hline(lcl, "axp-c", true);
    let pts: Vec<(f64, f64)> = v.iter().enumerate().map(|(i, y)| (i as f64 + 1.0, *y)).collect();
    p.path(&pts, "axp-c4");
    for (x, y) in &pts {
        let out = *y > ucl || *y < lcl;
        p.dot(*x, *y, if out { 5.5 } else { 4.0 }, if out { "axp-d" } else { "axp-d3" });
    }
    for (val, name, dy) in [(ucl, "UCL", -5.0), (centre, "centre", -5.0), (lcl, "LCL", 15.0)] {
        p.text(X1 - 4.0, p.y(val) + dy, "end", "axp-vt", &format!("{name} {}", num(val)));
    }
    p.frame("sample", ylab, header);
    p.body()
}

fn control(s: &Solved) -> Option<Figure> {
    match s.key {
        "c_chart" => {
            let v = s.vec("counts")?;
            let (c, u, l) = (s.out("center")?, s.out("ucl")?, s.out("lcl")?);
            if v.len() > 400 {
                return None;
            }
            let body = cc_panel(&v, c, u, l, "defects", &format!("c̄ = {}", num(c)));
            Some(s.chart("control-chart", "c chart of defect counts",
                         &format!("A c chart of the defect counts {} in time order with the returned centre line c̄ = {} and \
                                   control limits UCL = {}, LCL = {}.", list(&v, 12), num(c), num(u), num(l)), &body))
        }
        "p_chart" => {
            let (d, n) = (s.vec("defectives")?, nin(s, "n")?);
            let (c, u, l, sg) = (s.out("center")?, s.out("ucl")?, s.out("lcl")?, s.out("sigma")?);
            if !(n > 0.0) || d.len() > 400 {
                return None;
            }
            let f: Vec<f64> = d.iter().map(|x| x / n).collect();
            let body = cc_panel(&f, c, u, l, "fraction defective", &format!("p̄ = {}", num(c)));
            Some(s.chart("control-chart", "p chart of fractions defective",
                         &format!("A p chart: the fraction defective of each sample of n = {} (defectives {}, divided by n) with \
                                   the returned centre p̄ = {} and limits UCL = {}, LCL = {} (σ = {}).", num(n), list(&d, 12),
                                  num(c), num(u), num(l), num(sg)), &body))
        }
        "individuals_moving_range_chart" => {
            let x = s.vec("x")?;
            if x.len() < 2 || x.len() > 400 {
                return None;
            }
            let (c, mr, xu, xl, mu, ml) = (s.out("center")?, s.out("mean_moving_range")?, s.out("x_ucl")?, s.out("x_lcl")?,
                                           s.out("mr_ucl")?, s.out("mr_lcl")?);
            let m: Vec<f64> = x.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
            let (body, h) = k::stack(&[cc_panel(&x, c, xu, xl, "x", "individuals"), cc_panel(&m, mr, mu, ml, "moving range", "moving ranges")]);
            Some(s.figure("control-chart", "Individuals and moving-range charts",
                          &format!("Individuals chart (top) of x = {} with the returned centre {} and limits {} to {}; \
                                    moving-range chart (bottom) of the absolute differences of successive values (from the \
                                    inputs) with the returned mean moving range {} and limits {} to {}.", list(&x, 12), num(c),
                                   num(xl), num(xu), num(mr), num(ml), num(mu)), &body, h))
        }
        _ => {
            let data = s.mat("data")?;
            if data.len() > 100 || data.iter().any(|r| r.len() < 2) {
                return None;
            }
            let r_chart = s.key == "xbar_r_chart";
            let (k2, o2, name) = if r_chart { ("range", "range", "range") } else { ("sample_standard_deviation", "standard_deviation", "standard deviation") };
            let mut means = vec![];
            let mut sp = vec![];
            for row in &data {
                let t = tx(&text_list(row));
                means.push(o1(s, "mean", &[("x", t.clone())], "mean")?);
                sp.push(o1(s, k2, &[("x", t)], o2)?);
            }
            let (c, xu, xl) = (s.out("center_xbar")?, s.out("xbar_ucl")?, s.out("xbar_lcl")?);
            let (bar, su, sl) = if r_chart { (s.out("mean_range")?, s.out("r_ucl")?, s.out("r_lcl")?) } else {
                (s.out("mean_sd")?, s.out("s_ucl")?, s.out("s_lcl")?)
            };
            let (body, h) = k::stack(&[cc_panel(&means, c, xu, xl, "subgroup mean", "x̄ chart"),
                                       cc_panel(&sp, bar, su, sl, if r_chart { "subgroup range" } else { "subgroup sd" },
                                                if r_chart { "R chart" } else { "s chart" })]);
            Some(s.figure("control-chart", if r_chart { "x̄ and R control charts" } else { "x̄ and s control charts" },
                          &format!("x̄ chart (top) and {} chart (bottom) for {} subgroups: subgroup means and {}s from the \
                                    model's mean and {name} runs; returned centre {}, limits {} to {}; {} {}, limits {} to {}.",
                                   if r_chart { "R" } else { "s" }, data.len(), name, num(c), num(xl), num(xu),
                                   if r_chart { "R̄" } else { "s̄" }, num(bar), num(sl), num(su)), &body, h))
        }
    }
}

/// Components in series or in parallel as a block diagram.
fn blocks(s: &Solved) -> Option<Figure> {
    let r = s.vec("r")?;
    let rel = s.out("reliability")?;
    if r.is_empty() || r.len() > 8 {
        return None;
    }
    let series = s.key.starts_with("series");
    let n = r.len();
    let mut c;
    if series {
        c = Canvas::new(200.0);
        let w = (380.0 / n as f64).min(110.0);
        let bw = w * 0.72;
        let x0 = 260.0 - w * n as f64 / 2.0;
        c.terminal((x0 - 30.0, 110.0), "in");
        c.line((x0 - 26.0, 110.0), (x0 + (w - bw) / 2.0, 110.0));
        for (i, v) in r.iter().enumerate() {
            let bx = x0 + w * i as f64 + (w - bw) / 2.0;
            c.block(bx, 88.0, bw, 44.0, &num(*v));
            c.name(bx + bw / 2.0, 80.0, "middle", &format!("R{}", i + 1));
            let next = if i + 1 < n { x0 + w * (i + 1) as f64 + (w - bw) / 2.0 } else { x0 + w * n as f64 + 26.0 };
            c.line((bx + bw, 110.0), (next, 110.0));
        }
        c.terminal((x0 + w * n as f64 + 30.0, 110.0), "out");
        c.bold(260.0, 30.0, "middle", &format!("system reliability {}", num(rel)));
        c.value(260.0, 170.0, "middle", &format!("weakest component {}", num(pl::minmax(&r).0)));
    } else {
        let h = 90.0 + 54.0 * n as f64;
        c = Canvas::new(h);
        let (xl, xr) = (150.0, 370.0);
        let (top, bot) = (70.0, 70.0 + 54.0 * (n as f64 - 1.0));
        c.line((xl, top), (xl, bot));
        c.line((xr, top), (xr, bot));
        c.terminal((90.0, (top + bot) / 2.0), "in");
        c.line((94.0, (top + bot) / 2.0), (xl, (top + bot) / 2.0));
        c.terminal((430.0, (top + bot) / 2.0), "out");
        c.line((xr, (top + bot) / 2.0), (426.0, (top + bot) / 2.0));
        for (i, v) in r.iter().enumerate() {
            let y = top + 54.0 * i as f64;
            c.line((xl, y), (205.0, y));
            c.block(205.0, y - 19.0, 110.0, 38.0, &format!("R{} = {}", i + 1, num(*v)));
            c.line((315.0, y), (xr, y));
            c.dot((xl, y));
            c.dot((xr, y));
        }
        c.bold(260.0, 30.0, "middle", &format!("system reliability {}", num(rel)));
        c.value(260.0, h - 12.0, "middle", &format!("best component {}", num(pl::minmax(&r).1)));
    }
    let body = c.body().to_string();
    Some(s.figure("diagram", if series { "Components in series" } else { "Redundant components in parallel" },
                  &format!("{n} components {} with reliabilities {}: the system {}, so its returned reliability {} {}.",
                           if series { "in series" } else { "in parallel" }, list(&r, 8),
                           if series { "works only if every block works" } else { "fails only if every block fails" }, num(rel),
                           if series { "is below that of the weakest component" } else { "exceeds that of the best component" }),
                  &body, c.h))
}

/// Weibull reliability and hazard over time for three shapes, one run of the model per point.
fn weibull(s: &Solved) -> Option<Figure> {
    let (b, eta, t) = (s.inp("beta")?, s.inp("eta")?, s.inp("t")?);
    let (r0, h0, mttf) = (s.out("reliability")?, s.out("hazard_rate")?, s.out("mttf")?);
    if !(eta > 0.0 && b > 0.0) {
        return None;
    }
    let mut shapes = vec![1.0, b, 4.0];
    shapes.sort_by(|a, c| a.partial_cmp(c).unwrap());
    shapes.dedup_by(|a, c| (*a - *c).abs() < 1e-9);
    let top = (2.5 * eta).max(t * 1.1);
    let ts: Vec<f64> = (1..=40).map(|i| top * i as f64 / 40.0).collect();
    let (mut rs, mut hs) = (vec![], vec![]);
    for sh in &shapes {
        let (mut rp, mut hp) = (vec![], vec![]);
        for tv in &ts {
            if let Some(a) = s.with(&[("beta", *sh), ("t", *tv)]) {
                if let Some(v) = k::num_of(&a, "reliability") {
                    rp.push((*tv, v));
                }
                if let Some(v) = k::num_of(&a, "hazard_rate") {
                    hp.push((*tv, v));
                }
            }
        }
        let name = format!("β = {}", num(*sh));
        rs.push((name.clone(), rp));
        hs.push((name, hp));
    }
    let p1 = k::lines_y(&rs, "time t", "reliability R(t)", &format!("R({}) = {}", num(t), num(r0)), Some((t, r0)),
                        &format!("t = {}", num(t)), false, true);
    let p2 = k::lines_y(&hs, "time t", "hazard rate h(t)", &format!("h({}) = {}", num(t), num(h0)), Some((t, h0)),
                        &format!("t = {}", num(t)), false, true);
    let (body, h) = k::stack(&[p1, p2]);
    Some(s.figure("curves", "Weibull reliability and hazard by shape",
                  &format!("Weibull reliability (top) and hazard rate (bottom) over time for η = {} and shapes {}, one run of the \
                            model per point: β < 1 failures slow down, β > 1 wear out. Marked at t = {}: R = {}, h = {} (MTTF {}).",
                           num(eta), shapes.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(t), num(r0), num(h0),
                           num(mttf)), &body, h))
}

fn interaction(s: &Solved) -> Option<Figure> {
    let y = s.vec("y")?;
    if y.len() != 4 {
        return None;
    }
    let (gm, a, b, ab) = (s.out("grand_mean")?, s.out("A")?, s.out("B")?, s.out("AB")?);
    let series = vec![("B low".to_string(), vec![(-1.0, y[0]), (1.0, y[1])]), ("B high".to_string(), vec![(-1.0, y[2]), (1.0, y[3])])];
    let body = k::lines(&series, "level of A (-1 low, +1 high)", "response", &format!("A = {}, B = {}, AB = {}", num(a), num(b), num(ab)),
                        None, "", false);
    Some(s.chart("interaction", "2×2 factorial: interaction plot",
                 &format!("Interaction plot of the 2×2 factorial responses y = {} in standard order ((1), a, b, ab): one line \
                           per level of B across the levels of A; non-parallel lines show interaction. Returned: grand mean {}, \
                           main effects A = {}, B = {}, interaction AB = {}.", list(&y, 4), num(gm), num(a), num(b), num(ab)), &body))
}

fn bars(s: &Solved) -> Option<Figure> {
    match s.key {
        "factorial_2x2x2_effects" => {
            let names = ["A", "B", "C", "AB", "AC", "BC", "ABC"];
            let v: Vec<f64> = names.iter().map(|n| s.out(n)).collect::<Option<Vec<_>>>()?;
            let big = v.iter().enumerate().fold(0, |m, (i, x)| if x.abs() > v[m].abs() { i } else { m });
            let labels: Vec<String> = names.iter().map(|x| x.to_string()).collect();
            let body = k::bars(&labels, &v, &[big], "effect", "estimate", &format!("grand mean {}", num(s.out("grand_mean")?)));
            Some(s.chart("bars", "2×2×2 factorial: main effects and interactions",
                         &format!("The seven effects returned for the 2×2×2 factorial responses y = {} (standard order): {}; grand \
                                   mean {}; the largest in size in the first colour.", list(&s.vec("y")?, 8),
                                  names.iter().zip(&v).map(|(n, x)| format!("{n} = {}", num(*x))).collect::<Vec<_>>().join(", "),
                                  num(s.out("grand_mean")?)), &body))
        }
        "frequency_table_statistics" => {
            let (x, f) = (s.vec("x")?, s.vec("f")?);
            if x.len() != f.len() || x.len() > 60 {
                return None;
            }
            let (m, sd, psd, n) = (s.out("mean")?, s.out("sample_standard_deviation")?, s.out("population_standard_deviation")?, s.out("n")?);
            let mut sorted = x.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let gap = sorted.windows(2).map(|w| w[1] - w[0]).filter(|d| *d > 0.0).fold(f64::INFINITY, f64::min);
            let bw = if gap.is_finite() { gap * 0.35 } else { 0.4 };
            let xs: Vec<f64> = x.iter().cloned().chain([m - sd, m + sd]).collect();
            let (x0, x1) = pl::minmax(&xs);
            let mut p = Plot::from_zero((x0 - bw, x1 + bw), pl::minmax(&f).1, true);
            for (a, b) in x.iter().zip(&f) {
                p.rect((a - bw, 0.0), (a + bw, *b), "axp-b4");
                p.label(*a, *b, 0.0, -5.0, "middle", "axp-lt", &num(*b));
            }
            mark(&mut p, m, &format!("mean {}", num(m)), "axp-c3", false, 0);
            p.vline(m - sd, "axp-c4", true);
            p.vline(m + sd, "axp-c4", true);
            p.frame("x", "frequency", &format!("n = {}, sd = {}", num(n), num(sd)));
            Some(s.chart("bars", "A frequency table, its mean and spread",
                         &format!("The frequency table x = {}, f = {} (n = {}) as bars; marked: the returned mean {} and, dashed, \
                                   one returned sample standard deviation {} either side (population sd {}).", list(&x, 12),
                                  list(&f, 12), num(n), num(m), num(sd), num(psd)), &p.body()))
        }
        "mode" => {
            let x = s.vec("x")?;
            let modes = s.out_list("modes").unwrap_or_default();
            let fr = s.out("frequency")?;
            let mut vals: Vec<(f64, usize)> = vec![];
            for v in &x {
                match vals.iter_mut().find(|e| e.0 == *v) {
                    Some(e) => e.1 += 1,
                    None => vals.push((*v, 1)),
                }
            }
            if vals.len() > 40 {
                return None;
            }
            vals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let hl: Vec<usize> = vals.iter().enumerate().filter(|(_, e)| modes.contains(&e.0)).map(|(i, _)| i).collect();
            let labels: Vec<String> = vals.iter().map(|e| num(e.0)).collect();
            let counts: Vec<f64> = vals.iter().map(|e| e.1 as f64).collect();
            let body = k::bars(&labels, &counts, &hl, "value", "count",
                               &format!("mode {} (frequency {})", if modes.is_empty() { "none".into() } else { list(&modes, 6) }, num(fr)));
            Some(s.chart("bars", "The most frequent value",
                         &format!("How often each value occurs in x = {} (counts tallied from the input); the returned mode {} \
                                   with frequency {} is in the first colour.", list(&x, 12),
                                  if modes.is_empty() { "none".into() } else { list(&modes, 6) }, num(fr)), &body))
        }
        _ => {
            let x = s.vec("x")?;
            if x.len() > 40 {
                return None;
            }
            let (var, m, sd) = (s.out("variance")?, s.out("mean")?, s.out("standard_deviation")?);
            let sq: Vec<f64> = x.iter().map(|v| (v - m) * (v - m)).collect();
            let labels: Vec<String> = x.iter().map(|v| num(*v)).collect();
            let pop = s.key == "population_variance";
            let body = k::bars(&labels, &sq, &[], "observation", "(x - mean)²",
                               &format!("{} variance = {}", if pop { "population" } else { "sample" }, num(var)));
            Some(s.chart("bars", if pop { "Population variance: the mean squared deviation" } else { "Sample variance from squared deviations" },
                         &format!("For x = {}: each observation's squared distance from the returned mean {} (from the inputs and \
                                   the mean); the returned {} variance {} is their sum divided by {} (standard deviation {}).",
                                  list(&x, 12), num(m), if pop { "population" } else { "sample" }, num(var),
                                  if pop { "n" } else { "n - 1" }, num(sd)), &body))
        }
    }
}

fn durbin(s: &Solved) -> Option<Figure> {
    let r = s.vec("residuals")?;
    let dw = s.out("durbin_watson")?;
    if r.len() < 2 || r.len() > 400 {
        return None;
    }
    let pts: Vec<(f64, f64)> = r.iter().enumerate().map(|(i, v)| (i as f64 + 1.0, *v)).collect();
    let mut ys = r.clone();
    ys.push(0.0);
    let mut p = Plot::new((1.0, r.len() as f64), pl::minmax(&ys));
    p.hline(0.0, "axp-c4", true);
    p.path(&pts, "axp-c3");
    for (x, y) in &pts {
        p.dot(*x, *y, 4.0, if *y >= 0.0 { "axp-d" } else { "axp-d3" });
    }
    p.frame("time order", "residual", &format!("Durbin-Watson = {}", num(dw)));
    Some(s.chart("series", "Residuals in time order",
                 &format!("Regression residuals {} in time order, consecutive ones joined. The returned Durbin-Watson statistic {} \
                           compares successive differences with the residuals: near 2 no autocorrelation, below 2 positive, \
                           above 2 negative.", list(&r, 12), num(dw)), &p.body()))
}

/// One box plot across: the five-number summary, or the 1.5 IQR fences and outliers.
fn one_box(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    if x.len() > 400 || x.is_empty() {
        return None;
    }
    let fences = s.key == "iqr_outliers";
    let (q1, q3) = (s.out("q1")?, s.out("q3")?);
    let (md, mn, mx, lf, uf) = if fences {
        let a = s.solve_other("five_number_summary", &[("x", tx(&text_list(&x)))])?;
        let (lf, uf) = (s.out("lower_fence")?, s.out("upper_fence")?);
        let inside: Vec<f64> = x.iter().cloned().filter(|v| *v >= lf && *v <= uf).collect();
        let (a0, b0) = pl::minmax(&inside);
        (k::num_of(&a, "median")?, a0, b0, Some(lf), Some(uf))
    } else {
        (s.out("median")?, s.out("minimum")?, s.out("maximum")?, None, None)
    };
    let mut xs = x.clone();
    xs.extend(lf);
    xs.extend(uf);
    let mut p = Plot::axes(pl::Ax::new(pl::minmax(&xs).0, pl::minmax(&xs).1, X0, X1, true), pl::Ax::new(0.0, 1.0, Y1, Y0, false));
    p.rect((q1, 0.45), (q3, 0.75), "axp-ka");
    p.rect((q1, 0.45), (q3, 0.75), "axp-f");
    p.seg((md, 0.45), (md, 0.75), "axp-c");
    p.seg((mn, 0.6), (q1, 0.6), "axp-k");
    p.seg((q3, 0.6), (mx, 0.6), "axp-k");
    p.seg((mn, 0.53), (mn, 0.67), "axp-k");
    p.seg((mx, 0.53), (mx, 0.67), "axp-k");
    let outl = s.out_list("outliers").unwrap_or_default();
    for (i, v) in x.iter().enumerate() {
        let out = fences && (lf.map(|f| *v < f).unwrap_or(false) || uf.map(|f| *v > f).unwrap_or(false));
        p.dot(*v, 0.18 + 0.05 * (i % 3) as f64, if out { 6.0 } else { 3.5 }, if out { "axp-d" } else { "axp-d3" });
    }
    let labels = [(q1, "Q1"), (md, "median"), (q3, "Q3")];
    // the labels on free lines from the top of the plot down to the box (a narrow box beside a far outlier crowds)
    let top = (p.y(0.75) - 8.0 - Y0 - 16.0).max(0.0);
    let rows = ((top / 15.0) as usize + 1).max(3);
    for (v, n) in labels.iter() {
        let px = p.x(*v);
        p.place(px, Y0 + 16.0, 15.0, rows, "axp-vt", &format!("{n} {}", num(*v)));
    }
    let caption;
    if let (Some(lf), Some(uf)) = (lf, uf) {
        p.vline(lf, "axp-c3", true);
        p.vline(uf, "axp-c3", true);
        let (a, b) = (p.x(lf), p.x(uf));
        p.place(a, Y0 + 16.0, 15.0, 8, "axp-lt", &format!("fence {}", num(lf)));
        p.place(b, Y0 + 16.0, 15.0, 8, "axp-lt", &format!("fence {}", num(uf)));
        p.frame_with("x", "", &format!("IQR = {}, outliers: {}", num(s.out("iqr")?), if outl.is_empty() { "none".into() } else { list(&outl, 6) }), true, false);
        caption = format!("x = {} with its box (Q1 {}, median {} from the model's five-number summary, Q3 {}; whiskers to the \
                           most extreme values inside the fences) and the returned fences Q1 - 1.5·IQR = {} and Q3 + 1.5·IQR = {} \
                           (IQR {}); the returned outliers {} lie beyond them.", list(&x, 12), num(q1), num(md), num(q3), num(lf),
                          num(uf), num(s.out("iqr")?), if outl.is_empty() { "(none)".into() } else { list(&outl, 6) });
    } else {
        let (a, b, y) = (p.x(mn), p.x(mx), p.y(0.6));
        p.place_mid(a, y - 12.0, -15.0, 2, "axp-vt", &format!("min {}", num(mn)));
        p.place_mid(b, y - 12.0, -15.0, 2, "axp-vt", &format!("max {}", num(mx)));
        p.frame_with("x", "", &format!("IQR = {}", num(s.out("iqr")?)), true, false);
        caption = format!("A box plot of x = {} drawn from the returned five-number summary: minimum {}, Q1 {}, median {}, Q3 {}, \
                           maximum {} (IQR {}), with the data below.", list(&x, 12), num(mn), num(q1), num(md), num(q3), num(mx),
                          num(s.out("iqr")?));
    }
    Some(s.chart("box", if fences { "Outliers beyond the 1.5 IQR fences" } else { "Five-number summary as a box plot" }, &caption, &p.body()))
}
