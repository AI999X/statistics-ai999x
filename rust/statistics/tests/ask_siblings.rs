//! The question reader between capabilities that take inputs of the same kinds (ask.rs, siblings): the "two" of
//! "two-sample" (a part of a hyphenated name word) is not written by a "two-sided" alternative. Found by tools/ask_module_compare.py (2026-11-03): a
//! one-sample power question was answered as a two-sample one (0.063 where the one-sample power is 0.135).

use serde_json::Value;
use statistics_ai999x::pack;

fn ask(q: &str) -> Value {
    serde_json::from_str(&ax_engine::ask::ask_json(pack(), q)).unwrap()
}

#[test]
fn a_two_sided_alternative_does_not_make_a_two_sample_test() {
    let a = ask("Given a standardized effect size (difference / sigma) of 1.042, a sample size of 2, a significance \
                 level of 0.01, and a two-sided alternative hypothesis, compute the probability of rejecting H0 when \
                 the effect is d.");
    assert!(a["outcome"] != "SOLVED" || a["capability"] == "power_one_sample_z", "{a}");
}
