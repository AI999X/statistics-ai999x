//! Statistics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p statistics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&statistics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "one_way_anova", "inputs": {"g1": [23, 25, 21, 22], "g2": [30, 28, 31, 29], "g3": [26, 27, 24, 25]}}));
    println!();
    show(json!({"key": "fisher_exact_test", "inputs": {"table": [[8, 2], [1, 5]], "alternative": "two-sided"}}));
    println!();
    show(json!({"key": "ci_mean_known_sigma", "inputs": {"xbar": "52", "sigma": "8", "n": "36", "confidence": "0.95"}}));
    println!();
    show(json!({"key": "chi_square_critical_value", "inputs": {"df": "4", "alpha": "0.05"}}));
    println!();
    show(json!({"key": "mean", "inputs": {"x": [2, 4, 9]}}));
    println!();
    show(json!({"key": "cohens_d", "inputs": {"x": [12, 15, 11, 18, 14], "y": [10, 9, 13, 11, 8]}}));
    println!();
    show(json!({"key": "factorial_2x2_effects", "inputs": {"y": [28, 36, 18, 31]}}));
    println!();
    show(json!({"key": "bartlett_test", "inputs": {"g1": [21, 24, 19, 26, 23], "g2": [22, 23, 22, 24, 23], "g3": [18, 28, 20, 30, 24]}}));
    println!();
    show(json!({"key": "friedman_test", "inputs": {"data": [[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]}}));
    println!();
    show(json!({"key": "power_one_sample_z", "inputs": {"d": "0.5", "n": "30", "alpha": "0.05", "alternative": "two-sided"}}));
    println!();
    show(json!({"key": "two_way_table_probabilities", "inputs": {"table": [[30, 20], [10, 40]], "row": "1", "column": "2"}}));
    println!();
    show(json!({"key": "binomial_at_least", "inputs": {"n": "10", "k": "3", "p": "0.4"}}));
    println!();
    show(json!({"key": "c_chart", "inputs": {"counts": [3, 5, 2, 6, 4, 7, 3, 4]}}));
    println!();
    show(json!({"key": "correlation_confidence_interval", "inputs": {"x": [1, 2, 3, 4, 5, 6], "y": [2, 3, 5, 4, 6, 7], "confidence": "0.95"}}));
    println!();
    show(json!({"key": "exponential_reliability", "inputs": {"t": "1000", "rate": "0.0002"}}));
}
