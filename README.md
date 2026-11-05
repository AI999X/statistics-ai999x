# statistics-ai999x: offline, verified statistics solver with step-by-step answers

[![tests](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/statistics-ai999x)](https://pypi.org/project/statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/statistics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/statistics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/statistics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/statistics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/statistics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Mathematics & Statistics** · Tasks: Question Answering, Tabular Regression

statistics-ai999x is an offline, deterministic statistics solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers statistics questions with verified results from 110 certified capabilities, gives 609 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**statistics-ai999x** reads a question in plain English (or a typed call), picks a certified statistics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** statistics-ai999x comes as the AI999X desktop app and as the command-line program `statistics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Statistics desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`statistics`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/statistics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install statistics-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Statistics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Statistics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-statistics-1.5.0-linux-x86_64.tar.gz
./ai999x-statistics-1.5.0-linux-x86_64/ai999x-statistics
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
statistics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
statistics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ statistics ask "Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided."
Read as: One-sample t-test
t = 1.889822365
df = 5
p_value = 0.1173867951
mean = 5.166666667
standard_deviation = 0.2160246899
  by: One-sample t-test  (t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function)
  mu0 = 5
  alternative = two-sided
  assumes: random sample from a roughly normal population (or large n); the alternative must be stated for a one-sided claim
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ statistics info t_test_one_sample
One-sample t-test
  t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function
  in   x        the data values (at least two) (vector) [1]
  in   mu0      hypothesised mean [1]
  in   alternative alternative hypothesis: two-sided (default), greater or less (choice) [1]
  out  t        (mean - mu0)/(s/sqrt(n)) [1]
  out  df       n - 1 [1]
  out  p_value  p-value [1]
  out  mean     sample mean [1]
  out  standard_deviation sample standard deviation [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ statistics solve t_test_one_sample "x=[5.1, 4.9, 5.3, 5.5, 5, 5.2]" mu0=5.0 alternative=two-sided
t = 1.889822365
df = 5
p_value = 0.1173867951
mean = 5.166666667
standard_deviation = 0.2160246899
  by: One-sample t-test  (t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function)
  mu0 = 5
  alternative = two-sided
  assumes: random sample from a roughly normal population (or large n); the alternative must be stated for a one-sided claim
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | statistics batch

`cases.jsonl`:

```json
{"key": "t_test_one_sample", "inputs": {"x": "[5.1, 4.9, 5.3, 5.5, 5, 5.2]", "mu0": 5.0, "alternative": "two-sided"}}
{"key": "normal_probability_below", "inputs": {"mu": 100.0, "sigma": 15.0, "x": 120.0}}
```

```console
$ statistics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.11738679514629244,"unit":"","solved_for":"p_value","si":[["mu0",5.0]],"values":[["t",1.88982236504614,""],["df",5,""],["p_value",0.11738679514629244,""],["mean",5.166666666666667,""],["standard_deviation",0.2160246899469286,""]],"target":"p_value","phrase":"(mean - mu0)/(s/sqrt(n))","assumptions":["random sample from a roughly normal population (or large n)","the alternative must be stated for a one-sided claim"],"name":"One-sample t-test","equation":"t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function"}
{"outcome":"SOLVED","message":"","value":0.9087887802741321,"unit":"","solved_for":"probability","si":[["mu",100.0],["sigma",15.0],["x",120.0]],"values":[["probability",0.9087887802741321,""],["z",1.3333333333333333,""]],"target":"probability","phrase":"P(X < x)","assumptions":["X normally distributed","sigma > 0"],"name":"Normal probability below a value","equation":"z = (x - mu)/sigma; Phi(z) from the complementary error function"}
```

### In your browser

Open [Statistics on ai999x.com](https://www.ai999x.com/models/statistics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `statistics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install statistics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### One-way ANOVA

One-way analysis of variance for 2 to 5 independent groups: F statistic, p-value, sums of squares and eta squared.

Command line:

```console
$ statistics solve one_way_anova "g1=[23, 25, 21, 22]" "g2=[30, 28, 31, 29]" "g3=[26, 27, 24, 25]"
f = 22.12
p_value = 0.0003357531392
df_between = 2
df_within = 9
ss_between = 92.16666667
ss_within = 18.75
ms_between = 46.08333333
ms_within = 2.083333333
eta_squared = 0.8309541698
  by: One-way ANOVA  (partition the total sum of squares into between- and within-group parts; F with (k - 1, N - k) df)
  assumes: independent groups; roughly normal populations with equal variances
```

Python:

```python
import statistics_ai999x as m

a = m.solve("one_way_anova", g1=[23, 25, 21, 22], g2=[30, 28, 31, 29], g3=[26, 27, 24, 25])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/statistics-ai999x/#cap-one_way_anova) and press **Compute in your browser**.

_What this means:_ One-way analysis of variance partitions the total variability into between-group and within-group components. The computed F ratio is 22.12, derived from the mean square between groups of 46.0833 divided by the mean square within groups of 2.08333. This corresponds to degrees of freedom 2 for the numerator and 9 for the denominator. The resulting upper-tail probability is 0.000335753, indicating whether the observed differences among group means are statistically significant relative to the background noise.

### Fisher's exact test (2 x 2)

Fisher's exact test of association in a 2 x 2 table, with the sample odds ratio.

Command line:

```console
$ statistics solve fisher_exact_test "table=[[8, 2], [1, 5]]" alternative=two-sided
p_value = 0.03496503497
odds_ratio = 20
  by: Fisher's exact test (2 x 2)  (hypergeometric probabilities of all tables with the observed margins, in exact integer arithmetic)
  alternative = two-sided
  assumes: fixed margins (conditional test); whole-number counts
```

Python:

```python
import statistics_ai999x as m

a = m.solve("fisher_exact_test", table=[[8, 2], [1, 5]], alternative="two-sided")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/statistics-ai999x/#cap-fisher_exact_test) and press **Compute in your browser**.

_What this means:_ This calculation determines the probability of observing a table as extreme or more extreme than the one provided, assuming no association between the variables. The result yields an exact p-value of 0.034965 and a sample odds ratio of 20. A low p-value suggests that the observed distribution of counts across the categories is unlikely to have occurred by chance alone under the assumption of independence.

### McNemar's test for paired proportions

McNemar's test for a 2 x 2 table of paired outcomes: continuity-corrected chi-square and the exact binomial p-value.

Command line:

```console
$ statistics solve mcnemar_test "table=[[30, 12], [5, 53]]"
chi_square = 2.117647059
p_value = 0.1456100954
exact_p_value = 0.1434631348
discordant = 17
  by: McNemar's test for paired proportions  (only the discordant cells b and c carry information)
  assumes: paired (matched) binary outcomes; b + c > 0
```

Python:

```python
import statistics_ai999x as m

a = m.solve("mcnemar_test", table=[[30, 12], [5, 53]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/statistics-ai999x/#cap-mcnemar_test) and press **Compute in your browser**.

_What this means:_ McNemar's test evaluates whether the proportion of binary outcomes differs between two related groups, such as before and after treatment. The calculation relies solely on the discordant pairs, where the outcome changed from yes to no or no to yes. Based on these counts, the continuity-corrected chi-square value is computed as 2.11765. The resulting p-value is 0.14561, which measures the probability of observing such an imbalance by chance if there were no true difference. An exact binomial p-value of 0.143463 is also provided for greater accuracy when sample sizes are small.

### Confidence interval for a mean (sigma known, z-interval)

The z confidence interval for a population mean when the population standard deviation sigma is known.

Command line:

```console
$ statistics solve ci_mean_known_sigma xbar=52 sigma=8 n=36 confidence=0.95
lower = 49.38671469
upper = 54.61328531
margin_of_error = 2.613285313
critical_value = 1.959963985
  by: Confidence interval for a mean (sigma known, z-interval)  (xbar +/- z_(alpha/2) sigma / sqrt(n))
  xbar = 52
  sigma = 8
  confidence = 0.95
  assumes: sigma known; normal population or large n; random sample
```

Python:

```python
import statistics_ai999x as m

a = m.solve("ci_mean_known_sigma", xbar="52", sigma="8", n="36", confidence="0.95")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/statistics-ai999x/#cap-ci_mean_known_sigma) and press **Compute in your browser**.

_What this means:_ This calculation determines a range of plausible values for the population mean using the sample mean and known standard deviation. The interval spans from 49.3867 to 54.6133, centered on the observed sample mean of 52. The width of this interval is defined by the margin of error, which equals 2.61329. This value represents the maximum expected difference between the true population mean and the sample estimate at the specified confidence level.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/statistics-ai999x/blob/main/docs/statistics_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/statistics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified statistics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 609 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `statistics` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability.

## Model overview

| Property | Value |
|---|---|
| Package | `statistics-ai999x` 1.5.0 (import `statistics_ai999x`) |
| Type | Deterministic neuro-symbolic statistics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 110 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 609 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `statistics-ai999x "<question>"`; for the native program use `statistics` (above). The outputs above were produced by running this exact code on statistics-ai999x 1.5.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![One-sample t test: statistic against the rejection region, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/t-test-one-sample-density.svg)

_The one-sample t test of x = 5.1, 4.9, 5.3, 5.5, 5.0, 5.2 against μ₀ = 5: the t density with 5 degrees of freedom (from the package's quantiles at 121 cumulative probabilities, differenced), the two-tailed rejection region beyond ±2.57058 for α = 0.05 shaded; marked: t = 1.88982, p-value = 0.117387._

**Poisson distribution and an at-least probability**

![Poisson distribution and an at-least probability, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/poisson-at-least-stems.svg)

_The Poisson distribution with mean λ = 4: P(X = k) for k = 0 … 14, each from the package; the stems with k ≥ 2 carry P(X ≥ 2) = 0.908422; marked: the most probable count k = 4, P = 0.195367._

**Three groups as box plots, with their ANOVA**

![Three groups as box plots, with their ANOVA, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/one-way-anova-boxes.svg)

_Box plots of 3 groups (g1 = 23, 25, 21, 22; g2 = 30, 28, 31, 29; g3 = 26, 27, 24, 25): each box spans the quartiles and the bar is the median, all from the package's percentiles (g1 median 22.5, quartiles 21.75–23.5; g2 median 29.5, quartiles 28.75–30.25; g3 median 25.5, quartiles 24.75–26.25). One-way ANOVA: F = 22.12 with 2 and 9 degrees of freedom, p-value 0.000335753, η² = 0.830954._

**Simple linear regression: data and fitted line**

![Simple linear regression: data and fitted line, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/simple-linear-regression-fit.svg)

_The least-squares line through x = 1, 2, 3, 4, 5, y = 2.2, 4.1, 6.1, 7.9, 10.2: intercept 0.16 (standard error 0.143295), slope 1.98 (standard error 0.0432049), r = 0.999287, R² = 0.998574, p-value of the slope 2.2873e-05, as returned by the package._

**Kaplan–Meier survival curve**

![Kaplan–Meier survival curve, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/kaplan-meier-survival.svg)

_The Kaplan–Meier survival curve returned by the package for times 6, 6, 6, 7, 10, 13, 16, 22, 23 with event indicators 1, 1, 1, 0, 1, 1, 1, 1, 1 (0 = censored): it steps down at each event time 6, 10, 13, 16, 22, 23; marked: t = 12, S = 0.533333._

**Confidence intervals for a mean at five levels**

![Confidence intervals for a mean at five levels, computed by statistics-ai999x](https://huggingface.co/AI999X/statistics-ai999x/resolve/main/figures/ci-mean-t-intervals.svg)

_t confidence intervals for the mean of x = 12, 15, 11, 18, 14, 16 at 5 confidence levels, each from the package: 50 % [13.5673, 15.0993]; 80 % [12.7776, 15.8891]; 90 % [12.2093, 16.4574]; 95 % [11.6237, 17.043]; 99 % [10.0831, 18.5836] (sample mean 14.3333, 5 degrees of freedom); higher confidence, wider interval._

## Capabilities

110 certified procedures. A few of them:

- **Procedures:** 2^2 factorial design: main effects and interaction; 2^3 factorial design: main effects and interactions; Arithmetic mean; Bartlett's test for equal variances; Binomial probability: at least k successes; Binomial probability: at most k successes; Binomial probability: exactly k successes; Chi-square goodness-of-fit test; Chi-square test for a variance; Chi-square test of independence; Coefficient of variation; Cohen's d and Hedges' g; Confidence interval for a correlation (Fisher z); Confidence interval for a mean (sigma known, z-interval); Confidence interval for a mean from data (t-interval); Confidence interval for a mean from summary statistics (t-interval); Confidence interval for a proportion (Wald); Confidence interval for a variance and standard deviation; Confidence interval for the difference of two means; Covariance; Critical value of F; Critical value of Student's t; Critical value of chi-square; Critical value of z …

`statistics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified statistics capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{statistics_ai999x_1_5_0,
  title  = {statistics-ai999x: an offline, verified statistics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/statistics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/statistics-ai999x/discussions or visit https://ai999x.com.

## Support the project

statistics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/statistics-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `statistics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/statistics`) and its data (`rust/models/statistics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p statistics-ai999x
./target/release/statistics --help
cargo test -p statistics-ai999x
```
