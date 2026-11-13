//! statistics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/statistics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "bartlett_test",
    "binomial_at_least",
    "binomial_at_most",
    "binomial_exactly",
    "binomial_mean_sd",
    "c_chart",
    "chi_square_critical_value",
    "chi_square_goodness_of_fit",
    "chi_square_independence",
    "chi_square_variance_test",
    "ci_mean_known_sigma",
    "ci_mean_t",
    "ci_mean_t_summary",
    "ci_proportion",
    "ci_two_means",
    "ci_variance",
    "coefficient_of_variation",
    "cohens_d",
    "correlation_confidence_interval",
    "correlation_t_test",
    "covariance",
    "cronbach_alpha",
    "discrete_random_variable",
    "durbin_watson",
    "exponential_probability_above",
    "exponential_probability_below",
    "exponential_reliability",
    "f_critical_value",
    "f_test_variances",
    "factorial_2x2_effects",
    "factorial_2x2x2_effects",
    "fisher_exact_test",
    "five_number_summary",
    "frequency_table_statistics",
    "friedman_test",
    "geometric_mean",
    "geometric_probability",
    "harmonic_mean",
    "hypergeometric_probability",
    "individuals_moving_range_chart",
    "inverse_normal",
    "iqr_outliers",
    "k_out_of_n_reliability",
    "kaplan_meier",
    "kendall_tau",
    "kruskal_wallis",
    "levene_test",
    "log_rank_test",
    "mann_whitney_exact",
    "mann_whitney_u",
    "mcnemar_test",
    "mean",
    "mean_absolute_deviation",
    "median",
    "median_absolute_deviation",
    "mode",
    "multiple_linear_regression",
    "negative_binomial_probability",
    "normal_probability_above",
    "normal_probability_below",
    "normal_probability_between",
    "one_way_anova",
    "p_chart",
    "paired_t_test",
    "parallel_system_reliability",
    "pearson_correlation",
    "percentile",
    "poisson_at_least",
    "poisson_at_most",
    "poisson_exactly",
    "population_standard_deviation",
    "population_variance",
    "power_one_sample_z",
    "power_two_sample_z",
    "probability_at_least_one",
    "probability_rules",
    "process_capability",
    "proportion_z_test",
    "randomized_block_anova",
    "range",
    "regression_prediction",
    "sample_mean_probability_above",
    "sample_mean_probability_below",
    "sample_size_mean",
    "sample_size_proportion",
    "sample_size_two_means",
    "sample_standard_deviation",
    "sample_variance",
    "series_system_reliability",
    "sign_test",
    "simple_linear_regression",
    "skewness_kurtosis",
    "spearman_correlation",
    "standard_error_of_mean",
    "t_critical_value",
    "t_test_one_sample",
    "t_test_summary_statistics",
    "t_test_two_sample_pooled",
    "trimmed_mean",
    "two_proportion_z_test",
    "two_way_table_probabilities",
    "weibull_reliability",
    "weighted_mean",
    "welch_t_test",
    "wilcoxon_signed_rank",
    "wilcoxon_signed_rank_exact",
    "wilson_interval",
    "xbar_r_chart",
    "xbar_s_chart",
    "z_critical_value",
    "z_score",
    "z_test_mean",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "bartlett_test" => {
            let r = f_run_bartlett(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g4") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g5") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_40(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_at_least" => {
            let r = f_run_binom_at_least(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_103(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_at_most" => {
            let r = f_run_binom_at_most(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_166(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_exactly" => {
            let r = f_run_binom_exact(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_226(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_mean_sd" => {
            let r = f_run_binom_moments(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_276(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "c_chart" => {
            let r = f_run_c_chart(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_318(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chi_square_critical_value" => {
            let r = f_run_chi2_critical(match v.get("df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_412(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chi_square_goodness_of_fit" => {
            let r = f_run_gof(match v.get("observed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("proportions") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_486(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chi_square_independence" => {
            let r = f_run_independence(match v.get("table") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_563(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chi_square_variance_test" => {
            let r = f_run_chi2_var(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_629(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_mean_known_sigma" => {
            let r = f_run_ci_mean_z(match v.get("xbar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_687(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_mean_t" => {
            let r = f_run_ci_mean_t(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_772(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_mean_t_summary" => {
            let r = f_run_ci_mean_t_summary(match v.get("xbar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_863(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_proportion" => {
            let r = f_run_ci_prop(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_925(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_two_means" => {
            let r = f_run_ci_two_means(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1018(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ci_variance" => {
            let r = f_run_ci_variance(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1111(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coefficient_of_variation" => {
            let r = f_run_cv(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1153(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cohens_d" => {
            let r = f_run_cohens_d(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1197(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "correlation_confidence_interval" => {
            let r = f_run_corr_ci(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1265(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "correlation_t_test" => {
            let r = f_run_corr_test(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1344(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "covariance" => {
            let r = f_run_cov(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1390(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cronbach_alpha" => {
            let r = f_run_cronbach(match v.get("data") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1442(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discrete_random_variable" => {
            let r = f_run_discrete_rv(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1492(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "durbin_watson" => {
            let r = f_run_durbin_watson(match v.get("residuals") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1535(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_probability_above" => {
            let r = f_run_exp_above(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("mean") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1583(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_probability_below" => {
            let r = f_run_exp_below(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("mean") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1631(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_reliability" => {
            let r = f_run_exp_rel(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("mtbf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1679(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "f_critical_value" => {
            let r = f_run_f_critical(match v.get("df1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("df2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1780(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "f_test_variances" => {
            let r = f_run_f_test(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1857(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "factorial_2x2_effects" => {
            let r = s_1891(match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1911(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "factorial_2x2x2_effects" => {
            let r = s_1945(match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1965(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fisher_exact_test" => {
            let r = f_run_fisher(match v.get("table") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2051(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "five_number_summary" => {
            let r = f_run_five_number(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("linear")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2098(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_table_statistics" => {
            let r = f_run_frequency(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2150(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "friedman_test" => {
            let r = f_run_friedman(match v.get("data") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2232(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_mean" => {
            let r = f_run_gmean(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2275(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_probability" => {
            let r = f_run_geometric(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2325(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "harmonic_mean" => {
            let r = f_run_hmean(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2368(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hypergeometric_probability" => {
            let r = f_run_hypergeom(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2425(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "individuals_moving_range_chart" => {
            let r = f_run_imr(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2465(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_normal" => {
            let r = f_run_inverse_normal(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2519(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "iqr_outliers" => {
            let r = f_run_outliers(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2572(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "k_out_of_n_reliability" => {
            let r = f_run_k_of_n(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2625(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kaplan_meier" => {
            let r = f_run_km(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2692(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kendall_tau" => {
            let r = f_run_kendall(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2761(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kruskal_wallis" => {
            let r = f_run_kruskal(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g4") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g5") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2842(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "levene_test" => {
            let r = f_run_levene(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g4") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g5") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("center") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("median")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2926(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_rank_test" => {
            let r = f_run_logrank(match v.get("times1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3021(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mann_whitney_exact" => {
            let r = f_run_mann_whitney_exact(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3090(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mann_whitney_u" => {
            let r = f_run_mann_whitney(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3150(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mcnemar_test" => {
            let r = f_run_mcnemar(match v.get("table") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3233(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean" => {
            let r = f_run_mean(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3272(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_absolute_deviation" => {
            let r = f_run_mad(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3312(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "median" => {
            let r = f_run_median(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3351(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "median_absolute_deviation" => {
            let r = f_run_mad_median(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3391(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mode" => {
            let r = f_run_mode(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3440(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multiple_linear_regression" => {
            let r = f_run_multiple_regression(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3554(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "negative_binomial_probability" => {
            let r = f_run_negbinom(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3606(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_probability_above" => {
            let r = f_run_normal_above(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3644(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_probability_below" => {
            let r = f_run_normal_below(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3682(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_probability_between" => {
            let r = f_run_normal_between(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3725(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "one_way_anova" => {
            let r = f_run_anova(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g3") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g4") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("g5") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3804(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "p_chart" => {
            let r = f_run_p_chart(match v.get("defectives") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3859(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "paired_t_test" => {
            let r = f_run_paired_t(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3942(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_system_reliability" => {
            let r = s_3976(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3988(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pearson_correlation" => {
            let r = f_run_pearson(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4067(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percentile" => {
            let r = f_run_percentile(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("linear")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4115(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poisson_at_least" => {
            let r = s_4149(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4177(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poisson_at_most" => {
            let r = s_4211(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4229(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poisson_exactly" => {
            let r = s_4263(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4278(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "population_standard_deviation" => {
            let r = s_4312(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4319(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "population_variance" => {
            let r = s_4353(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4360(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_one_sample_z" => {
            let r = f_run_power_one(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4434(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_two_sample_z" => {
            let r = f_run_power_two(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4508(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "probability_at_least_one" => {
            let r = f_run_at_least_one(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4559(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "probability_rules" => {
            let r = f_run_probability_rules(match v.get("p_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_a_and_b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("p_a_or_b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("relation") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4629(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "process_capability" => {
            let r = f_run_capability(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lsl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("usl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4670(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "proportion_z_test" => {
            let r = f_run_prop_z(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4729(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "randomized_block_anova" => {
            let r = f_run_block_anova(match v.get("data") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4812(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "range" => {
            let r = f_run_range(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4851(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regression_prediction" => {
            let r = f_run_predict(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4901(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_mean_probability_above" => {
            let r = s_4935(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4947(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_mean_probability_below" => {
            let r = s_4981(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4993(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_size_mean" => {
            let r = f_run_n_mean(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("margin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5044(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_size_proportion" => {
            let r = f_run_n_prop(match v.get("margin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.5)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5105(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_size_two_means" => {
            let r = f_run_n_two_means(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("power") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5190(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_standard_deviation" => {
            let r = s_5224(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5231(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_variance" => {
            let r = s_5265(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5272(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_system_reliability" => {
            let r = s_5306(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5317(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sign_test" => {
            let r = f_run_sign_test(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5386(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_linear_regression" => {
            let r = f_run_simple_regression(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5466(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "skewness_kurtosis" => {
            let r = f_run_skew_kurt(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5511(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spearman_correlation" => {
            let r = f_run_spearman(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5596(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "standard_error_of_mean" => {
            let r = f_run_sem(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5636(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_critical_value" => {
            let r = f_run_t_critical(match v.get("df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tails") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5727(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_test_one_sample" => {
            let r = f_run_t_one(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5804(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_test_summary_statistics" => {
            let r = f_run_t_summary(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5890(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_test_two_sample_pooled" => {
            let r = s_5924(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5974(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trimmed_mean" => {
            let r = f_run_trimmed(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("proportion") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6016(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_proportion_z_test" => {
            let r = f_run_two_prop_z(match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6070(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_way_table_probabilities" => {
            let r = f_run_two_way_table(match v.get("table") { Some(x) => py::from_val(x), None => { return None } }, match v.get("row") { Some(x) => py::from_val(x), None => { return None } }, match v.get("column") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6153(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weibull_reliability" => {
            let r = f_run_weibull(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6199(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weighted_mean" => {
            let r = f_run_weighted_mean(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6247(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "welch_t_test" => {
            let r = s_6281(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6331(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wilcoxon_signed_rank" => {
            let r = f_run_wilcoxon(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6403(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wilcoxon_signed_rank_exact" => {
            let r = f_run_wilcoxon_exact(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6486(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wilson_interval" => {
            let r = f_run_wilson(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6543(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "xbar_r_chart" => {
            let r = f_run_xbar_r(match v.get("data") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6595(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "xbar_s_chart" => {
            let r = f_run_xbar_s(match v.get("data") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6648(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "z_critical_value" => {
            let r = f_run_z_critical(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tails") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6702(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "z_score" => {
            let r = f_run_zscore(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6740(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "z_test_mean" => {
            let r = f_run_z_test(match v.get("xbar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alternative") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("two-sided")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6792(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__A2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(1.88)), (Py::Int(3i64), Py::Float(1.023)), (Py::Int(4i64), Py::Float(0.729)), (Py::Int(5i64), Py::Float(0.577)), (Py::Int(6i64), Py::Float(0.483)), (Py::Int(7i64), Py::Float(0.419)), (Py::Int(8i64), Py::Float(0.373)), (Py::Int(9i64), Py::Float(0.337)), (Py::Int(10i64), Py::Float(0.308))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__A3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(2.659)), (Py::Int(3i64), Py::Float(1.954)), (Py::Int(4i64), Py::Float(1.628)), (Py::Int(5i64), Py::Float(1.427)), (Py::Int(6i64), Py::Float(1.287)), (Py::Int(7i64), Py::Float(1.182)), (Py::Int(8i64), Py::Float(1.099)), (Py::Int(9i64), Py::Float(1.032)), (Py::Int(10i64), Py::Float(0.975))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(0.0)), (Py::Int(3i64), Py::Float(0.0)), (Py::Int(4i64), Py::Float(0.0)), (Py::Int(5i64), Py::Float(0.0)), (Py::Int(6i64), Py::Float(0.03)), (Py::Int(7i64), Py::Float(0.118)), (Py::Int(8i64), Py::Float(0.185)), (Py::Int(9i64), Py::Float(0.239)), (Py::Int(10i64), Py::Float(0.284))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B4() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(3.267)), (Py::Int(3i64), Py::Float(2.568)), (Py::Int(4i64), Py::Float(2.266)), (Py::Int(5i64), Py::Float(2.089)), (Py::Int(6i64), Py::Float(1.97)), (Py::Int(7i64), Py::Float(1.882)), (Py::Int(8i64), Py::Float(1.815)), (Py::Int(9i64), Py::Float(1.761)), (Py::Int(10i64), Py::Float(1.716))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(0.0)), (Py::Int(3i64), Py::Float(0.0)), (Py::Int(4i64), Py::Float(0.0)), (Py::Int(5i64), Py::Float(0.0)), (Py::Int(6i64), Py::Float(0.0)), (Py::Int(7i64), Py::Float(0.076)), (Py::Int(8i64), Py::Float(0.136)), (Py::Int(9i64), Py::Float(0.184)), (Py::Int(10i64), Py::Float(0.223))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__D4() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(2i64), Py::Float(3.267)), (Py::Int(3i64), Py::Float(2.574)), (Py::Int(4i64), Py::Float(2.282)), (Py::Int(5i64), Py::Float(2.114)), (Py::Int(6i64), Py::Float(2.004)), (Py::Int(7i64), Py::Float(1.924)), (Py::Int(8i64), Py::Float(1.864)), (Py::Int(9i64), Py::Float(1.816)), (Py::Int(10i64), Py::Float(1.777))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__E2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.66)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SQRT2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.4142135623730951)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TINY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-300)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__alt_p(mut v_z_or_t: Py, mut v_alternative: Py, mut v_sf: Py, mut v_cdf: Py) -> R {
    if ({ let t6788 = v_alternative.clone(); Py::Bool({ let t6789 = Py::str("two-sided"); if py::eq(&t6788, &t6789) { true } else { false } }) }).truthy() {
        return Ok(py::minmax(&[Py::Float(1.0), py::mul(&Py::Float(2.0), &py::minmax(&[py::call(&v_sf.clone(), &[v_z_or_t.clone()])?, py::call(&v_cdf.clone(), &[v_z_or_t.clone()])?], false)?)?], false)?);
    }
    return Ok((if ({ let t6790 = v_alternative.clone(); Py::Bool({ let t6791 = Py::str("greater"); if py::eq(&t6790, &t6791) { true } else { false } }) }).truthy() { py::call(&v_sf.clone(), &[v_z_or_t.clone()])? } else { py::call(&v_cdf.clone(), &[v_z_or_t.clone()])? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bcf(mut v_a: Py, mut v_b: Py, mut v_x: Py) -> R {
    let mut v_aa: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_qab: Py = Py::None;
    let mut v_qam: Py = Py::None;
    let mut v_qap: Py = Py::None;
    let t6310 = (Py::tuple(vec![py::add(&v_a.clone(), &v_b.clone())?, py::add(&v_a.clone(), &Py::Float(1.0))?, py::sub(&v_a.clone(), &Py::Float(1.0))?])).items()?;
    if t6310.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qab = t6310[0].clone();
    v_qap = t6310[1].clone();
    v_qam = t6310[2].clone();
    let t6311 = (Py::tuple(vec![Py::Float(1.0), py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&v_qab.clone(), &v_x.clone())?, &v_qap.clone())?)?])).items()?;
    if t6311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t6311[0].clone();
    v_d = t6311[1].clone();
    if ({ let t6312 = py::abs(&v_d.clone())?; Py::Bool({ let t6313 = c__TINY()?; if py::lt(&t6312, &t6313)? { true } else { false } }) }).truthy() {
        v_d = c__TINY()?;
    }
    v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
    v_h = v_d.clone();
    for t6314 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(300000i64)).i()?]) {
        v_m = t6314;
        v_m2 = py::mul(&Py::Int(2i64), &v_m.clone())?;
        v_aa = py::truediv(&py::mul(&py::mul(&v_m.clone(), &py::sub(&v_b.clone(), &v_m.clone())?)?, &v_x.clone())?, &py::mul(&py::add(&v_qam.clone(), &v_m2.clone())?, &py::add(&v_a.clone(), &v_m2.clone())?)?)?;
        v_d = py::add(&Py::Float(1.0), &py::mul(&v_aa.clone(), &v_d.clone())?)?;
        v_d = (if ({ let t6315 = py::abs(&v_d.clone())?; Py::Bool({ let t6316 = c__TINY()?; if py::lt(&t6315, &t6316)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&Py::Float(1.0), &py::truediv(&v_aa.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t6317 = py::abs(&v_c.clone())?; Py::Bool({ let t6318 = c__TINY()?; if py::lt(&t6317, &t6318)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_h = py::mul(&v_h, &py::mul(&v_d.clone(), &v_c.clone())?)?;
        v_aa = py::truediv(&py::mul(&py::mul(&py::neg(&py::add(&v_a.clone(), &v_m.clone())?)?, &py::add(&v_qab.clone(), &v_m.clone())?)?, &v_x.clone())?, &py::mul(&py::add(&v_a.clone(), &v_m2.clone())?, &py::add(&v_qap.clone(), &v_m2.clone())?)?)?;
        v_d = py::add(&Py::Float(1.0), &py::mul(&v_aa.clone(), &v_d.clone())?)?;
        v_d = (if ({ let t6319 = py::abs(&v_d.clone())?; Py::Bool({ let t6320 = c__TINY()?; if py::lt(&t6319, &t6320)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&Py::Float(1.0), &py::truediv(&v_aa.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t6321 = py::abs(&v_c.clone())?; Py::Bool({ let t6322 = c__TINY()?; if py::lt(&t6321, &t6322)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t6323 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t6324 = Py::Float(4e-16); if py::lt(&t6323, &t6324)? { true } else { false } }) }).truthy() {
            return Ok(v_h.clone());
        }
    }
    return Err(PyErr::Other("ArithmeticError".into()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bin_args(mut v_n: Py, mut v_k: Py, mut v_p: Py) -> R {
    let t207 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64))?])).items()?;
    if t207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t207[0].clone();
    v_k = t207[1].clone();
    if ({ let t208 = v_n.clone(); Py::Bool({ let t209 = Py::Int(100000i64); if py::gt(&t208, &t209)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be at most 100000 trials"))));
    }
    if ({ let t210 = v_k.clone(); Py::Bool({ let t211 = v_n.clone(); if py::gt(&t210, &t211)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k cannot exceed the number of trials n"))));
    }
    return Ok(Py::tuple(vec![v_n.clone(), v_k.clone(), f__prob(v_p.clone(), Py::str("p"), Py::Bool(true))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__binom_cdf(mut v_k: Py, mut v_n: Py, mut v_p: Py) -> R {
    return Ok((if ({ let t158 = v_k.clone(); Py::Bool({ let t159 = Py::Int(0i64); if py::ge(&t158, &t159)? { true } else { false } }) }).truthy() { py::minmax(&[Py::Float(1.0), py::fsum(&{ let mut out = vec![]; for t160 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&py::minmax(&[v_k.clone(), v_n.clone()], false)?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t160;
out.push(f__binom_pmf(v_j.clone(), v_n.clone(), v_p.clone())?); } Py::list(out) })?], false)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__binom_pmf(mut v_k: Py, mut v_n: Py, mut v_p: Py) -> R {
    if ({ let t225 = { let t221 = v_k.clone(); Py::Bool({ let t222 = Py::Int(0i64); if py::lt(&t221, &t222)? { true } else { false } }) }; if t225.truthy() { t225 } else { { let t223 = v_k.clone(); Py::Bool({ let t224 = v_n.clone(); if py::gt(&t223, &t224)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::math1("exp", &py::add(&py::add(&py::math1("log", &py::comb(&v_n.clone(), &v_k.clone())?)?, &py::mul(&v_k.clone(), &py::math1("log", &v_p.clone())?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_k.clone())?, &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__binom_sf_ge(mut v_k: Py, mut v_n: Py, mut v_p: Py) -> R {
    return Ok((if ({ let t95 = v_k.clone(); Py::Bool({ let t96 = v_n.clone(); if py::le(&t95, &t96)? { true } else { false } }) }).truthy() { py::minmax(&[Py::Float(1.0), py::fsum(&{ let mut out = vec![]; for t97 in py::range_iter(&[(py::minmax(&[v_k.clone(), Py::Int(0i64)], true)?).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t97;
out.push(f__binom_pmf(v_j.clone(), v_n.clone(), v_p.clone())?); } Py::list(out) })?], false)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect_decreasing(mut v_sf: Py, mut v_q: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_mid: Py = Py::None;
    while ({ let t5696 = py::call(&v_sf.clone(), &[v_hi.clone()])?; Py::Bool({ let t5697 = v_q.clone(); if py::gt(&t5696, &t5697)? { true } else { false } }) }).truthy() {
        let t5693 = (Py::tuple(vec![v_hi.clone(), py::mul(&v_hi.clone(), &Py::Float(2.0))?])).items()?;
        if t5693.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t5693[0].clone();
        v_hi = t5693[1].clone();
        if ({ let t5694 = v_hi.clone(); Py::Bool({ let t5695 = Py::Float(1e+300); if py::gt(&t5694, &t5695)? { true } else { false } }) }).truthy() {
            return Err(PyErr::Other("ArithmeticError".into()));
        }
    }
    for t5698 in py::range_iter(&[(Py::Int(3000i64)).i()?]) {
        v__ = t5698;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t5703 = { let t5699 = v_mid.clone(); Py::Bool({ let t5700 = v_lo.clone(); if py::le(&t5699, &t5700)? { true } else { false } }) }; if t5703.truthy() { t5703 } else { { let t5701 = v_mid.clone(); Py::Bool({ let t5702 = v_hi.clone(); if py::ge(&t5701, &t5702)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t5704 = py::call(&v_sf.clone(), &[v_mid.clone()])?; Py::Bool({ let t5705 = v_q.clone(); if py::gt(&t5704, &t5705)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect_increasing(mut v_cdf: Py, mut v_p: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_mid: Py = Py::None;
    while ({ let t1734 = py::call(&v_cdf.clone(), &[v_hi.clone()])?; Py::Bool({ let t1735 = v_p.clone(); if py::lt(&t1734, &t1735)? { true } else { false } }) }).truthy() {
        let t1731 = (Py::tuple(vec![v_hi.clone(), py::mul(&v_hi.clone(), &Py::Float(2.0))?])).items()?;
        if t1731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1731[0].clone();
        v_hi = t1731[1].clone();
        if ({ let t1732 = v_hi.clone(); Py::Bool({ let t1733 = Py::Float(1e+300); if py::gt(&t1732, &t1733)? { true } else { false } }) }).truthy() {
            return Err(PyErr::Other("ArithmeticError".into()));
        }
    }
    for t1736 in py::range_iter(&[(Py::Int(3000i64)).i()?]) {
        v__ = t1736;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t1741 = { let t1737 = v_mid.clone(); Py::Bool({ let t1738 = v_lo.clone(); if py::le(&t1737, &t1738)? { true } else { false } }) }; if t1741.truthy() { t1741 } else { { let t1739 = v_mid.clone(); Py::Bool({ let t1740 = v_hi.clone(); if py::ge(&t1739, &t1740)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1742 = py::call(&v_cdf.clone(), &[v_mid.clone()])?; Py::Bool({ let t1743 = v_p.clone(); if py::lt(&t1742, &t1743)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_alt(mut v_alternative: Py) -> R {
    if ({ let t6774 = v_alternative.clone(); Py::Bool({ let t6775 = Py::tuple(vec![Py::str("two-sided"), Py::str("greater"), Py::str("less")]); if !py::contains(&t6775, &t6774)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("alternative must be two-sided, greater or less"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_cdf(mut v_x: Py, mut v_k: Py) -> R {
    return Ok(py::getitem(&f__igamma(py::truediv(&v_k.clone(), &Py::Float(2.0))?, py::truediv(&v_x.clone(), &Py::Float(2.0))?)?, &Py::Int(0i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_ppf(mut v_p: Py, mut v_k: Py) -> R {
    if (py::not(&{ let t1063 = Py::Float(0.0); Py::Bool({ let t1064 = v_p.clone(); if py::lt(&t1063, &t1064)? { { let t1065 = Py::Float(1.0); if py::lt(&t1064, &t1065)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    if ({ let t1066 = v_p.clone(); Py::Bool({ let t1067 = Py::Float(0.5); if py::le(&t1066, &t1067)? { true } else { false } }) }).truthy() {
        return Ok(f__bisect_increasing({ let v_k = v_k.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__chi2_cdf(v_x.clone(), v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_p.clone(), Py::Float(0.0), Py::Float(1.0))?);
    }
    return Ok(f__bisect_decreasing({ let v_k = v_k.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__chi2_sf(v_x.clone(), v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::sub(&Py::Float(1.0), &v_p.clone())?, Py::Float(0.0), Py::Float(1.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_sf(mut v_x: Py, mut v_k: Py) -> R {
    return Ok(py::getitem(&f__igamma(py::truediv(&v_k.clone(), &Py::Float(2.0))?, py::truediv(&v_x.clone(), &Py::Float(2.0))?)?, &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conf(mut v_c: Py) -> R {
    if ({ let t6542 = py::not(&Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t6542.truthy() { t6542 } else { py::not(&{ let t6539 = Py::Int(0i64); Py::Bool({ let t6540 = v_c.clone(); if py::lt(&t6539, &t6540)? { { let t6541 = Py::Int(1i64); if py::lt(&t6540, &t6541)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the confidence level must be strictly between 0 and 1 (for example 0.95)"))));
    }
    return Ok(py::float(&v_c.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__crit(mut v_q_of: Py, mut v_alpha: Py, mut v_tails: Py) -> R {
    v_alpha = f__prob(v_alpha.clone(), Py::str("alpha"), Py::Bool(true))?;
    if ({ let t6690 = v_tails.clone(); Py::Bool({ let t6691 = Py::str("two-tailed"); if py::eq(&t6690, &t6691) { true } else { false } }) }).truthy() {
        return Ok(py::call(&v_q_of.clone(), &[py::sub(&Py::Float(1.0), &py::truediv(&v_alpha.clone(), &Py::Float(2.0))?)?])?);
    }
    return Ok((if ({ let t6692 = v_tails.clone(); Py::Bool({ let t6693 = Py::str("right-tailed"); if py::eq(&t6692, &t6693) { true } else { false } }) }).truthy() { py::call(&v_q_of.clone(), &[py::sub(&Py::Float(1.0), &v_alpha.clone())?])? } else { py::neg(&py::call(&v_q_of.clone(), &[py::sub(&Py::Float(1.0), &v_alpha.clone())?])?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t6793 = Py::Bool(matches!(v_a.clone(), Py::Dict(_))); if t6793.truthy() { t6793 } else { Py::Bool(matches!(v_b.clone(), Py::Dict(_))) } }).truthy() {
        if ({ let t6797 = py::not(&{ let t6794 = Py::Bool(matches!(v_a.clone(), Py::Dict(_))); if !t6794.truthy() { t6794 } else { Py::Bool(matches!(v_b.clone(), Py::Dict(_))) } }); if t6797.truthy() { t6797 } else { { let t6795 = py::method(&v_a.clone(), "keys", &[])?; Py::Bool({ let t6796 = py::method(&v_b.clone(), "keys", &[])?; if py::ne(&t6795, &t6796) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t6798 in py::iter(&(v_a.clone()))? { let mut v_k: Py; 
v_k = t6798;
out.push(f__diff(py::getitem(&v_a.clone(), &v_k.clone())?, py::getitem(&v_b.clone(), &v_k.clone())?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t6799 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t6799.truthy() { t6799 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t6803 = py::not(&{ let t6800 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t6800.truthy() { t6800 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t6803.truthy() { t6803 } else { { let t6801 = py::len(&v_a.clone())?; Py::Bool({ let t6802 = py::len(&v_b.clone())?; if py::ne(&t6801, &t6802) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t6804 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t6805 = (t6804).items()?;
if t6805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t6805[0].clone();
v_v = t6805[1].clone();
out.push(f__diff(v_u.clone(), v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t6810 = { let t6806 = v_a.clone(); Py::Bool({ let t6807 = Py::None; if py::is(&t6806, &t6807) { true } else { false } }) }; if t6810.truthy() { t6810 } else { { let t6808 = v_b.clone(); Py::Bool({ let t6809 = Py::None; if py::is(&t6808, &t6809) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t6811 = v_a.clone(); Py::Bool({ let t6812 = v_b.clone(); if py::is(&t6811, &t6812) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if ({ let t6813 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t6813.truthy() { t6813 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t6814 = v_a.clone(); Py::Bool({ let t6815 = v_b.clone(); if py::eq(&t6814, &t6815) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    let t6817: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t6816 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t6816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t6816[0].clone();
            v_b = t6816[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6817 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t6818 = py::math1("isfinite", &v_a.clone())?; if !t6818.truthy() { t6818 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok((if ({ let t6819 = v_a.clone(); Py::Bool({ let t6820 = v_b.clone(); if py::eq(&t6819, &t6820) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    return Ok((if ({ let t6821 = v_d.clone(); Py::Bool({ let t6822 = Py::Float(1e-15); if py::le(&t6821, &t6822)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exp_rate(mut v_rate: Py, mut v_mean: Py) -> R {
    if ({ let t1617 = { let t1619 = v_rate.clone(); Py::Bool({ let t1620 = Py::None; if py::is(&t1619, &t1620) { true } else { false } }) }; Py::Bool({ let t1618 = { let t1621 = v_mean.clone(); Py::Bool({ let t1622 = Py::None; if py::is(&t1621, &t1622) { true } else { false } }) }; if py::eq(&t1617, &t1618) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give either the rate or the mean (not both)"))));
    }
    return Ok((if ({ let t1623 = v_rate.clone(); Py::Bool({ let t1624 = Py::None; if !py::is(&t1623, &t1624) { true } else { false } }) }).truthy() { f__pos(v_rate.clone(), Py::str("the rate"))? } else { py::truediv(&Py::Float(1.0), &f__pos(v_mean.clone(), Py::str("the mean"))?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__f_ppf(mut v_p: Py, mut v_d1: Py, mut v_d2: Py) -> R {
    if (py::not(&{ let t1726 = Py::Float(0.0); Py::Bool({ let t1727 = v_p.clone(); if py::lt(&t1726, &t1727)? { { let t1728 = Py::Float(1.0); if py::lt(&t1727, &t1728)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    if ({ let t1729 = v_p.clone(); Py::Bool({ let t1730 = Py::Float(0.5); if py::le(&t1729, &t1730)? { true } else { false } }) }).truthy() {
        return Ok(f__bisect_increasing({ let v_d1 = v_d1.clone(); let v_d2 = v_d2.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__f_tails(v_x.clone(), v_d1.clone(), v_d2.clone())?, &Py::Int(0i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_p.clone(), Py::Float(0.0), Py::Float(1.0))?);
    }
    return Ok(f__bisect_decreasing({ let v_d1 = v_d1.clone(); let v_d2 = v_d2.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__f_tails(v_x.clone(), v_d1.clone(), v_d2.clone())?, &Py::Int(1i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::sub(&Py::Float(1.0), &v_p.clone())?, Py::Float(0.0), Py::Float(1.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__f_tails(mut v_f: Py, mut v_d1: Py, mut v_d2: Py) -> R {
    let mut v_den: Py = Py::None;
    if ({ let t4789 = v_f.clone(); Py::Bool({ let t4790 = Py::Int(0i64); if py::le(&t4789, &t4790)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    v_den = py::add(&py::mul(&v_d1.clone(), &v_f.clone())?, &v_d2.clone())?;
    return Ok(f__ibeta(py::truediv(&v_d1.clone(), &Py::Float(2.0))?, py::truediv(&v_d2.clone(), &Py::Float(2.0))?, py::truediv(&py::mul(&v_d1.clone(), &v_f.clone())?, &v_den.clone())?, py::truediv(&v_d2.clone(), &v_den.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__factorial(mut v_y: Py, mut v_k: Py) -> R {
    let mut v_cols: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lv: Py = Py::None;
    let mut v_mask: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_y = f__finite(v_y.clone(), Py::str("responses"))?;
    if ({ let t1949 = py::len(&v_y.clone())?; Py::Bool({ let t1950 = py::pow(&Py::Int(2i64), &v_k.clone())?; if py::ne(&t1949, &t1950) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give ".to_string(), py::format(&py::pow(&Py::Int(2i64), &v_k.clone())?, "")?, " responses in standard (Yates) order".to_string()].concat()))));
    }
    v_lv = { let mut out = vec![]; for t1951 in py::range_iter(&[(py::pow(&Py::Int(2i64), &v_k.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1951;
out.push({ let mut out = vec![]; for t1952 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t1952;
out.push((if (py::bitand(&py::rshift(&v_i.clone(), &v_j.clone())?, &Py::Int(1i64))?).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? })); } Py::list(out) }); } Py::list(out) };
    let t1953 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1953.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_names = t1953[0].clone();
    v_cols = t1953[1].clone();
    for t1954 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::pow(&Py::Int(2i64), &v_k.clone())?).i()?]) {
        v_mask = t1954;
        let _ = py::method(&v_names.clone(), "append", &[py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t1955 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t1955;
if (py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?).truthy() { out.push(py::getitem(&Py::str("ABC"), &v_j.clone())?); }} Py::list(out) }])?])?;
        let _ = py::method(&v_cols.clone(), "append", &[{ let mut out = vec![]; for t1956 in py::iter(&(v_lv.clone()))? { let mut v_r: Py; 
v_r = t1956;
out.push(py::prod(&{ let mut out = vec![]; for t1957 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t1957;
if (py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?).truthy() { out.push(py::getitem(&v_r.clone(), &v_j.clone())?); }} Py::list(out) })?); } Py::list(out) }])?;
    }
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_names.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_cols = v_cols.clone(); let v_k = v_k.clone(); let v_lv = v_lv.clone(); let v_mask = v_mask.clone(); let v_names = v_names.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::len(&py::getitem(&v_names.clone(), &v_i.clone())?)?, py::getitem(&v_names.clone(), &v_i.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_out = Py::dict(vec![(Py::str("grand_mean"), f__mean(v_y.clone())?)]);
    v_scale = { let t1959 = py::minmax(&[{ let mut out = vec![]; for t1958 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t1958;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?; if t1959.truthy() { t1959 } else { Py::Float(1.0) } };
    for t1960 in py::iter(&(v_order.clone()))? {
        v_i = t1960;
        v_e = py::truediv(&py::fsum(&{ let mut out = vec![]; for t1961 in py::iter(&(py::zip(&[py::getitem(&v_cols.clone(), &v_i.clone())?, v_y.clone()])?))? { let mut v_c: Py; let mut v_v: Py; 
let t1962 = (t1961).items()?;
if t1962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t1962[0].clone();
v_v = t1962[1].clone();
out.push(py::mul(&v_c.clone(), &v_v.clone())?); } Py::list(out) })?, &py::pow(&Py::Int(2i64), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?;
        py::setitem(&v_out.clone(), &py::getitem(&v_names.clone(), &v_i.clone())?, (if ({ let t1963 = py::abs(&v_e.clone())?; Py::Bool({ let t1964 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::lt(&t1963, &t1964)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_e.clone() }))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_x: Py, mut v_name: Py) -> R {
    let mut v_v: Py = Py::None;
    for t6639 in py::iter(&(v_x.clone()))? {
        v_v = t6639;
        if (py::not(&{ let t6640 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t6640.truthy() { t6640 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must contain finite numbers only".to_string()].concat()))));
        }
    }
    return Ok({ let mut out = vec![]; for t6641 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t6641;
out.push(py::float(&v_v.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss_binom_counts(mut v_m: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_deg = py::mul(&v_m.clone(), &v_n.clone())?;
    v_c = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&py::add(&py::add(&v_deg.clone(), &v_m.clone())?, &v_n.clone())?, &Py::Int(1i64))?)?;
    py::setitem(&v_c.clone(), &Py::Int(0i64), Py::Int(1i64))?;
    for t3082 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t3082;
        v_s = py::add(&v_n.clone(), &v_i.clone())?;
        for t3083 in py::range_iter(&[(py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?).i()?, (py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_k = t3083;
            let t3084 = v_c.clone(); let t3085 = v_k.clone();
            py::setitem(&t3084, &t3085, py::sub(&py::getitem(&t3084, &t3085)?, &py::getitem(&v_c.clone(), &py::sub(&v_k.clone(), &v_s.clone())?)?)?)?;
        }
        for t3086 in py::range_iter(&[(v_i.clone()).i()?, (py::len(&v_c.clone())?).i()?]) {
            v_k = t3086;
            let t3087 = v_c.clone(); let t3088 = v_k.clone();
            py::setitem(&t3087, &t3088, py::iadd(&py::getitem(&t3087, &t3088)?, &py::getitem(&v_c.clone(), &py::sub(&v_k.clone(), &v_i.clone())?)?)?)?;
        }
    }
    return Ok(py::slice(&v_c.clone(), None, Some((py::add(&v_deg.clone(), &Py::Int(1i64))?).i()?), None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__groups(mut v_gs: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut out = vec![]; for t3759 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t3759;
if ({ let t3760 = v_g.clone(); Py::Bool({ let t3761 = Py::None; if !py::is(&t3760, &t3761) { true } else { false } }) }).truthy() { out.push(f__finite(v_g.clone(), Py::str("each group"))?); }} Py::list(out) };
    if ({ let t3765 = py::len(&v_out.clone())?; Py::Bool({ let t3766 = Py::Int(2i64); if py::lt(&t3765, &t3766)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least two groups are required"))));
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ibeta(mut v_a: Py, mut v_b: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_lf: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t6304 = v_x.clone(); Py::Bool({ let t6305 = Py::Float(0.0); if py::le(&t6304, &t6305)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    if ({ let t6306 = v_y.clone(); Py::Bool({ let t6307 = Py::Float(0.0); if py::le(&t6306, &t6307)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]));
    }
    v_lf = py::add(&py::add(&py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_a.clone(), &v_b.clone())?)?, &py::math1("lgamma", &v_a.clone())?)?, &py::math1("lgamma", &v_b.clone())?)?, &py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?)?, &py::mul(&v_b.clone(), &py::math1("log", &v_y.clone())?)?)?;
    if ({ let t6308 = v_x.clone(); Py::Bool({ let t6309 = py::truediv(&py::add(&v_a.clone(), &Py::Float(1.0))?, &py::add(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Float(2.0))?)?; if py::lt(&t6308, &t6309)? { true } else { false } }) }).truthy() {
        v_v = py::truediv(&py::mul(&py::math1("exp", &v_lf.clone())?, &f__bcf(v_a.clone(), v_b.clone(), v_x.clone())?)?, &v_a.clone())?;
        return Ok(Py::tuple(vec![v_v.clone(), py::sub(&Py::Float(1.0), &v_v.clone())?]));
    }
    v_v = py::truediv(&py::mul(&py::math1("exp", &v_lf.clone())?, &f__bcf(v_b.clone(), v_a.clone(), v_y.clone())?)?, &v_b.clone())?;
    return Ok(Py::tuple(vec![py::sub(&Py::Float(1.0), &v_v.clone())?, v_v.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__igamma(mut v_a: Py, mut v_x: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lf: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t3217 = v_x.clone(); Py::Bool({ let t3218 = Py::Float(0.0); if py::le(&t3217, &t3218)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    v_lf = py::sub(&py::sub(&py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?, &v_x.clone())?, &py::math1("lgamma", &v_a.clone())?)?;
    if ({ let t3219 = v_x.clone(); Py::Bool({ let t3220 = py::add(&v_a.clone(), &Py::Float(1.0))?; if py::lt(&t3219, &t3220)? { true } else { false } }) }).truthy() {
        let t3221 = (Py::tuple(vec![v_a.clone(), py::truediv(&Py::Float(1.0), &v_a.clone())?])).items()?;
        if t3221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ap = t3221[0].clone();
        v_s = t3221[1].clone();
        v_d = v_s.clone();
        for t3222 in py::range_iter(&[(Py::Int(300000i64)).i()?]) {
            v__ = t3222;
            v_ap = py::iadd(&v_ap, &Py::Float(1.0))?;
            v_d = py::mul(&v_d, &py::truediv(&v_x.clone(), &v_ap.clone())?)?;
            v_s = py::iadd(&v_s, &v_d.clone())?;
            if ({ let t3223 = py::abs(&v_d.clone())?; Py::Bool({ let t3224 = py::mul(&py::abs(&v_s.clone())?, &Py::Float(1e-17))?; if py::lt(&t3223, &t3224)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_p = py::mul(&v_s.clone(), &py::math1("exp", &v_lf.clone())?)?;
        return Ok(Py::tuple(vec![v_p.clone(), py::sub(&Py::Float(1.0), &v_p.clone())?]));
    }
    v_b = py::sub(&py::add(&v_x.clone(), &Py::Float(1.0))?, &v_a.clone())?;
    let t3225 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &c__TINY()?)?, py::truediv(&Py::Float(1.0), &v_b.clone())?])).items()?;
    if t3225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3225[0].clone();
    v_d = t3225[1].clone();
    v_h = v_d.clone();
    for t3226 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(300000i64)).i()?]) {
        v_i = t3226;
        v_an = py::mul(&py::neg(&v_i.clone())?, &py::sub(&v_i.clone(), &v_a.clone())?)?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?;
        v_d = (if ({ let t3227 = py::abs(&v_d.clone())?; Py::Bool({ let t3228 = c__TINY()?; if py::lt(&t3227, &t3228)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t3229 = py::abs(&v_c.clone())?; Py::Bool({ let t3230 = c__TINY()?; if py::lt(&t3229, &t3230)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t3231 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t3232 = Py::Float(4e-16); if py::lt(&t3231, &t3232)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_q = py::mul(&py::math1("exp", &v_lf.clone())?, &v_h.clone())?;
    return Ok(Py::tuple(vec![py::sub(&Py::Float(1.0), &v_q.clone())?, v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__matrix_ok(mut v_m: Py, mut v_name: Py, mut v_min_rows: Py, mut v_min_cols: Py) -> R {
    if ({ let t6632 = py::not(&v_m.clone()); if t6632.truthy() { t6632 } else { Py::Bool(({ let mut out = vec![]; for t6629 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t6629;
out.push({ let t6630 = py::len(&v_r.clone())?; Py::Bool({ let t6631 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; if py::ne(&t6630, &t6631) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular".to_string()].concat()))));
    }
    if ({ let t6637 = { let t6633 = py::len(&v_m.clone())?; Py::Bool({ let t6634 = v_min_rows.clone(); if py::lt(&t6633, &t6634)? { true } else { false } }) }; if t6637.truthy() { t6637 } else { { let t6635 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t6636 = v_min_cols.clone(); if py::lt(&t6635, &t6636)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_min_rows.clone(), "")?, " rows and ".to_string(), py::format(&v_min_cols.clone(), "")?, " columns".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t6638 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t6638;
out.push(f__finite(v_r.clone(), v_name.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::fsum(&v_x.clone())?, &py::len(&v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__median(mut v_x: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_s.clone())?;
    return Ok((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::getitem(&v_s.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)? } else { py::truediv(&py::add(&py::getitem(&v_s.clone(), &py::sub(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::getitem(&v_s.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)?)?, &Py::Float(2.0))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mw_core(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let t3129 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("x"))?, f__finite(v_y.clone(), Py::str("y"))?])).items()?;
    if t3129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3129[0].clone();
    v_y = t3129[1].clone();
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the first sample"))?;
    let _ = f__need(v_y.clone(), Py::Int(1i64), Py::str("the second sample"))?;
    let t3132 = (Py::tuple(vec![py::len(&v_x.clone())?, py::len(&v_y.clone())?])).items()?;
    if t3132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t3132[0].clone();
    v_n2 = t3132[1].clone();
    v_r = f__ranks(py::add(&v_x.clone(), &v_y.clone())?)?;
    v_u1 = py::sub(&py::fsum(&py::slice(&v_r.clone(), None, Some((v_n1.clone()).i()?), None)?)?, &py::truediv(&py::mul(&v_n1.clone(), &py::add(&v_n1.clone(), &Py::Int(1i64))?)?, &Py::Float(2.0))?)?;
    return Ok(Py::tuple(vec![v_x.clone(), v_y.clone(), v_n1.clone(), v_n2.clone(), v_u1.clone(), py::sub(&py::mul(&v_n1.clone(), &v_n2.clone())?, &v_u1.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_x: Py, mut v_n: Py, mut v_name: Py) -> R {
    if ({ let t6288 = py::len(&v_x.clone())?; Py::Bool({ let t6289 = v_n.clone(); if py::lt(&t6288, &t6289)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n.clone(), "")?, " values".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair(mut v_x: Py, mut v_y: Py, mut v_n: Py) -> R {
    let t5548 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("x"))?, f__finite(v_y.clone(), Py::str("y"))?])).items()?;
    if t5548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t5548[0].clone();
    v_y = t5548[1].clone();
    if ({ let t5549 = py::len(&v_x.clone())?; Py::Bool({ let t5550 = py::len(&v_y.clone())?; if py::ne(&t5549, &t5550) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y must have the same number of values (paired data)"))));
    }
    let _ = f__need(v_x.clone(), v_n.clone(), Py::str("the paired data"))?;
    return Ok(Py::tuple(vec![v_x.clone(), v_y.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pearson(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let t5554 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t5554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t5554[0].clone();
    v_my = t5554[1].clone();
    v_sxy = py::fsum(&{ let mut out = vec![]; for t5555 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5556 = (t5555).items()?;
if t5556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5556[0].clone();
v_b = t5556[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    v_sxx = py::fsum(&{ let mut out = vec![]; for t5557 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t5557;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_syy = py::fsum(&{ let mut out = vec![]; for t5558 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t5558;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    if ({ let t5563 = { let t5559 = v_sxx.clone(); Py::Bool({ let t5560 = Py::Int(0i64); if py::eq(&t5559, &t5560) { true } else { false } }) }; if t5563.truthy() { t5563 } else { { let t5561 = v_syy.clone(); Py::Bool({ let t5562 = Py::Int(0i64); if py::eq(&t5561, &t5562) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a variable is constant: the correlation is undefined"))));
    }
    v_r = py::truediv(&v_sxy.clone(), &py::math1("sqrt", &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?;
    return Ok(py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), v_r.clone()], false)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phi(mut v_z: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_z.clone())?, &c__SQRT2()?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phic(mut v_z: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_z.clone(), &c__SQRT2()?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_args(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(Py::tuple(vec![f__pos(v_lam.clone(), Py::str("lambda"))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_cdf(mut v_k: Py, mut v_lam: Py) -> R {
    return Ok((if ({ let t4213 = v_k.clone(); Py::Bool({ let t4214 = Py::Int(0i64); if py::ge(&t4213, &t4214)? { true } else { false } }) }).truthy() { py::minmax(&[Py::Float(1.0), py::fsum(&{ let mut out = vec![]; for t4215 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t4215;
out.push(f__pois_pmf(v_j.clone(), v_lam.clone())?); } Py::list(out) })?], false)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_pmf(mut v_k: Py, mut v_lam: Py) -> R {
    return Ok((if ({ let t4265 = v_k.clone(); Py::Bool({ let t4266 = Py::Int(0i64); if py::ge(&t4265, &t4266)? { true } else { false } }) }).truthy() { py::math1("exp", &py::sub(&py::sub(&py::mul(&v_k.clone(), &py::math1("log", &v_lam.clone())?)?, &v_lam.clone())?, &py::math1("lgamma", &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_sf_ge(mut v_k: Py, mut v_lam: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_terms: Py = Py::None;
    if ({ let t4151 = v_k.clone(); Py::Bool({ let t4152 = Py::Int(0i64); if py::le(&t4151, &t4152)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t4153 = v_k.clone(); Py::Bool({ let t4154 = v_lam.clone(); if py::le(&t4153, &t4154)? { true } else { false } }) }).truthy() {
        return Ok(py::minmax(&[Py::Float(0.0), py::sub(&Py::Float(1.0), &f__pois_cdf(py::sub(&v_k.clone(), &Py::Int(1i64))?, v_lam.clone())?)?], true)?);
    }
    let t4160 = (Py::tuple(vec![Py::list(vec![]), v_k.clone()])).items()?;
    if t4160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_terms = t4160[0].clone();
    v_j = t4160[1].clone();
    while (Py::Bool(true)).truthy() {
        v_t = f__pois_pmf(v_j.clone(), v_lam.clone())?;
        let _ = py::method(&v_terms.clone(), "append", &[v_t.clone()])?;
        if ({ let t4165 = { let t4161 = v_t.clone(); Py::Bool({ let t4162 = py::mul(&Py::Float(1e-18), &py::fsum(&v_terms.clone())?)?; if py::lt(&t4161, &t4162)? { true } else { false } }) }; if t4165.truthy() { t4165 } else { { let t4163 = v_t.clone(); Py::Bool({ let t4164 = Py::Float(0.0); if py::eq(&t4163, &t4164) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_j = py::iadd(&v_j, &Py::Int(1i64))?;
    }
    return Ok(py::minmax(&[Py::Float(1.0), py::fsum(&v_terms.clone())?], false)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t6779 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t6779.truthy() { t6779 } else { { let t6778 = py::not(&py::math1("isfinite", &v_v.clone())?); if t6778.truthy() { t6778 } else { { let t6776 = v_v.clone(); Py::Bool({ let t6777 = Py::Int(0i64); if py::le(&t6776, &t6777)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__positive_x(mut v_x: Py) -> R {
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    if (Py::Bool(({ let mut out = vec![]; for t2362 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2362;
out.push({ let t2363 = v_v.clone(); Py::Bool({ let t2364 = Py::Int(0i64); if py::le(&t2363, &t2364)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all values must be positive"))));
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prob(mut v_p: Py, mut v_name: Py, mut v_open_: Py) -> R {
    if ({ let t6689 = py::not(&Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t6689.truthy() { t6689 } else { { let t6688 = py::not(&py::math1("isfinite", &v_p.clone())?); if t6688.truthy() { t6688 } else { (if (v_open_.clone()).truthy() { py::not(&{ let t6682 = Py::Int(0i64); Py::Bool({ let t6683 = v_p.clone(); if py::lt(&t6682, &t6683)? { { let t6684 = Py::Int(1i64); if py::lt(&t6683, &t6684)? { true } else { false } } } else { false } }) }) } else { py::not(&{ let t6685 = Py::Int(0i64); Py::Bool({ let t6686 = v_p.clone(); if py::le(&t6685, &t6686)? { { let t6687 = Py::Int(1i64); if py::le(&t6686, &t6687)? { true } else { false } } } else { false } }) }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a probability ".to_string(), py::format(&(if (v_open_.clone()).truthy() { Py::str("strictly ") } else { Py::str("") }), "")?, "between 0 and 1".to_string()].concat()))));
    }
    return Ok(py::float(&v_p.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prop_args(mut v_x: Py, mut v_n: Py) -> R {
    let t6527 = (Py::tuple(vec![f__whole(v_x.clone(), Py::str("x"), Py::Int(0i64))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?])).items()?;
    if t6527.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t6527[0].clone();
    v_n = t6527[1].clone();
    if ({ let t6528 = v_x.clone(); Py::Bool({ let t6529 = v_n.clone(); if py::gt(&t6528, &t6529)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of successes x cannot exceed n"))));
    }
    return Ok(Py::tuple(vec![v_x.clone(), v_n.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pw_args(mut v_d: Py, mut v_alpha: Py, mut v_alternative: Py) -> R {
    let _ = f__check_alt(v_alternative.clone())?;
    if ({ let t5143 = py::not(&py::math1("isfinite", &v_d.clone())?); if t5143.truthy() { t5143 } else { { let t5141 = v_d.clone(); Py::Bool({ let t5142 = Py::Int(0i64); if py::eq(&t5141, &t5142) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the effect size must be a non-zero number"))));
    }
    if ({ let t5154 = { let t5148 = { let t5144 = v_alternative.clone(); Py::Bool({ let t5145 = Py::str("greater"); if py::eq(&t5144, &t5145) { true } else { false } }) }; if !t5148.truthy() { t5148 } else { { let t5146 = v_d.clone(); Py::Bool({ let t5147 = Py::Int(0i64); if py::lt(&t5146, &t5147)? { true } else { false } }) } } }; if t5154.truthy() { t5154 } else { { let t5153 = { let t5149 = v_alternative.clone(); Py::Bool({ let t5150 = Py::str("less"); if py::eq(&t5149, &t5150) { true } else { false } }) }; if !t5153.truthy() { t5153 } else { { let t5151 = v_d.clone(); Py::Bool({ let t5152 = Py::Int(0i64); if py::gt(&t5151, &t5152)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the effect size has the opposite sign of the one-sided alternative"))));
    }
    return Ok(f__prob(v_alpha.clone(), Py::str("alpha"), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quantile(mut v_x: Py, mut v_p: Py, mut v_method: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_s.clone())?;
    v_h = (if ({ let t4109 = v_method.clone(); Py::Bool({ let t4110 = Py::str("linear"); if py::eq(&t4109, &t4110) { true } else { false } }) }).truthy() { py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_p.clone())? } else { py::sub(&py::mul(&py::add(&v_n.clone(), &Py::Int(1i64))?, &v_p.clone())?, &Py::Float(1.0))? });
    if ({ let t4111 = v_h.clone(); Py::Bool({ let t4112 = Py::Int(0i64); if py::le(&t4111, &t4112)? { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_s.clone(), &Py::Int(0i64))?);
    }
    if ({ let t4113 = v_h.clone(); Py::Bool({ let t4114 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::ge(&t4113, &t4114)? { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?);
    }
    v_lo = py::int(&py::math1("floor", &v_h.clone())?)?;
    return Ok(py::add(&py::getitem(&v_s.clone(), &v_lo.clone())?, &py::mul(&py::sub(&v_h.clone(), &v_lo.clone())?, &py::sub(&py::getitem(&v_s.clone(), &py::add(&v_lo.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_s.clone(), &v_lo.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ranks(mut v_x: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_idx = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_x.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_x.clone(), &v_i.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_r = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::len(&v_x.clone())?)?;
    v_i = Py::Int(0i64);
    while ({ let t6457 = v_i.clone(); Py::Bool({ let t6458 = py::len(&v_x.clone())?; if py::lt(&t6457, &t6458)? { true } else { false } }) }).truthy() {
        v_j = v_i.clone();
        while ({ let t6455 = { let t6451 = py::add(&v_j.clone(), &Py::Int(1i64))?; Py::Bool({ let t6452 = py::len(&v_x.clone())?; if py::lt(&t6451, &t6452)? { true } else { false } }) }; if !t6455.truthy() { t6455 } else { { let t6453 = py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t6454 = py::getitem(&v_x.clone(), &py::getitem(&v_idx.clone(), &v_i.clone())?)?; if py::eq(&t6453, &t6454) { true } else { false } }) } } }).truthy() {
            v_j = py::iadd(&v_j, &Py::Int(1i64))?;
        }
        for t6456 in py::range_iter(&[(v_i.clone()).i()?, (py::add(&v_j.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t6456;
            py::setitem(&v_r.clone(), &py::getitem(&v_idx.clone(), &v_k.clone())?, py::add(&py::truediv(&py::add(&v_i.clone(), &v_j.clone())?, &Py::Float(2.0))?, &Py::Float(1.0))?)?;
        }
        v_i = py::add(&v_j.clone(), &Py::Int(1i64))?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rels(mut v_r: Py) -> R {
    v_r = f__finite(v_r.clone(), Py::str("component reliabilities"))?;
    if (Py::Bool(({ let mut out = vec![]; for t5311 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t5311;
out.push(py::not(&{ let t5312 = Py::Int(0i64); Py::Bool({ let t5313 = v_v.clone(); if py::le(&t5312, &t5313)? { { let t5314 = Py::Int(1i64); if py::le(&t5313, &t5314)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each reliability must be between 0 and 1"))));
    }
    if ({ let t5315 = py::len(&v_r.clone())?; Py::Bool({ let t5316 = Py::Int(20i64); if py::gt(&t5315, &t5316)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 20 components"))));
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd(mut v_x: Py, mut v_ddof: Py) -> R {
    return Ok(py::math1("sqrt", &f__var(v_x.clone(), v_ddof.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__signed(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rp: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("x"))?;
    if ({ let t6442 = v_y.clone(); Py::Bool({ let t6443 = Py::None; if !py::is(&t6442, &t6443) { true } else { false } }) }).truthy() {
        v_y = f__finite(v_y.clone(), Py::str("y"))?;
        if ({ let t6444 = py::len(&v_x.clone())?; Py::Bool({ let t6445 = py::len(&v_y.clone())?; if py::ne(&t6444, &t6445) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("x and y must have the same length (paired data)"))));
        }
        v_d = { let mut out = vec![]; for t6446 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6447 = (t6446).items()?;
if t6447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6447[0].clone();
v_b = t6447[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    } else {
        v_d = v_x.clone();
    }
    v_d = { let mut out = vec![]; for t6448 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t6448;
if ({ let t6449 = v_v.clone(); Py::Bool({ let t6450 = Py::Int(0i64); if py::ne(&t6449, &t6450) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    if (py::not(&v_d.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all differences are zero"))));
    }
    v_r = f__ranks({ let mut out = vec![]; for t6459 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t6459;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?;
    v_rp = py::fsum(&{ let mut out = vec![]; for t6460 in py::iter(&(py::zip(&[v_r.clone(), v_d.clone()])?))? { let mut v_ri: Py; let mut v_v: Py; 
let t6461 = (t6460).items()?;
if t6461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ri = t6461[0].clone();
v_v = t6461[1].clone();
if ({ let t6462 = v_v.clone(); Py::Bool({ let t6463 = Py::Int(0i64); if py::gt(&t6462, &t6463)? { true } else { false } }) }).truthy() { out.push(v_ri.clone()); }} Py::list(out) })?;
    return Ok(Py::tuple(vec![v_d.clone(), v_r.clone(), v_rp.clone(), py::sub(&py::truediv(&py::mul(&py::len(&v_d.clone())?, &py::add(&py::len(&v_d.clone())?, &Py::Int(1i64))?)?, &Py::Float(2.0))?, &v_rp.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__simple_fit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_b1: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let t5428 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(3i64))?).items()?;
    if t5428.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t5428[0].clone();
    v_y = t5428[1].clone();
    let t5429 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t5429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t5429[0].clone();
    v_my = t5429[1].clone();
    v_sxx = py::fsum(&{ let mut out = vec![]; for t5430 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t5430;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    if ({ let t5431 = v_sxx.clone(); Py::Bool({ let t5432 = Py::Int(0i64); if py::eq(&t5431, &t5432) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all x values are equal: the slope is undefined"))));
    }
    v_sxy = py::fsum(&{ let mut out = vec![]; for t5433 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5434 = (t5433).items()?;
if t5434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5434[0].clone();
v_b = t5434[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    v_b1 = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::tuple(vec![v_x.clone(), v_y.clone(), v_mx.clone(), v_my.clone(), v_sxx.clone(), v_b1.clone(), py::sub(&v_my.clone(), &py::mul(&v_b1.clone(), &v_mx.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sm_args(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py, mut v_n: Py) -> R {
    return Ok(py::truediv(&py::sub(&v_x.clone(), &v_mu.clone())?, &py::truediv(&f__pos(v_sigma.clone(), Py::str("sigma"))?, &py::math1("sqrt", &f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t3503 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3503;
out.push(py::add(&Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &py::getitem(&v_A.clone(), &v_i.clone())?)?).items()?), &Py::list(vec![py::float(&py::getitem(&v_b.clone(), &v_i.clone())?)?]))?); } Py::list(out) };
    v_scale = { let t3505 = { let t = { let mut out = vec![]; for t3504 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3504;
out.push(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t3505.truthy() { t3505 } else { Py::Float(1.0) } };
    for t3506 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t3506;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t3507 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t3508 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t3507, &t3508)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the predictors are collinear (the design matrix is singular)"))));
        }
        let t3509 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t3509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t3509[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t3509[1].clone())?;
        for t3510 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t3510;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                for t3511 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t3511;
                    let t3512 = py::getitem(&v_M.clone(), &v_r.clone())?; let t3513 = v_k.clone();
                    py::setitem(&t3512, &t3513, py::sub(&py::getitem(&t3512, &t3513)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t3514 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_i = t3514;
        py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_n.clone())?, &py::fsum(&{ let mut out = vec![]; for t3515 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3515;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ss(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    v_m = f__mean(v_x.clone())?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t6647 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t6647;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__subgroups(mut v_data: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = f__matrix_ok(v_data.clone(), Py::str("the subgroup table"), Py::Int(2i64), Py::Int(2i64))?;
    if ({ let t6642 = py::len(&py::getitem(&v_d.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t6643 = Py::Int(10i64); if py::gt(&t6642, &t6643)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("subgroup sizes from 2 to 10 are supported"))));
    }
    return Ok(Py::tuple(vec![v_d.clone(), py::len(&py::getitem(&v_d.clone(), &Py::Int(0i64))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__surv_data(mut v_times: Py, mut v_events: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2963 = (Py::tuple(vec![f__finite(v_times.clone(), Py::str("times"))?, f__finite(v_events.clone(), Py::str("events"))?])).items()?;
    if t2963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t2963[0].clone();
    v_e = t2963[1].clone();
    if ({ let t2964 = py::len(&v_t.clone())?; Py::Bool({ let t2965 = py::len(&v_e.clone())?; if py::ne(&t2964, &t2965) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one event indicator per time"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2966 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t2966;
out.push({ let t2967 = v_v.clone(); Py::Bool({ let t2968 = Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]); if !py::contains(&t2968, &t2967)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("event indicators must be 1 (event) or 0 (censored)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2969 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t2969;
out.push({ let t2970 = v_v.clone(); Py::Bool({ let t2971 = Py::Int(0i64); if py::lt(&t2970, &t2971)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("times must be >= 0"))));
    }
    return Ok(Py::tuple(vec![v_t.clone(), v_e.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_cdf(mut v_t: Py, mut v_df: Py) -> R {
    let mut v_half: Py = Py::None;
    if ({ let t6327 = v_t.clone(); Py::Bool({ let t6328 = Py::Int(0i64); if py::eq(&t6327, &t6328) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.5));
    }
    v_half = py::mul(&Py::Float(0.5), &f__t_two_sided(v_t.clone(), v_df.clone())?)?;
    return Ok((if ({ let t6329 = v_t.clone(); Py::Bool({ let t6330 = Py::Int(0i64); if py::gt(&t6329, &t6330)? { true } else { false } }) }).truthy() { py::sub(&Py::Float(1.0), &v_half.clone())? } else { v_half.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_interval(mut v_m: Py, mut v_s: Py, mut v_n: Py, mut v_confidence: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f__tc(v_confidence.clone(), py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    v_e = py::truediv(&py::mul(&v_t.clone(), &v_s.clone())?, &py::math1("sqrt", &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("lower"), py::sub(&v_m.clone(), &v_e.clone())?), (Py::str("upper"), py::add(&v_m.clone(), &v_e.clone())?), (Py::str("margin_of_error"), v_e.clone()), (Py::str("critical_value"), v_t.clone()), (Py::str("df"), py::sub(&v_n.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_out(mut v_d: Py, mut v_se: Py, mut v_df: Py, mut v_alternative: Py) -> R {
    let mut v_t: Py = Py::None;
    v_t = py::truediv(&v_d.clone(), &v_se.clone())?;
    return Ok(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("df"), v_df.clone()), (Py::str("p_value"), f__alt_p(v_t.clone(), v_alternative.clone(), { let v_alternative = v_alternative.clone(); let v_d = v_d.clone(); let v_df = v_df.clone(); let v_se = v_se.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_sf(v_v.clone(), v_df.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_alternative = v_alternative.clone(); let v_d = v_d.clone(); let v_df = v_df.clone(); let v_se = v_se.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_cdf(v_v.clone(), v_df.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?), (Py::str("difference"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_ppf(mut v_p: Py, mut v_df: Py) -> R {
    let mut v_q: Py = Py::None;
    if (py::not(&{ let t5686 = Py::Float(0.0); Py::Bool({ let t5687 = v_p.clone(); if py::lt(&t5686, &t5687)? { { let t5688 = Py::Float(1.0); if py::lt(&t5687, &t5688)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    if ({ let t5689 = v_p.clone(); Py::Bool({ let t5690 = Py::Float(0.5); if py::eq(&t5689, &t5690) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5691 = v_p.clone(); Py::Bool({ let t5692 = Py::Float(0.5); if py::lt(&t5691, &t5692)? { true } else { false } }) }).truthy() {
        return Ok(py::neg(&f__t_ppf(py::sub(&Py::Float(1.0), &v_p.clone())?, v_df.clone())?)?);
    }
    v_q = py::sub(&Py::Float(1.0), &v_p.clone())?;
    return Ok(f__bisect_decreasing({ let v_df = v_df.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&Py::Float(0.5), &f__t_two_sided(v_t.clone(), v_df.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_q.clone(), Py::Float(0.0), Py::Float(1.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_sf(mut v_t: Py, mut v_df: Py) -> R {
    let mut v_half: Py = Py::None;
    if ({ let t6302 = v_t.clone(); Py::Bool({ let t6303 = Py::Int(0i64); if py::eq(&t6302, &t6303) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.5));
    }
    v_half = py::mul(&Py::Float(0.5), &f__t_two_sided(v_t.clone(), v_df.clone())?)?;
    return Ok((if ({ let t6325 = v_t.clone(); Py::Bool({ let t6326 = Py::Int(0i64); if py::gt(&t6325, &t6326)? { true } else { false } }) }).truthy() { v_half.clone() } else { py::sub(&Py::Float(1.0), &v_half.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_two_sided(mut v_t: Py, mut v_df: Py) -> R {
    let mut v_t2: Py = Py::None;
    v_t2 = py::mul(&v_t.clone(), &v_t.clone())?;
    return Ok(py::getitem(&f__ibeta(py::truediv(&v_df.clone(), &Py::Float(2.0))?, Py::Float(0.5), py::truediv(&v_df.clone(), &py::add(&v_df.clone(), &v_t2.clone())?)?, py::truediv(&v_t2.clone(), &py::add(&v_df.clone(), &v_t2.clone())?)?)?, &Py::Int(0i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tc(mut v_conf: Py, mut v_df: Py) -> R {
    return Ok(f__t_ppf(py::sub(&Py::Float(1.0), &py::truediv(&py::sub(&Py::Float(1.0), &f__conf(v_conf.clone())?)?, &Py::Float(2.0))?)?, v_df.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tie_counts(mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_c = Py::dict(vec![]);
    for t6465 in py::iter(&(v_x.clone()))? {
        v_v = t6465;
        py::setitem(&v_c.clone(), &v_v.clone(), py::add(&py::method(&v_c.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    return Ok({ let mut out = vec![]; for t6466 in py::iter(&(py::method(&v_c.clone(), "values", &[])?))? { let mut v_t: Py; 
v_t = t6466;
if ({ let t6467 = v_t.clone(); Py::Bool({ let t6468 = Py::Int(1i64); if py::gt(&t6467, &t6468)? { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_by_two(mut v_table: Py) -> R {
    let mut v_t: Py = Py::None;
    v_t = f__matrix_ok(v_table.clone(), Py::str("the 2 x 2 table"), Py::Int(2i64), Py::Int(2i64))?;
    if ({ let t3201 = { let t3197 = py::len(&v_t.clone())?; Py::Bool({ let t3198 = Py::Int(2i64); if py::ne(&t3197, &t3198) { true } else { false } }) }; if t3201.truthy() { t3201 } else { { let t3199 = py::len(&py::getitem(&v_t.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3200 = Py::Int(2i64); if py::ne(&t3199, &t3200) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a 2 x 2 table is required"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3202 in py::iter(&(v_t.clone()))? { let mut v_r: Py; 
v_r = t3202;
for t3203 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3203;
out.push({ let t3208 = { let t3204 = v_v.clone(); Py::Bool({ let t3205 = Py::Int(0i64); if py::lt(&t3204, &t3205)? { true } else { false } }) }; if t3208.truthy() { t3208 } else { { let t3206 = v_v.clone(); Py::Bool({ let t3207 = py::int(&v_v.clone())?; if py::ne(&t3206, &t3207) { true } else { false } }) } } }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("counts must be whole numbers >= 0"))));
    }
    return Ok({ let mut out = vec![]; for t3209 in py::iter(&(v_t.clone()))? { let mut v_r: Py; 
v_r = t3209;
out.push({ let mut out = vec![]; for t3210 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3210;
out.push(py::int(&v_v.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_sample(mut v_x: Py, mut v_y: Py, mut v_variance: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_sp2: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let t6287 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("x"))?, f__finite(v_y.clone(), Py::str("y"))?])).items()?;
    if t6287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t6287[0].clone();
    v_y = t6287[1].clone();
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the first sample"))?;
    let _ = f__need(v_y.clone(), Py::Int(2i64), Py::str("the second sample"))?;
    let t6291 = (Py::tuple(vec![py::len(&v_x.clone())?, py::len(&v_y.clone())?, f__var(v_x.clone(), Py::Int(1i64))?, f__var(v_y.clone(), Py::Int(1i64))?])).items()?;
    if t6291.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t6291[0].clone();
    v_n2 = t6291[1].clone();
    v_v1 = t6291[2].clone();
    v_v2 = t6291[3].clone();
    v_d = py::sub(&f__mean(v_x.clone())?, &f__mean(v_y.clone())?)?;
    if ({ let t6292 = v_variance.clone(); Py::Bool({ let t6293 = Py::str("pooled"); if py::eq(&t6292, &t6293) { true } else { false } }) }).truthy() {
        v_df = py::sub(&py::add(&v_n1.clone(), &v_n2.clone())?, &Py::Int(2i64))?;
        v_sp2 = py::truediv(&py::add(&py::mul(&py::sub(&v_n1.clone(), &Py::Int(1i64))?, &v_v1.clone())?, &py::mul(&py::sub(&v_n2.clone(), &Py::Int(1i64))?, &v_v2.clone())?)?, &v_df.clone())?;
        v_se = py::math1("sqrt", &py::mul(&v_sp2.clone(), &py::add(&py::truediv(&Py::Int(1i64), &v_n1.clone())?, &py::truediv(&Py::Int(1i64), &v_n2.clone())?)?)?)?;
    } else {
        let t6294 = (Py::tuple(vec![py::truediv(&v_v1.clone(), &v_n1.clone())?, py::truediv(&v_v2.clone(), &v_n2.clone())?])).items()?;
        if t6294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t6294[0].clone();
        v_b = t6294[1].clone();
        v_se = py::math1("sqrt", &py::add(&v_a.clone(), &v_b.clone())?)?;
        v_df = py::truediv(&py::pow(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &py::add(&py::truediv(&py::mul(&v_a.clone(), &v_a.clone())?, &py::sub(&v_n1.clone(), &Py::Int(1i64))?)?, &py::truediv(&py::mul(&v_b.clone(), &v_b.clone())?, &py::sub(&v_n2.clone(), &Py::Int(1i64))?)?)?)?;
    }
    if ({ let t6295 = v_se.clone(); Py::Bool({ let t6296 = Py::Int(0i64); if py::eq(&t6295, &t6296) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both samples have zero variance"))));
    }
    return Ok(Py::tuple(vec![v_d.clone(), v_se.clone(), v_df.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__var(mut v_x: Py, mut v_ddof: Py) -> R {
    return Ok(py::truediv(&f__ss(v_x.clone())?, &py::sub(&py::len(&v_x.clone())?, &v_ddof.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py, mut v_lo: Py) -> R {
    if ({ let t6786 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t6786.truthy() { t6786 } else { { let t6785 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t6785.truthy() { t6785 } else { { let t6784 = { let t6780 = py::float(&v_v.clone())?; Py::Bool({ let t6781 = py::int(&py::float(&v_v.clone())?)?; if py::ne(&t6780, &t6781) { true } else { false } }) }; if t6784.truthy() { t6784 } else { { let t6782 = py::int(&v_v.clone())?; Py::Bool({ let t6783 = v_lo.clone(); if py::lt(&t6782, &t6783)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number >= ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    return Ok(py::int(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__z(mut v_p: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    if (py::not(&{ let t6694 = Py::Float(0.0); Py::Bool({ let t6695 = v_p.clone(); if py::lt(&t6694, &t6695)? { { let t6696 = Py::Float(1.0); if py::lt(&t6695, &t6696)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    if ({ let t6697 = v_p.clone(); Py::Bool({ let t6698 = Py::Float(0.5); if py::gt(&t6697, &t6698)? { true } else { false } }) }).truthy() {
        return Ok(py::neg(&f__z(py::sub(&Py::Float(1.0), &v_p.clone())?)?)?);
    }
    v_a = Py::tuple(vec![py::neg(&Py::Float(39.69683028665376))?, Py::Float(220.9460984245205), py::neg(&Py::Float(275.9285104469687))?, Py::Float(138.357751867269), py::neg(&Py::Float(30.66479806614716))?, Py::Float(2.506628277459239)]);
    v_b = Py::tuple(vec![py::neg(&Py::Float(54.47609879822406))?, Py::Float(161.5858368580409), py::neg(&Py::Float(155.6989798598866))?, Py::Float(66.80131188771972), py::neg(&Py::Float(13.28068155288572))?]);
    v_c = Py::tuple(vec![py::neg(&Py::Float(0.007784894002430293))?, py::neg(&Py::Float(0.3223964580411365))?, py::neg(&Py::Float(2.400758277161838))?, py::neg(&Py::Float(2.549732539343734))?, Py::Float(4.374664141464968), Py::Float(2.938163982698783)]);
    v_d = Py::tuple(vec![Py::Float(0.007784695709041462), Py::Float(0.3224671290700398), Py::Float(2.445134137142996), Py::Float(3.754408661907416)]);
    if ({ let t6699 = v_p.clone(); Py::Bool({ let t6700 = Py::Float(0.02425); if py::lt(&t6699, &t6700)? { true } else { false } }) }).truthy() {
        v_q = py::math1("sqrt", &py::mul(&py::neg(&Py::Float(2.0))?, &py::math1("log", &v_p.clone())?)?)?;
        v_x = py::truediv(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_q.clone())?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &v_q.clone())?, &py::getitem(&v_c.clone(), &Py::Int(2i64))?)?, &v_q.clone())?, &py::getitem(&v_c.clone(), &Py::Int(3i64))?)?, &v_q.clone())?, &py::getitem(&v_c.clone(), &Py::Int(4i64))?)?, &v_q.clone())?, &py::getitem(&v_c.clone(), &Py::Int(5i64))?)?, &py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_d.clone(), &Py::Int(0i64))?, &v_q.clone())?, &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?, &v_q.clone())?, &py::getitem(&v_d.clone(), &Py::Int(2i64))?)?, &v_q.clone())?, &py::getitem(&v_d.clone(), &Py::Int(3i64))?)?, &v_q.clone())?, &Py::Float(1.0))?)?;
    } else {
        v_q = py::sub(&v_p.clone(), &Py::Float(0.5))?;
        v_r = py::mul(&v_q.clone(), &v_q.clone())?;
        v_x = py::truediv(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &v_r.clone())?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &v_r.clone())?, &py::getitem(&v_a.clone(), &Py::Int(2i64))?)?, &v_r.clone())?, &py::getitem(&v_a.clone(), &Py::Int(3i64))?)?, &v_r.clone())?, &py::getitem(&v_a.clone(), &Py::Int(4i64))?)?, &v_r.clone())?, &py::getitem(&v_a.clone(), &Py::Int(5i64))?)?, &v_q.clone())?, &py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &v_r.clone())?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &v_r.clone())?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?, &v_r.clone())?, &py::getitem(&v_b.clone(), &Py::Int(3i64))?)?, &v_r.clone())?, &py::getitem(&v_b.clone(), &Py::Int(4i64))?)?, &v_r.clone())?, &Py::Float(1.0))?)?;
    }
    for t6701 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t6701;
        v_e = py::sub(&f__phi(v_x.clone())?, &v_p.clone())?;
        v_u = py::mul(&py::mul(&v_e.clone(), &py::math1("sqrt", &py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?)?)?, &py::math1("exp", &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Float(2.0))?)?)?;
        v_x = py::sub(&v_x.clone(), &py::truediv(&v_u.clone(), &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_x.clone(), &v_u.clone())?, &Py::Float(2.0))?)?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zc(mut v_conf: Py) -> R {
    return Ok(f__z(py::sub(&Py::Float(1.0), &py::truediv(&py::sub(&Py::Float(1.0), &f__conf(v_conf.clone())?)?, &Py::Float(2.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zpower(mut v_d: Py, mut v_nobs: Py, mut v_alpha: Py, mut v_alternative: Py) -> R {
    let mut v_c: Py = Py::None;
    if ({ let t5165 = v_alternative.clone(); Py::Bool({ let t5166 = Py::str("two-sided"); if py::eq(&t5165, &t5166) { true } else { false } }) }).truthy() {
        v_c = f__z(py::sub(&Py::Int(1i64), &py::truediv(&v_alpha.clone(), &Py::Int(2i64))?)?)?;
        return Ok(py::add(&f__phic(py::sub(&v_c.clone(), &py::mul(&v_d.clone(), &py::math1("sqrt", &v_nobs.clone())?)?)?)?, &f__phi(py::sub(&py::neg(&v_c.clone())?, &py::mul(&v_d.clone(), &py::math1("sqrt", &v_nobs.clone())?)?)?)?)?);
    }
    v_c = f__z(py::sub(&Py::Int(1i64), &v_alpha.clone())?)?;
    return Ok((if ({ let t5175 = v_alternative.clone(); Py::Bool({ let t5176 = Py::str("greater"); if py::eq(&t5175, &t5176) { true } else { false } }) }).truthy() { f__phic(py::sub(&v_c.clone(), &py::mul(&v_d.clone(), &py::math1("sqrt", &v_nobs.clone())?)?)?)? } else { f__phi(py::sub(&py::neg(&v_c.clone())?, &py::mul(&v_d.clone(), &py::math1("sqrt", &v_nobs.clone())?)?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_anova(mut v_g1: Py, mut v_g2: Py, mut v_g3: Py, mut v_g4: Py, mut v_g5: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_allv: Py = Py::None;
    let mut v_dfb: Py = Py::None;
    let mut v_dfw: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_gs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ssb: Py = Py::None;
    let mut v_ssw: Py = Py::None;
    v_gs = f__groups(Py::tuple(vec![v_g1.clone(), v_g2.clone(), v_g3.clone(), v_g4.clone(), v_g5.clone()]))?;
    for t3767 in py::iter(&(v_gs.clone()))? {
        v_g = t3767;
        let _ = f__need(v_g.clone(), Py::Int(1i64), Py::str("each group"))?;
    }
    v_allv = { let mut out = vec![]; for t3770 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t3770;
for t3771 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3771;
out.push(v_v.clone()); }} Py::list(out) };
    let t3772 = (Py::tuple(vec![py::len(&v_allv.clone())?, py::len(&v_gs.clone())?])).items()?;
    if t3772.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t3772[0].clone();
    v_k = t3772[1].clone();
    if ({ let t3773 = v_N.clone(); Py::Bool({ let t3774 = v_k.clone(); if py::le(&t3773, &t3774)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("more observations than groups are required"))));
    }
    v_gm = f__mean(v_allv.clone())?;
    v_ssb = py::fsum(&{ let mut out = vec![]; for t3775 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t3775;
out.push(py::mul(&py::len(&v_g.clone())?, &py::pow(&py::sub(&f__mean(v_g.clone())?, &v_gm.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    v_ssw = py::fsum(&{ let mut out = vec![]; for t3776 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t3776;
out.push(f__ss(v_g.clone())?); } Py::list(out) })?;
    if ({ let t3778 = v_ssw.clone(); Py::Bool({ let t3779 = Py::Int(0i64); if py::eq(&t3778, &t3779) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no variation within the groups"))));
    }
    let t3780 = (Py::tuple(vec![py::sub(&v_k.clone(), &Py::Int(1i64))?, py::sub(&v_N.clone(), &v_k.clone())?])).items()?;
    if t3780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dfb = t3780[0].clone();
    v_dfw = t3780[1].clone();
    v_f = py::truediv(&py::truediv(&v_ssb.clone(), &v_dfb.clone())?, &py::truediv(&v_ssw.clone(), &v_dfw.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("f"), v_f.clone()), (Py::str("p_value"), py::getitem(&f__f_tails(v_f.clone(), v_dfb.clone(), v_dfw.clone())?, &Py::Int(1i64))?), (Py::str("df_between"), v_dfb.clone()), (Py::str("df_within"), v_dfw.clone()), (Py::str("ss_between"), v_ssb.clone()), (Py::str("ss_within"), v_ssw.clone()), (Py::str("ms_between"), py::truediv(&v_ssb.clone(), &v_dfb.clone())?), (Py::str("ms_within"), py::truediv(&v_ssw.clone(), &v_dfw.clone())?), (Py::str("eta_squared"), py::truediv(&v_ssb.clone(), &py::add(&v_ssb.clone(), &v_ssw.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_at_least_one(mut v_p: Py, mut v_n: Py) -> R {
    v_p = f__prob(v_p.clone(), Py::str("the probability of the event on one trial"), Py::Bool(false))?;
    v_n = f__whole(v_n.clone(), Py::str("the number of trials"), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("probability"), (if ({ let t4557 = v_p.clone(); Py::Bool({ let t4558 = Py::Int(1i64); if py::lt(&t4557, &t4558)? { true } else { false } }) }).truthy() { py::neg(&py::math1("expm1", &py::mul(&v_n.clone(), &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)? } else { Py::Float(1.0) })), (Py::str("none"), py::pow(&py::sub(&Py::Float(1.0), &v_p.clone())?, &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bartlett(mut v_g1: Py, mut v_g2: Py, mut v_g3: Py, mut v_g4: Py, mut v_g5: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_vs: Py = Py::None;
    v_gs = f__groups(Py::tuple(vec![v_g1.clone(), v_g2.clone(), v_g3.clone(), v_g4.clone(), v_g5.clone()]))?;
    for t9 in py::iter(&(v_gs.clone()))? {
        v_g = t9;
        let _ = f__need(v_g.clone(), Py::Int(2i64), Py::str("each group"))?;
    }
    v_vs = { let mut out = vec![]; for t12 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t12;
out.push(f__var(v_g.clone(), Py::Int(1i64))?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t14 in py::iter(&(v_vs.clone()))? { let mut v_v: Py; 
v_v = t14;
out.push({ let t15 = v_v.clone(); Py::Bool({ let t16 = Py::Int(0i64); if py::eq(&t15, &t16) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a group has zero variance"))));
    }
    let t18 = (Py::tuple(vec![py::len(&v_gs.clone())?, py::sum(&{ let mut out = vec![]; for t17 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t17;
out.push(py::len(&v_g.clone())?); } Py::list(out) }, None)?])).items()?;
    if t18.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t18[0].clone();
    v_N = t18[1].clone();
    v_sp = py::truediv(&py::fsum(&{ let mut out = vec![]; for t19 in py::iter(&(py::zip(&[v_gs.clone(), v_vs.clone()])?))? { let mut v_g: Py; let mut v_v: Py; 
let t20 = (t19).items()?;
if t20.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t20[0].clone();
v_v = t20[1].clone();
out.push(py::mul(&py::sub(&py::len(&v_g.clone())?, &Py::Int(1i64))?, &v_v.clone())?); } Py::list(out) })?, &py::sub(&v_N.clone(), &v_k.clone())?)?;
    v_num = py::sub(&py::mul(&py::sub(&v_N.clone(), &v_k.clone())?, &py::math1("log", &v_sp.clone())?)?, &py::fsum(&{ let mut out = vec![]; for t21 in py::iter(&(py::zip(&[v_gs.clone(), v_vs.clone()])?))? { let mut v_g: Py; let mut v_v: Py; 
let t22 = (t21).items()?;
if t22.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t22[0].clone();
v_v = t22[1].clone();
out.push(py::mul(&py::sub(&py::len(&v_g.clone())?, &Py::Int(1i64))?, &py::math1("log", &v_v.clone())?)?); } Py::list(out) })?)?;
    v_den = py::add(&Py::Int(1i64), &py::truediv(&py::sub(&py::fsum(&{ let mut out = vec![]; for t23 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t23;
out.push(py::truediv(&Py::Int(1i64), &py::sub(&py::len(&v_g.clone())?, &Py::Int(1i64))?)?); } Py::list(out) })?, &py::truediv(&Py::Int(1i64), &py::sub(&v_N.clone(), &v_k.clone())?)?)?, &py::mul(&Py::Int(3i64), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?;
    v_t = py::truediv(&v_num.clone(), &v_den.clone())?;
    return Ok(Py::dict(vec![(Py::str("statistic"), v_t.clone()), (Py::str("df"), py::sub(&v_k.clone(), &Py::Int(1i64))?), (Py::str("p_value"), f__chi2_sf(v_t.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_binom_at_least(mut v_n: Py, mut v_k: Py, mut v_p: Py) -> R {
    let t94 = (f__bin_args(v_n.clone(), v_k.clone(), v_p.clone())?).items()?;
    if t94.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t94[0].clone();
    v_k = t94[1].clone();
    v_p = t94[2].clone();
    return Ok(Py::dict(vec![(Py::str("probability"), f__binom_sf_ge(v_k.clone(), v_n.clone(), v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_binom_at_most(mut v_n: Py, mut v_k: Py, mut v_p: Py) -> R {
    let t157 = (f__bin_args(v_n.clone(), v_k.clone(), v_p.clone())?).items()?;
    if t157.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t157[0].clone();
    v_k = t157[1].clone();
    v_p = t157[2].clone();
    return Ok(Py::dict(vec![(Py::str("probability"), f__binom_cdf(v_k.clone(), v_n.clone(), v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_binom_exact(mut v_n: Py, mut v_k: Py, mut v_p: Py) -> R {
    let t220 = (f__bin_args(v_n.clone(), v_k.clone(), v_p.clone())?).items()?;
    if t220.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t220[0].clone();
    v_k = t220[1].clone();
    v_p = t220[2].clone();
    return Ok(Py::dict(vec![(Py::str("probability"), f__binom_pmf(v_k.clone(), v_n.clone(), v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_binom_moments(mut v_n: Py, mut v_p: Py) -> R {
    let mut v_v: Py = Py::None;
    let t275 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?, f__prob(v_p.clone(), Py::str("p"), Py::Bool(true))?])).items()?;
    if t275.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t275[0].clone();
    v_p = t275[1].clone();
    v_v = py::mul(&py::mul(&v_n.clone(), &v_p.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::mul(&v_n.clone(), &v_p.clone())?), (Py::str("variance"), v_v.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_block_anova(mut v_data: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_cm: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dfb: Py = Py::None;
    let mut v_dfe: Py = Py::None;
    let mut v_dft: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_ft: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_ss_bl: Py = Py::None;
    let mut v_ss_tr: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_d = f__matrix_ok(v_data.clone(), Py::str("the data table"), Py::Int(2i64), Py::Int(2i64))?;
    let t4776 = (Py::tuple(vec![py::len(&v_d.clone())?, py::len(&py::getitem(&v_d.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t4776[0].clone();
    v_t = t4776[1].clone();
    v_gm = f__mean({ let mut out = vec![]; for t4777 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t4777;
for t4778 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4778;
out.push(v_v.clone()); }} Py::list(out) })?;
    v_rm = { let mut out = vec![]; for t4779 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t4779;
out.push(f__mean(v_r.clone())?); } Py::list(out) };
    v_cm = { let mut out = vec![]; for t4780 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_d.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t4780;
out.push(f__mean(v_c.clone())?); } Py::list(out) };
    v_sst = py::fsum(&{ let mut out = vec![]; for t4781 in py::range_iter(&[(v_b.clone()).i()?]) { let mut v_i: Py; 
v_i = t4781;
for t4782 in py::range_iter(&[(v_t.clone()).i()?]) { let mut v_j: Py; 
v_j = t4782;
out.push(py::pow(&py::add(&py::sub(&py::sub(&py::getitem(&py::getitem(&v_d.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_cm.clone(), &v_j.clone())?)?, &py::getitem(&v_rm.clone(), &v_i.clone())?)?, &v_gm.clone())?, &Py::Int(2i64))?); }} Py::list(out) })?;
    v_ss_tr = py::mul(&v_b.clone(), &py::fsum(&{ let mut out = vec![]; for t4783 in py::iter(&(v_cm.clone()))? { let mut v_c: Py; 
v_c = t4783;
out.push(py::pow(&py::sub(&v_c.clone(), &v_gm.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
    v_ss_bl = py::mul(&v_t.clone(), &py::fsum(&{ let mut out = vec![]; for t4784 in py::iter(&(v_rm.clone()))? { let mut v_r: Py; 
v_r = t4784;
out.push(py::pow(&py::sub(&v_r.clone(), &v_gm.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
    if ({ let t4785 = v_sst.clone(); Py::Bool({ let t4786 = Py::Int(0i64); if py::eq(&t4785, &t4786) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no residual variation"))));
    }
    let t4787 = (Py::tuple(vec![py::sub(&v_t.clone(), &Py::Int(1i64))?, py::sub(&v_b.clone(), &Py::Int(1i64))?, py::mul(&py::sub(&v_t.clone(), &Py::Int(1i64))?, &py::sub(&v_b.clone(), &Py::Int(1i64))?)?])).items()?;
    if t4787.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dft = t4787[0].clone();
    v_dfb = t4787[1].clone();
    v_dfe = t4787[2].clone();
    let t4788 = (Py::tuple(vec![py::truediv(&py::truediv(&v_ss_tr.clone(), &v_dft.clone())?, &py::truediv(&v_sst.clone(), &v_dfe.clone())?)?, py::truediv(&py::truediv(&v_ss_bl.clone(), &v_dfb.clone())?, &py::truediv(&v_sst.clone(), &v_dfe.clone())?)?])).items()?;
    if t4788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ft = t4788[0].clone();
    v_fb = t4788[1].clone();
    return Ok(Py::dict(vec![(Py::str("f_treatment"), v_ft.clone()), (Py::str("p_treatment"), py::getitem(&f__f_tails(v_ft.clone(), v_dft.clone(), v_dfe.clone())?, &Py::Int(1i64))?), (Py::str("f_block"), v_fb.clone()), (Py::str("p_block"), py::getitem(&f__f_tails(v_fb.clone(), v_dfb.clone(), v_dfe.clone())?, &Py::Int(1i64))?), (Py::str("ss_treatment"), v_ss_tr.clone()), (Py::str("ss_block"), v_ss_bl.clone()), (Py::str("ss_error"), v_sst.clone()), (Py::str("df_treatment"), v_dft.clone()), (Py::str("df_block"), v_dfb.clone()), (Py::str("df_error"), v_dfe.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_c_chart(mut v_counts: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cb: Py = Py::None;
    v_c = f__finite(v_counts.clone(), Py::str("counts"))?;
    if (Py::Bool(({ let mut out = vec![]; for t313 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t313;
out.push({ let t314 = v_v.clone(); Py::Bool({ let t315 = Py::Int(0i64); if py::lt(&t314, &t315)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("counts must be >= 0"))));
    }
    v_cb = f__mean(v_c.clone())?;
    if ({ let t316 = v_cb.clone(); Py::Bool({ let t317 = Py::Int(0i64); if py::le(&t316, &t317)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the average count must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("center"), v_cb.clone()), (Py::str("ucl"), py::add(&v_cb.clone(), &py::mul(&Py::Int(3i64), &py::math1("sqrt", &v_cb.clone())?)?)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&v_cb.clone(), &py::mul(&Py::Int(3i64), &py::math1("sqrt", &v_cb.clone())?)?)?], true)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_capability(mut v_mu: Py, mut v_sigma: Py, mut v_lsl: Py, mut v_usl: Py) -> R {
    let mut v_cpl: Py = Py::None;
    let mut v_cpu: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = f__pos(v_sigma.clone(), Py::str("sigma"))?;
    if (py::not(&{ let t4667 = v_usl.clone(); Py::Bool({ let t4668 = v_lsl.clone(); if py::gt(&t4667, &t4668)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the upper specification limit must exceed the lower one"))));
    }
    let t4669 = (Py::tuple(vec![py::truediv(&py::sub(&v_usl.clone(), &v_mu.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?, py::truediv(&py::sub(&v_mu.clone(), &v_lsl.clone())?, &py::mul(&Py::Int(3i64), &v_s.clone())?)?])).items()?;
    if t4669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cpu = t4669[0].clone();
    v_cpl = t4669[1].clone();
    return Ok(Py::dict(vec![(Py::str("cp"), py::truediv(&py::sub(&v_usl.clone(), &v_lsl.clone())?, &py::mul(&Py::Int(6i64), &v_s.clone())?)?), (Py::str("cpk"), py::minmax(&[v_cpu.clone(), v_cpl.clone()], false)?), (Py::str("cpu"), v_cpu.clone()), (Py::str("cpl"), v_cpl.clone()), (Py::str("ppm_out"), py::mul(&Py::Float(1000000.0), &py::add(&f__phi(py::truediv(&py::sub(&v_lsl.clone(), &v_mu.clone())?, &v_s.clone())?)?, &f__phic(py::truediv(&py::sub(&v_usl.clone(), &v_mu.clone())?, &v_s.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_chi2_critical(mut v_df: Py, mut v_alpha: Py) -> R {
    let t364 = (Py::tuple(vec![f__pos(v_df.clone(), Py::str("the degrees of freedom"))?, f__prob(v_alpha.clone(), Py::str("alpha"), Py::Bool(true))?])).items()?;
    if t364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_df = t364[0].clone();
    v_alpha = t364[1].clone();
    return Ok(Py::dict(vec![(Py::str("critical_value"), f__chi2_ppf(py::sub(&Py::Float(1.0), &v_alpha.clone())?, v_df.clone())?), (Py::str("left_critical_value"), f__chi2_ppf(v_alpha.clone(), v_df.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_chi2_var(mut v_x: Py, mut v_sigma0: Py, mut v_alternative: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_stat: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_sigma0 = f__pos(v_sigma0.clone(), Py::str("sigma0"))?;
    v_n = py::len(&v_x.clone())?;
    v_stat = py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &f__var(v_x.clone(), Py::Int(1i64))?)?, &py::pow(&v_sigma0.clone(), &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_stat.clone()), (Py::str("df"), py::sub(&v_n.clone(), &Py::Int(1i64))?), (Py::str("p_value"), f__alt_p(v_stat.clone(), v_alternative.clone(), { let v_alternative = v_alternative.clone(); let v_n = v_n.clone(); let v_sigma0 = v_sigma0.clone(); let v_stat = v_stat.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__chi2_sf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_alternative = v_alternative.clone(); let v_n = v_n.clone(); let v_sigma0 = v_sigma0.clone(); let v_stat = v_stat.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__chi2_cdf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?), (Py::str("sample_variance"), f__var(v_x.clone(), Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_mean_t(mut v_x: Py, mut v_confidence: Py) -> R {
    let mut v_out: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_out = f__t_interval(f__mean(v_x.clone())?, f__sd(v_x.clone(), Py::Int(1i64))?, py::len(&v_x.clone())?, v_confidence.clone())?;
    py::setitem(&v_out.clone(), &Py::str("mean"), f__mean(v_x.clone())?)?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_mean_t_summary(mut v_xbar: Py, mut v_s: Py, mut v_n: Py, mut v_confidence: Py) -> R {
    let t817 = (Py::tuple(vec![f__pos(v_s.clone(), Py::str("s"))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(2i64))?])).items()?;
    if t817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t817[0].clone();
    v_n = t817[1].clone();
    return Ok(f__t_interval(v_xbar.clone(), v_s.clone(), v_n.clone(), v_confidence.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_mean_z(mut v_xbar: Py, mut v_sigma: Py, mut v_n: Py, mut v_confidence: Py) -> R {
    let mut v_m: Py = Py::None;
    let t674 = (Py::tuple(vec![f__pos(v_sigma.clone(), Py::str("sigma"))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?])).items()?;
    if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sigma = t674[0].clone();
    v_n = t674[1].clone();
    v_m = py::truediv(&py::mul(&f__zc(v_confidence.clone())?, &v_sigma.clone())?, &py::math1("sqrt", &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("lower"), py::sub(&v_xbar.clone(), &v_m.clone())?), (Py::str("upper"), py::add(&v_xbar.clone(), &v_m.clone())?), (Py::str("margin_of_error"), v_m.clone()), (Py::str("critical_value"), f__zc(v_confidence.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_prop(mut v_x: Py, mut v_n: Py, mut v_confidence: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t907 = (f__prop_args(v_x.clone(), v_n.clone())?).items()?;
    if t907.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t907[0].clone();
    v_n = t907[1].clone();
    if ({ let t912 = { let t908 = v_x.clone(); Py::Bool({ let t909 = Py::Int(0i64); if py::eq(&t908, &t909) { true } else { false } }) }; if t912.truthy() { t912 } else { { let t910 = v_x.clone(); Py::Bool({ let t911 = v_n.clone(); if py::eq(&t910, &t911) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Wald interval needs 0 < x < n; use the Wilson interval"))));
    }
    v_p = py::truediv(&v_x.clone(), &v_n.clone())?;
    v_e = py::mul(&f__zc(v_confidence.clone())?, &py::math1("sqrt", &py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("proportion"), v_p.clone()), (Py::str("lower"), py::minmax(&[Py::Float(0.0), py::sub(&v_p.clone(), &v_e.clone())?], true)?), (Py::str("upper"), py::minmax(&[Py::Float(1.0), py::add(&v_p.clone(), &v_e.clone())?], false)?), (Py::str("margin_of_error"), v_e.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_two_means(mut v_x: Py, mut v_y: Py, mut v_confidence: Py, mut v_variance: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t972 = (f__two_sample(v_x.clone(), v_y.clone(), v_variance.clone())?).items()?;
    if t972.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t972[0].clone();
    v_se = t972[1].clone();
    v_df = t972[2].clone();
    v_t = f__tc(v_confidence.clone(), v_df.clone())?;
    return Ok(Py::dict(vec![(Py::str("difference"), v_d.clone()), (Py::str("lower"), py::sub(&v_d.clone(), &py::mul(&v_t.clone(), &v_se.clone())?)?), (Py::str("upper"), py::add(&v_d.clone(), &py::mul(&v_t.clone(), &v_se.clone())?)?), (Py::str("df"), v_df.clone()), (Py::str("standard_error"), v_se.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ci_variance(mut v_x: Py, mut v_confidence: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    let t1058 = (Py::tuple(vec![py::len(&v_x.clone())?, f__var(v_x.clone(), Py::Int(1i64))?])).items()?;
    if t1058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1058[0].clone();
    v_v = t1058[1].clone();
    v_a = py::sub(&Py::Float(1.0), &f__conf(v_confidence.clone())?)?;
    let t1110 = (Py::tuple(vec![py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_v.clone())?, &f__chi2_ppf(py::sub(&Py::Int(1i64), &py::truediv(&v_a.clone(), &Py::Int(2i64))?)?, py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_v.clone())?, &f__chi2_ppf(py::truediv(&v_a.clone(), &Py::Int(2i64))?, py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?])).items()?;
    if t1110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1110[0].clone();
    v_hi = t1110[1].clone();
    return Ok(Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("lower"), v_lo.clone()), (Py::str("upper"), v_hi.clone()), (Py::str("sd_lower"), py::math1("sqrt", &v_lo.clone())?), (Py::str("sd_upper"), py::math1("sqrt", &v_hi.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cohens_d(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t1190 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("x"))?, f__finite(v_y.clone(), Py::str("y"))?])).items()?;
    if t1190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1190[0].clone();
    v_y = t1190[1].clone();
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the first sample"))?;
    let _ = f__need(v_y.clone(), Py::Int(2i64), Py::str("the second sample"))?;
    let t1193 = (Py::tuple(vec![py::len(&v_x.clone())?, py::len(&v_y.clone())?])).items()?;
    if t1193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1193[0].clone();
    v_n2 = t1193[1].clone();
    v_sp = py::math1("sqrt", &py::truediv(&py::add(&py::mul(&py::sub(&v_n1.clone(), &Py::Int(1i64))?, &f__var(v_x.clone(), Py::Int(1i64))?)?, &py::mul(&py::sub(&v_n2.clone(), &Py::Int(1i64))?, &f__var(v_y.clone(), Py::Int(1i64))?)?)?, &py::sub(&py::add(&v_n1.clone(), &v_n2.clone())?, &Py::Int(2i64))?)?)?;
    if ({ let t1195 = v_sp.clone(); Py::Bool({ let t1196 = Py::Int(0i64); if py::eq(&t1195, &t1196) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both samples have zero variance"))));
    }
    v_d = py::truediv(&py::sub(&f__mean(v_x.clone())?, &f__mean(v_y.clone())?)?, &v_sp.clone())?;
    return Ok(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("hedges_g"), py::mul(&v_d.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(3i64), &py::sub(&py::mul(&Py::Int(4i64), &py::add(&v_n1.clone(), &v_n2.clone())?)?, &Py::Int(9i64))?)?)?)?), (Py::str("pooled_sd"), v_sp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_corr_ci(mut v_x: Py, mut v_y: Py, mut v_confidence: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1239 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(4i64))?).items()?;
    if t1239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1239[0].clone();
    v_y = t1239[1].clone();
    v_r = f__pearson(v_x.clone(), v_y.clone())?;
    if ({ let t1250 = py::abs(&v_r.clone())?; Py::Bool({ let t1251 = Py::Int(1i64); if py::ge(&t1250, &t1251)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|r| = 1: the Fisher interval is degenerate"))));
    }
    let t1264 = (Py::tuple(vec![py::math1("atanh", &v_r.clone())?, py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::sub(&py::len(&v_x.clone())?, &Py::Int(3i64))?)?)?, f__zc(v_confidence.clone())?])).items()?;
    if t1264.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_z = t1264[0].clone();
    v_se = t1264[1].clone();
    v_c = t1264[2].clone();
    return Ok(Py::dict(vec![(Py::str("r"), v_r.clone()), (Py::str("lower"), py::math1("tanh", &py::sub(&v_z.clone(), &py::mul(&v_c.clone(), &v_se.clone())?)?)?), (Py::str("upper"), py::math1("tanh", &py::add(&v_z.clone(), &py::mul(&v_c.clone(), &v_se.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_corr_test(mut v_r: Py, mut v_n: Py, mut v_alternative: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(3i64))?;
    if (py::not(&{ let t1308 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1309 = v_r.clone(); if py::lt(&t1308, &t1309)? { { let t1310 = Py::Int(1i64); if py::lt(&t1309, &t1310)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("r must be strictly between -1 and 1"))));
    }
    v_t = py::mul(&v_r.clone(), &py::math1("sqrt", &py::truediv(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &py::sub(&Py::Int(1i64), &py::mul(&v_r.clone(), &v_r.clone())?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("df"), py::sub(&v_n.clone(), &Py::Int(2i64))?), (Py::str("p_value"), f__alt_p(v_t.clone(), v_alternative.clone(), { let v_alternative = v_alternative.clone(); let v_n = v_n.clone(); let v_r = v_r.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_sf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(2i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_alternative = v_alternative.clone(); let v_n = v_n.clone(); let v_r = v_r.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_cdf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(2i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cov(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1386 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(2i64))?).items()?;
    if t1386.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1386[0].clone();
    v_y = t1386[1].clone();
    let t1387 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t1387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t1387[0].clone();
    v_my = t1387[1].clone();
    v_s = py::fsum(&{ let mut out = vec![]; for t1388 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1389 = (t1388).items()?;
if t1389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1389[0].clone();
v_b = t1389[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("sample_covariance"), py::truediv(&v_s.clone(), &py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?), (Py::str("population_covariance"), py::truediv(&v_s.clone(), &py::len(&v_x.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cronbach(mut v_data: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_iv: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tv: Py = Py::None;
    v_d = f__matrix_ok(v_data.clone(), Py::str("the item table"), Py::Int(2i64), Py::Int(2i64))?;
    v_k = py::len(&py::getitem(&v_d.clone(), &Py::Int(0i64))?)?;
    v_iv = py::fsum(&{ let mut out = vec![]; for t1437 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_d.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t1437;
out.push(f__var(Py::list((v_c.clone()).items()?), Py::Int(1i64))?); } Py::list(out) })?;
    v_tv = f__var({ let mut out = vec![]; for t1439 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t1439;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) }, Py::Int(1i64))?;
    if ({ let t1440 = v_tv.clone(); Py::Bool({ let t1441 = Py::Int(0i64); if py::eq(&t1440, &t1441) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the total scores do not vary"))));
    }
    return Ok(Py::dict(vec![(Py::str("alpha"), py::mul(&py::truediv(&v_k.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &py::sub(&Py::Int(1i64), &py::truediv(&v_iv.clone(), &v_tv.clone())?)?)?), (Py::str("items"), v_k.clone()), (Py::str("respondents"), py::len(&v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cv(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_m = f__mean(v_x.clone())?;
    if ({ let t1150 = v_m.clone(); Py::Bool({ let t1151 = Py::Int(0i64); if py::le(&t1150, &t1151)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficient of variation needs a positive mean (ratio-scale data)"))));
    }
    v_s = f__sd(v_x.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("cv"), py::truediv(&v_s.clone(), &v_m.clone())?), (Py::str("cv_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_s.clone())?, &v_m.clone())?), (Py::str("mean"), v_m.clone()), (Py::str("standard_deviation"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_discrete_rv(mut v_x: Py, mut v_p: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_var: Py = Py::None;
    let t1479 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("values"))?, f__finite(v_p.clone(), Py::str("probabilities"))?])).items()?;
    if t1479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1479[0].clone();
    v_p = t1479[1].clone();
    if ({ let t1480 = py::len(&v_x.clone())?; Py::Bool({ let t1481 = py::len(&v_p.clone())?; if py::ne(&t1480, &t1481) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values and probabilities must have the same length"))));
    }
    if ({ let t1487 = Py::Bool(({ let mut out = vec![]; for t1482 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t1482;
out.push({ let t1483 = v_v.clone(); Py::Bool({ let t1484 = Py::Int(0i64); if py::lt(&t1483, &t1484)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1487.truthy() { t1487 } else { { let t1485 = py::abs(&py::sub(&py::fsum(&v_p.clone())?, &Py::Float(1.0))?)?; Py::Bool({ let t1486 = Py::Float(1e-09); if py::gt(&t1485, &t1486)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("probabilities must be non-negative and sum to 1"))));
    }
    v_m = py::fsum(&{ let mut out = vec![]; for t1488 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1489 = (t1488).items()?;
if t1489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1489[0].clone();
v_b = t1489[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    v_var = py::fsum(&{ let mut out = vec![]; for t1490 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1491 = (t1490).items()?;
if t1491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1491[0].clone();
v_b = t1491[1].clone();
out.push(py::mul(&v_b.clone(), &py::pow(&py::sub(&v_a.clone(), &v_m.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("expected_value"), v_m.clone()), (Py::str("variance"), v_var.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_var.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_durbin_watson(mut v_residuals: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_e = f__finite(v_residuals.clone(), Py::str("residuals"))?;
    let _ = f__need(v_e.clone(), Py::Int(2i64), Py::str("the residuals"))?;
    v_den = py::fsum(&{ let mut out = vec![]; for t1531 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1531;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    if ({ let t1532 = v_den.clone(); Py::Bool({ let t1533 = Py::Int(0i64); if py::eq(&t1532, &t1533) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all residuals are zero"))));
    }
    return Ok(Py::dict(vec![(Py::str("durbin_watson"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t1534 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_e.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1534;
out.push(py::pow(&py::sub(&py::getitem(&v_e.clone(), &v_i.clone())?, &py::getitem(&v_e.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) })?, &v_den.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_exp_above(mut v_x: Py, mut v_rate: Py, mut v_mean: Py) -> R {
    let mut v_lam: Py = Py::None;
    v_lam = f__exp_rate(v_rate.clone(), v_mean.clone())?;
    if ({ let t1581 = v_x.clone(); Py::Bool({ let t1582 = Py::Int(0i64); if py::lt(&t1581, &t1582)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x must be >= 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("probability"), py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_x.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_exp_below(mut v_x: Py, mut v_rate: Py, mut v_mean: Py) -> R {
    let mut v_lam: Py = Py::None;
    v_lam = f__exp_rate(v_rate.clone(), v_mean.clone())?;
    if ({ let t1629 = v_x.clone(); Py::Bool({ let t1630 = Py::Int(0i64); if py::lt(&t1629, &t1630)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x must be >= 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("probability"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_x.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_exp_rel(mut v_t: Py, mut v_rate: Py, mut v_mtbf: Py) -> R {
    let mut v_lam: Py = Py::None;
    if ({ let t1665 = { let t1667 = v_rate.clone(); Py::Bool({ let t1668 = Py::None; if py::is(&t1667, &t1668) { true } else { false } }) }; Py::Bool({ let t1666 = { let t1669 = v_mtbf.clone(); Py::Bool({ let t1670 = Py::None; if py::is(&t1669, &t1670) { true } else { false } }) }; if py::eq(&t1665, &t1666) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give either the failure rate or the MTBF (not both)"))));
    }
    v_lam = (if ({ let t1671 = v_rate.clone(); Py::Bool({ let t1672 = Py::None; if !py::is(&t1671, &t1672) { true } else { false } }) }).truthy() { f__pos(v_rate.clone(), Py::str("the failure rate"))? } else { py::truediv(&Py::Int(1i64), &f__pos(v_mtbf.clone(), Py::str("the MTBF"))?)? });
    if ({ let t1677 = v_t.clone(); Py::Bool({ let t1678 = Py::Int(0i64); if py::lt(&t1677, &t1678)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("t must be >= 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("reliability"), py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?), (Py::str("unreliability"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?)?), (Py::str("mttf"), py::truediv(&Py::Int(1i64), &v_lam.clone())?), (Py::str("failure_rate"), v_lam.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_f_critical(mut v_df1: Py, mut v_df2: Py, mut v_alpha: Py) -> R {
    let t1725 = (Py::tuple(vec![f__pos(v_df1.clone(), Py::str("df1"))?, f__pos(v_df2.clone(), Py::str("df2"))?, f__prob(v_alpha.clone(), Py::str("alpha"), Py::Bool(true))?])).items()?;
    if t1725.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_df1 = t1725[0].clone();
    v_df2 = t1725[1].clone();
    v_alpha = t1725[2].clone();
    return Ok(Py::dict(vec![(Py::str("critical_value"), f__f_ppf(py::sub(&Py::Float(1.0), &v_alpha.clone())?, v_df1.clone(), v_df2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_f_test(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t1819 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("x"))?, f__finite(v_y.clone(), Py::str("y"))?])).items()?;
    if t1819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1819[0].clone();
    v_y = t1819[1].clone();
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the first sample"))?;
    let _ = f__need(v_y.clone(), Py::Int(2i64), Py::str("the second sample"))?;
    let t1823 = (Py::tuple(vec![f__var(v_x.clone(), Py::Int(1i64))?, f__var(v_y.clone(), Py::Int(1i64))?])).items()?;
    if t1823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1823[0].clone();
    v_v2 = t1823[1].clone();
    if ({ let t1828 = { let t1824 = v_v1.clone(); Py::Bool({ let t1825 = Py::Int(0i64); if py::eq(&t1824, &t1825) { true } else { false } }) }; if t1828.truthy() { t1828 } else { { let t1826 = v_v2.clone(); Py::Bool({ let t1827 = Py::Int(0i64); if py::eq(&t1826, &t1827) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a sample has zero variance"))));
    }
    let t1829 = (Py::tuple(vec![py::truediv(&v_v1.clone(), &v_v2.clone())?, py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?, py::sub(&py::len(&v_y.clone())?, &Py::Int(1i64))?])).items()?;
    if t1829.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f = t1829[0].clone();
    v_d1 = t1829[1].clone();
    v_d2 = t1829[2].clone();
    return Ok(Py::dict(vec![(Py::str("f"), v_f.clone()), (Py::str("df1"), v_d1.clone()), (Py::str("df2"), v_d2.clone()), (Py::str("p_value"), f__alt_p(v_f.clone(), v_alternative.clone(), { let v_alternative = v_alternative.clone(); let v_d1 = v_d1.clone(); let v_d2 = v_d2.clone(); let v_f = v_f.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__f_tails(v_v.clone(), v_d1.clone(), v_d2.clone())?, &Py::Int(1i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_alternative = v_alternative.clone(); let v_d1 = v_d1.clone(); let v_d2 = v_d2.clone(); let v_f = v_f.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__f_tails(v_v.clone(), v_d1.clone(), v_d2.clone())?, &Py::Int(0i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_fisher(mut v_table: Py, mut v_alternative: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hi_: Py = Py::None;
    let mut v_lo_: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t2028 = (f__two_by_two(v_table.clone())?).items()?;
    if t2028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2029 = (t2028[0].clone()).items()?;
    if t2029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2029[0].clone();
    v_b = t2029[1].clone();
    let t2030 = (t2028[1].clone()).items()?;
    if t2030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2030[0].clone();
    v_d = t2030[1].clone();
    let t2031 = (Py::tuple(vec![py::add(&v_a.clone(), &v_b.clone())?, py::add(&v_c.clone(), &v_d.clone())?, py::add(&v_a.clone(), &v_c.clone())?])).items()?;
    if t2031.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t2031[0].clone();
    v_r2 = t2031[1].clone();
    v_c1 = t2031[2].clone();
    v_N = py::add(&v_r1.clone(), &v_r2.clone())?;
    if ({ let t2032 = v_N.clone(); Py::Bool({ let t2033 = Py::Int(0i64); if py::eq(&t2032, &t2033) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the table is empty"))));
    }
    let t2034 = (Py::tuple(vec![py::minmax(&[Py::Int(0i64), py::sub(&v_c1.clone(), &v_r2.clone())?], true)?, py::minmax(&[v_r1.clone(), v_c1.clone()], false)?])).items()?;
    if t2034.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo_ = t2034[0].clone();
    v_hi_ = t2034[1].clone();
    v_w = { let mut out = vec![]; for t2035 in py::range_iter(&[(v_lo_.clone()).i()?, (py::add(&v_hi_.clone(), &Py::Int(1i64))?).i()?]) { let mut v_x: Py; 
v_x = t2035;
out.push((v_x.clone(), py::mul(&py::comb(&v_r1.clone(), &v_x.clone())?, &py::comb(&v_r2.clone(), &py::sub(&v_c1.clone(), &v_x.clone())?)?)?)); } Py::dict(out) };
    v_tot = py::comb(&v_N.clone(), &v_c1.clone())?;
    if ({ let t2036 = v_alternative.clone(); Py::Bool({ let t2037 = Py::str("greater"); if py::eq(&t2036, &t2037) { true } else { false } }) }).truthy() {
        v_s = py::sum(&{ let mut out = vec![]; for t2038 in py::iter(&(py::method(&v_w.clone(), "items", &[])?))? { let mut v_v: Py; let mut v_x: Py; 
let t2039 = (t2038).items()?;
if t2039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2039[0].clone();
v_v = t2039[1].clone();
if ({ let t2040 = v_x.clone(); Py::Bool({ let t2041 = v_a.clone(); if py::ge(&t2040, &t2041)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, None)?;
    } else {
        if ({ let t2042 = v_alternative.clone(); Py::Bool({ let t2043 = Py::str("less"); if py::eq(&t2042, &t2043) { true } else { false } }) }).truthy() {
            v_s = py::sum(&{ let mut out = vec![]; for t2044 in py::iter(&(py::method(&v_w.clone(), "items", &[])?))? { let mut v_v: Py; let mut v_x: Py; 
let t2045 = (t2044).items()?;
if t2045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2045[0].clone();
v_v = t2045[1].clone();
if ({ let t2046 = v_x.clone(); Py::Bool({ let t2047 = v_a.clone(); if py::le(&t2046, &t2047)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, None)?;
        } else {
            v_s = py::sum(&{ let mut out = vec![]; for t2048 in py::iter(&(py::method(&v_w.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t2048;
if ({ let t2049 = py::mul(&v_v.clone(), &py::pow(&Py::Int(10i64), &Py::Int(7i64))?)?; Py::Bool({ let t2050 = py::mul(&py::getitem(&v_w.clone(), &v_a.clone())?, &py::add(&py::pow(&Py::Int(10i64), &Py::Int(7i64))?, &Py::Int(1i64))?)?; if py::le(&t2049, &t2050)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, None)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("p_value"), py::minmax(&[Py::Float(1.0), py::float(&py::fraction(&[v_s.clone(), v_tot.clone()])?)?], false)?), (Py::str("odds_ratio"), (if (py::mul(&v_b.clone(), &v_c.clone())?).truthy() { py::truediv(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_five_number(mut v_x: Py, mut v_method: Py) -> R {
    let mut v_q1: Py = Py::None;
    let mut v_q2: Py = Py::None;
    let mut v_q3: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    let t2097 = ({ let mut out = vec![]; for t2090 in py::iter(&(Py::tuple(vec![Py::Float(0.25), Py::Float(0.5), Py::Float(0.75)])))? { let mut v_p: Py; 
v_p = t2090;
out.push(f__quantile(v_x.clone(), v_p.clone(), v_method.clone())?); } Py::list(out) }).items()?;
    if t2097.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q1 = t2097[0].clone();
    v_q2 = t2097[1].clone();
    v_q3 = t2097[2].clone();
    return Ok(Py::dict(vec![(Py::str("minimum"), py::minmax(&[v_x.clone()], false)?), (Py::str("q1"), v_q1.clone()), (Py::str("median"), v_q2.clone()), (Py::str("q3"), v_q3.clone()), (Py::str("maximum"), py::minmax(&[v_x.clone()], true)?), (Py::str("iqr"), py::sub(&v_q3.clone(), &v_q1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_frequency(mut v_x: Py, mut v_f: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let t2135 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("values"))?, f__finite(v_f.clone(), Py::str("frequencies"))?])).items()?;
    if t2135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2135[0].clone();
    v_f = t2135[1].clone();
    if ({ let t2136 = py::len(&v_x.clone())?; Py::Bool({ let t2137 = py::len(&v_f.clone())?; if py::ne(&t2136, &t2137) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values and frequencies must have the same length"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2138 in py::iter(&(v_f.clone()))? { let mut v_v: Py; 
v_v = t2138;
out.push({ let t2143 = { let t2139 = v_v.clone(); Py::Bool({ let t2140 = Py::Int(0i64); if py::lt(&t2139, &t2140)? { true } else { false } }) }; if t2143.truthy() { t2143 } else { { let t2141 = v_v.clone(); Py::Bool({ let t2142 = py::int(&v_v.clone())?; if py::ne(&t2141, &t2142) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("frequencies must be whole numbers >= 0"))));
    }
    v_n = py::int(&py::fsum(&v_f.clone())?)?;
    if ({ let t2144 = v_n.clone(); Py::Bool({ let t2145 = Py::Int(2i64); if py::lt(&t2144, &t2145)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the table needs a total frequency of at least 2"))));
    }
    v_m = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2146 in py::iter(&(py::zip(&[v_x.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2147 = (t2146).items()?;
if t2147.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2147[0].clone();
v_b = t2147[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_n.clone())?;
    v_ss = py::fsum(&{ let mut out = vec![]; for t2148 in py::iter(&(py::zip(&[v_x.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2149 = (t2148).items()?;
if t2149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2149[0].clone();
v_b = t2149[1].clone();
out.push(py::mul(&v_b.clone(), &py::pow(&py::sub(&v_a.clone(), &v_m.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("mean"), v_m.clone()), (Py::str("sample_standard_deviation"), py::math1("sqrt", &py::truediv(&v_ss.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?), (Py::str("population_standard_deviation"), py::math1("sqrt", &py::truediv(&v_ss.clone(), &v_n.clone())?)?), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_friedman(mut v_data: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_ranks: Py = Py::None;
    let mut v_ssbn: Py = Py::None;
    let mut v_ties: Py = Py::None;
    v_d = f__matrix_ok(v_data.clone(), Py::str("the data table"), Py::Int(2i64), Py::Int(3i64))?;
    let t2197 = (Py::tuple(vec![py::len(&v_d.clone())?, py::len(&py::getitem(&v_d.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2197[0].clone();
    v_k = t2197[1].clone();
    v_ranks = { let mut out = vec![]; for t2198 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t2198;
out.push(f__ranks(v_r.clone())?); } Py::list(out) };
    v_ties = py::sum(&{ let mut out = vec![]; for t2207 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t2207;
for t2212 in py::iter(&(f__tie_counts(v_r.clone())?))? { let mut v_t: Py; 
v_t = t2212;
out.push(py::mul(&v_t.clone(), &py::sub(&py::mul(&v_t.clone(), &v_t.clone())?, &Py::Int(1i64))?)?); }} Py::list(out) }, None)?;
    v_c = py::sub(&Py::Int(1i64), &py::truediv(&v_ties.clone(), &py::mul(&py::mul(&v_k.clone(), &py::sub(&py::mul(&v_k.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &v_n.clone())?)?)?;
    if ({ let t2213 = v_c.clone(); Py::Bool({ let t2214 = Py::Int(0i64); if py::eq(&t2213, &t2214) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every block is fully tied"))));
    }
    v_ssbn = py::fsum(&{ let mut out = vec![]; for t2215 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_ranks.clone()).items()?); v })?))? { let mut v_col: Py; 
v_col = t2215;
out.push(py::pow(&py::fsum(&v_col.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_q = py::truediv(&py::sub(&py::mul(&py::truediv(&Py::Float(12.0), &py::mul(&py::mul(&v_k.clone(), &v_n.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &v_ssbn.clone())?, &py::mul(&py::mul(&Py::Int(3i64), &v_n.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_q.clone()), (Py::str("df"), py::sub(&v_k.clone(), &Py::Int(1i64))?), (Py::str("p_value"), f__chi2_sf(v_q.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_geometric(mut v_p: Py, mut v_k: Py) -> R {
    let t2324 = (Py::tuple(vec![f__prob(v_p.clone(), Py::str("p"), Py::Bool(true))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(1i64))?])).items()?;
    if t2324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2324[0].clone();
    v_k = t2324[1].clone();
    return Ok(Py::dict(vec![(Py::str("probability"), py::mul(&v_p.clone(), &py::math1("exp", &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)?), (Py::str("cumulative"), py::neg(&py::math1("expm1", &py::mul(&v_k.clone(), &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gmean(mut v_x: Py) -> R {
    v_x = f__positive_x(v_x.clone())?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    return Ok(Py::dict(vec![(Py::str("geometric_mean"), py::math1("exp", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2274 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2274;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?, &py::len(&v_x.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gof(mut v_observed: Py, mut v_proportions: Py) -> R {
    let mut v_chi2: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_o = f__finite(v_observed.clone(), Py::str("observed counts"))?;
    let _ = f__need(v_o.clone(), Py::Int(2i64), Py::str("the observed counts"))?;
    if (Py::Bool(({ let mut out = vec![]; for t451 in py::iter(&(v_o.clone()))? { let mut v_v: Py; 
v_v = t451;
out.push({ let t456 = { let t452 = v_v.clone(); Py::Bool({ let t453 = Py::Int(0i64); if py::lt(&t452, &t453)? { true } else { false } }) }; if t456.truthy() { t456 } else { { let t454 = v_v.clone(); Py::Bool({ let t455 = py::int(&v_v.clone())?; if py::ne(&t454, &t455) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("observed counts must be whole numbers >= 0"))));
    }
    v_w = (if ({ let t457 = v_proportions.clone(); Py::Bool({ let t458 = Py::None; if py::is(&t457, &t458) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_o.clone())?)? } else { f__finite(v_proportions.clone(), Py::str("expected proportions"))? });
    if ({ let t459 = py::len(&v_w.clone())?; Py::Bool({ let t460 = py::len(&v_o.clone())?; if py::ne(&t459, &t460) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one expected proportion per category"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t461 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t461;
out.push({ let t462 = v_v.clone(); Py::Bool({ let t463 = Py::Int(0i64); if py::le(&t462, &t463)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("expected proportions must be positive"))));
    }
    let t464 = (Py::tuple(vec![py::fsum(&v_o.clone())?, py::fsum(&v_w.clone())?])).items()?;
    if t464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t464[0].clone();
    v_sw = t464[1].clone();
    if ({ let t465 = v_n.clone(); Py::Bool({ let t466 = Py::Int(0i64); if py::le(&t465, &t466)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the total count must be positive"))));
    }
    v_e = { let mut out = vec![]; for t467 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t467;
out.push(py::truediv(&py::mul(&v_n.clone(), &v_v.clone())?, &v_sw.clone())?); } Py::list(out) };
    v_chi2 = py::fsum(&{ let mut out = vec![]; for t468 in py::iter(&(py::zip(&[v_o.clone(), v_e.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t469 = (t468).items()?;
if t469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t469[0].clone();
v_b = t469[1].clone();
out.push(py::truediv(&py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &v_b.clone())?); } Py::list(out) })?;
    v_k = py::sub(&py::len(&v_o.clone())?, &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_chi2.clone()), (Py::str("df"), v_k.clone()), (Py::str("p_value"), f__chi2_sf(v_chi2.clone(), v_k.clone())?), (Py::str("expected"), v_e.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_hmean(mut v_x: Py) -> R {
    v_x = f__positive_x(v_x.clone())?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    return Ok(Py::dict(vec![(Py::str("harmonic_mean"), py::truediv(&py::len(&v_x.clone())?, &py::fsum(&{ let mut out = vec![]; for t2367 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2367;
out.push(py::truediv(&Py::Float(1.0), &v_v.clone())?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_hypergeom(mut v_N: Py, mut v_K: Py, mut v_n: Py, mut v_k: Py) -> R {
    let mut v_pmf: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t2409 = (Py::tuple(vec![f__whole(v_N.clone(), Py::str("N"), Py::Int(1i64))?, f__whole(v_K.clone(), Py::str("K"), Py::Int(0i64))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(0i64))?])).items()?;
    if t2409.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_N = t2409[0].clone();
    v_K = t2409[1].clone();
    v_n = t2409[2].clone();
    v_k = t2409[3].clone();
    if ({ let t2414 = { let t2410 = v_K.clone(); Py::Bool({ let t2411 = v_N.clone(); if py::gt(&t2410, &t2411)? { true } else { false } }) }; if t2414.truthy() { t2414 } else { { let t2412 = v_n.clone(); Py::Bool({ let t2413 = v_N.clone(); if py::gt(&t2412, &t2413)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the successes in the population (K) and the draws (n) cannot exceed the population size (N)"))));
    }
    if ({ let t2415 = v_k.clone(); Py::Bool({ let t2416 = v_n.clone(); if py::gt(&t2415, &t2416)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k cannot exceed the number of draws n"))));
    }
    v_tot = py::comb(&v_N.clone(), &v_n.clone())?;
    v_pmf = { let v_K = v_K.clone(); let v_N = v_N.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_tot = v_tot.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t2423 = { let t2417 = Py::Int(0i64); Py::Bool({ let t2418 = v_j.clone(); if py::le(&t2417, &t2418)? { { let t2419 = v_K.clone(); if py::le(&t2418, &t2419)? { true } else { false } } } else { false } }) }; if !t2423.truthy() { t2423 } else { { let t2420 = Py::Int(0i64); Py::Bool({ let t2421 = py::sub(&v_n.clone(), &v_j.clone())?; if py::le(&t2420, &t2421)? { { let t2422 = py::sub(&v_N.clone(), &v_K.clone())?; if py::le(&t2421, &t2422)? { true } else { false } } } else { false } }) } } }).truthy() { py::fraction(&[py::mul(&py::comb(&v_K.clone(), &v_j.clone())?, &py::comb(&py::sub(&v_N.clone(), &v_K.clone())?, &py::sub(&v_n.clone(), &v_j.clone())?)?)?, v_tot.clone()])? } else { py::fraction(&[Py::Int(0i64)])? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("probability"), py::float(&py::call(&v_pmf.clone(), &[v_k.clone()])?)?), (Py::str("cumulative"), py::float(&py::sum(&{ let mut out = vec![]; for t2424 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2424;
out.push(py::call(&v_pmf.clone(), &[v_j.clone()])?); } Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_imr(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_mrb: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_mrb = f__mean({ let mut out = vec![]; for t2464 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_x.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2464;
out.push(py::abs(&py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?); } Py::list(out) })?;
    v_m = f__mean(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("center"), v_m.clone()), (Py::str("mean_moving_range"), v_mrb.clone()), (Py::str("x_ucl"), py::add(&v_m.clone(), &py::mul(&c__E2()?, &v_mrb.clone())?)?), (Py::str("x_lcl"), py::sub(&v_m.clone(), &py::mul(&c__E2()?, &v_mrb.clone())?)?), (Py::str("mr_ucl"), py::mul(&py::getitem(&c__D4()?, &Py::Int(2i64))?, &v_mrb.clone())?), (Py::str("mr_lcl"), Py::Float(0.0))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_independence(mut v_table: Py) -> R {
    let mut v_chi2: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f__matrix_ok(v_table.clone(), Py::str("the contingency table"), Py::Int(2i64), Py::Int(2i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t533 in py::iter(&(v_t.clone()))? { let mut v_r: Py; 
v_r = t533;
for t534 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t534;
out.push({ let t535 = v_v.clone(); Py::Bool({ let t536 = Py::Int(0i64); if py::lt(&t535, &t536)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("counts must be >= 0"))));
    }
    let t539 = (Py::tuple(vec![{ let mut out = vec![]; for t537 in py::iter(&(v_t.clone()))? { let mut v_r: Py; 
v_r = t537;
out.push(py::fsum(&v_r.clone())?); } Py::list(out) }, { let mut out = vec![]; for t538 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_t.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t538;
out.push(py::fsum(&v_c.clone())?); } Py::list(out) }])).items()?;
    if t539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t539[0].clone();
    v_cs = t539[1].clone();
    v_n = py::fsum(&v_rs.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t540 in py::iter(&(py::add(&v_rs.clone(), &v_cs.clone())?))? { let mut v_v: Py; 
v_v = t540;
out.push({ let t541 = v_v.clone(); Py::Bool({ let t542 = Py::Int(0i64); if py::le(&t541, &t542)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every row and column needs a positive total"))));
    }
    v_e = { let mut out = vec![]; for t543 in py::iter(&(v_rs.clone()))? { let mut v_a: Py; 
v_a = t543;
out.push({ let mut out = vec![]; for t544 in py::iter(&(v_cs.clone()))? { let mut v_b: Py; 
v_b = t544;
out.push(py::truediv(&py::mul(&v_a.clone(), &v_b.clone())?, &v_n.clone())?); } Py::list(out) }); } Py::list(out) };
    v_chi2 = py::fsum(&{ let mut out = vec![]; for t545 in py::range_iter(&[(py::len(&v_t.clone())?).i()?]) { let mut v_i: Py; 
v_i = t545;
for t546 in py::range_iter(&[(py::len(&py::getitem(&v_t.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t546;
out.push(py::truediv(&py::pow(&py::sub(&py::getitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_e.clone(), &v_i.clone())?, &v_j.clone())?)?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_e.clone(), &v_i.clone())?, &v_j.clone())?)?); }} Py::list(out) })?;
    v_df = py::mul(&py::sub(&py::len(&v_t.clone())?, &Py::Int(1i64))?, &py::sub(&py::len(&py::getitem(&v_t.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_chi2.clone()), (Py::str("df"), v_df.clone()), (Py::str("p_value"), f__chi2_sf(v_chi2.clone(), v_df.clone())?), (Py::str("expected"), v_e.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_inverse_normal(mut v_p: Py, mut v_mu: Py, mut v_sigma: Py) -> R {
    let mut v_z: Py = Py::None;
    v_sigma = f__pos(v_sigma.clone(), Py::str("the standard deviation"))?;
    v_p = f__prob(v_p.clone(), Py::str("the cumulative probability"), Py::Bool(true))?;
    v_z = f__z(v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("x"), py::add(&v_mu.clone(), &py::mul(&v_z.clone(), &v_sigma.clone())?)?), (Py::str("z"), v_z.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_k_of_n(mut v_n: Py, mut v_k: Py, mut v_r: Py) -> R {
    let t2621 = (Py::tuple(vec![f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?, f__whole(v_k.clone(), Py::str("k"), Py::Int(1i64))?, f__prob(v_r.clone(), Py::str("r"), Py::Bool(false))?])).items()?;
    if t2621.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2621[0].clone();
    v_k = t2621[1].clone();
    v_r = t2621[2].clone();
    if ({ let t2622 = v_k.clone(); Py::Bool({ let t2623 = v_n.clone(); if py::gt(&t2622, &t2623)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k cannot exceed n"))));
    }
    return Ok(Py::dict(vec![(Py::str("reliability"), py::minmax(&[Py::Float(1.0), py::fsum(&{ let mut out = vec![]; for t2624 in py::range_iter(&[(v_k.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2624;
out.push(py::mul(&py::mul(&py::comb(&v_n.clone(), &v_j.clone())?, &py::pow(&v_r.clone(), &v_j.clone())?)?, &py::pow(&py::sub(&Py::Int(1i64), &v_r.clone())?, &py::sub(&v_n.clone(), &v_j.clone())?)?)?); } Py::list(out) })?], false)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kendall(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_con: Py = Py::None;
    let mut v_dis: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_tx: Py = Py::None;
    let mut v_ty: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_y1: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2734 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(3i64))?).items()?;
    if t2734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2734[0].clone();
    v_y = t2734[1].clone();
    v_n = py::len(&v_x.clone())?;
    let t2735 = Py::Int(0i64);
    v_con = t2735.clone();
    v_dis = t2735.clone();
    for t2736 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2736;
        for t2737 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2737;
            v_s = py::mul(&py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, &py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?)?;
            v_con = py::iadd(&v_con, &{ let t2738 = v_s.clone(); Py::Bool({ let t2739 = Py::Int(0i64); if py::gt(&t2738, &t2739)? { true } else { false } }) })?;
            v_dis = py::iadd(&v_dis, &{ let t2740 = v_s.clone(); Py::Bool({ let t2741 = Py::Int(0i64); if py::lt(&t2740, &t2741)? { true } else { false } }) })?;
        }
    }
    let t2746 = (Py::tuple(vec![f__tie_counts(v_x.clone())?, f__tie_counts(v_y.clone())?])).items()?;
    if t2746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tx = t2746[0].clone();
    v_ty = t2746[1].clone();
    v_n0 = py::truediv(&py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?;
    let t2749 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t2747 in py::iter(&(v_tx.clone()))? { let mut v_t: Py; 
v_t = t2747;
out.push(py::truediv(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t2748 in py::iter(&(v_ty.clone()))? { let mut v_t: Py; 
v_t = t2748;
out.push(py::truediv(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?])).items()?;
    if t2749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2749[0].clone();
    v_n2 = t2749[1].clone();
    if ({ let t2754 = { let t2750 = v_n0.clone(); Py::Bool({ let t2751 = v_n1.clone(); if py::eq(&t2750, &t2751) { true } else { false } }) }; if t2754.truthy() { t2754 } else { { let t2752 = v_n0.clone(); Py::Bool({ let t2753 = v_n2.clone(); if py::eq(&t2752, &t2753) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a variable is constant: Kendall's tau is undefined"))));
    }
    v_tau = py::truediv(&py::sub(&v_con.clone(), &v_dis.clone())?, &py::math1("sqrt", &py::mul(&py::sub(&v_n0.clone(), &v_n1.clone())?, &py::sub(&v_n0.clone(), &v_n2.clone())?)?)?)?;
    v_m = py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Float(1.0))?)?;
    let t2757 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t2755 in py::iter(&(v_tx.clone()))? { let mut v_t: Py; 
v_t = t2755;
out.push(py::mul(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &py::sub(&v_t.clone(), &Py::Int(2i64))?)?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t2756 in py::iter(&(v_ty.clone()))? { let mut v_t: Py; 
v_t = t2756;
out.push(py::mul(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &py::sub(&v_t.clone(), &Py::Int(2i64))?)?); } Py::list(out) }, None)?])).items()?;
    if t2757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t2757[0].clone();
    v_y0 = t2757[1].clone();
    let t2760 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t2758 in py::iter(&(v_tx.clone()))? { let mut v_t: Py; 
v_t = t2758;
out.push(py::mul(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &py::add(&py::mul(&Py::Int(2i64), &v_t.clone())?, &Py::Int(5i64))?)?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t2759 in py::iter(&(v_ty.clone()))? { let mut v_t: Py; 
v_t = t2759;
out.push(py::mul(&py::mul(&v_t.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, &py::add(&py::mul(&Py::Int(2i64), &v_t.clone())?, &Py::Int(5i64))?)?); } Py::list(out) }, None)?])).items()?;
    if t2760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x1 = t2760[0].clone();
    v_y1 = t2760[1].clone();
    v_var = py::add(&py::add(&py::truediv(&py::sub(&py::sub(&py::mul(&v_m.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(5i64))?)?, &v_x1.clone())?, &v_y1.clone())?, &Py::Int(18i64))?, &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_n1.clone())?, &v_n2.clone())?, &v_m.clone())?)?, &py::truediv(&py::mul(&v_x0.clone(), &v_y0.clone())?, &py::mul(&py::mul(&Py::Int(9i64), &v_m.clone())?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?)?)?;
    v_z = py::truediv(&py::sub(&v_con.clone(), &v_dis.clone())?, &py::math1("sqrt", &v_var.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("p_value"), py::minmax(&[Py::Float(1.0), py::mul(&Py::Int(2i64), &f__phic(py::abs(&v_z.clone())?)?)?], false)?), (Py::str("concordant"), v_con.clone()), (Py::str("discordant"), v_dis.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_km(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_at_risk: Py = Py::None;
    let mut v_at_t: Py = Py::None;
    let mut v_curve_s: Py = Py::None;
    let mut v_curve_t: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2671 = (f__surv_data(v_times.clone(), v_events.clone())?).items()?;
    if t2671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2671[0].clone();
    v_E = t2671[1].clone();
    let t2672 = (Py::tuple(vec![Py::Float(1.0), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2672.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2672[0].clone();
    v_curve_t = t2672[1].clone();
    v_curve_s = t2672[2].clone();
    for t2677 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t2673 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_ev: Py; let mut v_v: Py; 
let t2674 = (t2673).items()?;
if t2674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2674[0].clone();
v_ev = t2674[1].clone();
if ({ let t2675 = v_ev.clone(); Py::Bool({ let t2676 = Py::Int(1i64); if py::eq(&t2675, &t2676) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t2677;
        v_at_risk = py::sum(&{ let mut out = vec![]; for t2678 in py::iter(&(v_T.clone()))? { let mut v_v: Py; 
v_v = t2678;
if ({ let t2679 = v_v.clone(); Py::Bool({ let t2680 = v_u.clone(); if py::ge(&t2679, &t2680)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_d = py::sum(&{ let mut out = vec![]; for t2681 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_ev: Py; let mut v_v: Py; 
let t2682 = (t2681).items()?;
if t2682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2682[0].clone();
v_ev = t2682[1].clone();
if ({ let t2687 = { let t2683 = v_v.clone(); Py::Bool({ let t2684 = v_u.clone(); if py::eq(&t2683, &t2684) { true } else { false } }) }; if !t2687.truthy() { t2687 } else { { let t2685 = v_ev.clone(); Py::Bool({ let t2686 = Py::Int(1i64); if py::eq(&t2685, &t2686) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_s = py::mul(&v_s, &py::sub(&Py::Int(1i64), &py::truediv(&v_d.clone(), &v_at_risk.clone())?)?)?;
        let _ = py::method(&v_curve_t.clone(), "append", &[v_u.clone()])?;
        let _ = py::method(&v_curve_s.clone(), "append", &[v_s.clone()])?;
    }
    v_at_t = Py::Float(1.0);
    for t2688 in py::iter(&(py::zip(&[v_curve_t.clone(), v_curve_s.clone()])?))? {
        let t2691 = (t2688).items()?;
        if t2691.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2691[0].clone();
        v_v = t2691[1].clone();
        if ({ let t2689 = v_u.clone(); Py::Bool({ let t2690 = v_t.clone(); if py::le(&t2689, &t2690)? { true } else { false } }) }).truthy() {
            v_at_t = v_v.clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("survival"), v_at_t.clone()), (Py::str("event_times"), v_curve_t.clone()), (Py::str("survival_curve"), v_curve_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kruskal(mut v_g1: Py, mut v_g2: Py, mut v_g3: Py, mut v_g4: Py, mut v_g5: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_allv: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gs: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_gs = f__groups(Py::tuple(vec![v_g1.clone(), v_g2.clone(), v_g3.clone(), v_g4.clone(), v_g5.clone()]))?;
    for t2803 in py::iter(&(v_gs.clone()))? {
        v_g = t2803;
        let _ = f__need(v_g.clone(), Py::Int(1i64), Py::str("each group"))?;
    }
    v_allv = { let mut out = vec![]; for t2806 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t2806;
for t2807 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t2807;
out.push(v_v.clone()); }} Py::list(out) };
    let t2808 = (Py::tuple(vec![py::len(&v_allv.clone())?, py::len(&v_gs.clone())?])).items()?;
    if t2808.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t2808[0].clone();
    v_k = t2808[1].clone();
    v_r = f__ranks(v_allv.clone())?;
    let t2817 = (Py::tuple(vec![Py::Float(0.0), Py::Int(0i64)])).items()?;
    if t2817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2817[0].clone();
    v_i = t2817[1].clone();
    for t2818 in py::iter(&(v_gs.clone()))? {
        v_g = t2818;
        v_h = py::iadd(&v_h, &py::truediv(&py::pow(&py::fsum(&py::slice(&v_r.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &py::len(&v_g.clone())?)?).i()?), None)?)?, &Py::Int(2i64))?, &py::len(&v_g.clone())?)?)?;
        v_i = py::iadd(&v_i, &py::len(&v_g.clone())?)?;
    }
    v_h = py::sub(&py::mul(&py::truediv(&Py::Float(12.0), &py::mul(&v_N.clone(), &py::add(&v_N.clone(), &Py::Int(1i64))?)?)?, &v_h.clone())?, &py::mul(&Py::Int(3i64), &py::add(&v_N.clone(), &Py::Int(1i64))?)?)?;
    v_c = py::sub(&Py::Int(1i64), &py::truediv(&py::sum(&{ let mut out = vec![]; for t2823 in py::iter(&(f__tie_counts(v_allv.clone())?))? { let mut v_t: Py; 
v_t = t2823;
out.push(py::sub(&py::pow(&v_t.clone(), &Py::Int(3i64))?, &v_t.clone())?); } Py::list(out) }, None)?, &py::sub(&py::pow(&v_N.clone(), &Py::Int(3i64))?, &v_N.clone())?)?)?;
    if ({ let t2824 = v_c.clone(); Py::Bool({ let t2825 = Py::Int(0i64); if py::eq(&t2824, &t2825) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all values are tied"))));
    }
    v_h = py::truediv(&v_h, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("df"), py::sub(&v_k.clone(), &Py::Int(1i64))?), (Py::str("p_value"), f__chi2_sf(v_h.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_levene(mut v_g1: Py, mut v_g2: Py, mut v_g3: Py, mut v_g4: Py, mut v_g5: Py, mut v_center: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gs: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_gs = f__groups(Py::tuple(vec![v_g1.clone(), v_g2.clone(), v_g3.clone(), v_g4.clone(), v_g5.clone()]))?;
    v_c = (if ({ let t2884 = v_center.clone(); Py::Bool({ let t2885 = Py::str("median"); if py::eq(&t2884, &t2885) { true } else { false } }) }).truthy() { py::func(|a: &[Py]| -> R { f__median(a.get(0).cloned().unwrap_or(Py::None)) }) } else { py::func(|a: &[Py]| -> R { f__mean(a.get(0).cloned().unwrap_or(Py::None)) }) });
    v_z = { let mut out = vec![]; for t2886 in py::iter(&(v_gs.clone()))? { let mut v_g: Py; 
v_g = t2886;
out.push({ let mut out = vec![]; for t2887 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t2887;
out.push(py::abs(&py::sub(&v_v.clone(), &py::call(&v_c.clone(), &[v_g.clone()])?)?)?); } Py::list(out) }); } Py::list(out) };
    for t2888 in py::iter(&(v_gs.clone()))? {
        v_g = t2888;
        let _ = f__need(v_g.clone(), Py::Int(2i64), Py::str("each group"))?;
    }
    v_r = { let mut v: Vec<Py> = vec![]; v.extend((v_z.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "g1") { Some(x) => x, None => { return Err(PyErr::Type("run_anova() missing argument g1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "g2") { Some(x) => x, None => { return Err(PyErr::Type("run_anova() missing argument g2".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "g3") { Some(x) => x, None => Py::None } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "g4") { Some(x) => x, None => Py::None } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "g5") { Some(x) => x, None => Py::None } }; if v.len() > 5 { return Err(PyErr::Type("run_anova() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("run_anova() got an unexpected keyword argument".into())) } f_run_anova(a0, a1, a2, a3, a4)? };
    return Ok(Py::dict(vec![(Py::str("w"), py::getitem(&v_r.clone(), &Py::str("f"))?), (Py::str("p_value"), py::getitem(&v_r.clone(), &Py::str("p_value"))?), (Py::str("df1"), py::getitem(&v_r.clone(), &Py::str("df_between"))?), (Py::str("df2"), py::getitem(&v_r.clone(), &Py::str("df_within"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_logrank(mut v_times1: Py, mut v_events1: Py, mut v_times2: Py, mut v_events2: Py) -> R {
    let mut v_E1: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_o1: Py = Py::None;
    let mut v_stat: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2972 = (f__surv_data(v_times1.clone(), v_events1.clone())?).items()?;
    if t2972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2972[0].clone();
    v_E1 = t2972[1].clone();
    let t2973 = (f__surv_data(v_times2.clone(), v_events2.clone())?).items()?;
    if t2973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T2 = t2973[0].clone();
    v_E2 = t2973[1].clone();
    let t2974 = Py::Float(0.0);
    v_o1 = t2974.clone();
    v_e1 = t2974.clone();
    v_v = t2974.clone();
    for t2979 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t2975 in py::iter(&(py::zip(&[py::add(&v_T1.clone(), &v_T2.clone())?, py::add(&v_E1.clone(), &v_E2.clone())?])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2976 = (t2975).items()?;
if t2976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2976[0].clone();
v_e = t2976[1].clone();
if ({ let t2977 = v_e.clone(); Py::Bool({ let t2978 = Py::Int(1i64); if py::eq(&t2977, &t2978) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t2979;
        let t2986 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t2980 in py::iter(&(v_T1.clone()))? { let mut v_t: Py; 
v_t = t2980;
if ({ let t2981 = v_t.clone(); Py::Bool({ let t2982 = v_u.clone(); if py::ge(&t2981, &t2982)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t2983 in py::iter(&(v_T2.clone()))? { let mut v_t: Py; 
v_t = t2983;
if ({ let t2984 = v_t.clone(); Py::Bool({ let t2985 = v_u.clone(); if py::ge(&t2984, &t2985)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])).items()?;
        if t2986.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n1 = t2986[0].clone();
        v_n2 = t2986[1].clone();
        v_d1 = py::sum(&{ let mut out = vec![]; for t2987 in py::iter(&(py::zip(&[v_T1.clone(), v_E1.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2988 = (t2987).items()?;
if t2988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2988[0].clone();
v_e = t2988[1].clone();
if ({ let t2993 = { let t2989 = v_t.clone(); Py::Bool({ let t2990 = v_u.clone(); if py::eq(&t2989, &t2990) { true } else { false } }) }; if !t2993.truthy() { t2993 } else { { let t2991 = v_e.clone(); Py::Bool({ let t2992 = Py::Int(1i64); if py::eq(&t2991, &t2992) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_d = py::add(&v_d1.clone(), &py::sum(&{ let mut out = vec![]; for t2994 in py::iter(&(py::zip(&[v_T2.clone(), v_E2.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2995 = (t2994).items()?;
if t2995.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2995[0].clone();
v_e = t2995[1].clone();
if ({ let t3000 = { let t2996 = v_t.clone(); Py::Bool({ let t2997 = v_u.clone(); if py::eq(&t2996, &t2997) { true } else { false } }) }; if !t3000.truthy() { t3000 } else { { let t2998 = v_e.clone(); Py::Bool({ let t2999 = Py::Int(1i64); if py::eq(&t2998, &t2999) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?;
        v_n = py::add(&v_n1.clone(), &v_n2.clone())?;
        v_o1 = py::iadd(&v_o1, &v_d1.clone())?;
        v_e1 = py::iadd(&v_e1, &py::truediv(&py::mul(&v_d.clone(), &v_n1.clone())?, &v_n.clone())?)?;
        if ({ let t3001 = v_n.clone(); Py::Bool({ let t3002 = Py::Int(1i64); if py::gt(&t3001, &t3002)? { true } else { false } }) }).truthy() {
            v_v = py::iadd(&v_v, &py::truediv(&py::mul(&py::mul(&py::mul(&v_d.clone(), &py::truediv(&v_n1.clone(), &v_n.clone())?)?, &py::truediv(&v_n2.clone(), &v_n.clone())?)?, &py::sub(&v_n.clone(), &v_d.clone())?)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
        }
    }
    if ({ let t3003 = v_v.clone(); Py::Bool({ let t3004 = Py::Int(0i64); if py::le(&t3003, &t3004)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the log-rank variance is zero (no informative event times)"))));
    }
    v_stat = py::truediv(&py::pow(&py::sub(&v_o1.clone(), &v_e1.clone())?, &Py::Int(2i64))?, &v_v.clone())?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_stat.clone()), (Py::str("p_value"), f__chi2_sf(v_stat.clone(), Py::Int(1i64))?), (Py::str("observed_1"), v_o1.clone()), (Py::str("expected_1"), v_e1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mad(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    v_m = f__mean(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("mean_absolute_deviation"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t3311 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3311;
out.push(py::abs(&py::sub(&v_v.clone(), &v_m.clone())?)?); } Py::list(out) })?, &py::len(&v_x.clone())?)?), (Py::str("mean"), v_m.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mad_median(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    v_m = f__median(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("median_absolute_deviation"), f__median({ let mut out = vec![]; for t3390 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3390;
out.push(py::abs(&py::sub(&v_v.clone(), &v_m.clone())?)?); } Py::list(out) })?), (Py::str("median"), v_m.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mann_whitney(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tie: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let mut v_u2: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t3141 = (f__mw_core(v_x.clone(), v_y.clone())?).items()?;
    if t3141.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3141[0].clone();
    v_y = t3141[1].clone();
    v_n1 = t3141[2].clone();
    v_n2 = t3141[3].clone();
    v_u1 = t3141[4].clone();
    v_u2 = t3141[5].clone();
    v_n = py::add(&v_n1.clone(), &v_n2.clone())?;
    v_tie = py::sum(&{ let mut out = vec![]; for t3146 in py::iter(&(f__tie_counts(py::add(&v_x.clone(), &v_y.clone())?)?))? { let mut v_t: Py; 
v_t = t3146;
out.push(py::sub(&py::pow(&v_t.clone(), &Py::Int(3i64))?, &v_t.clone())?); } Py::list(out) }, None)?;
    v_s = py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&v_n1.clone(), &v_n2.clone())?, &Py::Float(12.0))?, &py::sub(&py::add(&v_n.clone(), &Py::Int(1i64))?, &py::truediv(&v_tie.clone(), &py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?)?)?;
    if ({ let t3147 = v_s.clone(); Py::Bool({ let t3148 = Py::Int(0i64); if py::eq(&t3147, &t3148) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all values are tied"))));
    }
    let t3149 = (py::method(&Py::dict(vec![(Py::str("greater"), Py::tuple(vec![v_u1.clone(), Py::Int(1i64)])), (Py::str("less"), Py::tuple(vec![v_u2.clone(), Py::Int(1i64)]))]), "get", &[v_alternative.clone(), Py::tuple(vec![py::minmax(&[v_u1.clone(), v_u2.clone()], true)?, Py::Int(2i64)])])?).items()?;
    if t3149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3149[0].clone();
    v_f = t3149[1].clone();
    v_z = py::truediv(&py::sub(&py::sub(&v_u.clone(), &py::truediv(&py::mul(&v_n1.clone(), &v_n2.clone())?, &Py::Float(2.0))?)?, &Py::Float(0.5))?, &v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("u"), v_u1.clone()), (Py::str("p_value"), py::minmax(&[Py::Float(1.0), py::mul(&v_f.clone(), &f__phic(v_z.clone())?)?], false)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mann_whitney_exact(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let mut v_u2: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t3072 = (f__mw_core(v_x.clone(), v_y.clone())?).items()?;
    if t3072.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3072[0].clone();
    v_y = t3072[1].clone();
    v_n1 = t3072[2].clone();
    v_n2 = t3072[3].clone();
    v_u1 = t3072[4].clone();
    v_u2 = t3072[5].clone();
    if (f__tie_counts(py::add(&v_x.clone(), &v_y.clone())?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the exact test needs data without ties; use the large-sample Mann-Whitney test"))));
    }
    if ({ let t3081 = { let t3077 = v_n1.clone(); Py::Bool({ let t3078 = Py::Int(30i64); if py::gt(&t3077, &t3078)? { true } else { false } }) }; if t3081.truthy() { t3081 } else { { let t3079 = v_n2.clone(); Py::Bool({ let t3080 = Py::Int(30i64); if py::gt(&t3079, &t3080)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the exact test is limited to samples of at most 30; use the large-sample test"))));
    }
    v_c = f__gauss_binom_counts(v_n1.clone(), v_n2.clone())?;
    v_tot = py::sum(&v_c.clone(), None)?;
    let t3089 = (py::method(&Py::dict(vec![(Py::str("greater"), Py::tuple(vec![v_u1.clone(), Py::Int(1i64)])), (Py::str("less"), Py::tuple(vec![v_u2.clone(), Py::Int(1i64)]))]), "get", &[v_alternative.clone(), Py::tuple(vec![py::minmax(&[v_u1.clone(), v_u2.clone()], true)?, Py::Int(2i64)])])?).items()?;
    if t3089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3089[0].clone();
    v_f = t3089[1].clone();
    return Ok(Py::dict(vec![(Py::str("u"), v_u1.clone()), (Py::str("p_value"), py::minmax(&[Py::Float(1.0), py::mul(&v_f.clone(), &py::float(&py::fraction(&[py::sum(&py::slice(&v_c.clone(), Some((py::int(&py::round(&v_u.clone(), None)?)?).i()?), None, None)?, None)?, v_tot.clone()])?)?)?], false)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mcnemar(mut v_table: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_stat: Py = Py::None;
    let t3211 = (f__two_by_two(v_table.clone())?).items()?;
    if t3211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t3212 = (t3211[0].clone()).items()?;
    if t3212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3212[0].clone();
    v_b = t3212[1].clone();
    let t3213 = (t3211[1].clone()).items()?;
    if t3213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3213[0].clone();
    v_d = t3213[1].clone();
    if ({ let t3214 = py::add(&v_b.clone(), &v_c.clone())?; Py::Bool({ let t3215 = Py::Int(0i64); if py::eq(&t3214, &t3215) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there are no discordant pairs (b + c = 0)"))));
    }
    v_stat = py::truediv(&py::pow(&py::sub(&py::abs(&py::sub(&v_b.clone(), &v_c.clone())?)?, &Py::Int(1i64))?, &Py::Int(2i64))?, &py::add(&v_b.clone(), &v_c.clone())?)?;
    v_m = py::minmax(&[v_b.clone(), v_c.clone()], false)?;
    v_exact = py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::mul(&Py::Int(2i64), &py::fraction(&[py::sum(&{ let mut out = vec![]; for t3216 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t3216;
out.push(py::comb(&py::add(&v_b.clone(), &v_c.clone())?, &v_j.clone())?); } Py::list(out) }, None)?, py::pow(&Py::Int(2i64), &py::add(&v_b.clone(), &v_c.clone())?)?])?)?], false)?;
    return Ok(Py::dict(vec![(Py::str("chi_square"), v_stat.clone()), (Py::str("p_value"), f__chi2_sf(v_stat.clone(), Py::Int(1i64))?), (Py::str("exact_p_value"), py::float(&v_exact.clone())?), (Py::str("discordant"), py::add(&v_b.clone(), &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mean(mut v_x: Py) -> R {
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    return Ok(Py::dict(vec![(Py::str("mean"), f__mean(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?), (Py::str("sum"), py::fsum(&v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_median(mut v_x: Py) -> R {
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    return Ok(Py::dict(vec![(Py::str("median"), f__median(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mode(mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_modes: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    v_c = Py::dict(vec![]);
    for t3430 in py::iter(&(v_x.clone()))? {
        v_v = t3430;
        py::setitem(&v_c.clone(), &v_v.clone(), py::add(&py::method(&v_c.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    v_top = py::minmax(&[py::method(&v_c.clone(), "values", &[])?], true)?;
    v_modes = py::sorted(&{ let mut out = vec![]; for t3431 in py::iter(&(py::method(&v_c.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3432 = (t3431).items()?;
if t3432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3432[0].clone();
v_k = t3432[1].clone();
if ({ let t3433 = v_k.clone(); Py::Bool({ let t3434 = v_top.clone(); if py::eq(&t3433, &t3434) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    if ({ let t3439 = { let t3435 = py::len(&v_c.clone())?; Py::Bool({ let t3436 = Py::Int(1i64); if py::gt(&t3435, &t3436)? { true } else { false } }) }; if !t3439.truthy() { t3439 } else { { let t3437 = py::minmax(&[py::method(&v_c.clone(), "values", &[])?], false)?; Py::Bool({ let t3438 = v_top.clone(); if py::eq(&t3437, &t3438) { true } else { false } }) } } }).truthy() {
        v_modes = Py::list(vec![]);
    }
    return Ok(Py::dict(vec![(Py::str("modes"), v_modes.clone()), (Py::str("frequency"), v_top.clone()), (Py::str("has_mode"), (if (v_modes.clone()).truthy() { Py::str("yes") } else { Py::str("no") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_multiple_regression(mut v_X: Py, mut v_y: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_Sy: Py = Py::None;
    let mut v_Xc: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_dfe: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let mut v_yc: Py = Py::None;
    v_X = f__matrix_ok(v_X.clone(), Py::str("the predictor matrix"), Py::Int(3i64), Py::Int(1i64))?;
    v_y = f__finite(v_y.clone(), Py::str("y"))?;
    let t3487 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3487[0].clone();
    v_k = t3487[1].clone();
    if ({ let t3488 = py::len(&v_y.clone())?; Py::Bool({ let t3489 = v_n.clone(); if py::ne(&t3488, &t3489) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("y needs one value per row of the predictor matrix"))));
    }
    if ({ let t3490 = v_n.clone(); Py::Bool({ let t3491 = py::add(&v_k.clone(), &Py::Int(1i64))?; if py::le(&t3490, &t3491)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("more observations than predictors + 1 are required"))));
    }
    v_mx = { let mut out = vec![]; for t3492 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3492;
out.push(f__mean({ let mut out = vec![]; for t3493 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t3493;
out.push(py::getitem(&v_r.clone(), &v_j.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_my = f__mean(v_y.clone())?;
    v_Xc = { let mut out = vec![]; for t3494 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t3494;
out.push({ let mut out = vec![]; for t3495 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3495;
out.push(py::sub(&py::getitem(&v_r.clone(), &v_j.clone())?, &py::getitem(&v_mx.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    v_yc = { let mut out = vec![]; for t3496 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3496;
out.push(py::sub(&v_v.clone(), &v_my.clone())?); } Py::list(out) };
    v_S = { let mut out = vec![]; for t3497 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_a: Py; 
v_a = t3497;
out.push({ let mut out = vec![]; for t3498 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_b: Py; 
v_b = t3498;
out.push(py::fsum(&{ let mut out = vec![]; for t3499 in py::iter(&(v_Xc.clone()))? { let mut v_row: Py; 
v_row = t3499;
out.push(py::mul(&py::getitem(&v_row.clone(), &v_a.clone())?, &py::getitem(&v_row.clone(), &v_b.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_Sy = { let mut out = vec![]; for t3500 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_a: Py; 
v_a = t3500;
out.push(py::fsum(&{ let mut out = vec![]; for t3501 in py::iter(&(py::zip(&[v_Xc.clone(), v_yc.clone()])?))? { let mut v_row: Py; let mut v_v: Py; 
let t3502 = (t3501).items()?;
if t3502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t3502[0].clone();
v_v = t3502[1].clone();
out.push(py::mul(&py::getitem(&v_row.clone(), &v_a.clone())?, &v_v.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_b = f__solve(v_S.clone(), v_Sy.clone())?;
    v_b0 = py::sub(&v_my.clone(), &py::fsum(&{ let mut out = vec![]; for t3516 in py::iter(&(py::zip(&[v_b.clone(), v_mx.clone()])?))? { let mut v_bj: Py; let mut v_m: Py; 
let t3517 = (t3516).items()?;
if t3517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bj = t3517[0].clone();
v_m = t3517[1].clone();
out.push(py::mul(&v_bj.clone(), &v_m.clone())?); } Py::list(out) })?)?;
    v_res = { let mut out = vec![]; for t3518 in py::iter(&(py::zip(&[v_X.clone(), v_y.clone()])?))? { let mut v_r: Py; let mut v_v: Py; 
let t3519 = (t3518).items()?;
if t3519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3519[0].clone();
v_v = t3519[1].clone();
out.push(py::sub(&py::sub(&v_v.clone(), &v_b0.clone())?, &py::fsum(&{ let mut out = vec![]; for t3520 in py::iter(&(py::zip(&[v_b.clone(), v_r.clone()])?))? { let mut v_bj: Py; let mut v_xj: Py; 
let t3521 = (t3520).items()?;
if t3521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bj = t3521[0].clone();
v_xj = t3521[1].clone();
out.push(py::mul(&v_bj.clone(), &v_xj.clone())?); } Py::list(out) })?)?); } Py::list(out) };
    let t3524 = (Py::tuple(vec![py::fsum(&{ let mut out = vec![]; for t3522 in py::iter(&(v_res.clone()))? { let mut v_e: Py; 
v_e = t3522;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t3523 in py::iter(&(v_yc.clone()))? { let mut v_v: Py; 
v_v = t3523;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?])).items()?;
    if t3524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sse = t3524[0].clone();
    v_sst = t3524[1].clone();
    if ({ let t3525 = v_sst.clone(); Py::Bool({ let t3526 = Py::Int(0i64); if py::eq(&t3525, &t3526) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("y is constant"))));
    }
    v_r2 = py::sub(&Py::Int(1i64), &py::truediv(&v_sse.clone(), &v_sst.clone())?)?;
    v_dfe = py::sub(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?;
    v_adj = py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&py::sub(&Py::Int(1i64), &v_r2.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &v_dfe.clone())?)?;
    v_f = (if ({ let t3527 = v_sse.clone(); Py::Bool({ let t3528 = Py::Int(0i64); if py::gt(&t3527, &t3528)? { true } else { false } }) }).truthy() { py::truediv(&py::truediv(&py::sub(&v_sst.clone(), &v_sse.clone())?, &v_k.clone())?, &py::truediv(&v_sse.clone(), &v_dfe.clone())?)? } else { Py::Float(f64::INFINITY) });
    return Ok(Py::dict(vec![(Py::str("coefficients"), py::add(&Py::list(vec![v_b0.clone()]), &v_b.clone())?), (Py::str("r_squared"), v_r2.clone()), (Py::str("adj_r_squared"), v_adj.clone()), (Py::str("f"), v_f.clone()), (Py::str("f_p_value"), (if ({ let t3529 = v_sse.clone(); Py::Bool({ let t3530 = Py::Int(0i64); if py::gt(&t3529, &t3530)? { true } else { false } }) }).truthy() { py::getitem(&f__f_tails(v_f.clone(), v_k.clone(), v_dfe.clone())?, &Py::Int(1i64))? } else { Py::Float(0.0) })), (Py::str("residual_se"), py::math1("sqrt", &py::truediv(&v_sse.clone(), &v_dfe.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_n_mean(mut v_sigma: Py, mut v_margin: Py, mut v_confidence: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t5031 = (Py::tuple(vec![f__pos(v_sigma.clone(), Py::str("sigma"))?, f__pos(v_margin.clone(), Py::str("the margin of error"))?])).items()?;
    if t5031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t5031[0].clone();
    v_e = t5031[1].clone();
    v_z = f__zc(v_confidence.clone())?;
    return Ok(Py::dict(vec![(Py::str("n"), py::math1("ceil", &py::pow(&py::truediv(&py::mul(&v_z.clone(), &v_s.clone())?, &v_e.clone())?, &Py::Int(2i64))?)?), (Py::str("n_exact"), py::pow(&py::truediv(&py::mul(&v_z.clone(), &v_s.clone())?, &v_e.clone())?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_n_prop(mut v_margin: Py, mut v_confidence: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t5090 = (Py::tuple(vec![f__pos(v_margin.clone(), Py::str("the margin of error"))?, f__prob(v_p.clone(), Py::str("p"), Py::Bool(true))?])).items()?;
    if t5090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t5090[0].clone();
    v_p = t5090[1].clone();
    if ({ let t5091 = v_e.clone(); Py::Bool({ let t5092 = Py::Int(1i64); if py::ge(&t5091, &t5092)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the margin of error must be below 1"))));
    }
    v_z = f__zc(v_confidence.clone())?;
    return Ok(Py::dict(vec![(Py::str("n"), py::math1("ceil", &py::truediv(&py::mul(&py::mul(&py::mul(&v_z.clone(), &v_z.clone())?, &v_p.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &py::mul(&v_e.clone(), &v_e.clone())?)?)?), (Py::str("n_exact"), py::truediv(&py::mul(&py::mul(&py::mul(&v_z.clone(), &v_z.clone())?, &v_p.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &py::mul(&v_e.clone(), &v_e.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_n_two_means(mut v_d: Py, mut v_alpha: Py, mut v_power: Py, mut v_alternative: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_alpha = f__pw_args(v_d.clone(), v_alpha.clone(), v_alternative.clone())?;
    v_power = f__prob(v_power.clone(), Py::str("power"), Py::Bool(true))?;
    if ({ let t5163 = v_power.clone(); Py::Bool({ let t5164 = v_alpha.clone(); if py::le(&t5163, &t5164)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the power must exceed alpha"))));
    }
    v_f = { let v_alpha = v_alpha.clone(); let v_alternative = v_alternative.clone(); let v_d = v_d.clone(); let v_power = v_power.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__zpower(v_d.clone(), py::truediv(&v_n.clone(), &Py::Float(2.0))?, v_alpha.clone(), v_alternative.clone())?, &v_power.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5177 = (Py::tuple(vec![Py::Float(1e-09), Py::Float(2.0)])).items()?;
    if t5177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5177[0].clone();
    v_hi = t5177[1].clone();
    while ({ let t5179 = py::call(&v_f.clone(), &[v_hi.clone()])?; Py::Bool({ let t5180 = Py::Int(0i64); if py::lt(&t5179, &t5180)? { true } else { false } }) }).truthy() {
        let t5178 = (Py::tuple(vec![v_hi.clone(), py::mul(&v_hi.clone(), &Py::Int(2i64))?])).items()?;
        if t5178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t5178[0].clone();
        v_hi = t5178[1].clone();
    }
    for t5181 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t5181;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t5186 = { let t5182 = v_mid.clone(); Py::Bool({ let t5183 = v_lo.clone(); if py::le(&t5182, &t5183)? { true } else { false } }) }; if t5186.truthy() { t5186 } else { { let t5184 = v_mid.clone(); Py::Bool({ let t5185 = v_hi.clone(); if py::ge(&t5184, &t5185)? { true } else { false } }) } } }).truthy() {
            break;
        }
        let t5189 = ((if ({ let t5187 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t5188 = Py::Int(0i64); if py::lt(&t5187, &t5188)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_mid.clone(), v_hi.clone()]) } else { Py::tuple(vec![v_lo.clone(), v_mid.clone()]) })).items()?;
        if t5189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t5189[0].clone();
        v_hi = t5189[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("n_per_group"), py::math1("ceil", &v_hi.clone())?), (Py::str("n_exact"), v_hi.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_negbinom(mut v_r: Py, mut v_p: Py, mut v_n: Py) -> R {
    let t3603 = (Py::tuple(vec![f__whole(v_r.clone(), Py::str("r"), Py::Int(1i64))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?, f__prob(v_p.clone(), Py::str("p"), Py::Bool(true))?])).items()?;
    if t3603.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3603[0].clone();
    v_n = t3603[1].clone();
    v_p = t3603[2].clone();
    if ({ let t3604 = v_n.clone(); Py::Bool({ let t3605 = v_r.clone(); if py::lt(&t3604, &t3605)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the r-th success cannot occur before trial r"))));
    }
    return Ok(Py::dict(vec![(Py::str("probability"), py::math1("exp", &py::add(&py::add(&py::math1("log", &py::comb(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::sub(&v_r.clone(), &Py::Int(1i64))?)?)?, &py::mul(&v_r.clone(), &py::math1("log", &v_p.clone())?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_r.clone())?, &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_normal_above(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py) -> R {
    let mut v_z: Py = Py::None;
    v_sigma = f__pos(v_sigma.clone(), Py::str("the standard deviation"))?;
    v_z = py::truediv(&py::sub(&v_x.clone(), &v_mu.clone())?, &v_sigma.clone())?;
    return Ok(Py::dict(vec![(Py::str("probability"), f__phic(v_z.clone())?), (Py::str("z"), v_z.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_normal_below(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py) -> R {
    let mut v_z: Py = Py::None;
    v_sigma = f__pos(v_sigma.clone(), Py::str("the standard deviation"))?;
    v_z = py::truediv(&py::sub(&v_x.clone(), &v_mu.clone())?, &v_sigma.clone())?;
    return Ok(Py::dict(vec![(Py::str("probability"), f__phi(v_z.clone())?), (Py::str("z"), v_z.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_normal_between(mut v_a: Py, mut v_b: Py, mut v_mu: Py, mut v_sigma: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_za: Py = Py::None;
    let mut v_zb: Py = Py::None;
    v_sigma = f__pos(v_sigma.clone(), Py::str("the standard deviation"))?;
    if (py::not(&{ let t3720 = v_a.clone(); Py::Bool({ let t3721 = v_b.clone(); if py::lt(&t3720, &t3721)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the lower bound a must be below the upper bound b"))));
    }
    let t3722 = (Py::tuple(vec![py::truediv(&py::sub(&v_a.clone(), &v_mu.clone())?, &v_sigma.clone())?, py::truediv(&py::sub(&v_b.clone(), &v_mu.clone())?, &v_sigma.clone())?])).items()?;
    if t3722.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_za = t3722[0].clone();
    v_zb = t3722[1].clone();
    v_p = (if ({ let t3723 = v_za.clone(); Py::Bool({ let t3724 = Py::Int(0i64); if py::gt(&t3723, &t3724)? { true } else { false } }) }).truthy() { py::sub(&f__phic(v_za.clone())?, &f__phic(v_zb.clone())?)? } else { py::sub(&f__phi(v_zb.clone())?, &f__phi(v_za.clone())?)? });
    return Ok(Py::dict(vec![(Py::str("probability"), v_p.clone()), (Py::str("z_a"), v_za.clone()), (Py::str("z_b"), v_zb.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_outliers(mut v_x: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_iqr: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let mut v_q3: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the sample"))?;
    let t2564 = (Py::tuple(vec![f__quantile(v_x.clone(), Py::Float(0.25), Py::str("linear"))?, f__quantile(v_x.clone(), Py::Float(0.75), Py::str("linear"))?])).items()?;
    if t2564.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q1 = t2564[0].clone();
    v_q3 = t2564[1].clone();
    v_iqr = py::sub(&v_q3.clone(), &v_q1.clone())?;
    let t2565 = (Py::tuple(vec![py::sub(&v_q1.clone(), &py::mul(&Py::Float(1.5), &v_iqr.clone())?)?, py::add(&v_q3.clone(), &py::mul(&Py::Float(1.5), &v_iqr.clone())?)?])).items()?;
    if t2565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2565[0].clone();
    v_hi = t2565[1].clone();
    return Ok(Py::dict(vec![(Py::str("lower_fence"), v_lo.clone()), (Py::str("upper_fence"), v_hi.clone()), (Py::str("outliers"), py::sorted(&{ let mut out = vec![]; for t2566 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2566;
if ({ let t2571 = { let t2567 = v_v.clone(); Py::Bool({ let t2568 = v_lo.clone(); if py::lt(&t2567, &t2568)? { true } else { false } }) }; if t2571.truthy() { t2571 } else { { let t2569 = v_v.clone(); Py::Bool({ let t2570 = v_hi.clone(); if py::gt(&t2569, &t2570)? { true } else { false } }) } } }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, (Py::Bool(false)).truthy())?), (Py::str("q1"), v_q1.clone()), (Py::str("q3"), v_q3.clone()), (Py::str("iqr"), v_iqr.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_p_chart(mut v_defectives: Py, mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_pb: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_d = f__finite(v_defectives.clone(), Py::str("defectives"))?;
    v_n = f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t3848 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t3848;
out.push({ let t3856 = { let t3849 = v_v.clone(); Py::Bool({ let t3850 = Py::Int(0i64); if py::lt(&t3849, &t3850)? { true } else { false } }) }; if t3856.truthy() { t3856 } else { { let t3855 = { let t3851 = v_v.clone(); Py::Bool({ let t3852 = v_n.clone(); if py::gt(&t3851, &t3852)? { true } else { false } }) }; if t3855.truthy() { t3855 } else { { let t3853 = v_v.clone(); Py::Bool({ let t3854 = py::int(&v_v.clone())?; if py::ne(&t3853, &t3854) { true } else { false } }) } } } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each count of defectives must be a whole number between 0 and n"))));
    }
    v_pb = py::truediv(&py::fsum(&v_d.clone())?, &py::mul(&py::len(&v_d.clone())?, &v_n.clone())?)?;
    if ({ let t3857 = v_pb.clone(); Py::Bool({ let t3858 = Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]); if py::contains(&t3858, &t3857)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the average fraction defective is 0 or 1"))));
    }
    v_s = py::math1("sqrt", &py::truediv(&py::mul(&v_pb.clone(), &py::sub(&Py::Int(1i64), &v_pb.clone())?)?, &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("center"), v_pb.clone()), (Py::str("ucl"), py::minmax(&[Py::Float(1.0), py::add(&v_pb.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], false)?), (Py::str("lcl"), py::minmax(&[Py::Float(0.0), py::sub(&v_pb.clone(), &py::mul(&Py::Int(3i64), &v_s.clone())?)?], true)?), (Py::str("sigma"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_paired_t(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t3903 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(2i64))?).items()?;
    if t3903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3903[0].clone();
    v_y = t3903[1].clone();
    v_d = { let mut out = vec![]; for t3904 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3905 = (t3904).items()?;
if t3905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3905[0].clone();
v_b = t3905[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_s = f__sd(v_d.clone(), Py::Int(1i64))?;
    if ({ let t3907 = v_s.clone(); Py::Bool({ let t3908 = Py::Int(0i64); if py::eq(&t3907, &t3908) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all differences are equal: the t statistic is undefined"))));
    }
    v_n = py::len(&v_d.clone())?;
    v_out = f__t_out(f__mean(v_d.clone())?, py::truediv(&v_s.clone(), &py::math1("sqrt", &v_n.clone())?)?, py::sub(&v_n.clone(), &Py::Int(1i64))?, v_alternative.clone())?;
    py::setitem(&v_out.clone(), &Py::str("mean_difference"), py::method(&v_out.clone(), "pop", &[Py::str("difference")])?)?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_pearson(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t4030 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(3i64))?).items()?;
    if t4030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4030[0].clone();
    v_y = t4030[1].clone();
    let t4041 = (Py::tuple(vec![f__pearson(v_x.clone(), v_y.clone())?, py::len(&v_x.clone())?])).items()?;
    if t4041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t4041[0].clone();
    v_n = t4041[1].clone();
    v_t = (if ({ let t4042 = py::abs(&v_r.clone())?; Py::Bool({ let t4043 = Py::Int(1i64); if py::lt(&t4042, &t4043)? { true } else { false } }) }).truthy() { py::mul(&v_r.clone(), &py::math1("sqrt", &py::truediv(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &py::sub(&Py::Int(1i64), &py::mul(&v_r.clone(), &v_r.clone())?)?)?)?)? } else { py::copysign(&Py::Float(f64::INFINITY), &v_r.clone())? });
    return Ok(Py::dict(vec![(Py::str("r"), v_r.clone()), (Py::str("r_squared"), py::mul(&v_r.clone(), &v_r.clone())?), (Py::str("p_value"), (if ({ let t4044 = py::abs(&v_r.clone())?; Py::Bool({ let t4045 = Py::Int(1i64); if py::lt(&t4044, &t4045)? { true } else { false } }) }).truthy() { f__t_two_sided(v_t.clone(), py::sub(&v_n.clone(), &Py::Int(2i64))?)? } else { Py::Float(0.0) })), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_percentile(mut v_x: Py, mut v_p: Py, mut v_method: Py) -> R {
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    if (py::not(&{ let t4106 = Py::Int(0i64); Py::Bool({ let t4107 = v_p.clone(); if py::le(&t4106, &t4107)? { { let t4108 = Py::Int(100i64); if py::le(&t4107, &t4108)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the percentile must be between 0 and 100"))));
    }
    return Ok(Py::dict(vec![(Py::str("percentile"), f__quantile(v_x.clone(), py::truediv(&v_p.clone(), &Py::Float(100.0))?, v_method.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_power_one(mut v_d: Py, mut v_n: Py, mut v_alpha: Py, mut v_alternative: Py) -> R {
    v_alpha = f__pw_args(v_d.clone(), v_alpha.clone(), v_alternative.clone())?;
    return Ok(Py::dict(vec![(Py::str("power"), f__zpower(v_d.clone(), f__pos(v_n.clone(), Py::str("n"))?, v_alpha.clone(), v_alternative.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_power_two(mut v_d: Py, mut v_n: Py, mut v_alpha: Py, mut v_alternative: Py) -> R {
    v_alpha = f__pw_args(v_d.clone(), v_alpha.clone(), v_alternative.clone())?;
    return Ok(Py::dict(vec![(Py::str("power"), f__zpower(v_d.clone(), py::truediv(&f__pos(v_n.clone(), Py::str("n"))?, &Py::Float(2.0))?, v_alpha.clone(), v_alternative.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_predict(mut v_x: Py, mut v_y: Py, mut v_x0: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let t4900 = (f__simple_fit(v_x.clone(), v_y.clone())?).items()?;
    if t4900.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4900[0].clone();
    v_y = t4900[1].clone();
    v_mx = t4900[2].clone();
    v_my = t4900[3].clone();
    v_sxx = t4900[4].clone();
    v_b1 = t4900[5].clone();
    v_b0 = t4900[6].clone();
    return Ok(Py::dict(vec![(Py::str("y_hat"), py::add(&v_b0.clone(), &py::mul(&v_b1.clone(), &v_x0.clone())?)?), (Py::str("slope"), v_b1.clone()), (Py::str("intercept"), v_b0.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_probability_rules(mut v_p_a: Py, mut v_p_b: Py, mut v_p_a_and_b: Py, mut v_p_a_or_b: Py, mut v_relation: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_pa: Py = Py::None;
    let mut v_pb: Py = Py::None;
    let mut v_stated: Py = Py::None;
    let mut v_union: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_pa = f__prob(v_p_a.clone(), Py::str("P(A)"), Py::Bool(false))?;
    v_pb = f__prob(v_p_b.clone(), Py::str("P(B)"), Py::Bool(false))?;
    v_stated = Py::Int(0i64);
    for t4601 in py::iter(&(Py::tuple(vec![v_p_a_and_b.clone(), v_p_a_or_b.clone(), v_relation.clone()])))? {
        v_v = t4601;
        if ({ let t4602 = v_v.clone(); Py::Bool({ let t4603 = Py::None; if !py::is(&t4602, &t4603) { true } else { false } }) }).truthy() {
            v_stated = py::iadd(&v_stated, &Py::Int(1i64))?;
        }
    }
    if ({ let t4604 = v_stated.clone(); Py::Bool({ let t4605 = Py::Int(1i64); if py::ne(&t4604, &t4605) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("state exactly one of: P(A and B), P(A or B), independent, mutually exclusive"))));
    }
    if ({ let t4606 = v_relation.clone(); Py::Bool({ let t4607 = Py::None; if !py::is(&t4606, &t4607) { true } else { false } }) }).truthy() {
        if ({ let t4608 = v_relation.clone(); Py::Bool({ let t4609 = Py::str("independent"); if py::eq(&t4608, &t4609) { true } else { false } }) }).truthy() {
            v_j = py::mul(&v_pa.clone(), &v_pb.clone())?;
        } else {
            if ({ let t4610 = v_relation.clone(); Py::Bool({ let t4611 = Py::str("mutually exclusive"); if py::eq(&t4610, &t4611) { true } else { false } }) }).truthy() {
                v_j = Py::Float(0.0);
            } else {
                return Err(py::value_error(py::to_str(&Py::str("the relation is independent or mutually exclusive"))));
            }
        }
        v_union = py::sub(&py::add(&v_pa.clone(), &v_pb.clone())?, &v_j.clone())?;
    } else {
        if ({ let t4612 = v_p_a_and_b.clone(); Py::Bool({ let t4613 = Py::None; if !py::is(&t4612, &t4613) { true } else { false } }) }).truthy() {
            v_j = f__prob(v_p_a_and_b.clone(), Py::str("P(A and B)"), Py::Bool(false))?;
            v_union = py::sub(&py::add(&v_pa.clone(), &v_pb.clone())?, &v_j.clone())?;
        } else {
            v_union = f__prob(v_p_a_or_b.clone(), Py::str("P(A or B)"), Py::Bool(false))?;
            v_j = py::sub(&py::add(&v_pa.clone(), &v_pb.clone())?, &v_union.clone())?;
        }
    }
    let t4614 = (Py::tuple(vec![py::minmax(&[Py::Float(0.0), py::sub(&py::add(&v_pa.clone(), &v_pb.clone())?, &Py::Float(1.0))?], true)?, py::minmax(&[v_pa.clone(), v_pb.clone()], false)?])).items()?;
    if t4614.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4614[0].clone();
    v_hi = t4614[1].clone();
    if ({ let t4619 = { let t4615 = v_j.clone(); Py::Bool({ let t4616 = py::sub(&v_lo.clone(), &Py::Float(1e-12))?; if py::lt(&t4615, &t4616)? { true } else { false } }) }; if t4619.truthy() { t4619 } else { { let t4617 = v_j.clone(); Py::Bool({ let t4618 = py::add(&v_hi.clone(), &Py::Float(1e-12))?; if py::gt(&t4617, &t4618)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("these probabilities are inconsistent: P(A and B) must lie between max(0, P(A) + P(B) - 1) and min(P(A), P(B))"))));
    }
    if ({ let t4624 = { let t4620 = v_relation.clone(); Py::Bool({ let t4621 = Py::str("mutually exclusive"); if py::eq(&t4620, &t4621) { true } else { false } }) }; if !t4624.truthy() { t4624 } else { { let t4622 = py::add(&v_pa.clone(), &v_pb.clone())?; Py::Bool({ let t4623 = py::add(&Py::Float(1.0), &Py::Float(1e-12))?; if py::gt(&t4622, &t4623)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("mutually exclusive events need P(A) + P(B) <= 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("p_not_a"), py::sub(&Py::Float(1.0), &v_pa.clone())?), (Py::str("p_not_b"), py::sub(&Py::Float(1.0), &v_pb.clone())?), (Py::str("p_a_and_b"), py::add(&v_j.clone(), &Py::Float(0.0))?), (Py::str("p_a_or_b"), v_union.clone()), (Py::str("p_neither"), py::sub(&Py::Float(1.0), &v_union.clone())?), (Py::str("p_a_only"), py::add(&py::sub(&v_pa.clone(), &v_j.clone())?, &Py::Float(0.0))?), (Py::str("p_b_only"), py::add(&py::sub(&v_pb.clone(), &v_j.clone())?, &Py::Float(0.0))?), (Py::str("p_a_given_b"), (if ({ let t4625 = v_pb.clone(); Py::Bool({ let t4626 = Py::Int(0i64); if py::gt(&t4625, &t4626)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&v_j.clone(), &v_pb.clone())?, &Py::Float(0.0))? } else { Py::None })), (Py::str("p_b_given_a"), (if ({ let t4627 = v_pa.clone(); Py::Bool({ let t4628 = Py::Int(0i64); if py::gt(&t4627, &t4628)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&v_j.clone(), &v_pa.clone())?, &Py::Float(0.0))? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_prop_z(mut v_x: Py, mut v_n: Py, mut v_p0: Py, mut v_alternative: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t4716 = (f__prop_args(v_x.clone(), v_n.clone())?).items()?;
    if t4716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4716[0].clone();
    v_n = t4716[1].clone();
    v_p0 = f__prob(v_p0.clone(), Py::str("p0"), Py::Bool(true))?;
    v_z = py::truediv(&py::sub(&py::truediv(&v_x.clone(), &v_n.clone())?, &v_p0.clone())?, &py::math1("sqrt", &py::truediv(&py::mul(&v_p0.clone(), &py::sub(&Py::Int(1i64), &v_p0.clone())?)?, &v_n.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("p_value"), f__alt_p(v_z.clone(), v_alternative.clone(), py::func(|a: &[Py]| -> R { f__phic(a.get(0).cloned().unwrap_or(Py::None)) }), py::func(|a: &[Py]| -> R { f__phi(a.get(0).cloned().unwrap_or(Py::None)) }))?), (Py::str("proportion"), py::truediv(&v_x.clone(), &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_range(mut v_x: Py) -> R {
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    return Ok(Py::dict(vec![(Py::str("range"), py::sub(&py::minmax(&[v_x.clone()], true)?, &py::minmax(&[v_x.clone()], false)?)?), (Py::str("minimum"), py::minmax(&[v_x.clone()], false)?), (Py::str("maximum"), py::minmax(&[v_x.clone()], true)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sem(mut v_x: Py) -> R {
    let mut v_s: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_s = f__sd(v_x.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("standard_error"), py::truediv(&v_s.clone(), &py::math1("sqrt", &py::len(&v_x.clone())?)?)?), (Py::str("standard_deviation"), v_s.clone()), (Py::str("n"), py::len(&v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sign_test(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pmf: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t5378 = (f__signed(v_x.clone(), v_y.clone())?).items()?;
    if t5378.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d = t5378[0].clone();
    v_r = t5378[1].clone();
    v_rp = t5378[2].clone();
    v_rm = t5378[3].clone();
    let t5382 = (Py::tuple(vec![py::len(&v_d.clone())?, py::sum(&{ let mut out = vec![]; for t5379 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t5379;
if ({ let t5380 = v_v.clone(); Py::Bool({ let t5381 = Py::Int(0i64); if py::gt(&t5380, &t5381)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])).items()?;
    if t5382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5382[0].clone();
    v_k = t5382[1].clone();
    v_pmf = { let v_alternative = v_alternative.clone(); let v_d = v_d.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_r = v_r.clone(); let v_rm = v_rm.clone(); let v_rp = v_rp.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::fraction(&[py::comb(&v_n.clone(), &v_j.clone())?, py::pow(&Py::Int(2i64), &v_n.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5385 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t5383 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t5383;
out.push(py::call(&v_pmf.clone(), &[v_j.clone()])?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t5384 in py::range_iter(&[(v_k.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t5384;
out.push(py::call(&v_pmf.clone(), &[v_j.clone()])?); } Py::list(out) }, None)?])).items()?;
    if t5385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5385[0].clone();
    v_hi = t5385[1].clone();
    v_p = py::method(&Py::dict(vec![(Py::str("greater"), v_hi.clone()), (Py::str("less"), v_lo.clone())]), "get", &[v_alternative.clone(), py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::mul(&Py::Int(2i64), &py::minmax(&[v_lo.clone(), v_hi.clone()], false)?)?], false)?])?;
    return Ok(Py::dict(vec![(Py::str("positives"), v_k.clone()), (Py::str("n"), v_n.clone()), (Py::str("p_value"), py::float(&v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_simple_regression(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_se0: Py = Py::None;
    let mut v_se1: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let t5435 = (f__simple_fit(v_x.clone(), v_y.clone())?).items()?;
    if t5435.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_x = t5435[0].clone();
    v_y = t5435[1].clone();
    v_mx = t5435[2].clone();
    v_my = t5435[3].clone();
    v_sxx = t5435[4].clone();
    v_b1 = t5435[5].clone();
    v_b0 = t5435[6].clone();
    v_n = py::len(&v_x.clone())?;
    v_sse = py::fsum(&{ let mut out = vec![]; for t5436 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5437 = (t5436).items()?;
if t5437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5437[0].clone();
v_b = t5437[1].clone();
out.push(py::pow(&py::sub(&py::sub(&v_b.clone(), &v_b0.clone())?, &py::mul(&v_b1.clone(), &v_a.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_syy = py::fsum(&{ let mut out = vec![]; for t5438 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t5438;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_r2 = (if ({ let t5439 = v_syy.clone(); Py::Bool({ let t5440 = Py::Int(0i64); if py::gt(&t5439, &t5440)? { true } else { false } }) }).truthy() { py::sub(&Py::Int(1i64), &py::truediv(&v_sse.clone(), &v_syy.clone())?)? } else { Py::Float(1.0) });
    v_s2 = py::truediv(&v_sse.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?;
    v_se1 = py::math1("sqrt", &py::truediv(&v_s2.clone(), &v_sxx.clone())?)?;
    v_se0 = py::math1("sqrt", &py::mul(&v_s2.clone(), &py::add(&py::truediv(&Py::Int(1i64), &v_n.clone())?, &py::truediv(&py::mul(&v_mx.clone(), &v_mx.clone())?, &v_sxx.clone())?)?)?)?;
    v_p = (if ({ let t5441 = v_se1.clone(); Py::Bool({ let t5442 = Py::Int(0i64); if py::gt(&t5441, &t5442)? { true } else { false } }) }).truthy() { f__t_two_sided(py::truediv(&v_b1.clone(), &v_se1.clone())?, py::sub(&v_n.clone(), &Py::Int(2i64))?)? } else { Py::Float(0.0) });
    v_r = (if ({ let t5464 = v_b1.clone(); Py::Bool({ let t5465 = Py::Int(0i64); if py::ne(&t5464, &t5465) { true } else { false } }) }).truthy() { py::copysign(&py::math1("sqrt", &py::minmax(&[v_r2.clone(), Py::Float(0.0)], true)?)?, &v_b1.clone())? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("slope"), v_b1.clone()), (Py::str("intercept"), v_b0.clone()), (Py::str("r"), v_r.clone()), (Py::str("r_squared"), v_r2.clone()), (Py::str("p_value"), v_p.clone()), (Py::str("slope_se"), v_se1.clone()), (Py::str("intercept_se"), v_se0.clone()), (Py::str("residual_se"), py::math1("sqrt", &v_s2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_skew_kurt(mut v_x: Py) -> R {
    let mut v_G1: Py = Py::None;
    let mut v_G2: Py = Py::None;
    let mut v_g1: Py = Py::None;
    let mut v_g2: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_m3: Py = Py::None;
    let mut v_m4: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the sample"))?;
    v_n = py::len(&v_x.clone())?;
    v_m = f__mean(v_x.clone())?;
    v_m2 = py::truediv(&py::fsum(&{ let mut out = vec![]; for t5505 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5505;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_n.clone())?;
    v_m3 = py::truediv(&py::fsum(&{ let mut out = vec![]; for t5506 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5506;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(3i64))?); } Py::list(out) })?, &v_n.clone())?;
    v_m4 = py::truediv(&py::fsum(&{ let mut out = vec![]; for t5507 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5507;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(4i64))?); } Py::list(out) })?, &v_n.clone())?;
    if ({ let t5508 = v_m2.clone(); Py::Bool({ let t5509 = Py::Int(0i64); if py::eq(&t5508, &t5509) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all values are equal: skewness and kurtosis are undefined"))));
    }
    let t5510 = (Py::tuple(vec![py::truediv(&v_m3.clone(), &py::pow(&v_m2.clone(), &Py::Float(1.5))?)?, py::sub(&py::truediv(&v_m4.clone(), &py::pow(&v_m2.clone(), &Py::Int(2i64))?)?, &Py::Float(3.0))?])).items()?;
    if t5510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g1 = t5510[0].clone();
    v_g2 = t5510[1].clone();
    v_G1 = py::truediv(&py::mul(&v_g1.clone(), &py::math1("sqrt", &py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?;
    v_G2 = py::truediv(&py::mul(&py::add(&py::mul(&py::add(&v_n.clone(), &Py::Int(1i64))?, &v_g2.clone())?, &Py::Float(6.0))?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &py::sub(&v_n.clone(), &Py::Int(3i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("skewness"), v_G1.clone()), (Py::str("excess_kurtosis"), v_G2.clone()), (Py::str("skewness_biased"), v_g1.clone()), (Py::str("excess_kurtosis_biased"), v_g2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_spearman(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t5553 = (f__pair(v_x.clone(), v_y.clone(), Py::Int(3i64))?).items()?;
    if t5553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t5553[0].clone();
    v_y = t5553[1].clone();
    let t5572 = (Py::tuple(vec![f__pearson(f__ranks(v_x.clone())?, f__ranks(v_y.clone())?)?, py::len(&v_x.clone())?])).items()?;
    if t5572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t5572[0].clone();
    v_n = t5572[1].clone();
    v_p = (if ({ let t5573 = py::abs(&v_rho.clone())?; Py::Bool({ let t5574 = Py::Int(1i64); if py::ge(&t5573, &t5574)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { f__t_two_sided(py::mul(&v_rho.clone(), &py::math1("sqrt", &py::truediv(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &py::sub(&Py::Int(1i64), &py::mul(&v_rho.clone(), &v_rho.clone())?)?)?)?)?, py::sub(&v_n.clone(), &Py::Int(2i64))?)? });
    return Ok(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("p_value"), v_p.clone()), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_t_critical(mut v_df: Py, mut v_alpha: Py, mut v_tails: Py) -> R {
    v_df = f__pos(v_df.clone(), Py::str("the degrees of freedom"))?;
    return Ok(Py::dict(vec![(Py::str("critical_value"), f__crit({ let v_alpha = v_alpha.clone(); let v_df = v_df.clone(); let v_tails = v_tails.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_ppf(v_p.clone(), v_df.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_alpha.clone(), v_tails.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_t_one(mut v_x: Py, mut v_mu0: Py, mut v_alternative: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the sample"))?;
    v_s = f__sd(v_x.clone(), Py::Int(1i64))?;
    if ({ let t5769 = v_s.clone(); Py::Bool({ let t5770 = Py::Int(0i64); if py::eq(&t5769, &t5770) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all values are equal: the t statistic is undefined"))));
    }
    v_n = py::len(&v_x.clone())?;
    v_t = py::truediv(&py::sub(&f__mean(v_x.clone())?, &v_mu0.clone())?, &py::truediv(&v_s.clone(), &py::math1("sqrt", &v_n.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("df"), py::sub(&v_n.clone(), &Py::Int(1i64))?), (Py::str("p_value"), f__alt_p(v_t.clone(), v_alternative.clone(), { let v_alternative = v_alternative.clone(); let v_mu0 = v_mu0.clone(); let v_n = v_n.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_sf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_alternative = v_alternative.clone(); let v_mu0 = v_mu0.clone(); let v_n = v_n.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_cdf(v_v.clone(), py::sub(&v_n.clone(), &Py::Int(1i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?), (Py::str("mean"), f__mean(v_x.clone())?), (Py::str("standard_deviation"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_t_summary(mut v_m1: Py, mut v_s1: Py, mut v_n1: Py, mut v_m2: Py, mut v_s2: Py, mut v_n2: Py, mut v_variance: Py, mut v_alternative: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t5844 = (Py::tuple(vec![f__pos(v_s1.clone(), Py::str("s1"))?, f__pos(v_s2.clone(), Py::str("s2"))?])).items()?;
    if t5844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t5844[0].clone();
    v_s2 = t5844[1].clone();
    let t5852 = (Py::tuple(vec![f__whole(v_n1.clone(), Py::str("n1"), Py::Int(2i64))?, f__whole(v_n2.clone(), Py::str("n2"), Py::Int(2i64))?])).items()?;
    if t5852.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t5852[0].clone();
    v_n2 = t5852[1].clone();
    let t5853 = (Py::tuple(vec![py::mul(&v_s1.clone(), &v_s1.clone())?, py::mul(&v_s2.clone(), &v_s2.clone())?])).items()?;
    if t5853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t5853[0].clone();
    v_v2 = t5853[1].clone();
    if ({ let t5854 = v_variance.clone(); Py::Bool({ let t5855 = Py::str("pooled"); if py::eq(&t5854, &t5855) { true } else { false } }) }).truthy() {
        v_df = py::sub(&py::add(&v_n1.clone(), &v_n2.clone())?, &Py::Int(2i64))?;
        v_se = py::math1("sqrt", &py::mul(&py::truediv(&py::add(&py::mul(&py::sub(&v_n1.clone(), &Py::Int(1i64))?, &v_v1.clone())?, &py::mul(&py::sub(&v_n2.clone(), &Py::Int(1i64))?, &v_v2.clone())?)?, &v_df.clone())?, &py::add(&py::truediv(&Py::Int(1i64), &v_n1.clone())?, &py::truediv(&Py::Int(1i64), &v_n2.clone())?)?)?)?;
    } else {
        let t5856 = (Py::tuple(vec![py::truediv(&v_v1.clone(), &v_n1.clone())?, py::truediv(&v_v2.clone(), &v_n2.clone())?])).items()?;
        if t5856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t5856[0].clone();
        v_b = t5856[1].clone();
        v_se = py::math1("sqrt", &py::add(&v_a.clone(), &v_b.clone())?)?;
        v_df = py::truediv(&py::pow(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &py::add(&py::truediv(&py::mul(&v_a.clone(), &v_a.clone())?, &py::sub(&v_n1.clone(), &Py::Int(1i64))?)?, &py::truediv(&py::mul(&v_b.clone(), &v_b.clone())?, &py::sub(&v_n2.clone(), &Py::Int(1i64))?)?)?)?;
    }
    return Ok(f__t_out(py::sub(&v_m1.clone(), &v_m2.clone())?, v_se.clone(), v_df.clone(), v_alternative.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_trimmed(mut v_x: Py, mut v_proportion: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the sample"))?;
    if (py::not(&{ let t6013 = Py::Int(0i64); Py::Bool({ let t6014 = v_proportion.clone(); if py::le(&t6013, &t6014)? { { let t6015 = Py::Float(0.5); if py::lt(&t6014, &t6015)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the proportion cut from each end must be at least 0 and below 0.5"))));
    }
    v_k = py::int(&py::add(&py::mul(&v_proportion.clone(), &py::len(&v_x.clone())?)?, &Py::Float(1e-12))?)?;
    v_s = py::slice(&py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?, Some((v_k.clone()).i()?), Some((py::sub(&py::len(&v_x.clone())?, &v_k.clone())?).i()?), None)?;
    return Ok(Py::dict(vec![(Py::str("trimmed_mean"), f__mean(v_s.clone())?), (Py::str("removed_each_end"), v_k.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_two_prop_z(mut v_x1: Py, mut v_n1: Py, mut v_x2: Py, mut v_n2: Py, mut v_alternative: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t6062 = (f__prop_args(v_x1.clone(), v_n1.clone())?).items()?;
    if t6062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x1 = t6062[0].clone();
    v_n1 = t6062[1].clone();
    let t6063 = (f__prop_args(v_x2.clone(), v_n2.clone())?).items()?;
    if t6063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x2 = t6063[0].clone();
    v_n2 = t6063[1].clone();
    v_p = py::truediv(&py::add(&v_x1.clone(), &v_x2.clone())?, &py::add(&v_n1.clone(), &v_n2.clone())?)?;
    if ({ let t6064 = v_p.clone(); Py::Bool({ let t6065 = Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]); if py::contains(&t6065, &t6064)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pooled proportion is 0 or 1: the test is undefined"))));
    }
    v_z = py::truediv(&py::sub(&py::truediv(&v_x1.clone(), &v_n1.clone())?, &py::truediv(&v_x2.clone(), &v_n2.clone())?)?, &py::math1("sqrt", &py::mul(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &py::add(&py::truediv(&Py::Int(1i64), &v_n1.clone())?, &py::truediv(&Py::Int(1i64), &v_n2.clone())?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("p_value"), f__alt_p(v_z.clone(), v_alternative.clone(), py::func(|a: &[Py]| -> R { f__phic(a.get(0).cloned().unwrap_or(Py::None)) }), py::func(|a: &[Py]| -> R { f__phi(a.get(0).cloned().unwrap_or(Py::None)) }))?), (Py::str("pooled_proportion"), v_p.clone()), (Py::str("difference"), py::sub(&py::truediv(&v_x1.clone(), &v_n1.clone())?, &py::truediv(&v_x2.clone(), &v_n2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_two_way_table(mut v_table: Py, mut v_row: Py, mut v_column: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cgr: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_joint: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nc: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_r_: Py = Py::None;
    let mut v_rgc: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_t = f__matrix_ok(v_table.clone(), Py::str("the two-way table"), Py::Int(1i64), Py::Int(1i64))?;
    let t6117 = (Py::tuple(vec![py::len(&v_t.clone())?, py::len(&py::getitem(&v_t.clone(), &Py::Int(0i64))?)?])).items()?;
    if t6117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nr = t6117[0].clone();
    v_nc = t6117[1].clone();
    if ({ let t6118 = py::mul(&v_nr.clone(), &v_nc.clone())?; Py::Bool({ let t6119 = Py::Int(2i64); if py::lt(&t6118, &t6119)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a two-way table needs at least two cells"))));
    }
    for t6120 in py::iter(&(v_t.clone()))? {
        v_r_ = t6120;
        for t6121 in py::iter(&(v_r_.clone()))? {
            v_v = t6121;
            if ({ let t6122 = v_v.clone(); Py::Bool({ let t6123 = Py::Int(0i64); if py::lt(&t6122, &t6123)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("counts must be >= 0"))));
            }
        }
    }
    let t6131 = (Py::tuple(vec![py::sub(&f__whole(v_row.clone(), Py::str("row"), Py::Int(1i64))?, &Py::Int(1i64))?, py::sub(&f__whole(v_column.clone(), Py::str("column"), Py::Int(1i64))?, &Py::Int(1i64))?])).items()?;
    if t6131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i = t6131[0].clone();
    v_j = t6131[1].clone();
    if ({ let t6136 = { let t6132 = v_i.clone(); Py::Bool({ let t6133 = v_nr.clone(); if py::ge(&t6132, &t6133)? { true } else { false } }) }; if t6136.truthy() { t6136 } else { { let t6134 = v_j.clone(); Py::Bool({ let t6135 = v_nc.clone(); if py::ge(&t6134, &t6135)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the row and column must be inside the table (numbered from 1)"))));
    }
    let t6137 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t6137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rs = t6137[0].clone();
    v_cs = t6137[1].clone();
    for t6138 in py::range_iter(&[(v_nr.clone()).i()?]) {
        v_a = t6138;
        v_s = Py::Float(0.0);
        for t6139 in py::range_iter(&[(v_nc.clone()).i()?]) {
            v_b = t6139;
            v_s = py::iadd(&v_s, &py::getitem(&py::getitem(&v_t.clone(), &v_a.clone())?, &v_b.clone())?)?;
        }
        let _ = py::method(&v_rs.clone(), "append", &[v_s.clone()])?;
    }
    for t6140 in py::range_iter(&[(v_nc.clone()).i()?]) {
        v_b = t6140;
        v_s = Py::Float(0.0);
        for t6141 in py::range_iter(&[(v_nr.clone()).i()?]) {
            v_a = t6141;
            v_s = py::iadd(&v_s, &py::getitem(&py::getitem(&v_t.clone(), &v_a.clone())?, &v_b.clone())?)?;
        }
        let _ = py::method(&v_cs.clone(), "append", &[v_s.clone()])?;
    }
    v_n = Py::Float(0.0);
    for t6142 in py::iter(&(v_rs.clone()))? {
        v_s = t6142;
        v_n = py::iadd(&v_n, &v_s.clone())?;
    }
    for t6143 in py::iter(&(py::add(&v_rs.clone(), &v_cs.clone())?))? {
        v_s = t6143;
        if ({ let t6144 = v_s.clone(); Py::Bool({ let t6145 = Py::Int(0i64); if py::le(&t6144, &t6145)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every row and column needs a positive total"))));
        }
    }
    let t6146 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t6146.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_joint = t6146[0].clone();
    v_rgc = t6146[1].clone();
    v_cgr = t6146[2].clone();
    for t6147 in py::range_iter(&[(v_nr.clone()).i()?]) {
        v_a = t6147;
        let _ = py::method(&v_joint.clone(), "append", &[{ let mut out = vec![]; for t6148 in py::range_iter(&[(v_nc.clone()).i()?]) { let mut v_b: Py; 
v_b = t6148;
out.push(py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_a.clone())?, &v_b.clone())?, &v_n.clone())?); } Py::list(out) }])?;
        let _ = py::method(&v_rgc.clone(), "append", &[{ let mut out = vec![]; for t6149 in py::range_iter(&[(v_nc.clone()).i()?]) { let mut v_b: Py; 
v_b = t6149;
out.push(py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_cs.clone(), &v_b.clone())?)?); } Py::list(out) }])?;
        let _ = py::method(&v_cgr.clone(), "append", &[{ let mut out = vec![]; for t6150 in py::range_iter(&[(v_nc.clone()).i()?]) { let mut v_b: Py; 
v_b = t6150;
out.push(py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_rs.clone(), &v_a.clone())?)?); } Py::list(out) }])?;
    }
    return Ok(Py::dict(vec![(Py::str("total"), v_n.clone()), (Py::str("joint"), v_joint.clone()), (Py::str("row_marginals"), { let mut out = vec![]; for t6151 in py::iter(&(v_rs.clone()))? { let mut v_s: Py; 
v_s = t6151;
out.push(py::truediv(&v_s.clone(), &v_n.clone())?); } Py::list(out) }), (Py::str("column_marginals"), { let mut out = vec![]; for t6152 in py::iter(&(v_cs.clone()))? { let mut v_s: Py; 
v_s = t6152;
out.push(py::truediv(&v_s.clone(), &v_n.clone())?); } Py::list(out) }), (Py::str("row_given_column"), v_rgc.clone()), (Py::str("column_given_row"), v_cgr.clone()), (Py::str("p_row_and_column"), py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone())?, &v_n.clone())?), (Py::str("p_row"), py::truediv(&py::getitem(&v_rs.clone(), &v_i.clone())?, &v_n.clone())?), (Py::str("p_column"), py::truediv(&py::getitem(&v_cs.clone(), &v_j.clone())?, &v_n.clone())?), (Py::str("p_row_given_column"), py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_cs.clone(), &v_j.clone())?)?), (Py::str("p_column_given_row"), py::truediv(&py::getitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_rs.clone(), &v_i.clone())?)?), (Py::str("p_row_or_column"), py::truediv(&py::sub(&py::add(&py::getitem(&v_rs.clone(), &v_i.clone())?, &py::getitem(&v_cs.clone(), &v_j.clone())?)?, &py::getitem(&py::getitem(&v_t.clone(), &v_i.clone())?, &v_j.clone())?)?, &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_weibull(mut v_beta: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t6191 = (Py::tuple(vec![f__pos(v_beta.clone(), Py::str("the shape beta"))?, f__pos(v_eta.clone(), Py::str("the scale eta"))?])).items()?;
    if t6191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t6191[0].clone();
    v_e = t6191[1].clone();
    if ({ let t6192 = v_t.clone(); Py::Bool({ let t6193 = Py::Int(0i64); if py::lt(&t6192, &t6193)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("t must be >= 0"))));
    }
    v_u = py::pow(&py::truediv(&v_t.clone(), &v_e.clone())?, &v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("reliability"), py::math1("exp", &py::neg(&v_u.clone())?)?), (Py::str("unreliability"), py::neg(&py::math1("expm1", &py::neg(&v_u.clone())?)?)?), (Py::str("hazard_rate"), (if ({ let t6198 = { let t6194 = v_t.clone(); Py::Bool({ let t6195 = Py::Int(0i64); if py::gt(&t6194, &t6195)? { true } else { false } }) }; if t6198.truthy() { t6198 } else { { let t6196 = v_b.clone(); Py::Bool({ let t6197 = Py::Int(1i64); if py::ge(&t6196, &t6197)? { true } else { false } }) } } }).truthy() { py::mul(&py::truediv(&v_b.clone(), &v_e.clone())?, &py::pow(&py::truediv(&v_t.clone(), &v_e.clone())?, &py::sub(&v_b.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(f64::INFINITY) })), (Py::str("mttf"), py::mul(&v_e.clone(), &py::math1("gamma", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_weighted_mean(mut v_x: Py, mut v_w: Py) -> R {
    let t6236 = (Py::tuple(vec![f__finite(v_x.clone(), Py::str("values"))?, f__finite(v_w.clone(), Py::str("weights"))?])).items()?;
    if t6236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t6236[0].clone();
    v_w = t6236[1].clone();
    if ({ let t6237 = py::len(&v_x.clone())?; Py::Bool({ let t6238 = py::len(&v_w.clone())?; if py::ne(&t6237, &t6238) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values and weights must have the same length"))));
    }
    if ({ let t6244 = Py::Bool(({ let mut out = vec![]; for t6239 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t6239;
out.push({ let t6240 = v_v.clone(); Py::Bool({ let t6241 = Py::Int(0i64); if py::lt(&t6240, &t6241)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t6244.truthy() { t6244 } else { { let t6242 = py::fsum(&v_w.clone())?; Py::Bool({ let t6243 = Py::Int(0i64); if py::le(&t6242, &t6243)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("weights must be non-negative with a positive sum"))));
    }
    return Ok(Py::dict(vec![(Py::str("weighted_mean"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t6245 in py::iter(&(py::zip(&[v_x.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6246 = (t6245).items()?;
if t6246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6246[0].clone();
v_b = t6246[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &py::fsum(&v_w.clone())?)?), (Py::str("total_weight"), py::fsum(&v_w.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_wilcoxon(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let mut v_se2: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t6392 = (f__signed(v_x.clone(), v_y.clone())?).items()?;
    if t6392.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6392[0].clone();
    v_r = t6392[1].clone();
    v_rp = t6392[2].clone();
    v_rm = t6392[3].clone();
    v_n = py::len(&v_d.clone())?;
    v_se2 = py::sub(&py::truediv(&py::mul(&py::mul(&v_n.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?)?, &Py::Float(24.0))?, &py::truediv(&py::sum(&{ let mut out = vec![]; for t6398 in py::iter(&(f__tie_counts({ let mut out = vec![]; for t6397 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t6397;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?))? { let mut v_t: Py; 
v_t = t6398;
out.push(py::sub(&py::pow(&v_t.clone(), &Py::Int(3i64))?, &v_t.clone())?); } Py::list(out) }, None)?, &Py::Float(48.0))?)?;
    if ({ let t6399 = v_se2.clone(); Py::Bool({ let t6400 = Py::Int(0i64); if py::le(&t6399, &t6400)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the variance of the statistic is zero"))));
    }
    v_z = py::truediv(&py::sub(&v_rp.clone(), &py::truediv(&py::mul(&v_n.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &Py::Float(4.0))?)?, &py::math1("sqrt", &v_se2.clone())?)?;
    v_p = py::method(&Py::dict(vec![(Py::str("greater"), f__phic(v_z.clone())?), (Py::str("less"), f__phi(v_z.clone())?)]), "get", &[v_alternative.clone(), py::minmax(&[Py::Float(1.0), py::mul(&Py::Int(2i64), &f__phic(py::abs(&v_z.clone())?)?)?], false)?])?;
    return Ok(Py::dict(vec![(Py::str("statistic"), (if ({ let t6401 = v_alternative.clone(); Py::Bool({ let t6402 = Py::str("two-sided"); if py::eq(&t6401, &t6402) { true } else { false } }) }).truthy() { py::minmax(&[v_rp.clone(), v_rm.clone()], false)? } else { v_rp.clone() })), (Py::str("p_value"), v_p.clone()), (Py::str("r_plus"), v_rp.clone()), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_wilcoxon_exact(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t6464 = (f__signed(v_x.clone(), v_y.clone())?).items()?;
    if t6464.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6464[0].clone();
    v_r = t6464[1].clone();
    v_rp = t6464[2].clone();
    v_rm = t6464[3].clone();
    if ({ let t6472 = f__tie_counts({ let mut out = vec![]; for t6469 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t6469;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?; if t6472.truthy() { t6472 } else { { let t6470 = py::len(&v_d.clone())?; Py::Bool({ let t6471 = py::len(&v_x.clone())?; if py::lt(&t6470, &t6471)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the exact test needs no zero differences and no tied absolute differences"))));
    }
    v_n = py::len(&v_d.clone())?;
    if ({ let t6473 = v_n.clone(); Py::Bool({ let t6474 = Py::Int(60i64); if py::gt(&t6473, &t6474)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the exact test is limited to 60 differences"))));
    }
    v_c = Py::list(vec![Py::Int(1i64)]);
    for t6475 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t6475;
        v_c = { let mut out = vec![]; for t6476 in py::range_iter(&[(py::add(&py::len(&v_c.clone())?, &v_i.clone())?).i()?]) { let mut v_k: Py; 
v_k = t6476;
out.push(py::add(&(if ({ let t6477 = v_k.clone(); Py::Bool({ let t6478 = py::len(&v_c.clone())?; if py::lt(&t6477, &t6478)? { true } else { false } }) }).truthy() { py::getitem(&v_c.clone(), &v_k.clone())? } else { Py::Int(0i64) }), &(if ({ let t6479 = Py::Int(0i64); Py::Bool({ let t6480 = py::sub(&v_k.clone(), &v_i.clone())?; if py::le(&t6479, &t6480)? { { let t6481 = py::len(&v_c.clone())?; if py::lt(&t6480, &t6481)? { true } else { false } } } else { false } }) }).truthy() { py::getitem(&v_c.clone(), &py::sub(&v_k.clone(), &v_i.clone())?)? } else { Py::Int(0i64) }))?); } Py::list(out) };
    }
    let t6482 = (Py::tuple(vec![py::pow(&Py::Int(2i64), &v_n.clone())?, py::int(&py::round(&v_rp.clone(), None)?)?])).items()?;
    if t6482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tot = t6482[0].clone();
    v_k = t6482[1].clone();
    let t6483 = (Py::tuple(vec![py::fraction(&[py::sum(&py::slice(&v_c.clone(), None, Some((py::add(&v_k.clone(), &Py::Int(1i64))?).i()?), None)?, None)?, v_tot.clone()])?, py::fraction(&[py::sum(&py::slice(&v_c.clone(), Some((v_k.clone()).i()?), None, None)?, None)?, v_tot.clone()])?])).items()?;
    if t6483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t6483[0].clone();
    v_hi = t6483[1].clone();
    v_p = py::method(&Py::dict(vec![(Py::str("greater"), v_hi.clone()), (Py::str("less"), v_lo.clone())]), "get", &[v_alternative.clone(), py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::mul(&Py::Int(2i64), &py::minmax(&[v_lo.clone(), v_hi.clone()], false)?)?], false)?])?;
    return Ok(Py::dict(vec![(Py::str("statistic"), (if ({ let t6484 = v_alternative.clone(); Py::Bool({ let t6485 = Py::str("two-sided"); if py::eq(&t6484, &t6485) { true } else { false } }) }).truthy() { py::minmax(&[v_rp.clone(), v_rm.clone()], false)? } else { v_rp.clone() })), (Py::str("p_value"), py::float(&v_p.clone())?), (Py::str("r_plus"), v_rp.clone()), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_wilson(mut v_x: Py, mut v_n: Py, mut v_confidence: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t6530 = (f__prop_args(v_x.clone(), v_n.clone())?).items()?;
    if t6530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t6530[0].clone();
    v_n = t6530[1].clone();
    v_z = f__zc(v_confidence.clone())?;
    v_p = py::truediv(&v_x.clone(), &v_n.clone())?;
    v_den = py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &v_n.clone())?)?;
    v_c = py::truediv(&py::add(&v_p.clone(), &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &py::mul(&Py::Int(2i64), &v_n.clone())?)?)?, &v_den.clone())?;
    v_h = py::truediv(&py::mul(&v_z.clone(), &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?, &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_n.clone())?, &v_n.clone())?)?)?)?)?, &v_den.clone())?;
    return Ok(Py::dict(vec![(Py::str("proportion"), v_p.clone()), (Py::str("lower"), py::minmax(&[Py::Float(0.0), py::sub(&v_c.clone(), &v_h.clone())?], true)?), (Py::str("upper"), py::minmax(&[Py::Float(1.0), py::add(&v_c.clone(), &v_h.clone())?], false)?), (Py::str("center"), v_c.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_xbar_r(mut v_data: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_xbb: Py = Py::None;
    let t6592 = (f__subgroups(v_data.clone())?).items()?;
    if t6592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6592[0].clone();
    v_n = t6592[1].clone();
    v_xbb = f__mean({ let mut out = vec![]; for t6593 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t6593;
out.push(f__mean(v_r.clone())?); } Py::list(out) })?;
    v_rb = f__mean({ let mut out = vec![]; for t6594 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t6594;
out.push(py::sub(&py::minmax(&[v_r.clone()], true)?, &py::minmax(&[v_r.clone()], false)?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("center_xbar"), v_xbb.clone()), (Py::str("mean_range"), v_rb.clone()), (Py::str("xbar_ucl"), py::add(&v_xbb.clone(), &py::mul(&py::getitem(&c__A2()?, &v_n.clone())?, &v_rb.clone())?)?), (Py::str("xbar_lcl"), py::sub(&v_xbb.clone(), &py::mul(&py::getitem(&c__A2()?, &v_n.clone())?, &v_rb.clone())?)?), (Py::str("r_ucl"), py::mul(&py::getitem(&c__D4()?, &v_n.clone())?, &v_rb.clone())?), (Py::str("r_lcl"), py::mul(&py::getitem(&c__D3()?, &v_n.clone())?, &v_rb.clone())?), (Py::str("A2"), py::getitem(&c__A2()?, &v_n.clone())?), (Py::str("D3"), py::getitem(&c__D3()?, &v_n.clone())?), (Py::str("D4"), py::getitem(&c__D4()?, &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_xbar_s(mut v_data: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_xbb: Py = Py::None;
    let t6644 = (f__subgroups(v_data.clone())?).items()?;
    if t6644.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6644[0].clone();
    v_n = t6644[1].clone();
    v_xbb = f__mean({ let mut out = vec![]; for t6645 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t6645;
out.push(f__mean(v_r.clone())?); } Py::list(out) })?;
    v_sb = f__mean({ let mut out = vec![]; for t6646 in py::iter(&(v_d.clone()))? { let mut v_r: Py; 
v_r = t6646;
out.push(f__sd(v_r.clone(), Py::Int(1i64))?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("center_xbar"), v_xbb.clone()), (Py::str("mean_sd"), v_sb.clone()), (Py::str("xbar_ucl"), py::add(&v_xbb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("xbar_lcl"), py::sub(&v_xbb.clone(), &py::mul(&py::getitem(&c__A3()?, &v_n.clone())?, &v_sb.clone())?)?), (Py::str("s_ucl"), py::mul(&py::getitem(&c__B4()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("s_lcl"), py::mul(&py::getitem(&c__B3()?, &v_n.clone())?, &v_sb.clone())?), (Py::str("A3"), py::getitem(&c__A3()?, &v_n.clone())?), (Py::str("B3"), py::getitem(&c__B3()?, &v_n.clone())?), (Py::str("B4"), py::getitem(&c__B4()?, &v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_z_critical(mut v_alpha: Py, mut v_tails: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("critical_value"), f__crit(py::func(|a: &[Py]| -> R { f__z(a.get(0).cloned().unwrap_or(Py::None)) }), v_alpha.clone(), v_tails.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_z_test(mut v_xbar: Py, mut v_mu0: Py, mut v_sigma: Py, mut v_n: Py, mut v_alternative: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t6787 = (Py::tuple(vec![f__pos(v_sigma.clone(), Py::str("sigma"))?, f__whole(v_n.clone(), Py::str("n"), Py::Int(1i64))?])).items()?;
    if t6787.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sigma = t6787[0].clone();
    v_n = t6787[1].clone();
    v_z = py::truediv(&py::sub(&v_xbar.clone(), &v_mu0.clone())?, &py::truediv(&v_sigma.clone(), &py::math1("sqrt", &v_n.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("p_value"), f__alt_p(v_z.clone(), v_alternative.clone(), py::func(|a: &[Py]| -> R { f__phic(a.get(0).cloned().unwrap_or(Py::None)) }), py::func(|a: &[Py]| -> R { f__phi(a.get(0).cloned().unwrap_or(Py::None)) }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_zscore(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py) -> R {
    v_sigma = f__pos(v_sigma.clone(), Py::str("the standard deviation"))?;
    return Ok(Py::dict(vec![(Py::str("z"), py::truediv(&py::sub(&v_x.clone(), &v_mu.clone())?, &v_sigma.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1018(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1049 = v_i.clone(); f_run_ci_two_means(py::getitem(&t1049, &Py::str("x"))?, py::getitem(&t1049, &Py::str("y"))?, py::getitem(&t1049, &Py::str("confidence"))?, py::getitem(&t1049, &Py::str("variance"))?)? }, { let mut out = vec![]; for t1050 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1051 = (t1050).items()?;
if t1051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1051[0].clone();
v_v = t1051[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_103(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t134 = v_i.clone(); f_run_binom_at_least(py::getitem(&t134, &Py::str("n"))?, py::getitem(&t134, &Py::str("k"))?, py::getitem(&t134, &Py::str("p"))?)? }, { let mut out = vec![]; for t135 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t136 = (t135).items()?;
if t136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t136[0].clone();
v_v = t136[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1111(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1142 = v_i.clone(); f_run_ci_variance(py::getitem(&t1142, &Py::str("x"))?, py::getitem(&t1142, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t1143 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1144 = (t1143).items()?;
if t1144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1144[0].clone();
v_v = t1144[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1153(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1184 = v_i.clone(); f_run_cv(py::getitem(&t1184, &Py::str("x"))?)? }, { let mut out = vec![]; for t1185 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1186 = (t1185).items()?;
if t1186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1186[0].clone();
v_v = t1186[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1197(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1228 = v_i.clone(); f_run_cohens_d(py::getitem(&t1228, &Py::str("x"))?, py::getitem(&t1228, &Py::str("y"))?)? }, { let mut out = vec![]; for t1229 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1230 = (t1229).items()?;
if t1230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1230[0].clone();
v_v = t1230[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1265(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1296 = v_i.clone(); f_run_corr_ci(py::getitem(&t1296, &Py::str("x"))?, py::getitem(&t1296, &Py::str("y"))?, py::getitem(&t1296, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t1297 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1298 = (t1297).items()?;
if t1298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1298[0].clone();
v_v = t1298[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1344(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1375 = v_i.clone(); f_run_corr_test(py::getitem(&t1375, &Py::str("r"))?, py::getitem(&t1375, &Py::str("n"))?, py::method(&t1375, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t1376 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1377 = (t1376).items()?;
if t1377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1377[0].clone();
v_v = t1377[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1390(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1421 = v_i.clone(); f_run_cov(py::getitem(&t1421, &Py::str("x"))?, py::getitem(&t1421, &Py::str("y"))?)? }, { let mut out = vec![]; for t1422 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1423 = (t1422).items()?;
if t1423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1423[0].clone();
v_v = t1423[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1442(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1473 = v_i.clone(); f_run_cronbach(py::getitem(&t1473, &Py::str("data"))?)? }, { let mut out = vec![]; for t1474 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1475 = (t1474).items()?;
if t1475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1475[0].clone();
v_v = t1475[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1492(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1523 = v_i.clone(); f_run_discrete_rv(py::getitem(&t1523, &Py::str("x"))?, py::getitem(&t1523, &Py::str("p"))?)? }, { let mut out = vec![]; for t1524 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1525 = (t1524).items()?;
if t1525.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1525[0].clone();
v_v = t1525[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1535(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1566 = v_i.clone(); f_run_durbin_watson(py::getitem(&t1566, &Py::str("residuals"))?)? }, { let mut out = vec![]; for t1567 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1568 = (t1567).items()?;
if t1568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1568[0].clone();
v_v = t1568[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1583(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1614 = v_i.clone(); f_run_exp_above(py::getitem(&t1614, &Py::str("x"))?, py::method(&t1614, "get", &[Py::str("rate"), Py::None])?, py::method(&t1614, "get", &[Py::str("mean"), Py::None])?)? }, { let mut out = vec![]; for t1615 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1616 = (t1615).items()?;
if t1616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1616[0].clone();
v_v = t1616[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1631(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1662 = v_i.clone(); f_run_exp_below(py::getitem(&t1662, &Py::str("x"))?, py::method(&t1662, "get", &[Py::str("rate"), Py::None])?, py::method(&t1662, "get", &[Py::str("mean"), Py::None])?)? }, { let mut out = vec![]; for t1663 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1664 = (t1663).items()?;
if t1664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1664[0].clone();
v_v = t1664[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_166(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t197 = v_i.clone(); f_run_binom_at_most(py::getitem(&t197, &Py::str("n"))?, py::getitem(&t197, &Py::str("k"))?, py::getitem(&t197, &Py::str("p"))?)? }, { let mut out = vec![]; for t198 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t199 = (t198).items()?;
if t199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t199[0].clone();
v_v = t199[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1679(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1710 = v_i.clone(); f_run_exp_rel(py::getitem(&t1710, &Py::str("t"))?, py::method(&t1710, "get", &[Py::str("rate"), Py::None])?, py::method(&t1710, "get", &[Py::str("mtbf"), Py::None])?)? }, { let mut out = vec![]; for t1711 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1712 = (t1711).items()?;
if t1712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1712[0].clone();
v_v = t1712[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1780(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1811 = v_i.clone(); f_run_f_critical(py::getitem(&t1811, &Py::str("df1"))?, py::getitem(&t1811, &Py::str("df2"))?, py::getitem(&t1811, &Py::str("alpha"))?)? }, { let mut out = vec![]; for t1812 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1813 = (t1812).items()?;
if t1813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1813[0].clone();
v_v = t1813[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1857(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1888 = v_i.clone(); f_run_f_test(py::getitem(&t1888, &Py::str("x"))?, py::getitem(&t1888, &Py::str("y"))?, py::method(&t1888, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t1889 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1890 = (t1889).items()?;
if t1890.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1890[0].clone();
v_v = t1890[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1891(mut v_y: Py) -> R {
    return Ok(f__factorial(v_y.clone(), Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1911(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1942 = v_i.clone(); s_1891(py::getitem(&t1942, &Py::str("y"))?)? }, { let mut out = vec![]; for t1943 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1944 = (t1943).items()?;
if t1944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1944[0].clone();
v_v = t1944[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1945(mut v_y: Py) -> R {
    return Ok(f__factorial(v_y.clone(), Py::Int(3i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1965(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1996 = v_i.clone(); s_1945(py::getitem(&t1996, &Py::str("y"))?)? }, { let mut out = vec![]; for t1997 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1998 = (t1997).items()?;
if t1998.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1998[0].clone();
v_v = t1998[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2051(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2082 = v_i.clone(); f_run_fisher(py::getitem(&t2082, &Py::str("table"))?, py::method(&t2082, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t2083 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2084 = (t2083).items()?;
if t2084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2084[0].clone();
v_v = t2084[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2098(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2129 = v_i.clone(); f_run_five_number(py::getitem(&t2129, &Py::str("x"))?, py::method(&t2129, "get", &[Py::str("method"), Py::str("linear")])?)? }, { let mut out = vec![]; for t2130 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2131 = (t2130).items()?;
if t2131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2131[0].clone();
v_v = t2131[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2150(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2181 = v_i.clone(); f_run_frequency(py::getitem(&t2181, &Py::str("x"))?, py::getitem(&t2181, &Py::str("f"))?)? }, { let mut out = vec![]; for t2182 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2183 = (t2182).items()?;
if t2183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2183[0].clone();
v_v = t2183[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2232(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2263 = v_i.clone(); f_run_friedman(py::getitem(&t2263, &Py::str("data"))?)? }, { let mut out = vec![]; for t2264 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2265 = (t2264).items()?;
if t2265.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2265[0].clone();
v_v = t2265[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_226(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t257 = v_i.clone(); f_run_binom_exact(py::getitem(&t257, &Py::str("n"))?, py::getitem(&t257, &Py::str("k"))?, py::getitem(&t257, &Py::str("p"))?)? }, { let mut out = vec![]; for t258 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t259 = (t258).items()?;
if t259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t259[0].clone();
v_v = t259[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2275(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2306 = v_i.clone(); f_run_gmean(py::getitem(&t2306, &Py::str("x"))?)? }, { let mut out = vec![]; for t2307 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2308 = (t2307).items()?;
if t2308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2308[0].clone();
v_v = t2308[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2325(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2356 = v_i.clone(); f_run_geometric(py::getitem(&t2356, &Py::str("p"))?, py::getitem(&t2356, &Py::str("k"))?)? }, { let mut out = vec![]; for t2357 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2358 = (t2357).items()?;
if t2358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2358[0].clone();
v_v = t2358[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2368(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2399 = v_i.clone(); f_run_hmean(py::getitem(&t2399, &Py::str("x"))?)? }, { let mut out = vec![]; for t2400 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2401 = (t2400).items()?;
if t2401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2401[0].clone();
v_v = t2401[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2425(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2456 = v_i.clone(); f_run_hypergeom(py::getitem(&t2456, &Py::str("N"))?, py::getitem(&t2456, &Py::str("K"))?, py::getitem(&t2456, &Py::str("n"))?, py::getitem(&t2456, &Py::str("k"))?)? }, { let mut out = vec![]; for t2457 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2458 = (t2457).items()?;
if t2458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2458[0].clone();
v_v = t2458[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2465(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2496 = v_i.clone(); f_run_imr(py::getitem(&t2496, &Py::str("x"))?)? }, { let mut out = vec![]; for t2497 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2498 = (t2497).items()?;
if t2498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2498[0].clone();
v_v = t2498[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2519(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2550 = v_i.clone(); f_run_inverse_normal(py::getitem(&t2550, &Py::str("p"))?, py::getitem(&t2550, &Py::str("mu"))?, py::getitem(&t2550, &Py::str("sigma"))?)? }, { let mut out = vec![]; for t2551 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2552 = (t2551).items()?;
if t2552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2552[0].clone();
v_v = t2552[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2572(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2603 = v_i.clone(); f_run_outliers(py::getitem(&t2603, &Py::str("x"))?)? }, { let mut out = vec![]; for t2604 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2605 = (t2604).items()?;
if t2605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2605[0].clone();
v_v = t2605[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2625(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2656 = v_i.clone(); f_run_k_of_n(py::getitem(&t2656, &Py::str("n"))?, py::getitem(&t2656, &Py::str("k"))?, py::getitem(&t2656, &Py::str("r"))?)? }, { let mut out = vec![]; for t2657 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2658 = (t2657).items()?;
if t2658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2658[0].clone();
v_v = t2658[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2692(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2723 = v_i.clone(); f_run_km(py::getitem(&t2723, &Py::str("times"))?, py::getitem(&t2723, &Py::str("events"))?, py::getitem(&t2723, &Py::str("t"))?)? }, { let mut out = vec![]; for t2724 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2725 = (t2724).items()?;
if t2725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2725[0].clone();
v_v = t2725[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_276(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t307 = v_i.clone(); f_run_binom_moments(py::getitem(&t307, &Py::str("n"))?, py::getitem(&t307, &Py::str("p"))?)? }, { let mut out = vec![]; for t308 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t309 = (t308).items()?;
if t309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t309[0].clone();
v_v = t309[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2761(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2792 = v_i.clone(); f_run_kendall(py::getitem(&t2792, &Py::str("x"))?, py::getitem(&t2792, &Py::str("y"))?)? }, { let mut out = vec![]; for t2793 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2794 = (t2793).items()?;
if t2794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2794[0].clone();
v_v = t2794[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2842(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2873 = v_i.clone(); f_run_kruskal(py::getitem(&t2873, &Py::str("g1"))?, py::getitem(&t2873, &Py::str("g2"))?, py::method(&t2873, "get", &[Py::str("g3"), Py::None])?, py::method(&t2873, "get", &[Py::str("g4"), Py::None])?, py::method(&t2873, "get", &[Py::str("g5"), Py::None])?)? }, { let mut out = vec![]; for t2874 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2875 = (t2874).items()?;
if t2875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2875[0].clone();
v_v = t2875[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2926(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2957 = v_i.clone(); f_run_levene(py::getitem(&t2957, &Py::str("g1"))?, py::getitem(&t2957, &Py::str("g2"))?, py::method(&t2957, "get", &[Py::str("g3"), Py::None])?, py::method(&t2957, "get", &[Py::str("g4"), Py::None])?, py::method(&t2957, "get", &[Py::str("g5"), Py::None])?, py::method(&t2957, "get", &[Py::str("center"), Py::str("median")])?)? }, { let mut out = vec![]; for t2958 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2959 = (t2958).items()?;
if t2959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2959[0].clone();
v_v = t2959[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3021(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3052 = v_i.clone(); f_run_logrank(py::getitem(&t3052, &Py::str("times1"))?, py::getitem(&t3052, &Py::str("events1"))?, py::getitem(&t3052, &Py::str("times2"))?, py::getitem(&t3052, &Py::str("events2"))?)? }, { let mut out = vec![]; for t3053 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3054 = (t3053).items()?;
if t3054.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3054[0].clone();
v_v = t3054[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3090(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3121 = v_i.clone(); f_run_mann_whitney_exact(py::getitem(&t3121, &Py::str("x"))?, py::getitem(&t3121, &Py::str("y"))?, py::method(&t3121, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t3122 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3123 = (t3122).items()?;
if t3123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3123[0].clone();
v_v = t3123[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3150(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3181 = v_i.clone(); f_run_mann_whitney(py::getitem(&t3181, &Py::str("x"))?, py::getitem(&t3181, &Py::str("y"))?, py::method(&t3181, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t3182 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3183 = (t3182).items()?;
if t3183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3183[0].clone();
v_v = t3183[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_318(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t349 = v_i.clone(); f_run_c_chart(py::getitem(&t349, &Py::str("counts"))?)? }, { let mut out = vec![]; for t350 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t351 = (t350).items()?;
if t351.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t351[0].clone();
v_v = t351[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3233(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3264 = v_i.clone(); f_run_mcnemar(py::getitem(&t3264, &Py::str("table"))?)? }, { let mut out = vec![]; for t3265 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3266 = (t3265).items()?;
if t3266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3266[0].clone();
v_v = t3266[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3272(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3303 = v_i.clone(); f_run_mean(py::getitem(&t3303, &Py::str("x"))?)? }, { let mut out = vec![]; for t3304 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3305 = (t3304).items()?;
if t3305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3305[0].clone();
v_v = t3305[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3312(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3343 = v_i.clone(); f_run_mad(py::getitem(&t3343, &Py::str("x"))?)? }, { let mut out = vec![]; for t3344 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3345 = (t3344).items()?;
if t3345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3345[0].clone();
v_v = t3345[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3351(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3382 = v_i.clone(); f_run_median(py::getitem(&t3382, &Py::str("x"))?)? }, { let mut out = vec![]; for t3383 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3384 = (t3383).items()?;
if t3384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3384[0].clone();
v_v = t3384[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3391(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3422 = v_i.clone(); f_run_mad_median(py::getitem(&t3422, &Py::str("x"))?)? }, { let mut out = vec![]; for t3423 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3424 = (t3423).items()?;
if t3424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3424[0].clone();
v_v = t3424[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3440(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3471 = v_i.clone(); f_run_mode(py::getitem(&t3471, &Py::str("x"))?)? }, { let mut out = vec![]; for t3472 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3473 = (t3472).items()?;
if t3473.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3473[0].clone();
v_v = t3473[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3554(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3585 = v_i.clone(); f_run_multiple_regression(py::getitem(&t3585, &Py::str("X"))?, py::getitem(&t3585, &Py::str("y"))?)? }, { let mut out = vec![]; for t3586 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3587 = (t3586).items()?;
if t3587.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3587[0].clone();
v_v = t3587[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3606(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3637 = v_i.clone(); f_run_negbinom(py::getitem(&t3637, &Py::str("r"))?, py::getitem(&t3637, &Py::str("p"))?, py::getitem(&t3637, &Py::str("n"))?)? }, { let mut out = vec![]; for t3638 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3639 = (t3638).items()?;
if t3639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3639[0].clone();
v_v = t3639[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3644(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3675 = v_i.clone(); f_run_normal_above(py::getitem(&t3675, &Py::str("x"))?, py::getitem(&t3675, &Py::str("mu"))?, py::getitem(&t3675, &Py::str("sigma"))?)? }, { let mut out = vec![]; for t3676 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3677 = (t3676).items()?;
if t3677.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3677[0].clone();
v_v = t3677[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3682(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3713 = v_i.clone(); f_run_normal_below(py::getitem(&t3713, &Py::str("x"))?, py::getitem(&t3713, &Py::str("mu"))?, py::getitem(&t3713, &Py::str("sigma"))?)? }, { let mut out = vec![]; for t3714 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3715 = (t3714).items()?;
if t3715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3715[0].clone();
v_v = t3715[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3725(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3756 = v_i.clone(); f_run_normal_between(py::getitem(&t3756, &Py::str("a"))?, py::getitem(&t3756, &Py::str("b"))?, py::getitem(&t3756, &Py::str("mu"))?, py::getitem(&t3756, &Py::str("sigma"))?)? }, { let mut out = vec![]; for t3757 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3758 = (t3757).items()?;
if t3758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3758[0].clone();
v_v = t3758[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3804(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3835 = v_i.clone(); f_run_anova(py::getitem(&t3835, &Py::str("g1"))?, py::getitem(&t3835, &Py::str("g2"))?, py::method(&t3835, "get", &[Py::str("g3"), Py::None])?, py::method(&t3835, "get", &[Py::str("g4"), Py::None])?, py::method(&t3835, "get", &[Py::str("g5"), Py::None])?)? }, { let mut out = vec![]; for t3836 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3837 = (t3836).items()?;
if t3837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3837[0].clone();
v_v = t3837[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3859(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3890 = v_i.clone(); f_run_p_chart(py::getitem(&t3890, &Py::str("defectives"))?, py::getitem(&t3890, &Py::str("n"))?)? }, { let mut out = vec![]; for t3891 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3892 = (t3891).items()?;
if t3892.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3892[0].clone();
v_v = t3892[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3942(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3973 = v_i.clone(); f_run_paired_t(py::getitem(&t3973, &Py::str("x"))?, py::getitem(&t3973, &Py::str("y"))?, py::method(&t3973, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t3974 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3975 = (t3974).items()?;
if t3975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3975[0].clone();
v_v = t3975[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3976(mut v_r: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("reliability"), s_3977(f__rels(v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3977(mut v_r: Py) -> R {
    return Ok(py::sub(&Py::Int(1i64), &py::prod(&{ let mut out = vec![]; for t3978 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3978;
out.push(py::sub(&Py::Int(1i64), &v_v.clone())?); } Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3988(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4019 = v_i.clone(); s_3976(py::getitem(&t4019, &Py::str("r"))?)? }, { let mut out = vec![]; for t4020 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4021 = (t4020).items()?;
if t4021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4021[0].clone();
v_v = t4021[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_40(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t71 = v_i.clone(); f_run_bartlett(py::getitem(&t71, &Py::str("g1"))?, py::getitem(&t71, &Py::str("g2"))?, py::method(&t71, "get", &[Py::str("g3"), Py::None])?, py::method(&t71, "get", &[Py::str("g4"), Py::None])?, py::method(&t71, "get", &[Py::str("g5"), Py::None])?)? }, { let mut out = vec![]; for t72 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t73 = (t72).items()?;
if t73.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t73[0].clone();
v_v = t73[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4067(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4098 = v_i.clone(); f_run_pearson(py::getitem(&t4098, &Py::str("x"))?, py::getitem(&t4098, &Py::str("y"))?)? }, { let mut out = vec![]; for t4099 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4100 = (t4099).items()?;
if t4100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4100[0].clone();
v_v = t4100[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4115(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4146 = v_i.clone(); f_run_percentile(py::getitem(&t4146, &Py::str("x"))?, py::getitem(&t4146, &Py::str("p"))?, py::method(&t4146, "get", &[Py::str("method"), Py::str("linear")])?)? }, { let mut out = vec![]; for t4147 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4148 = (t4147).items()?;
if t4148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4148[0].clone();
v_v = t4148[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_412(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t443 = v_i.clone(); f_run_chi2_critical(py::getitem(&t443, &Py::str("df"))?, py::getitem(&t443, &Py::str("alpha"))?)? }, { let mut out = vec![]; for t444 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t445 = (t444).items()?;
if t445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t445[0].clone();
v_v = t445[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4149(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(py::call(&{ let v_k = v_k.clone(); let v_lam = v_lam.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("probability"), { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lam") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument lam".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "k") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument k".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("f() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("f() got an unexpected keyword argument".into())) } s_4150(a0, a1)? })]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__pois_args(v_lam.clone(), v_k.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4150(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(f__pois_sf_ge(v_k.clone(), v_lam.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4177(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4208 = v_i.clone(); s_4149(py::getitem(&t4208, &Py::str("lam"))?, py::getitem(&t4208, &Py::str("k"))?)? }, { let mut out = vec![]; for t4209 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4210 = (t4209).items()?;
if t4210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4210[0].clone();
v_v = t4210[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4211(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(py::call(&{ let v_k = v_k.clone(); let v_lam = v_lam.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("probability"), { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lam") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument lam".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "k") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument k".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("f() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("f() got an unexpected keyword argument".into())) } s_4212(a0, a1)? })]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__pois_args(v_lam.clone(), v_k.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4212(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(f__pois_cdf(v_k.clone(), v_lam.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4229(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4260 = v_i.clone(); s_4211(py::getitem(&t4260, &Py::str("lam"))?, py::getitem(&t4260, &Py::str("k"))?)? }, { let mut out = vec![]; for t4261 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4262 = (t4261).items()?;
if t4262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4262[0].clone();
v_v = t4262[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4263(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(py::call(&{ let v_k = v_k.clone(); let v_lam = v_lam.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("probability"), { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lam") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument lam".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "k") { Some(x) => x, None => { return Err(PyErr::Type("f() missing argument k".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("f() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("f() got an unexpected keyword argument".into())) } s_4264(a0, a1)? })]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__pois_args(v_lam.clone(), v_k.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4264(mut v_lam: Py, mut v_k: Py) -> R {
    return Ok(f__pois_pmf(v_k.clone(), v_lam.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4278(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4309 = v_i.clone(); s_4263(py::getitem(&t4309, &Py::str("lam"))?, py::getitem(&t4309, &Py::str("k"))?)? }, { let mut out = vec![]; for t4310 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4311 = (t4310).items()?;
if t4311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4311[0].clone();
v_v = t4311[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4312(mut v_x: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), py::add(&Py::Int(1i64), &Py::Int(0i64))?, Py::str("the sample"))?;
    v_v = f__var(v_x.clone(), Py::Int(0i64))?;
    v_out = Py::dict(vec![(Py::str("mean"), f__mean(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[(if (Py::Bool(true)).truthy() { Py::dict(vec![(Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?), (Py::str("variance"), v_v.clone())]) } else { Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?)]) })])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4319(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4350 = v_i.clone(); s_4312(py::getitem(&t4350, &Py::str("x"))?)? }, { let mut out = vec![]; for t4351 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4352 = (t4351).items()?;
if t4352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4352[0].clone();
v_v = t4352[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4353(mut v_x: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), py::add(&Py::Int(1i64), &Py::Int(0i64))?, Py::str("the sample"))?;
    v_v = f__var(v_x.clone(), Py::Int(0i64))?;
    v_out = Py::dict(vec![(Py::str("mean"), f__mean(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[(if (Py::Bool(false)).truthy() { Py::dict(vec![(Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?), (Py::str("variance"), v_v.clone())]) } else { Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?)]) })])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4360(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4391 = v_i.clone(); s_4353(py::getitem(&t4391, &Py::str("x"))?)? }, { let mut out = vec![]; for t4392 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4393 = (t4392).items()?;
if t4393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4393[0].clone();
v_v = t4393[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4434(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4465 = v_i.clone(); f_run_power_one(py::getitem(&t4465, &Py::str("d"))?, py::getitem(&t4465, &Py::str("n"))?, py::getitem(&t4465, &Py::str("alpha"))?, py::method(&t4465, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t4466 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4467 = (t4466).items()?;
if t4467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4467[0].clone();
v_v = t4467[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4508(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4539 = v_i.clone(); f_run_power_two(py::getitem(&t4539, &Py::str("d"))?, py::getitem(&t4539, &Py::str("n"))?, py::getitem(&t4539, &Py::str("alpha"))?, py::method(&t4539, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t4540 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4541 = (t4540).items()?;
if t4541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4541[0].clone();
v_v = t4541[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4559(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4590 = v_i.clone(); f_run_at_least_one(py::getitem(&t4590, &Py::str("p"))?, py::getitem(&t4590, &Py::str("n"))?)? }, { let mut out = vec![]; for t4591 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4592 = (t4591).items()?;
if t4592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4592[0].clone();
v_v = t4592[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4629(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4660 = v_i.clone(); f_run_probability_rules(py::getitem(&t4660, &Py::str("p_a"))?, py::getitem(&t4660, &Py::str("p_b"))?, py::method(&t4660, "get", &[Py::str("p_a_and_b"), Py::None])?, py::method(&t4660, "get", &[Py::str("p_a_or_b"), Py::None])?, py::method(&t4660, "get", &[Py::str("relation"), Py::None])?)? }, { let mut out = vec![]; for t4661 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4662 = (t4661).items()?;
if t4662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4662[0].clone();
v_v = t4662[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4670(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4701 = v_i.clone(); f_run_capability(py::getitem(&t4701, &Py::str("mu"))?, py::getitem(&t4701, &Py::str("sigma"))?, py::getitem(&t4701, &Py::str("lsl"))?, py::getitem(&t4701, &Py::str("usl"))?)? }, { let mut out = vec![]; for t4702 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4703 = (t4702).items()?;
if t4703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4703[0].clone();
v_v = t4703[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4729(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4760 = v_i.clone(); f_run_prop_z(py::getitem(&t4760, &Py::str("x"))?, py::getitem(&t4760, &Py::str("n"))?, py::getitem(&t4760, &Py::str("p0"))?, py::method(&t4760, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t4761 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4762 = (t4761).items()?;
if t4762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4762[0].clone();
v_v = t4762[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4812(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4843 = v_i.clone(); f_run_block_anova(py::getitem(&t4843, &Py::str("data"))?)? }, { let mut out = vec![]; for t4844 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4845 = (t4844).items()?;
if t4845.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4845[0].clone();
v_v = t4845[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4851(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4882 = v_i.clone(); f_run_range(py::getitem(&t4882, &Py::str("x"))?)? }, { let mut out = vec![]; for t4883 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4884 = (t4883).items()?;
if t4884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4884[0].clone();
v_v = t4884[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_486(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t517 = v_i.clone(); f_run_gof(py::getitem(&t517, &Py::str("observed"))?, py::method(&t517, "get", &[Py::str("proportions"), Py::None])?)? }, { let mut out = vec![]; for t518 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t519 = (t518).items()?;
if t519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t519[0].clone();
v_v = t519[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4901(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4932 = v_i.clone(); f_run_predict(py::getitem(&t4932, &Py::str("x"))?, py::getitem(&t4932, &Py::str("y"))?, py::getitem(&t4932, &Py::str("x0"))?)? }, { let mut out = vec![]; for t4933 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4934 = (t4933).items()?;
if t4934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4934[0].clone();
v_v = t4934[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4935(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py, mut v_n: Py) -> R {
    return Ok(py::call(&{ let v_mu = v_mu.clone(); let v_n = v_n.clone(); let v_sigma = v_sigma.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("probability"), f__phic(v_z.clone())?), (Py::str("z"), v_z.clone()), (Py::str("standard_error"), py::truediv(&v_sigma.clone(), &py::math1("sqrt", &v_n.clone())?)?)]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__sm_args(v_x.clone(), v_mu.clone(), v_sigma.clone(), v_n.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4947(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4978 = v_i.clone(); s_4935(py::getitem(&t4978, &Py::str("x"))?, py::getitem(&t4978, &Py::str("mu"))?, py::getitem(&t4978, &Py::str("sigma"))?, py::getitem(&t4978, &Py::str("n"))?)? }, { let mut out = vec![]; for t4979 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4980 = (t4979).items()?;
if t4980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4980[0].clone();
v_v = t4980[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4981(mut v_x: Py, mut v_mu: Py, mut v_sigma: Py, mut v_n: Py) -> R {
    return Ok(py::call(&{ let v_mu = v_mu.clone(); let v_n = v_n.clone(); let v_sigma = v_sigma.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("probability"), f__phi(v_z.clone())?), (Py::str("z"), v_z.clone()), (Py::str("standard_error"), py::truediv(&v_sigma.clone(), &py::math1("sqrt", &v_n.clone())?)?)]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__sm_args(v_x.clone(), v_mu.clone(), v_sigma.clone(), v_n.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4993(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5024 = v_i.clone(); s_4981(py::getitem(&t5024, &Py::str("x"))?, py::getitem(&t5024, &Py::str("mu"))?, py::getitem(&t5024, &Py::str("sigma"))?, py::getitem(&t5024, &Py::str("n"))?)? }, { let mut out = vec![]; for t5025 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5026 = (t5025).items()?;
if t5026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5026[0].clone();
v_v = t5026[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5044(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5075 = v_i.clone(); f_run_n_mean(py::getitem(&t5075, &Py::str("sigma"))?, py::getitem(&t5075, &Py::str("margin"))?, py::getitem(&t5075, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t5076 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5077 = (t5076).items()?;
if t5077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5077[0].clone();
v_v = t5077[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5105(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5136 = v_i.clone(); f_run_n_prop(py::getitem(&t5136, &Py::str("margin"))?, py::getitem(&t5136, &Py::str("confidence"))?, py::method(&t5136, "get", &[Py::str("p"), Py::Float(0.5)])?)? }, { let mut out = vec![]; for t5137 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5138 = (t5137).items()?;
if t5138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5138[0].clone();
v_v = t5138[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5190(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5221 = v_i.clone(); f_run_n_two_means(py::getitem(&t5221, &Py::str("d"))?, py::getitem(&t5221, &Py::str("alpha"))?, py::getitem(&t5221, &Py::str("power"))?, py::method(&t5221, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t5222 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5223 = (t5222).items()?;
if t5223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5223[0].clone();
v_v = t5223[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5224(mut v_x: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), py::add(&Py::Int(1i64), &Py::Int(1i64))?, Py::str("the sample"))?;
    v_v = f__var(v_x.clone(), Py::Int(1i64))?;
    v_out = Py::dict(vec![(Py::str("mean"), f__mean(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[(if (Py::Bool(true)).truthy() { Py::dict(vec![(Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?), (Py::str("variance"), v_v.clone())]) } else { Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?)]) })])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5231(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5262 = v_i.clone(); s_5224(py::getitem(&t5262, &Py::str("x"))?)? }, { let mut out = vec![]; for t5263 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5264 = (t5263).items()?;
if t5264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5264[0].clone();
v_v = t5264[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5265(mut v_x: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__finite(v_x.clone(), Py::str("data"))?;
    let _ = f__need(v_x.clone(), py::add(&Py::Int(1i64), &Py::Int(1i64))?, Py::str("the sample"))?;
    v_v = f__var(v_x.clone(), Py::Int(1i64))?;
    v_out = Py::dict(vec![(Py::str("mean"), f__mean(v_x.clone())?), (Py::str("n"), py::len(&v_x.clone())?)]);
    let _ = py::method(&v_out.clone(), "update", &[(if (Py::Bool(false)).truthy() { Py::dict(vec![(Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?), (Py::str("variance"), v_v.clone())]) } else { Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("standard_deviation"), py::math1("sqrt", &v_v.clone())?)]) })])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5272(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5303 = v_i.clone(); s_5265(py::getitem(&t5303, &Py::str("x"))?)? }, { let mut out = vec![]; for t5304 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5305 = (t5304).items()?;
if t5305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5305[0].clone();
v_v = t5305[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5306(mut v_r: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("reliability"), s_5307(f__rels(v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5307(mut v_r: Py) -> R {
    return Ok(py::prod(&v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5317(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5348 = v_i.clone(); s_5306(py::getitem(&t5348, &Py::str("r"))?)? }, { let mut out = vec![]; for t5349 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5350 = (t5349).items()?;
if t5350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5350[0].clone();
v_v = t5350[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5386(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5417 = v_i.clone(); f_run_sign_test(py::getitem(&t5417, &Py::str("x"))?, py::method(&t5417, "get", &[Py::str("y"), Py::None])?, py::method(&t5417, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t5418 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5419 = (t5418).items()?;
if t5419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5419[0].clone();
v_v = t5419[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5466(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5497 = v_i.clone(); f_run_simple_regression(py::getitem(&t5497, &Py::str("x"))?, py::getitem(&t5497, &Py::str("y"))?)? }, { let mut out = vec![]; for t5498 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5499 = (t5498).items()?;
if t5499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5499[0].clone();
v_v = t5499[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5511(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5542 = v_i.clone(); f_run_skew_kurt(py::getitem(&t5542, &Py::str("x"))?)? }, { let mut out = vec![]; for t5543 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5544 = (t5543).items()?;
if t5544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5544[0].clone();
v_v = t5544[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5596(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5627 = v_i.clone(); f_run_spearman(py::getitem(&t5627, &Py::str("x"))?, py::getitem(&t5627, &Py::str("y"))?)? }, { let mut out = vec![]; for t5628 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5629 = (t5628).items()?;
if t5629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5629[0].clone();
v_v = t5629[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_563(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t594 = v_i.clone(); f_run_independence(py::getitem(&t594, &Py::str("table"))?)? }, { let mut out = vec![]; for t595 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t596 = (t595).items()?;
if t596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t596[0].clone();
v_v = t596[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5636(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5667 = v_i.clone(); f_run_sem(py::getitem(&t5667, &Py::str("x"))?)? }, { let mut out = vec![]; for t5668 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5669 = (t5668).items()?;
if t5669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5669[0].clone();
v_v = t5669[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5727(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5758 = v_i.clone(); f_run_t_critical(py::getitem(&t5758, &Py::str("df"))?, py::getitem(&t5758, &Py::str("alpha"))?, py::getitem(&t5758, &Py::str("tails"))?)? }, { let mut out = vec![]; for t5759 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5760 = (t5759).items()?;
if t5760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5760[0].clone();
v_v = t5760[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5804(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5835 = v_i.clone(); f_run_t_one(py::getitem(&t5835, &Py::str("x"))?, py::getitem(&t5835, &Py::str("mu0"))?, py::method(&t5835, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t5836 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5837 = (t5836).items()?;
if t5837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5837[0].clone();
v_v = t5837[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5890(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t5921 = v_i.clone(); f_run_t_summary(py::getitem(&t5921, &Py::str("m1"))?, py::getitem(&t5921, &Py::str("s1"))?, py::getitem(&t5921, &Py::str("n1"))?, py::getitem(&t5921, &Py::str("m2"))?, py::getitem(&t5921, &Py::str("s2"))?, py::getitem(&t5921, &Py::str("n2"))?, py::getitem(&t5921, &Py::str("variance"))?, py::method(&t5921, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t5922 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5923 = (t5922).items()?;
if t5923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5923[0].clone();
v_v = t5923[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5924(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_se: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t5940 = (f__two_sample(v_x.clone(), v_y.clone(), Py::str("pooled"))?).items()?;
    if t5940.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t5940[0].clone();
    v_se = t5940[1].clone();
    v_df = t5940[2].clone();
    return Ok(f__t_out(v_d.clone(), v_se.clone(), v_df.clone(), v_alternative.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5974(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6005 = v_i.clone(); s_5924(py::getitem(&t6005, &Py::str("x"))?, py::getitem(&t6005, &Py::str("y"))?, py::method(&t6005, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6006 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6007 = (t6006).items()?;
if t6007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6007[0].clone();
v_v = t6007[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6016(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6047 = v_i.clone(); f_run_trimmed(py::getitem(&t6047, &Py::str("x"))?, py::getitem(&t6047, &Py::str("proportion"))?)? }, { let mut out = vec![]; for t6048 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6049 = (t6048).items()?;
if t6049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6049[0].clone();
v_v = t6049[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6070(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6101 = v_i.clone(); f_run_two_prop_z(py::getitem(&t6101, &Py::str("x1"))?, py::getitem(&t6101, &Py::str("n1"))?, py::getitem(&t6101, &Py::str("x2"))?, py::getitem(&t6101, &Py::str("n2"))?, py::method(&t6101, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6102 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6103 = (t6102).items()?;
if t6103.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6103[0].clone();
v_v = t6103[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6153(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6184 = v_i.clone(); f_run_two_way_table(py::getitem(&t6184, &Py::str("table"))?, py::getitem(&t6184, &Py::str("row"))?, py::getitem(&t6184, &Py::str("column"))?)? }, { let mut out = vec![]; for t6185 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6186 = (t6185).items()?;
if t6186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6186[0].clone();
v_v = t6186[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6199(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6230 = v_i.clone(); f_run_weibull(py::getitem(&t6230, &Py::str("beta"))?, py::getitem(&t6230, &Py::str("eta"))?, py::getitem(&t6230, &Py::str("t"))?)? }, { let mut out = vec![]; for t6231 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6232 = (t6231).items()?;
if t6232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6232[0].clone();
v_v = t6232[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6247(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6278 = v_i.clone(); f_run_weighted_mean(py::getitem(&t6278, &Py::str("x"))?, py::getitem(&t6278, &Py::str("w"))?)? }, { let mut out = vec![]; for t6279 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6280 = (t6279).items()?;
if t6280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6280[0].clone();
v_v = t6280[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6281(mut v_x: Py, mut v_y: Py, mut v_alternative: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_se: Py = Py::None;
    let _ = f__check_alt(v_alternative.clone())?;
    let t6297 = (f__two_sample(v_x.clone(), v_y.clone(), Py::str("welch"))?).items()?;
    if t6297.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t6297[0].clone();
    v_se = t6297[1].clone();
    v_df = t6297[2].clone();
    return Ok(f__t_out(v_d.clone(), v_se.clone(), v_df.clone(), v_alternative.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_629(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t660 = v_i.clone(); f_run_chi2_var(py::getitem(&t660, &Py::str("x"))?, py::getitem(&t660, &Py::str("sigma0"))?, py::method(&t660, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t661 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t662 = (t661).items()?;
if t662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t662[0].clone();
v_v = t662[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6331(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6362 = v_i.clone(); s_6281(py::getitem(&t6362, &Py::str("x"))?, py::getitem(&t6362, &Py::str("y"))?, py::method(&t6362, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6363 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6364 = (t6363).items()?;
if t6364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6364[0].clone();
v_v = t6364[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6403(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6434 = v_i.clone(); f_run_wilcoxon(py::getitem(&t6434, &Py::str("x"))?, py::method(&t6434, "get", &[Py::str("y"), Py::None])?, py::method(&t6434, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6435 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6436 = (t6435).items()?;
if t6436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6436[0].clone();
v_v = t6436[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6486(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6517 = v_i.clone(); f_run_wilcoxon_exact(py::getitem(&t6517, &Py::str("x"))?, py::method(&t6517, "get", &[Py::str("y"), Py::None])?, py::method(&t6517, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6518 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6519 = (t6518).items()?;
if t6519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6519[0].clone();
v_v = t6519[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6543(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6574 = v_i.clone(); f_run_wilson(py::getitem(&t6574, &Py::str("x"))?, py::getitem(&t6574, &Py::str("n"))?, py::getitem(&t6574, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t6575 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6576 = (t6575).items()?;
if t6576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6576[0].clone();
v_v = t6576[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6595(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6626 = v_i.clone(); f_run_xbar_r(py::getitem(&t6626, &Py::str("data"))?)? }, { let mut out = vec![]; for t6627 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6628 = (t6627).items()?;
if t6628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6628[0].clone();
v_v = t6628[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6648(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6679 = v_i.clone(); f_run_xbar_s(py::getitem(&t6679, &Py::str("data"))?)? }, { let mut out = vec![]; for t6680 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6681 = (t6680).items()?;
if t6681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6681[0].clone();
v_v = t6681[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6702(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6733 = v_i.clone(); f_run_z_critical(py::getitem(&t6733, &Py::str("alpha"))?, py::getitem(&t6733, &Py::str("tails"))?)? }, { let mut out = vec![]; for t6734 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6735 = (t6734).items()?;
if t6735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6735[0].clone();
v_v = t6735[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6740(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6771 = v_i.clone(); f_run_zscore(py::getitem(&t6771, &Py::str("x"))?, py::getitem(&t6771, &Py::str("mu"))?, py::getitem(&t6771, &Py::str("sigma"))?)? }, { let mut out = vec![]; for t6772 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6773 = (t6772).items()?;
if t6773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6773[0].clone();
v_v = t6773[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6792(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t6823 = v_i.clone(); f_run_z_test(py::getitem(&t6823, &Py::str("xbar"))?, py::getitem(&t6823, &Py::str("mu0"))?, py::getitem(&t6823, &Py::str("sigma"))?, py::getitem(&t6823, &Py::str("n"))?, py::method(&t6823, "get", &[Py::str("alternative"), Py::str("two-sided")])?)? }, { let mut out = vec![]; for t6824 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6825 = (t6824).items()?;
if t6825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6825[0].clone();
v_v = t6825[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_687(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t718 = v_i.clone(); f_run_ci_mean_z(py::getitem(&t718, &Py::str("xbar"))?, py::getitem(&t718, &Py::str("sigma"))?, py::getitem(&t718, &Py::str("n"))?, py::getitem(&t718, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t719 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t720 = (t719).items()?;
if t720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t720[0].clone();
v_v = t720[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_772(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t803 = v_i.clone(); f_run_ci_mean_t(py::getitem(&t803, &Py::str("x"))?, py::getitem(&t803, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t804 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t805 = (t804).items()?;
if t805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t805[0].clone();
v_v = t805[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_863(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t894 = v_i.clone(); f_run_ci_mean_t_summary(py::getitem(&t894, &Py::str("xbar"))?, py::getitem(&t894, &Py::str("s"))?, py::getitem(&t894, &Py::str("n"))?, py::getitem(&t894, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t895 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t896 = (t895).items()?;
if t896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t896[0].clone();
v_v = t896[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_925(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t956 = v_i.clone(); f_run_ci_prop(py::getitem(&t956, &Py::str("x"))?, py::getitem(&t956, &Py::str("n"))?, py::getitem(&t956, &Py::str("confidence"))?)? }, { let mut out = vec![]; for t957 in py::iter(&(py::method(&v_o.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t958 = (t957).items()?;
if t958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t958[0].clone();
v_v = t958[1].clone();
out.push((v_k.clone(), v_v.clone())); } Py::dict(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
