# Statistics (statistics-ai999x)

[![tests](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/statistics-ai999x)](https://pypi.org/project/statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/statistics-ai999x)

Statistics covers descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability: 112 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Statistics desktop app: Arithmetic mean, with its worked example and live scene](https://www.ai999x.com/?ax_shot=statistics-ai999x/hero.webp)](https://www.ai999x.com/models/statistics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `statistics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/statistics-ai999x/#try) |
| **Python** 3.10+ | `pip install statistics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Statistics desktop app: Arithmetic mean, with its worked example and live scene](https://www.ai999x.com/?ax_shot=statistics-ai999x/hero-dark.webp)](https://www.ai999x.com/models/statistics-ai999x/)

[![Exponential probability above a value: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/see-1.webp)](https://www.ai999x.com/models/statistics-ai999x/)

*Exponential probability above a value*

[![Chi-square test of independence: the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/see-2.webp)](https://www.ai999x.com/models/statistics-ai999x/)

*Chi-square test of independence*

[![Confidence interval for a correlation (Fisher z): the live scene in the Statistics desktop app](https://www.ai999x.com/?ax_shot=statistics-ai999x/see-3.webp)](https://www.ai999x.com/models/statistics-ai999x/)

*Confidence interval for a correlation (Fisher z)*

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
