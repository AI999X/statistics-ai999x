# statistics-ai999x — offline, verified statistics solver with step-by-step answers

[![tests](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/statistics-ai999x)](https://pypi.org/project/statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/statistics-ai999x)

**[Install](https://pypi.org/project/statistics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/statistics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/statistics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.2

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Tabular Regression, Sentence Similarity, Feature Extraction

statistics-ai999x is an offline, deterministic Python solver that answers statistics questions with verified results from 110 certified capabilities, gives 609 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**statistics-ai999x** is a compact, deterministic statistics engine for Python. It reads a question in plain English (or a typed call), picks a certified statistics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified statistics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 609 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability.

## Model overview

| Property | Value |
|---|---|
| Package | `statistics-ai999x` 1.0.2 (import `statistics_ai999x`) |
| Type | Deterministic neuro-symbolic statistics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 110 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 609 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install statistics-ai999x
```

```python
import statistics_ai999x as statistics

statistics.ask("Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.").show()
# -> SOLVED — t = 1.88982, df = 5, p_value = 0.117387, mean = 5.16667  (t_test_one_sample)
statistics.ask("For a normal distribution with mean = 100 and standard deviation = 15, find P(X < 120).").show()
# -> SOLVED — probability = 0.908789, z = 1.33333  (normal_probability_below)
statistics.ask("Find the 95% confidence interval for the mean of [12, 15, 11, 18, 14, 16].").show()
# -> SOLVED — lower = 11.6237, upper = 17.043, margin_of_error = 2.70963, critical_value = 2.57058  (ci_mean_t)
statistics.ask("Chi-square test of independence for the table [[20, 30], [25, 25]].").show()
# -> SOLVED — chi_square = 1.0101, df = 1, p_value = 0.314879, expected = [[22.5, 27.5], [22.5, 27.5]]  (chi_square_independence)
statistics.solve("weibull_reliability", beta=2, eta=1000, t=500).show()
# -> SOLVED — reliability = 0.778801, unreliability = 0.221199, hazard_rate = 0.001, mttf = 886.227  (weibull_reliability)
statistics.ask("Who created you?").message
# -> statistics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `statistics-ai999x "<question>"`, `statistics-ai999x --capabilities`. The outputs above were produced by running this exact code on statistics-ai999x 1.0.2.

## Find the right capability, and draw it

`statistics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "statistics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import statistics_ai999x as statistics

statistics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `statistics-ai999x`; the answer is the one it returned, with the steps it took.

**One-sample t-test** · _Hypothesis tests_

> Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.

→ **t = 1.889822365, df = 5, p_value = 0.1173867951, mean = 5.166666667, standard_deviation = 0.2160246899**

<details><summary>Steps</summary>

- One-sample t-test: x = [5.1, 4.9, 5.3, 5.5, 5, 5.2], mu0 = 5, alternative = two-sided → t = 1.889822365, df = 5, p_value = 0.1173867951, mean = 5.166666667, standard_deviation = 0.2160246899

</details>

**Normal probability below a value** · _Probability distributions_

> For a normal distribution with mean = 100 and standard deviation = 15, find P(X < 120).

→ **probability = 0.9087887803, z = 1.333333333**

<details><summary>Steps</summary>

- Normal probability below a value: mu = 100, sigma = 15, x = 120 → probability = 0.9087887803, z = 1.333333333

</details>

**Confidence interval for a mean from data (t-interval)** · _Confidence intervals_

> Find the 95% confidence interval for the mean of [12, 15, 11, 18, 14, 16].

→ **lower = 11.62370216, upper = 17.0429645, margin_of_error = 2.709631171, critical_value = 2.570581836, df = 5, mean = 14.33333333**

<details><summary>Steps</summary>

- Confidence interval for a mean from data (t-interval): x = [12, 15, 11, 18, 14, 16], confidence = 0.95 → lower = 11.62370216, upper = 17.0429645, margin_of_error = 2.709631171, critical_value = 2.570581836, df = 5, mean = 14.33333333

</details>

**Chi-square test of independence** · _Hypothesis tests_

> Chi-square test of independence for the table [[20, 30], [25, 25]].

→ **chi_square = 1.01010101, df = 1, p_value = 0.3148786413, expected = [[22.5, 27.5], [22.5, 27.5]]**

<details><summary>Steps</summary>

- Chi-square test of independence: table = [20 30; 25 25] → chi_square = 1.01010101, df = 1, p_value = 0.3148786413, expected = [[22.5, 27.5], [22.5, 27.5]]

</details>

**Weibull reliability, hazard rate and MTTF** · _Reliability and survival_

`m.solve("weibull_reliability", beta=2, eta=1000, t=500)`

→ **reliability = 0.7788007831, unreliability = 0.2211992169, hazard_rate = 0.001, mttf = 886.2269255**

<details><summary>Steps</summary>

- Weibull reliability, hazard rate and MTTF: beta = 2, eta = 1000, t = 500 → reliability = 0.7788007831, unreliability = 0.2211992169, hazard_rate = 0.001, mttf = 886.2269255

</details>

**One-way ANOVA** · _Analysis of variance_

> Perform a one-way ANOVA on groups [23, 25, 21, 22], [30, 28, 31, 29] and [26, 27, 24, 25].

→ **f = 22.12, p_value = 0.0003357531392, df_between = 2, df_within = 9, ss_between = 92.16666667, ss_within = 18.75, ms_between = 46.08333333, ms_within = 2.083333333, eta_squared = 0.8309541698**

<details><summary>Steps</summary>

- One-way ANOVA: g1 = [23, 25, 21, 22], g2 = [30, 28, 31, 29], g3 = [26, 27, 24, 25] → f = 22.12, p_value = 0.0003357531392, df_between = 2, df_within = 9, ss_between = 92.16666667, ss_within = 18.75, ms_between = 46.08333333, ms_within = 2.083333333, eta_squared = 0.8309541698

</details>

## Figures

Every point was computed by `statistics-ai999x`.

**One-sample t test: statistic against the rejection region**

![One-sample t test: statistic against the rejection region — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/t-test-one-sample-density.svg)

_The one-sample t test of x = 5.1, 4.9, 5.3, 5.5, 5.0, 5.2 against μ₀ = 5: the t density with 5 degrees of freedom (from the package's quantiles at 121 cumulative probabilities, differenced), the two-tailed rejection region beyond ±2.57058 for α = 0.05 shaded; marked: t = 1.88982, p-value = 0.117387._

**Poisson distribution and an at-least probability**

![Poisson distribution and an at-least probability — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/poisson-at-least-stems.svg)

_The Poisson distribution with mean λ = 4: P(X = k) for k = 0 … 14, each from the package; the stems with k ≥ 2 carry P(X ≥ 2) = 0.908422; marked: the most probable count k = 4, P = 0.195367._

**Three groups as box plots, with their ANOVA**

![Three groups as box plots, with their ANOVA — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/one-way-anova-boxes.svg)

_Box plots of 3 groups (g1 = 23, 25, 21, 22; g2 = 30, 28, 31, 29; g3 = 26, 27, 24, 25): each box spans the quartiles and the bar is the median, all from the package's percentiles (g1 median 22.5, quartiles 21.75–23.5; g2 median 29.5, quartiles 28.75–30.25; g3 median 25.5, quartiles 24.75–26.25). One-way ANOVA: F = 22.12 with 2 and 9 degrees of freedom, p-value 0.000335753, η² = 0.830954._

**Simple linear regression: data and fitted line**

![Simple linear regression: data and fitted line — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/simple-linear-regression-fit.svg)

_The least-squares line through x = 1, 2, 3, 4, 5, y = 2.2, 4.1, 6.1, 7.9, 10.2: intercept 0.16 (standard error 0.143295), slope 1.98 (standard error 0.0432049), r = 0.999287, R² = 0.998574, p-value of the slope 2.2873e-05, as returned by the package._

**Kaplan–Meier survival curve**

![Kaplan–Meier survival curve — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/kaplan-meier-survival.svg)

_The Kaplan–Meier survival curve returned by the package for times 6, 6, 6, 7, 10, 13, 16, 22, 23 with event indicators 1, 1, 1, 0, 1, 1, 1, 1, 1 (0 = censored): it steps down at each event time 6, 10, 13, 16, 22, 23; marked: t = 12, S = 0.533333._

**Confidence intervals for a mean at five levels**

![Confidence intervals for a mean at five levels — computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/ci-mean-t-intervals.svg)

_t confidence intervals for the mean of x = 12, 15, 11, 18, 14, 16 at 5 confidence levels, each from the package: 50 % [13.5673, 15.0993]; 80 % [12.7776, 15.8891]; 90 % [12.2093, 16.4574]; 95 % [11.6237, 17.043]; 99 % [10.0831, 18.5836] (sample mean 14.3333, 5 degrees of freedom); higher confidence, wider interval._

## Capabilities

110 certified procedures. A few of them:

- **Procedures:** 2^2 factorial design: main effects and interaction; 2^3 factorial design: main effects and interactions; Arithmetic mean; Bartlett's test for equal variances; Binomial probability: at least k successes; Binomial probability: at most k successes; Binomial probability: exactly k successes; Chi-square goodness-of-fit test; Chi-square test for a variance; Chi-square test of independence; Coefficient of variation; Cohen's d and Hedges' g; Confidence interval for a correlation (Fisher z); Confidence interval for a mean (sigma known, z-interval); Confidence interval for a mean from data (t-interval); Confidence interval for a mean from summary statistics (t-interval); Confidence interval for a proportion (Wald); Confidence interval for a variance and standard deviation; Confidence interval for the difference of two means; Covariance; Critical value of F; Critical value of Student's t; Critical value of chi-square; Critical value of z …

`statistics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified statistics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{statistics_ai999x_1_0_2,
  title  = {statistics-ai999x: an offline, verified statistics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.2},
  url    = {https://huggingface.co/AI999X/statistics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/statistics-ai999x/discussions or visit https://ai999x.com.

## Support the project

statistics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/statistics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/statistics-ai999x/tree/main/onnx).
