//! Every Statistics capability's own live figures (src/figures.rs, src/figures_more.rs): its worked example gives
//! each published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number, text
//! outside its canvas or wording the public pages do not use.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use statistics_ai999x::pack;

/// The published figure kinds of every capability that the model draws itself (publish/website/models/
/// statistics-ai999x/figures, except the generic "curve" and the kinds the generic figures already give), as
/// tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("bartlett_test", &["boxes", "null-density"]),
    ("binomial_at_least", &["stems"]),
    ("binomial_at_most", &["stems"]),
    ("binomial_exactly", &["stems"]),
    ("binomial_mean_sd", &["stems"]),
    ("c_chart", &["control-chart"]),
    ("chi_square_critical_value", &["density"]),
    ("chi_square_goodness_of_fit", &["bars", "null-density"]),
    ("chi_square_independence", &["null-density"]),
    ("chi_square_variance_test", &["null-density"]),
    ("ci_mean_known_sigma", &["intervals"]),
    ("ci_mean_t", &["dots", "intervals"]),
    ("ci_mean_t_summary", &["intervals"]),
    ("ci_proportion", &["intervals"]),
    ("ci_two_means", &["intervals"]),
    ("ci_variance", &["intervals"]),
    ("coefficient_of_variation", &["dots"]),
    ("cohens_d", &["boxes"]),
    ("correlation_confidence_interval", &["intervals", "xy"]),
    ("correlation_t_test", &["null-density"]),
    ("covariance", &["scatter"]),
    ("cronbach_alpha", &["matrix"]),
    ("discrete_random_variable", &["stems"]),
    ("durbin_watson", &["series"]),
    ("exponential_probability_above", &["density"]),
    ("exponential_probability_below", &["density"]),
    ("f_critical_value", &["density"]),
    ("f_test_variances", &["boxes", "null-density"]),
    ("factorial_2x2_effects", &["interaction"]),
    ("factorial_2x2x2_effects", &["bars"]),
    ("fisher_exact_test", &["matrix", "stems"]),
    ("five_number_summary", &["box"]),
    ("frequency_table_statistics", &["bars"]),
    ("friedman_test", &["matrix", "null-density"]),
    ("geometric_mean", &["dots"]),
    ("geometric_probability", &["stems"]),
    ("harmonic_mean", &["dots"]),
    ("hypergeometric_probability", &["stems"]),
    ("individuals_moving_range_chart", &["control-chart"]),
    ("inverse_normal", &["density"]),
    ("iqr_outliers", &["box"]),
    ("kaplan_meier", &["survival"]),
    ("kendall_tau", &["pairs"]),
    ("kruskal_wallis", &["boxes", "null-density"]),
    ("levene_test", &["boxes", "null-density"]),
    ("log_rank_test", &["null-density", "survival"]),
    ("mann_whitney_exact", &["ranks"]),
    ("mann_whitney_u", &["ranks"]),
    ("mcnemar_test", &["matrix", "null-density"]),
    ("mean", &["dots"]),
    ("mean_absolute_deviation", &["dots"]),
    ("median", &["dots"]),
    ("median_absolute_deviation", &["dots"]),
    ("mode", &["bars"]),
    ("multiple_linear_regression", &["fit"]),
    ("negative_binomial_probability", &["stems"]),
    ("normal_probability_above", &["density"]),
    ("normal_probability_below", &["density"]),
    ("normal_probability_between", &["density"]),
    ("one_way_anova", &["boxes", "null-density"]),
    ("p_chart", &["control-chart"]),
    ("paired_t_test", &["null-density", "pairs"]),
    ("parallel_system_reliability", &["diagram"]),
    ("pearson_correlation", &["scatter"]),
    ("poisson_at_least", &["stems"]),
    ("poisson_at_most", &["stems"]),
    ("poisson_exactly", &["stems"]),
    ("population_standard_deviation", &["dots"]),
    ("population_variance", &["bars"]),
    ("process_capability", &["density"]),
    ("proportion_z_test", &["null-density"]),
    ("randomized_block_anova", &["matrix", "null-density"]),
    ("range", &["dots"]),
    ("regression_prediction", &["fit"]),
    ("sample_mean_probability_above", &["density"]),
    ("sample_mean_probability_below", &["density"]),
    ("sample_standard_deviation", &["dots"]),
    ("sample_variance", &["bars"]),
    ("series_system_reliability", &["diagram"]),
    ("sign_test", &["differences", "stems"]),
    ("simple_linear_regression", &["fit", "residuals"]),
    ("skewness_kurtosis", &["dots"]),
    ("spearman_correlation", &["scatter", "xy"]),
    ("standard_error_of_mean", &["dots"]),
    ("t_critical_value", &["density"]),
    ("t_test_one_sample", &["density", "dots"]),
    ("t_test_summary_statistics", &["null-density"]),
    ("t_test_two_sample_pooled", &["boxes", "null-density"]),
    ("trimmed_mean", &["dots"]),
    ("two_proportion_z_test", &["null-density"]),
    ("weibull_reliability", &["curves"]),
    ("weighted_mean", &["stems"]),
    ("welch_t_test", &["boxes", "null-density"]),
    ("wilcoxon_signed_rank", &["differences"]),
    ("wilcoxon_signed_rank_exact", &["differences"]),
    ("wilson_interval", &["intervals"]),
    ("xbar_r_chart", &["control-chart"]),
    ("xbar_s_chart", &["control-chart"]),
    ("z_critical_value", &["density"]),
    ("z_score", &["density"]),
    ("z_test_mean", &["null-density"]),
];

/// Inputs to change for a capability whose numbers cannot simply be scaled (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("t_critical_value", "tails", "right-tailed"),
    ("z_critical_value", "tails", "left-tailed"),
    ("factorial_2x2_effects", "y", "20, 36, 18, 40"),
    ("factorial_2x2x2_effects", "y", "22, 30, 35, 50, 44, 41, 66, 39"),
    ("mode", "x", "3, 3, 7, 2, 9, 2, 3"),
    ("kendall_tau", "y", "1, 2, 4, 3, 6, 5"),
    ("mann_whitney_u", "y", "1.5, 2.6, 5.2, 3.9"),
    ("mann_whitney_exact", "y", "1.5, 2.6, 5.5, 3.9"),
    ("fisher_exact_test", "table", "[[7, 2], [2, 6]]"),
    ("mcnemar_test", "table", "[[30, 14], [4, 53]]"),
    ("kaplan_meier", "events", "1, 1, 0, 0, 1, 1, 1, 1, 1"),
    ("log_rank_test", "events2", "1, 1, 1, 0, 1, 1, 1"),
    ("sign_test", "y", "110, 122, 135, 120, 140, 124, 123, 137"),
    ("wilcoxon_signed_rank", "y", "110, 122, 135, 120, 140, 124, 123, 137"),
    ("wilcoxon_signed_rank_exact", "y", "110, 122, 135, 120, 140, 124, 123, 137"),
    ("parallel_system_reliability", "r", "0.9, 0.8"),
    ("series_system_reliability", "r", "0.99, 0.9, 0.95, 0.97"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=[1, 2], c="(A·B)'"); a bracketed list is one
/// value.
pub fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0;
            while i < cs.len() && (depth > 0 || (cs[i] != ',' && cs[i] != ')')) {
                match cs[i] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure), public wording.
fn check(f: &Figure) {
    if let Err(e) = problems(f) {
        panic!("{e}");
    }
}

fn problems(f: &Figure) -> Result<(), String> {
    ax_engine::figkit::check_figure(f)?;
    let words = format!("{} {}", f.title, f.caption).to_lowercase();
    for bad in ["—", " we ", " our ", " us ", "package", "stax"] {
        if words.contains(bad) {
            return Err(format!("{}: '{bad}' in the public text: {}", f.slug, words));
        }
    }
    Ok(())
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind && !f.slug.contains("-live-")) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    if let Err(e) = problems(f) {
                        missing.push(e);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12, 3" -> "16.44, 4.11").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

/// A second example: a listed replacement, else each input's numbers scaled until the model solves it and every
/// kind is drawn differently.
fn changed_example(key: &str, kinds: &[&str], base: &[Figure]) -> Option<Vec<(String, Input)>> {
    let p = pack();
    let given = example(key);
    let mut tries: Vec<Vec<(String, Input)>> = vec![];
    if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == key) {
        let mut g = given.clone();
        for e in g.iter_mut() {
            if e.0 == *sym {
                e.1 = Input::Text(val.to_string());
            }
        }
        tries.push(g);
    }
    for f in [1.37, 0.73, 1.1, 2.0] {
        for i in 0..given.len() {
            if let Input::Text(t) = &given[i].1 {
                if let Some(t2) = scale_text(t, f) {
                    let mut g = given.clone();
                    g[i].1 = Input::Text(t2);
                    tries.push(g);
                }
            }
        }
    }
    for g in tries {
        if !ax_engine::solve(p, key, &g, None).solved() {
            continue;
        }
        let figs = figures(p, key, &g, None, None);
        let differs = kinds.iter().all(|kind| match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
            (Some(a), Some(b)) => a.svg != b.svg,
            _ => false,
        });
        if differs {
            for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                check(f);
            }
            return Some(g);
        }
    }
    None
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let base = figures(p, key, &example(key), None, None);
        if changed_example(key, kinds, &base).is_none() {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a probability above 1, a negative standard deviation: refused by the model, so no figure at all
    let p = pack();
    for (key, sym, bad) in [("binomial_exactly", "p", "1.5"), ("normal_probability_below", "sigma", "-2"),
                            ("ci_mean_t", "confidence", "1.5")] {
        let mut g = example(key);
        for e in g.iter_mut() {
            if e.0 == sym {
                e.1 = Input::Text(bad.into());
            }
        }
        let a = ax_engine::solve(p, key, &g, None);
        assert!(!a.solved(), "{key}: {sym} = {bad} was solved");
        assert!(figures(p, key, &g, None, None).is_empty(), "{key}: a figure for a refused input");
    }
}
