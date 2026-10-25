# statistics-ai999x — offline, verified statistics solver with step-by-step answers

[![tests](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/statistics-ai999x)](https://pypi.org/project/statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/statistics-ai999x)

**[Install](https://pypi.org/project/statistics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/statistics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/statistics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

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
| Package | `statistics-ai999x` 1.0.0 (import `statistics_ai999x`) |
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

Command line: `statistics-ai999x "<question>"`, `statistics-ai999x --capabilities`. The outputs above were produced by running this exact code on statistics-ai999x 1.0.0.

## Find the right capability, and draw it

`statistics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "statistics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import statistics_ai999x as statistics

statistics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{statistics_ai999x_1_0_0,
  title  = {statistics-ai999x: an offline, verified statistics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
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
