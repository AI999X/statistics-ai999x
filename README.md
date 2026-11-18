# Statistics (statistics-ai999x)

[![tests](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/statistics-ai999x)](https://pypi.org/project/statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/statistics-ai999x)

Statistics covers descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability: 112 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Statistics desktop app: One-sample t-test, with its worked example and live scene](https://www.ai999x.com/?ax_shot=statistics-ai999x/hero.webp)](https://www.ai999x.com/models/statistics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-statistics-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-statistics-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `statistics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/statistics-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install statistics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Statistics desktop app: every scene and chart is drawn from the computed answer.

[![Statistics desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=statistics-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=statistics-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=statistics-ai999x/clip.mp4)*

| | |
|---|---|
| [![One-sample t-test: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-1.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-t_test_one_sample)<br>One-sample t-test | [![Normal probability below a value: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-2.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-normal_probability_below)<br>Normal probability below a value |
| [![Confidence interval for a mean from data (t-interval): the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-3.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-ci_mean_t)<br>Confidence interval for a mean from data (t-interval) | [![Weibull reliability, hazard rate and MTTF: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-4.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-weibull_reliability)<br>Weibull reliability, hazard rate and MTTF |
| [![One-way ANOVA: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-5.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-one_way_anova)<br>One-way ANOVA | [![Fisher's exact test (2 x 2): the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-6.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-fisher_exact_test)<br>Fisher's exact test (2 x 2) |
| [![Critical value of chi-square: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-7.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-chi_square_critical_value)<br>Critical value of chi-square | [![Arithmetic mean: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-8.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-mean)<br>Arithmetic mean |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![One-sample t-test: a chart in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-1.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-t_test_one_sample)<br>One-sample t-test | [![Normal probability below a value: a chart in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-2.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-normal_probability_below)<br>Normal probability below a value | [![Confidence interval for a mean from data (t-interval): a chart in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-3.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-ci_mean_t)<br>Confidence interval for a mean from data (t-interval) | [![One-way ANOVA: a chart in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-4.webp)](https://www.ai999x.com/models/statistics-ai999x/#cap-one_way_anova)<br>One-way ANOVA |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import statistics_ai999x as statistics

a = statistics.ask(
    "Fisher's exact test for the table [[8, 2], [1, 5]], two-sided."
)
print(a.outcome, a.values)
# SOLVED {'p_value': 0.03496503496503497, 'odds_ratio': 20.0}

a = statistics.ask("Find the mean of [2, 4, 9].")
print(a.outcome, a.values)
# SOLVED {'mean': 5.0, 'n': 3, 'sum': 15.0}

a = statistics.ask(
    "Find the effects of the 2^2 factorial design with responses in "
    "standard order [28, 36, 18, 31]."
)
print(a.outcome, a.values)
# SOLVED {'grand_mean': 28.25, 'A': 10.5, 'B': -7.5, 'AB': 2.5}

a = statistics.ask(
    "Find the power of a one-sample z-test with effect size d = 0.5, n "
    "= 30 and alpha = 0.05, two-sided."
)
print(a.outcome, a.values)
# SOLVED {'power': 0.7819079987319799}

a = statistics.ask(
    "Find the binomial probability of at least k = 3 successes in n = "
    "10 trials with p = 0.4."
)
print(a.outcome, a.values)
# SOLVED {'probability': 0.8327102464000001}

a = statistics.ask(
    "Find the c chart limits for defect counts [3, 5, 2, 6, 4, 7, 3, "
    "4]."
)
print(a.outcome, a.values)
# SOLVED {'center': 4.25, 'ucl': 10.43465843842649, 'lcl': 0.0}

a = statistics.ask(
    "Find the F critical value with df1 = 3, df2 = 20 and alpha = "
    "0.05."
)
print(a.outcome, a.values)
# SOLVED {'critical_value': 3.0983912121407844}

a = statistics.ask(
    "Perform a Kruskal-Wallis test on [27, 2, 4, 18, 7, 9], [20, 8, "
    "14, 36, 21, 22] and [34, 31, 3, 23, 30, 6]."
)
print(a.outcome, a.values)
# SOLVED {'h': 2.853801169590639, 'df': 2, 'p_value': 0.24005179059000692}

a = statistics.ask(
    "Find the power of a two-sample z-test with effect size d = 0.5, n "
    "= 30 and alpha = 0.05, two-sided."
)
print(a.outcome, a.values)
# SOLVED {'power': 0.49068556766996907}

a = statistics.ask(
    "What is the probability of at least one success in n = 5 "
    "independent trials with probability = 0.2?"
)
print(a.outcome, a.values)
# SOLVED {'probability': 0.67232, 'none': 0.3276800000000001}

a = statistics.ask(
    "Find the binomial probability of at most k = 3 successes in n = "
    "10 trials with p = 0.4."
)
print(a.outcome, a.values)
# SOLVED {'probability': 0.3822806015999999}

a = statistics.ask(
    "Find the reliability of a k-out-of-n system with k = 2, n = 3 and "
    "component reliability = 0.9."
)
print(a.outcome, a.values)
# SOLVED {'reliability': 0.9720000000000001}

a = statistics.ask(
    "Find the critical t value for a 95% confidence interval with df = "
    "10."
)
print(a.outcome, a.values)
# SOLVED {'critical_value': 2.228138851986274}

a = statistics.ask(
    "Find Cronbach's alpha for [[4, 5, 4], [3, 3, 4], [5, 5, 5], [2, "
    "3, 2], [4, 4, 3]]."
)
print(a.outcome, a.values)
# SOLVED {'alpha': 0.9130434782608696, 'items': 3, 'respondents': 5}

a = statistics.ask(
    "Perform a exact Mann-Whitney U test on [3.1, 5.2, 4.3, 6.4, 7.5] "
    "and [1.5, 2.6, 2.2, 3.9], two-sided."
)
print(a.outcome, a.values)
# SOLVED {'u': 19.0, 'p_value': 0.031746031746031744}
```

## Command line: 15 examples

Each command below was run with `statistics` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ statistics solve one_way_anova "g1=[23, 25, 21, 22]" "g2=[30, 28, 31, 29]" "g3=[26, 27, 24, 25]"
f = 22.12
p_value = 0.0003357531392
df_between = 2
df_within = 9
ss_between = 92.16666667
ss_within = 18.75

$ statistics solve fisher_exact_test "table=[[8, 2], [1, 5]]" alternative=two-sided
p_value = 0.03496503497
odds_ratio = 20

$ statistics solve ci_mean_known_sigma xbar=52 sigma=8 n=36 confidence=0.95
lower = 49.38671469
upper = 54.61328531
margin_of_error = 2.613285313
critical_value = 1.959963985

$ statistics solve chi_square_critical_value df=4 alpha=0.05
critical_value = 9.487729037
left_critical_value = 0.7107230214

$ statistics solve mean "x=[2, 4, 9]"
mean = 5
n = 3
sum = 15

$ statistics solve cohens_d "x=[12, 15, 11, 18, 14]" "y=[10, 9, 13, 11, 8]"
d = 1.605793084
hedges_g = 1.450393753
pooled_sd = 2.366431913

$ statistics solve factorial_2x2_effects "y=[28, 36, 18, 31]"
grand_mean = 28.25
A = 10.5
B = -7.5
AB = 2.5

$ statistics solve bartlett_test "g1=[21, 24, 19, 26, 23]" "g2=[22, 23, 22, 24, 23]" "g3=[18, 28, 20, 30, 24]"
statistic = 8.618215556
df = 2
p_value = 0.01344554065

$ statistics solve friedman_test "data=[[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]"
chi_square = 0.4
df = 2
p_value = 0.8187307531

$ statistics solve power_one_sample_z d=0.5 n=30 alpha=0.05 alternative=two-sided
power = 0.7819079987

$ statistics solve two_way_table_probabilities "table=[[30, 20], [10, 40]]" row=1 column=2
total = 100
joint = [[0.3,0.2],[0.1,0.4]]
row_marginals = [0.5,0.5]
column_marginals = [0.4,0.6]
row_given_column = [[0.75,0.3333333333333333],[0.25,0.6666666666666666]]
column_given_row = [[0.6,0.4],[0.2,0.8]]

$ statistics solve binomial_at_least n=10 k=3 p=0.4
probability = 0.8327102464

$ statistics solve c_chart "counts=[3, 5, 2, 6, 4, 7, 3, 4]"
center = 4.25
ucl = 10.43465844
lcl = 0

$ statistics solve correlation_confidence_interval "x=[1, 2, 3, 4, 5, 6]" "y=[2, 3, 5, 4, 6, 7]" confidence=0.95
r = 0.9428571429
lower = 0.5591491951
upper = 0.9938998356

$ statistics solve exponential_reliability t=1000 rate=0.0002
reliability = 0.8187307531
unreliability = 0.1812692469
mttf = 5000
failure_rate = 0.0002
```

## Rust library: 15 examples

The crate `statistics-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `statistics_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/statistics/examples/readme.rs`: `cd rust && cargo run --release --example readme -p statistics-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&statistics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "one_way_anova", "inputs": {"g1": [23, 25, 21, 22], "g2": [30, 28, 31, 29], "g3": [26, 27, 24, 25]}}));
// SOLVED
// f = 22.12
// p_value = 0.00033575313917739546
// df_between = 2
// df_within = 9
// ss_between = 92.16666666666667
// ss_within = 18.75

show(json!({"key": "fisher_exact_test", "inputs": {"table": [[8, 2], [1, 5]], "alternative": "two-sided"}}));
// SOLVED
// p_value = 0.03496503496503497
// odds_ratio = 20.0

show(json!({"key": "ci_mean_known_sigma", "inputs": {"xbar": "52", "sigma": "8", "n": "36", "confidence": "0.95"}}));
// SOLVED
// lower = 49.38671468727993
// upper = 54.61328531272007
// margin_of_error = 2.613285312720072
// critical_value = 1.959963984540054

show(json!({"key": "chi_square_critical_value", "inputs": {"df": "4", "alpha": "0.05"}}));
// SOLVED
// critical_value = 9.487729036781154
// left_critical_value = 0.7107230213973241

show(json!({"key": "mean", "inputs": {"x": [2, 4, 9]}}));
// SOLVED
// mean = 5.0
// n = 3
// sum = 15.0

show(json!({"key": "cohens_d", "inputs": {"x": [12, 15, 11, 18, 14], "y": [10, 9, 13, 11, 8]}}));
// SOLVED
// d = 1.6057930839841819
// hedges_g = 1.4503937532760351
// pooled_sd = 2.3664319132398464

show(json!({"key": "factorial_2x2_effects", "inputs": {"y": [28, 36, 18, 31]}}));
// SOLVED
// grand_mean = 28.25
// A = 10.5
// B = -7.5
// AB = 2.5

show(json!({"key": "bartlett_test", "inputs": {"g1": [21, 24, 19, 26, 23], "g2": [22, 23, 22, 24, 23], "g3": [18, 28, 20, 30, 24]}}));
// SOLVED
// statistic = 8.618215556185413
// df = 2
// p_value = 0.013445540650031874

show(json!({"key": "friedman_test", "inputs": {"data": [[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]}}));
// SOLVED
// chi_square = 0.4000000000000057
// df = 2
// p_value = 0.8187307530779795

show(json!({"key": "power_one_sample_z", "inputs": {"d": "0.5", "n": "30", "alpha": "0.05", "alternative": "two-sided"}}));
// SOLVED
// power = 0.7819079987319799

show(json!({"key": "two_way_table_probabilities", "inputs": {"table": [[30, 20], [10, 40]], "row": "1", "column": "2"}}));
// SOLVED
// total = 100.0
// joint = [[0.3,0.2],[0.1,0.4]]
// row_marginals = [0.5,0.5]
// column_marginals = [0.4,0.6]
// row_given_column = [[0.75,0.3333333333333333],[0.25,0.6666666666666666]]
// column_given_row = [[0.6,0.4],[0.2,0.8]]

show(json!({"key": "binomial_at_least", "inputs": {"n": "10", "k": "3", "p": "0.4"}}));
// SOLVED
// probability = 0.8327102464000001

show(json!({"key": "c_chart", "inputs": {"counts": [3, 5, 2, 6, 4, 7, 3, 4]}}));
// SOLVED
// center = 4.25
// ucl = 10.43465843842649
// lcl = 0.0

show(json!({"key": "correlation_confidence_interval", "inputs": {"x": [1, 2, 3, 4, 5, 6], "y": [2, 3, 5, 4, 6, 7], "confidence": "0.95"}}));
// SOLVED
// r = 0.9428571428571428
// lower = 0.5591491951264971
// upper = 0.9938998355880978

show(json!({"key": "exponential_reliability", "inputs": {"t": "1000", "rate": "0.0002"}}));
// SOLVED
// reliability = 0.8187307530779818
// unreliability = 0.18126924692201815
// mttf = 5000.0
// failure_rate = 0.0002
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Statistics on ai999x.com](https://www.ai999x.com/models/statistics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/statistics-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `statistics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/statistics`) and its data (`rust/models/statistics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p statistics-ai999x
./target/release/statistics --help
cargo test -p statistics-ai999x
```
