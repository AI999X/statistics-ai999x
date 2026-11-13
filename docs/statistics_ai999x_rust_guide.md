# statistics: the Statistics program, user guide

**statistics-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Statistics covers descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability. This guide is about `statistics`, the model as a self-contained program for Windows and Linux: 112 capabilities in 16 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/statistics-ai999x/); the Python package (`pip install statistics-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Statistics | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-windows-x86_64.zip |
| Desktop app: Statistics | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Statistics in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-statistics-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-statistics.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-statistics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-statistics-1.5.0-linux-x86_64.tar.gz
./ai999x-statistics-1.5.0-linux-x86_64/ai999x-statistics
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `statistics` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
statistics --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/statistics-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install statistics-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ statistics --version
statistics-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ statistics search "one-sample t-test"
t_test_one_sample                    One-sample t-test  (hypothesis tests)
power_one_sample_z                   Power of a one-sample z-test  (power and sample size)
z_test_mean                          One-sample z-test for a mean (sigma known)  (hypothesis tests)
proportion_z_test                    One-proportion z-test  (hypothesis tests)
paired_t_test                        Paired t-test  (hypothesis tests)
t_test_summary_statistics            Two-sample t-test from summary statistics  (hypothesis tests)
t_test_two_sample_pooled             Two-sample t-test (pooled variances)  (hypothesis tests)
welch_t_test                         Welch's two-sample t-test (unequal variances)  (hypothesis tests)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ statistics list
bartlett_test                            Bartlett's test for equal variances
binomial_at_least                        Binomial probability: at least k successes
binomial_at_most                         Binomial probability: at most k successes
binomial_exactly                         Binomial probability: exactly k successes
binomial_mean_sd                         Mean and standard deviation of a binomial distribution
c_chart                                  c chart limits (defects per unit)
chi_square_critical_value                Critical value of chi-square
chi_square_goodness_of_fit               Chi-square goodness-of-fit test
… 104 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ statistics solve t_test_one_sample "x=[5.1, 4.9, 5.3, 5.5, 5, 5.2]" mu0=5.0 alternative=two-sided --json
{"outcome":"SOLVED","message":"","value":0.11738679514629244,"unit":"","solved_for":"p_value","si":[["mu0",5.0]],"values":[["t",1.88982236504614,""],["df",5,""],["p_value",0.11738679514629244,""],["mean",5.166666666666667,""],["standard_deviation",0.2160246899469286,""]],"target":"p_value","phrase":"(mean - mu0)/(s/sqrt(n))","assumptions":["random sample from a roughly normal population (or large n)","the alternative must be stated for a one-sided claim"],"name":"One-sample t-test","equation":"t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function"}
```

### 2.8 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | statistics batch

`cases.jsonl`:

```json
{"key": "t_test_one_sample", "inputs": {"x": "[5.1, 4.9, 5.3, 5.5, 5, 5.2]", "mu0": 5.0, "alternative": "two-sided"}}
{"key": "normal_probability_below", "inputs": {"mu": 100.0, "sigma": 15.0, "x": 120.0}}
```

```shell
$ statistics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.11738679514629244,"unit":"","solved_for":"p_value","si":[["mu0",5.0]],"values":[["t",1.88982236504614,""],["df",5,""],["p_value",0.11738679514629244,""],["mean",5.166666666666667,""],["standard_deviation",0.2160246899469286,""]],"target":"p_value","phrase":"(mean - mu0)/(s/sqrt(n))","assumptions":["random sample from a roughly normal population (or large n)","the alternative must be stated for a one-sided claim"],"name":"One-sample t-test","equation":"t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function"}
{"outcome":"SOLVED","message":"","value":0.9087887802741321,"unit":"","solved_for":"probability","si":[["mu",100.0],["sigma",15.0],["x",120.0]],"values":[["probability",0.9087887802741321,""],["z",1.3333333333333333,""]],"target":"probability","phrase":"P(X < x)","assumptions":["X normally distributed","sigma > 0"],"name":"Normal probability below a value","equation":"z = (x - mu)/sigma; Phi(z) from the complementary error function"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `statistics info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`statistics solve ... --json` prints one answer as one line of JSON. `statistics batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | statistics batch`. `statistics catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["statistics", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`statistics figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `statistics_ai999x.wasm` (https://www.ai999x.com/compute/rust/statistics_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("statistics_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `statistics info <id>` shows the same inputs and outputs in the terminal.

### Analysis of variance (1)

<a id="one_way_anova"></a>

#### One-way ANOVA: `one_way_anova`

One-way analysis of variance for 2 to 5 independent groups: F statistic, p-value, sums of squares and eta squared.

$$
F = \frac{SS_B/(k-1)}{SS_W/(N-k)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | group 1 | - |
| `g2` | in | group 2 | - |
| `g3` | in | group 3 (optional) | - |
| `g4` | in | group 4 (optional) | - |
| `g5` | in | group 5 (optional) | - |
| `f` | out | MS between / MS within | - |
| `p_value` | out | upper-tail p-value | - |
| `df_between` | out | k - 1 | - |
| `df_within` | out | N - k | - |
| `ss_between` | out | between-group sum of squares | - |
| `ss_within` | out | within-group sum of squares | - |
| `ms_between` | out | mean square between | - |
| `ms_within` | out | mean square within | - |
| `eta_squared` | out | SSB / SST | - |

**Applies when:** independent groups; roughly normal populations with equal variances.

```shell
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

**What the worked example means:** One-way analysis of variance partitions the total variability into between-group and within-group components. The computed F ratio is 22.12, derived from the mean square between groups of 46.0833 divided by the mean square within groups of 2.08333. This corresponds to degrees of freedom 2 for the numerator and 9 for the denominator. The resulting upper-tail probability is 0.000335753, indicating whether the observed differences among group means are statistically significant relative to the background noise.

### Categorical data (2)

<a id="fisher_exact_test"></a>

#### Fisher's exact test (2 x 2): `fisher_exact_test`

Fisher's exact test of association in a 2 x 2 table, with the sample odds ratio.

$$
P(a) = \frac{\binom{a+b}{a}\binom{c+d}{c}}{\binom{n}{a+c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `table` | in | 2 x 2 table of counts [[a, b], [c, d]] | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `p_value` | out | exact p-value (two-sided: tables no more likely than the observed one) | - |
| `odds_ratio` | out | ad/(bc); empty when b c = 0 | - |

**Applies when:** fixed margins (conditional test); whole-number counts.

```shell
$ statistics solve fisher_exact_test "table=[[8, 2], [1, 5]]" alternative=two-sided
p_value = 0.03496503497
odds_ratio = 20
  by: Fisher's exact test (2 x 2)  (hypergeometric probabilities of all tables with the observed margins, in exact integer arithmetic)
  alternative = two-sided
  assumes: fixed margins (conditional test); whole-number counts
```

**What the worked example means:** This calculation determines the probability of observing a table as extreme or more extreme than the one provided, assuming no association between the variables. The result yields an exact p-value of 0.034965 and a sample odds ratio of 20. A low p-value suggests that the observed distribution of counts across the categories is unlikely to have occurred by chance alone under the assumption of independence.

<a id="mcnemar_test"></a>

#### McNemar's test for paired proportions: `mcnemar_test`

McNemar's test for a 2 x 2 table of paired outcomes: continuity-corrected chi-square and the exact binomial p-value.

$$
\chi^2 = \frac{(|b - c| - 1)^2}{b + c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `table` | in | 2 x 2 table of paired counts [[both yes, yes/no], [no/yes, both no]] | - |
| `chi_square` | out | (\|b - c\| - 1)^2 / (b + c) | - |
| `p_value` | out | chi-square p-value (1 df) | - |
| `exact_p_value` | out | exact binomial p-value | - |
| `discordant` | out | b + c | - |

**Applies when:** paired (matched) binary outcomes; b + c > 0.

```shell
$ statistics solve mcnemar_test "table=[[30, 12], [5, 53]]"
chi_square = 2.117647059
p_value = 0.1456100954
exact_p_value = 0.1434631348
discordant = 17
  by: McNemar's test for paired proportions  (only the discordant cells b and c carry information)
  assumes: paired (matched) binary outcomes; b + c > 0
```

**What the worked example means:** McNemar's test evaluates whether the proportion of binary outcomes differs between two related groups, such as before and after treatment. The calculation relies solely on the discordant pairs, where the outcome changed from yes to no or no to yes. Based on these counts, the continuity-corrected chi-square value is computed as 2.11765. The resulting p-value is 0.14561, which measures the probability of observing such an imbalance by chance if there were no true difference. An exact binomial p-value of 0.143463 is also provided for greater accuracy when sample sizes are small.

### Confidence intervals (7)

<a id="ci_mean_known_sigma"></a>

#### Confidence interval for a mean (sigma known, z-interval), `ci_mean_known_sigma`

The z confidence interval for a population mean when the population standard deviation sigma is known.

$$
\bar{x} \pm z_{\alpha/2}\frac{\sigma}{\sqrt{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `xbar` | in | sample mean | - |
| `sigma` | in | population standard deviation (known) | - |
| `n` | in | sample size | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |
| `margin_of_error` | out | z sigma / sqrt(n) | - |
| `critical_value` | out | z critical value | - |

**Applies when:** sigma known; normal population or large n; random sample.

```shell
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

**What the worked example means:** This calculation determines a range of plausible values for the population mean using the sample mean and known standard deviation. The interval spans from 49.3867 to 54.6133, centered on the observed sample mean of 52. The width of this interval is defined by the margin of error, which equals 2.61329. This value represents the maximum expected difference between the true population mean and the sample estimate at the specified confidence level.

<a id="ci_mean_t"></a>

#### Confidence interval for a mean from data (t-interval), `ci_mean_t`

The t confidence interval for a population mean from sample data (sigma unknown).

$$
\bar{x} \pm t_{\alpha/2,\,n-1}\frac{s}{\sqrt{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |
| `mean` | out | sample mean | - |
| `margin_of_error` | out | t s / sqrt(n) | - |
| `critical_value` | out | t critical value | - |
| `df` | out | n - 1 | - |

**Applies when:** random sample from a roughly normal population (or large n); sigma unknown.

```shell
$ statistics solve ci_mean_t "x=[12, 15, 11, 18, 14, 16]" confidence=0.95
lower = 11.62370216
upper = 17.0429645
margin_of_error = 2.709631171
critical_value = 2.570581836
df = 5
mean = 14.33333333
  by: Confidence interval for a mean from data (t-interval)  (mean +/- t_(alpha/2, n-1) s / sqrt(n))
  confidence = 0.95
  assumes: random sample from a roughly normal population (or large n); sigma unknown
```

**What the worked example means:** This calculation determines a confidence interval for the population mean using sample data when the standard deviation is unknown. The result, ranging from 11.6237 to 17.043, is derived from the sample mean of 14.3333 plus or minus a margin of error of 2.70963. This margin depends on the critical value 2.57058 and degrees of freedom 5, reflecting the uncertainty inherent in estimating the spread from the sample itself.

<a id="ci_mean_t_summary"></a>

#### Confidence interval for a mean from summary statistics (t-interval), `ci_mean_t_summary`

The t confidence interval for a mean from the sample mean, sample standard deviation and n.

$$
\bar{x} \pm t_{\alpha/2,\,n-1}\frac{s}{\sqrt{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `xbar` | in | sample mean | - |
| `s` | in | sample standard deviation | - |
| `n` | in | sample size (>= 2) | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |
| `margin_of_error` | out | t s / sqrt(n) | - |
| `critical_value` | out | t critical value | - |
| `df` | out | n - 1 | - |

**Applies when:** random sample from a roughly normal population (or large n); sigma unknown.

```shell
$ statistics solve ci_mean_t_summary xbar=25 s=4 n=16 confidence=0.95
lower = 22.86855045
upper = 27.13144955
margin_of_error = 2.131449546
critical_value = 2.131449546
df = 15
  by: Confidence interval for a mean from summary statistics (t-interval)  (xbar +/- t_(alpha/2, n-1) s / sqrt(n))
  xbar = 25
  s = 4
  confidence = 0.95
  assumes: random sample from a roughly normal population (or large n); sigma unknown
```

**What the worked example means:** This calculation produces a t-confidence interval for the population mean using the sample mean, sample standard deviation, and sample size. The resulting range is from 22.8686 to 27.1314. This interval represents the plausible values for the true population mean based on the provided summary statistics and the specified confidence level.

<a id="ci_proportion"></a>

#### Confidence interval for a proportion (Wald), `ci_proportion`

The large-sample (Wald) confidence interval for a population proportion from x successes in n trials.

$$
\hat{p} \pm z_{\alpha/2}\sqrt{\frac{\hat{p}(1-\hat{p})}{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | number of successes | - |
| `n` | in | sample size | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `proportion` | out | sample proportion x/n | - |
| `lower` | out | lower limit (clipped at 0) | - |
| `upper` | out | upper limit (clipped at 1) | - |
| `margin_of_error` | out | z sqrt(p(1-p)/n) | - |

**Applies when:** 0 < x < n; large sample: n p and n(1 - p) at least about 10; limits clipped to [0, 1].

```shell
$ statistics solve ci_proportion x=48 n=120 confidence=0.95
proportion = 0.4
lower = 0.3123477459
upper = 0.4876522541
margin_of_error = 0.08765225406
  by: Confidence interval for a proportion (Wald)  (p +/- z_(alpha/2) sqrt(p(1 - p)/n))
  confidence = 0.95
  assumes: 0 < x < n; large sample: n p and n(1 - p) at least about 10; limits clipped to [0, 1]
```

**What the worked example means:** This calculation determines the large-sample Wald confidence interval for a population proportion using the observed number of successes and the total sample size. The method computes the sample proportion, which is 0.4, and then applies the normal approximation to find the margin of error, calculated as 0.0876523. Adding and subtracting this margin from the sample proportion gives the lower limit of 0.312348 and the upper limit of 0.487652. These limits define the range within which the true population proportion is expected to lie with the specified level of confidence, assuming the sample is sufficiently large and independent.

<a id="ci_variance"></a>

#### Confidence interval for a variance and standard deviation, `ci_variance`

The chi-square confidence interval for a population variance (and standard deviation) from sample data.

$$
\left[\frac{(n-1)s^2}{\chi^2_{1-\alpha/2,\,n-1}},\ \frac{(n-1)s^2}{\chi^2_{\alpha/2,\,n-1}}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `variance` | out | sample variance | - |
| `lower` | out | lower limit for the variance | - |
| `upper` | out | upper limit for the variance | - |
| `sd_lower` | out | lower limit for the standard deviation | - |
| `sd_upper` | out | upper limit for the standard deviation | - |

**Applies when:** normal population (the interval is sensitive to non-normality).

```shell
$ statistics solve ci_variance "x=[12, 15, 11, 18, 14, 16]" confidence=0.95
variance = 6.666666667
lower = 2.597570867
upper = 40.10210251
sd_lower = 1.611698132
sd_upper = 6.332622088
  by: Confidence interval for a variance and standard deviation  ([(n-1)s^2 / chi2_(1-alpha/2), (n-1)s^2 / chi2_(alpha/2)])
  confidence = 0.95
  assumes: normal population (the interval is sensitive to non-normality)
```

**What the worked example means:** This calculation determines the plausible range for the true population variance and standard deviation based on sample data. Using the sample size and observed variability, the bounds are 2.59757 to 40.1021 for the variance and 1.6117 to 6.33262 for the standard deviation. This interval reflects the uncertainty about the spread of the underlying process, assuming the data follows a normal distribution.

<a id="ci_two_means"></a>

#### Confidence interval for the difference of two means, `ci_two_means`

The t confidence interval for mu1 - mu2 from two independent samples (Welch or pooled variances).

$$
(\bar{x}_1 - \bar{x}_2) \pm t_{\alpha/2,\,\nu}\sqrt{\frac{s_1^2}{n_1} + \frac{s_2^2}{n_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `variance` | in | welch (unequal variances) or pooled (equal variances) | - |
| `difference` | out | mean(x) - mean(y) | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |
| `df` | out | degrees of freedom (Welch-Satterthwaite or n1 + n2 - 2) | - |
| `standard_error` | out | standard error of the difference | - |

**Applies when:** independent random samples; roughly normal populations or large samples.

```shell
$ statistics solve ci_two_means "x=[12, 15, 11, 18, 14]" "y=[10, 9, 13, 11, 8]" confidence=0.95 variance=welch
difference = 3.8
lower = 0.278295486
upper = 7.321704514
df = 7.174149271
standard_error = 1.496662955
  by: Confidence interval for the difference of two means  (difference +/- t_(alpha/2, df) SE, with Welch's SE and df or the pooled variance)
  confidence = 0.95
  variance = welch
  assumes: independent random samples; roughly normal populations or large samples
```

**What the worked example means:** This calculation determines the range within which the true difference between two population means likely falls. Using the sample data, it computes the observed mean difference as 3.8 and constructs an interval from 0.278295 to 7.3217. The width of this interval depends on the standard error 1.49666 and the critical value based on degrees of freedom 7.17415, reflecting the uncertainty inherent in estimating population parameters from samples.

<a id="wilson_interval"></a>

#### Wilson score interval for a proportion: `wilson_interval`

The Wilson score confidence interval for a proportion (good coverage for small n and for p near 0 or 1).

$$
\frac{\hat{p} + \frac{z^2}{2n} \pm z\sqrt{\frac{\hat{p}(1-\hat{p})}{n} + \frac{z^2}{4n^2}}}{1 + \frac{z^2}{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | number of successes | - |
| `n` | in | sample size | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `proportion` | out | x/n | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |
| `center` | out | interval centre | - |

**Applies when:** 0 <= x <= n; independent trials.

```shell
$ statistics solve wilson_interval x=3 n=20 confidence=0.95
proportion = 0.15
lower = 0.0523687459
upper = 0.3604188647
center = 0.2063938053
  by: Wilson score interval for a proportion  (invert the score test: centre (p + z^2/2n)/(1 + z^2/n), half-width z sqrt(p(1-p)/n + z^2/4n^2)/(1 + z^2/n))
  confidence = 0.95
  assumes: 0 <= x <= n; independent trials
```

**What the worked example means:** This calculation determines the Wilson score confidence interval for a binomial proportion based on 3 successes out of 20 trials at a confidence level of 0.95. The resulting point estimate for the proportion is 0.15. Unlike simpler methods, this interval centers around 0.206394 and extends from 0.0523687 to 0.360419, providing robust coverage especially when sample sizes are small or proportions are near zero or one.

### Confidence intervals and tests (4)

<a id="chi_square_critical_value"></a>

#### Critical value of chi-square: `chi_square_critical_value`

The right-tail chi-square critical value (P(X > c) = alpha) for given degrees of freedom, and the left-tail value (P(X < c) = alpha).

$$
\chi^2_{\alpha,\nu}:\ P(\chi^2_\nu > c) = \alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `df` | in | degrees of freedom (> 0) | - |
| `alpha` | in | tail area | - |
| `critical_value` | out | right-tail critical value | - |
| `left_critical_value` | out | left-tail critical value | - |

**Applies when:** df > 0; 0 < alpha < 1.

```shell
$ statistics solve chi_square_critical_value df=4 alpha=0.05
critical_value = 9.487729037
left_critical_value = 0.7107230214
  by: Critical value of chi-square  (chi-square quantiles by bisection on the regularized incomplete gamma function)
  df = 4
  alpha = 0.05
  assumes: df > 0; 0 < alpha < 1
```

**What the worked example means:** This calculation determines the threshold values for the chi-square distribution based on specified degrees of freedom and a tail area probability. The right-tail critical value, 9.48773, marks the boundary where the probability of observing a statistic greater than this value equals 0.05. Conversely, the left-tail critical value, 0.710723, indicates the point below which the cumulative probability is exactly 0.05. These values are essential for defining rejection regions in hypothesis testing, allowing analysts to distinguish between common variations and statistically significant deviations from expected patterns.

<a id="f_critical_value"></a>

#### Critical value of F: `f_critical_value`

The right-tail F critical value (P(F > c) = alpha) for numerator and denominator degrees of freedom.

$$
F_{\alpha;\,\nu_1,\nu_2}:\ P(F_{\nu_1,\nu_2} > c) = \alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `df1` | in | numerator degrees of freedom | - |
| `df2` | in | denominator degrees of freedom | - |
| `alpha` | in | tail area | - |
| `critical_value` | out | right-tail F critical value | - |

**Applies when:** df1, df2 > 0; 0 < alpha < 1.

```shell
$ statistics solve f_critical_value df1=3 df2=20 alpha=0.05
critical_value = 3.098391212
  by: Critical value of F  (F quantile by bisection on the regularized incomplete beta function)
  df1 = 3
  df2 = 20
  alpha = 0.05
  assumes: df1, df2 > 0; 0 < alpha < 1
```

**What the worked example means:** This calculation determines the right-tail cutoff point for the F distribution, which serves as a threshold for statistical significance in variance comparisons or analysis of variance tests. Given the numerator degrees of freedom 3, the denominator degrees of freedom 20, and the tail area 0.05, the method computes the specific value 3.09839 such that the probability of observing an F statistic greater than this value is exactly 0.05. This result indicates how extreme an observed ratio of variances must be to reject the null hypothesis at the specified confidence level.

<a id="t_critical_value"></a>

#### Critical value of Student's t: `t_critical_value`

The Student t critical value for given degrees of freedom and significance (or confidence) level.

$$
t_{\alpha/2,\,\nu}:\ P(T_\nu > t) = \alpha/2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `df` | in | degrees of freedom (> 0) | - |
| `alpha` | in | significance level (1 - confidence) | - |
| `tails` | in | two-tailed, right-tailed or left-tailed | - |
| `critical_value` | out | t critical value (negative for a left-tailed test) | - |

**Applies when:** df > 0; 0 < alpha < 1; a confidence level is read as two-tailed.

```shell
$ statistics solve t_critical_value df=10 alpha=0.05 tails=two-tailed
critical_value = 2.228138852
  by: Critical value of Student's t  (t quantile by bisection on the t distribution (regularized incomplete beta function))
  df = 10
  alpha = 0.05
  tails = two-tailed
  assumes: df > 0; 0 < alpha < 1; a confidence level is read as two-tailed
```

**What the worked example means:** This calculation determines the cutoff value from Student's t-distribution, which is essential for hypothesis testing and confidence intervals when the population standard deviation is unknown. By using 10 degrees of freedom and a significance level of 0.05, the method identifies the point where the probability of observing a more extreme result matches the specified tail area. The resulting critical value of 2.22814 defines the boundary for statistical significance, helping researchers decide whether to reject the null hypothesis based on their sample data.

<a id="z_critical_value"></a>

#### Critical value of z: `z_critical_value`

The standard normal critical value for a significance level (or confidence level), two-tailed or one-tailed.

$$
z_{\alpha/2} = \Phi^{-1}(1 - \alpha/2)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | significance level (1 - confidence) | - |
| `tails` | in | two-tailed, right-tailed or left-tailed | - |
| `critical_value` | out | z critical value (negative for a left-tailed test) | - |

**Applies when:** 0 < alpha < 1; a confidence level is read as two-tailed.

```shell
$ statistics solve z_critical_value alpha=0.05 tails=two-tailed
critical_value = 1.959963985
  by: Critical value of z  (standard normal quantile at 1 - alpha/2 (two-tailed) or 1 - alpha (one-tailed))
  alpha = 0.05
  tails = two-tailed
  assumes: 0 < alpha < 1; a confidence level is read as two-tailed
```

**What the worked example means:** This calculation determines the boundary point on the standard normal distribution corresponding to a significance level of 0.05. The resulting critical value, 1.95996, defines the cutoff for rejection regions in hypothesis testing or confidence intervals. For a two-tailed test, this value represents the distance from the mean that captures the central portion of the probability mass associated with the chosen confidence level.

### Descriptive statistics (22)

<a id="mean"></a>

#### Arithmetic mean: `mean`

The arithmetic mean (average) of a list of numbers.

$$
\bar{x} = \frac{1}{n}\sum_{i=1}^{n} x_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `mean` | out | arithmetic mean | - |
| `n` | out | number of values | - |
| `sum` | out | sum of the values | - |

```shell
$ statistics solve mean "x=[2, 4, 9]"
mean = 5
n = 3
sum = 15
  by: Arithmetic mean  (sum of the values divided by their number (compensated summation))
```

**What the worked example means:** This calculation determines the arithmetic mean, which serves as the central balance point for the dataset. It is derived by taking the total sum of all observations, represented here as 15, and dividing it by the count of those observations, denoted as 3. The resulting value, 5, provides a single representative magnitude that summarizes the typical size of the numbers in the list.

<a id="coefficient_of_variation"></a>

#### Coefficient of variation: `coefficient_of_variation`

The coefficient of variation (sample standard deviation divided by the mean), as a ratio and a percentage.

$$
CV = \frac{s}{\bar{x}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `cv` | out | s / mean | - |
| `cv_percent` | out | 100 s / mean | - |
| `mean` | out | mean | - |
| `standard_deviation` | out | sample standard deviation | - |

**Applies when:** positive mean; sample standard deviation (n - 1).

```shell
$ statistics solve coefficient_of_variation "x=[10, 12, 14, 16, 18]"
cv = 0.2258769757
cv_percent = 22.58769757
mean = 14
standard_deviation = 3.16227766
  by: Coefficient of variation  (CV = s / mean)
  assumes: positive mean; sample standard deviation (n - 1)
```

**What the worked example means:** This calculation determines the coefficient of variation by dividing the sample standard deviation, which is 3.16228, by the arithmetic mean, which is 14. The resulting ratio is 0.225877, equivalent to 22.5877 when expressed as a percentage. This metric quantifies relative variability, allowing for the comparison of dispersion between datasets that may have different units or significantly different average values.

<a id="five_number_summary"></a>

#### Five-number summary and interquartile range, `five_number_summary`

Minimum, first quartile, median, third quartile, maximum and the interquartile range of a data set.

$$
Q_p = x_{(\lfloor h \rfloor)} + (h - \lfloor h \rfloor)\,(x_{(\lfloor h \rfloor + 1)} - x_{(\lfloor h \rfloor)}),\ h = 1 + (n-1)p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `method` | in | quantile method: linear (type 7, spreadsheet/numpy default) or weibull (type 6, p(n+1)) (optional) | - |
| `minimum` | out | smallest value | - |
| `q1` | out | first quartile | - |
| `median` | out | median | - |
| `q3` | out | third quartile | - |
| `maximum` | out | largest value | - |
| `iqr` | out | interquartile range Q3 - Q1 | - |

**Applies when:** quartiles by linear interpolation between order statistics (Hyndman-Fan type 7, as in spreadsheets and numpy) unless method = weibull (type 6, as in Minitab and SPSS); textbooks that take medians of the halves can differ.

```shell
$ statistics solve five_number_summary "x=[1, 3, 5, 7, 9, 11, 13, 15]"
minimum = 1
q1 = 4.5
median = 8
q3 = 11.5
maximum = 15
iqr = 7
  by: Five-number summary and interquartile range  (sort; quartile at position 1 + (n - 1)p (linear) or p(n + 1) (weibull), interpolating between neighbours)
  assumes: quartiles by linear interpolation between order statistics (Hyndman-Fan type 7, as in spreadsheets and numpy) unless method = weibull (type 6, as in Minitab and SPSS); textbooks that take medians of the halves can differ
```

**What the worked example means:** This calculation determines the five-number summary for the provided data set. The values are derived by sorting the input numbers and applying a specific quantile interpolation method to find the first quartile, median, and third quartile, alongside the minimum and maximum. The result shows that the smallest value is 1, the lower boundary of the central half is 4.5, the middle value is 8, the upper boundary is 11.5, and the largest value is 15. The spread of the middle fifty percent of the data, known as the interquartile range, is calculated as 7. This summary provides a robust view of the distribution's center and variability, minimizing the influence of extreme outliers.

<a id="geometric_mean"></a>

#### Geometric mean: `geometric_mean`

The geometric mean of positive numbers (average growth factors, ratios).

$$
G = \left(\prod x_i\right)^{1/n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | positive values | - |
| `geometric_mean` | out | n-th root of the product | - |

**Applies when:** all values positive.

```shell
$ statistics solve geometric_mean "x=[2, 8]"
geometric_mean = 4
  by: Geometric mean  (exp of the mean of the logarithms)
  assumes: all values positive
```

**What the worked example means:** This calculation determines the geometric mean, which serves as the average for data that grows or changes multiplicatively rather than additively. Taking the n-th root of the product of all input values gives 4. This result indicates the constant factor that, if applied repeatedly over the same number of periods, would yield the same final value as the actual sequence of varying factors. It is particularly useful for understanding average growth rates or ratios where the arithmetic mean might be misleading due to skewness.

<a id="harmonic_mean"></a>

#### Harmonic mean: `harmonic_mean`

The harmonic mean of positive numbers (average speeds over equal distances, average rates).

$$
H = \frac{n}{\sum 1/x_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | positive values | - |
| `harmonic_mean` | out | n divided by the sum of reciprocals | - |

**Applies when:** all values positive.

```shell
$ statistics solve harmonic_mean "x=[40, 60]"
harmonic_mean = 48
  by: Harmonic mean  (n / sum(1/x))
  assumes: all values positive
```

**What the worked example means:** This calculation determines the harmonic mean, which is appropriate for averaging rates or speeds over equal distances. It takes the number of items and divides it by the sum of their reciprocals, yielding a result of 48. Unlike the arithmetic mean, this method gives less weight to larger values, ensuring that slow rates have a stronger impact on the average than fast ones do. The input values must all be positive for this metric to be valid.

<a id="mean_absolute_deviation"></a>

#### Mean absolute deviation: `mean_absolute_deviation`

The mean absolute deviation about the mean of a list of numbers.

$$
MAD = \frac{1}{n}\sum |x_i - \bar{x}|
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `mean_absolute_deviation` | out | average distance from the mean | - |
| `mean` | out | mean | - |

```shell
$ statistics solve mean_absolute_deviation "x=[2, 4, 6, 8]"
mean_absolute_deviation = 2
mean = 5
  by: Mean absolute deviation  (mean of |x - mean|)
```

**What the worked example means:** This calculation determines the mean absolute deviation by taking the arithmetic mean of the dataset, denoted as 5, and then averaging the absolute differences between each individual value in [2, 4, 6, 8] and that central mean. The resulting figure, 2, quantifies the typical spread or dispersion of the data points around their average, offering a straightforward measure of variability that is easier to interpret than variance because it remains in the original units of the data.

<a id="frequency_table_statistics"></a>

#### Mean and standard deviation of a frequency table, `frequency_table_statistics`

The mean and the sample and population standard deviations of data given as values with frequencies.

$$
\bar{x} = \frac{\sum f_i x_i}{\sum f_i},\quad s = \sqrt{\frac{\sum f_i (x_i - \bar{x})^2}{n - 1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the distinct values | - |
| `f` | in | their frequencies (whole numbers) | - |
| `mean` | out | sum(f x) / n | - |
| `sample_standard_deviation` | out | divisor n - 1 | - |
| `population_standard_deviation` | out | divisor n | - |
| `n` | out | total frequency | - |

**Applies when:** frequencies are whole numbers; total frequency >= 2.

```shell
$ statistics solve frequency_table_statistics "x=[1, 2, 3, 4]" "f=[3, 5, 8, 4]"
mean = 2.65
sample_standard_deviation = 0.9880869342
population_standard_deviation = 0.9630680142
n = 20
  by: Mean and standard deviation of a frequency table  (weighted sums: mean = sum(f x)/n, then sum(f (x - mean)^2) over n - 1 or n)
  assumes: frequencies are whole numbers; total frequency >= 2
```

**What the worked example means:** This calculation determines the central tendency and dispersion for data presented as distinct values with their corresponding frequencies. By computing weighted sums, it derives the mean value 2.65 and the sample standard deviation 0.988087, which measures how spread out the values are around the mean. The population standard deviation 0.963068 is also provided for cases where the data represents an entire group rather than a sample. These results rely on the total count of observations, denoted as 20, ensuring that the statistical measures accurately reflect the distribution of the grouped data without requiring individual entry of every observation.

<a id="median"></a>

#### Median: `median`

The median (middle value) of a list of numbers; the mean of the two middle values when n is even.

$$
\tilde{x} = x_{((n+1)/2)} \ \text{or}\ \tfrac12\left(x_{(n/2)} + x_{(n/2+1)}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `median` | out | median | - |
| `n` | out | number of values | - |

```shell
$ statistics solve median "x=[7, 1, 5, 3, 9]"
median = 5
n = 5
  by: Median  (sort the values; take the middle one (odd n) or the mean of the two middle ones (even n))
```

**What the worked example means:** This calculation determines the median, or central value, of the provided dataset. Sorting the 5 numeric values and selecting the middle one gives a result of 5. This measure serves as a robust indicator of typicality because it remains stable even when extreme outliers are present, unlike the mean which can be heavily skewed by such values.

<a id="median_absolute_deviation"></a>

#### Median absolute deviation: `median_absolute_deviation`

The median absolute deviation (MAD) about the median, a robust measure of spread (unscaled).

$$
MAD = \operatorname{median}\left(|x_i - \tilde{x}|\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `median_absolute_deviation` | out | median of \|x - median\| | - |
| `median` | out | median | - |

**Applies when:** unscaled (multiply by 1.4826 to estimate a normal sigma).

```shell
$ statistics solve median_absolute_deviation "x=[1, 1, 2, 2, 4, 6, 9]"
median_absolute_deviation = 1
median = 2
  by: Median absolute deviation  (median of the absolute deviations from the median)
  assumes: unscaled (multiply by 1.4826 to estimate a normal sigma)
```

**What the worked example means:** This calculation determines the median absolute deviation, which measures the spread of the dataset using the median as the central reference point. Taking the absolute differences between each data value and the median, then finding the median of those differences gives a robust estimate of variability equal to 1. Unlike standard deviation, this metric is less sensitive to extreme outliers, providing a more reliable sense of typical dispersion for skewed or heavy-tailed distributions.

<a id="mode"></a>

#### Mode: `mode`

The mode(s) of a list of numbers: every value that occurs most often (no mode when all values occur equally often).

$$
\operatorname{mode} = \arg\max_v \#\{i : x_i = v\}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `modes` | out | the most frequent value(s), ascending; empty if there is no mode | - |
| `frequency` | out | how often the mode occurs | - |
| `has_mode` | out | yes or no | - |

**Applies when:** no mode is reported when every distinct value occurs equally often.

```shell
$ statistics solve mode "x=[3, 7, 7, 2, 9, 7, 3]"
modes = [7.0]
frequency = 3
has_mode = yes
  by: Mode  (count each value; report all values with the largest count)
  assumes: no mode is reported when every distinct value occurs equally often
```

**What the worked example means:** This calculation identifies the most frequently occurring value(s) in the dataset. Counting how often each unique number appears shows which value or values have the highest frequency. The result shows that [7] appear 3 times, making them the modes of this distribution. If every distinct value appeared the same number of times, no mode would be reported.

<a id="iqr_outliers"></a>

#### Outliers by the 1.5 IQR rule: `iqr_outliers`

The lower and upper fences Q1 - 1.5 IQR and Q3 + 1.5 IQR and the values outside them.

$$
[Q_1 - 1.5\,IQR,\ Q_3 + 1.5\,IQR]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least four) | - |
| `lower_fence` | out | Q1 - 1.5 IQR | - |
| `upper_fence` | out | Q3 + 1.5 IQR | - |
| `outliers` | out | values outside the fences | - |
| `q1` | out | first quartile | - |
| `q3` | out | third quartile | - |
| `iqr` | out | interquartile range | - |

**Applies when:** quartiles by linear interpolation (type 7).

```shell
$ statistics solve iqr_outliers "x=[10, 12, 12, 13, 12, 11, 14, 13, 15, 102]"
lower_fence = 9.375
upper_fence = 16.375
outliers = [102.0]
q1 = 12
q3 = 13.75
iqr = 1.75
  by: Outliers by the 1.5 IQR rule  (Tukey's fences from the quartiles)
  assumes: quartiles by linear interpolation (type 7)
```

**What the worked example means:** This calculation determines statistical boundaries using Tukey's fences based on the interquartile range. The first quartile is 12 and the third quartile is 13.75, yielding an interquartile range of 1.75. Applying the one point five times IQR rule, the lower boundary is computed as 9.375 and the upper boundary as 16.375. Any data points falling outside these limits are identified as potential anomalies, specifically those listed in [102]. This method provides a robust measure of spread that is less sensitive to extreme values than standard deviation.

<a id="percentile"></a>

#### Percentile of a data set: `percentile`

The p-th percentile of a data set (linear interpolation, or the weibull/p(n+1) method).

$$
P_p = x_{(\lfloor h \rfloor)} + (h - \lfloor h \rfloor)(x_{(\lfloor h \rfloor+1)} - x_{(\lfloor h \rfloor)})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `p` | in | the percentile, 0 to 100 | - |
| `method` | in | quantile method: linear (type 7, spreadsheet/numpy default) or weibull (type 6, p(n+1)) (optional) | - |
| `percentile` | out | the p-th percentile | - |

**Applies when:** 0 <= p <= 100; linear interpolation (type 7) unless method = weibull (type 6).

```shell
$ statistics solve percentile "x=[15, 20, 35, 40, 50]" p=90
percentile = 46
  by: Percentile of a data set  (sort; position h = (n - 1)p/100 (0-based, linear) or (n + 1)p/100 - 1 (weibull); interpolate)
  p = 90
  assumes: 0 <= p <= 100; linear interpolation (type 7) unless method = weibull (type 6)
```

**What the worked example means:** This calculation determines the specific percentile rank within a sorted dataset. By applying linear interpolation between adjacent data values, the method estimates the precise point corresponding to the requested percentage. The result, 46, represents the threshold where 90 percent of the observations are equal to or less than this value, providing a standardized measure of relative position within the distribution.

<a id="population_standard_deviation"></a>

#### Population standard deviation: `population_standard_deviation`

The population standard deviation of a list of numbers (divisor n).

$$
\sigma = \sqrt{\frac{1}{n}\sum (x_i - \mu)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `standard_deviation` | out | population standard deviation | - |
| `mean` | out | arithmetic mean | - |
| `n` | out | number of values | - |
| `variance` | out | the companion measure | - |

**Applies when:** the data are the whole population (divisor n).

```shell
$ statistics solve population_standard_deviation "x=[2, 4, 4, 4, 5, 5, 7, 9]"
mean = 5
n = 8
standard_deviation = 2
variance = 4
  by: Population standard deviation  (two-pass: mean first, then the sum of squared deviations divided by n)
  assumes: the data are the whole population (divisor n)
```

**What the worked example means:** This calculation determines the population standard deviation, which represents the typical distance between data points and the arithmetic mean for an entire set of observations. By taking the square root of the variance, the result converts the measure of spread back into the original units of the data, making it easier to interpret than the squared units of the variance alone. The computed value of 2 reflects how much individual measurements deviate from the central tendency of 5 across the 8 items in the population.

<a id="population_variance"></a>

#### Population variance: `population_variance`

The population variance of a list of numbers (divisor n).

$$
\sigma^2 = \frac{1}{n}\sum (x_i - \mu)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `variance` | out | population variance | - |
| `mean` | out | arithmetic mean | - |
| `n` | out | number of values | - |
| `standard_deviation` | out | the companion measure | - |

**Applies when:** the data are the whole population (divisor n).

```shell
$ statistics solve population_variance "x=[2, 4, 4, 4, 5, 5, 7, 9]"
mean = 5
n = 8
variance = 4
standard_deviation = 2
  by: Population variance  (two-pass: mean first, then the sum of squared deviations divided by n)
  assumes: the data are the whole population (divisor n)
```

**What the worked example means:** This calculation determines the population variance by first finding the arithmetic mean of the data values, which resulted in 5. It then computes the average of the squared differences between each value and this mean. The final variance is 4, indicating how spread out the entire set of 8 numerical values is around their center.

<a id="range"></a>

#### Range: `range`

The range (largest minus smallest value) of a list of numbers.

$$
R = x_{\max} - x_{\min}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `range` | out | maximum minus minimum | - |
| `minimum` | out | smallest value | - |
| `maximum` | out | largest value | - |

```shell
$ statistics solve range "x=[12, 4, 19, 7]"
range = 15
minimum = 4
maximum = 19
  by: Range  (maximum minus minimum)
```

**What the worked example means:** This calculation determines the statistical spread of the data by subtracting the smallest observed value from the largest. The resulting range of 15 indicates the total distance covered by the dataset, derived from a maximum of 19 and a minimum of 4. This metric offers an immediate understanding of the extent of variation within the provided numerical inputs.

<a id="sample_standard_deviation"></a>

#### Sample standard deviation: `sample_standard_deviation`

The sample standard deviation of a list of numbers (divisor n - 1).

$$
s = \sqrt{\frac{1}{n-1}\sum (x_i - \bar{x})^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `standard_deviation` | out | sample standard deviation | - |
| `mean` | out | arithmetic mean | - |
| `n` | out | number of values | - |
| `variance` | out | the companion measure | - |

**Applies when:** the data are a sample (divisor n - 1, Bessel's correction); at least two values.

```shell
$ statistics solve sample_standard_deviation "x=[2, 4, 4, 4, 5, 5, 7, 9]"
mean = 5
n = 8
standard_deviation = 2.138089935
variance = 4.571428571
  by: Sample standard deviation  (two-pass: mean first, then the sum of squared deviations divided by n - 1)
  assumes: the data are a sample (divisor n - 1, Bessel's correction); at least two values
```

**What the worked example means:** This calculation determines the sample standard deviation using a two-pass method where the arithmetic mean is computed first, followed by the sum of squared deviations divided by the degrees of freedom. The result, 2.13809, represents the typical distance between each data point and the calculated mean of 5. This measure quantifies the variability within the subset of data consisting of 8 observations, providing an estimate of how much individual values deviate from the central tendency.

<a id="sample_variance"></a>

#### Sample variance: `sample_variance`

The sample variance of a list of numbers (divisor n - 1).

$$
s^2 = \frac{1}{n-1}\sum (x_i - \bar{x})^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `variance` | out | sample variance | - |
| `mean` | out | arithmetic mean | - |
| `n` | out | number of values | - |
| `standard_deviation` | out | the companion measure | - |

**Applies when:** the data are a sample (divisor n - 1, Bessel's correction); at least two values.

```shell
$ statistics solve sample_variance "x=[2, 4, 4, 4, 5, 5, 7, 9]"
mean = 5
n = 8
variance = 4.571428571
standard_deviation = 2.138089935
  by: Sample variance  (two-pass: mean first, then the sum of squared deviations divided by n - 1)
  assumes: the data are a sample (divisor n - 1, Bessel's correction); at least two values
```

**What the worked example means:** This calculation determines the sample variance, which measures how spread out the data points are from their average. First, the arithmetic mean 5 is found using all 8 input values. Then, the squared differences between each value and this mean are summed and divided by one less than the count of values. The resulting variance 4.57143 provides an unbiased estimate of the population's variability based on the provided sample.

<a id="skewness_kurtosis"></a>

#### Skewness and kurtosis: `skewness_kurtosis`

Sample skewness and excess kurtosis: the adjusted Fisher-Pearson estimates (G1, G2, as in spreadsheets and most software) and the moment estimates (g1, g2).

$$
G_1 = \frac{\sqrt{n(n-1)}}{n-2}\,\frac{m_3}{m_2^{3/2}},\quad G_2 = \frac{(n-1)\left((n+1)g_2 + 6\right)}{(n-2)(n-3)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least four) | - |
| `skewness` | out | adjusted skewness G1 | - |
| `excess_kurtosis` | out | adjusted excess kurtosis G2 | - |
| `skewness_biased` | out | moment skewness g1 | - |
| `excess_kurtosis_biased` | out | moment excess kurtosis g2 | - |

**Applies when:** at least four values, not all equal.

```shell
$ statistics solve skewness_kurtosis "x=[2, 3, 3, 4, 8, 12, 20]"
skewness = 1.399853757
excess_kurtosis = 1.372063741
skewness_biased = 1.080010621
excess_kurtosis_biased = -0.1783067746
  by: Skewness and kurtosis  (central moments m2, m3, m4; g1 = m3/m2^1.5, g2 = m4/m2^2 - 3; small-sample adjustments for G1 and G2)
  assumes: at least four values, not all equal
```

**What the worked example means:** This calculation determines the shape of your data by computing sample skewness and excess kurtosis. Using the central moments derived from the input values, it produces both moment estimates and adjusted Fisher-Pearson statistics. The result shows that the adjusted skewness G1 is 1.39985 and the adjusted excess kurtosis G2 is 1.37206. Positive skewness suggests a longer tail on the right, while positive excess kurtosis indicates heavier tails than a normal distribution.

<a id="standard_error_of_mean"></a>

#### Standard error of the mean: `standard_error_of_mean`

The standard error of the mean of a sample: s / sqrt(n).

$$
SE_{\bar{x}} = \frac{s}{\sqrt{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `standard_error` | out | s / sqrt(n) | - |
| `standard_deviation` | out | sample standard deviation | - |
| `n` | out | sample size | - |

```shell
$ statistics solve standard_error_of_mean "x=[12, 15, 11, 18, 14]"
standard_error = 1.224744871
standard_deviation = 2.738612788
n = 5
  by: Standard error of the mean  (sample standard deviation divided by the square root of n)
```

**What the worked example means:** This calculation determines the standard error of the mean, which estimates how much the sample mean would vary if samples of the same size were repeatedly drawn from the population. It is derived by dividing the sample standard deviation, shown here as 2.73861, by the square root of the number of observations in the sample, which is 5. The resulting value, 1.22474, represents the precision of the sample mean as an estimate of the true population mean; a smaller value indicates that the sample mean is likely closer to the actual population average.

<a id="trimmed_mean"></a>

#### Trimmed mean: `trimmed_mean`

The trimmed mean: the mean after removing the given proportion of values from EACH end.

$$
\bar{x}_{\alpha} = \frac{1}{n - 2k}\sum_{i=k+1}^{n-k} x_{(i)},\ k = \lfloor \alpha n \rfloor
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values | - |
| `proportion` | in | proportion cut from each end (0.1 for a 10% trimmed mean) | - |
| `trimmed_mean` | out | mean of the remaining values | - |
| `removed_each_end` | out | values removed at each end | - |

**Applies when:** 0 <= proportion < 0.5; floor(proportion * n) values removed from each end.

```shell
$ statistics solve trimmed_mean "x=[2, 4, 5, 6, 7, 8, 9, 10, 12, 50]" proportion=0.1
trimmed_mean = 7.625
removed_each_end = 1
  by: Trimmed mean  (sort, drop floor(proportion * n) values at each end, average the rest)
  proportion = 0.1
  assumes: 0 <= proportion < 0.5; floor(proportion * n) values removed from each end
```

**What the worked example means:** This calculation determines the trimmed mean by first sorting the data and removing a specific number of extreme values from both the lowest and highest ends. The proportion of values cut from each end was set to 0.1. Consequently, 1 values were discarded from each side, leaving the remaining central portion of the data to be averaged. The resulting trimmed mean is 7.625, which serves as a robust measure of central tendency less affected by outliers than the standard arithmetic mean.

<a id="weighted_mean"></a>

#### Weighted mean: `weighted_mean`

The weighted mean of values x with weights w (grade-point averages, portfolio averages).

$$
\bar{x}_w = \frac{\sum w_i x_i}{\sum w_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the values | - |
| `w` | in | the weights (non-negative, same length) | - |
| `weighted_mean` | out | sum(w x) / sum(w) | - |
| `total_weight` | out | sum of the weights | - |

**Applies when:** same number of values and weights; weights >= 0 with a positive sum.

```shell
$ statistics solve weighted_mean "x=[90, 80, 70]" "w=[3, 2, 1]"
weighted_mean = 83.33333333
total_weight = 6
  by: Weighted mean  (sum of weight times value divided by the sum of weights)
  assumes: same number of values and weights; weights >= 0 with a positive sum
```

**What the worked example means:** This calculation determines the weighted mean, which represents an average where each value contributes according to its specific weight rather than equally. The result of 83.3333 is obtained by dividing the sum of the products of values and their corresponding weights by the total weight of 6. This method ensures that more significant or frequent data points influence the final average proportionally to their assigned importance.

<a id="z_score"></a>

#### z-score of a value, `z_score`

The z-score (standard score) of a value x in a distribution with mean mu and standard deviation sigma.

$$
z = \frac{x - \mu}{\sigma}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value | - |
| `mu` | in | the mean | - |
| `sigma` | in | the standard deviation (> 0) | - |
| `z` | out | number of standard deviations x lies above (+) or below (-) the mean | - |

**Applies when:** sigma > 0.

```shell
$ statistics solve z_score x=85 mu=70 sigma=10
z = 1.5
  by: z-score of a value  (z = (x - mu) / sigma)
  x = 85
  mu = 70
  sigma = 10
  assumes: sigma > 0
```

**What the worked example means:** This calculation determines how far a specific data point deviates from the average of its distribution, measured in units of standard deviation. Taking the difference between the observed value 85 and the population mean 70, and dividing by the standard deviation 10 gives the standardized score 1.5. A positive result indicates the value is above the mean, while a negative result places it below. This allows for direct comparison across different datasets by normalizing the scale.

### Effect sizes (1)

<a id="cohens_d"></a>

#### Cohen's d and Hedges' g: `cohens_d`

The standardized mean difference of two independent samples: Cohen's d (pooled SD) and Hedges' g (bias-corrected).

$$
d = \frac{\bar{x}_1 - \bar{x}_2}{s_p},\ s_p = \sqrt{\frac{(n_1-1)s_1^2 + (n_2-1)s_2^2}{n_1+n_2-2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `d` | out | (mean x - mean y) / pooled SD | - |
| `hedges_g` | out | d (1 - 3/(4(n1 + n2) - 9)) | - |
| `pooled_sd` | out | pooled standard deviation | - |

**Applies when:** independent samples.

```shell
$ statistics solve cohens_d "x=[12, 15, 11, 18, 14]" "y=[10, 9, 13, 11, 8]"
d = 1.605793084
hedges_g = 1.450393753
pooled_sd = 2.366431913
  by: Cohen's d and Hedges' g  (difference of means over the pooled sample standard deviation)
  assumes: independent samples
```

**What the worked example means:** This calculation determines the standardized mean difference between two independent groups. It computes the pooled standard deviation from the variances and sample sizes of both sets, resulting in a value of 2.36643. The difference between the first sample's mean and the second sample's mean is then divided by this pooled standard deviation to yield Cohen's d, which is 1.60579. A small-sample correction is applied to derive Hedges' g, reported as 1.45039, providing a more accurate estimate of the population effect size when sample sizes are limited.

### Experimental design (4)

<a id="factorial_2x2_effects"></a>

#### 2^2 factorial design: main effects and interaction, `factorial_2x2_effects`

Effects of a two-level 4-run factorial experiment from one response per run in standard (Yates) order ((1), a, b, ab).

$$
\text{Effect} = \frac{\text{contrast}}{2^{k-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `y` | in | 4 responses in standard order | - |
| `grand_mean` | out | average response | - |
| `A` | out | effect A | - |
| `B` | out | effect B | - |
| `AB` | out | effect AB | - |

**Applies when:** responses in standard order; one response (or cell mean) per run.

```shell
$ statistics solve factorial_2x2_effects "y=[28, 36, 18, 31]"
grand_mean = 28.25
A = 10.5
B = -7.5
AB = 2.5
  by: 2^2 factorial design: main effects and interaction  (contrast of the +/- signs divided by 2^(k-1))
  assumes: responses in standard order; one response (or cell mean) per run
```

**What the worked example means:** This calculation determines the average response level and the specific impact of each factor on that response. The grand mean, reported as 28.25, represents the central tendency of all runs. The effect labeled A, calculated as 10.5, measures how much the response changes when factor A moves from its low to high setting, averaged over the levels of factor B. Similarly, effect B, shown as -7.5, quantifies the change due to factor B alone. Finally, the interaction effect AB, given by 2.5, indicates whether the effect of one factor depends on the level of the other. These values are derived from the four input responses using contrast coefficients divided by two.

<a id="factorial_2x2x2_effects"></a>

#### 2^3 factorial design: main effects and interactions, `factorial_2x2x2_effects`

Effects of a two-level 8-run factorial experiment from one response per run in standard (Yates) order ((1), a, b, ab, c, ac, bc, abc).

$$
\text{Effect} = \frac{\text{contrast}}{2^{k-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `y` | in | 8 responses in standard order | - |
| `grand_mean` | out | average response | - |
| `A` | out | effect A | - |
| `B` | out | effect B | - |
| `C` | out | effect C | - |
| `AB` | out | effect AB | - |
| `AC` | out | effect AC | - |
| `BC` | out | effect BC | - |
| `ABC` | out | effect ABC | - |

**Applies when:** responses in standard order; one response (or cell mean) per run.

```shell
$ statistics solve factorial_2x2x2_effects "y=[22, 32, 35, 55, 44, 40, 60, 39]"
grand_mean = 40.875
A = 1.25
B = 12.75
C = 9.75
AB = -1.75
AC = -13.75
BC = -5.25
ABC = -6.75
  by: 2^3 factorial design: main effects and interactions  (contrast of the +/- signs divided by 2^(k-1))
  assumes: responses in standard order; one response (or cell mean) per run
```

**What the worked example means:** This calculation determines the impact of individual variables and their combined interactions in a three-factor experiment. Using eight responses provided in standard order, the grand mean is calculated as 40.875. The main effects for factors A, B, and C are derived as 1.25, 12.75, and 9.75 respectively. Additionally, the two-factor interactions AB, AC, and BC are computed as -1.75, -13.75, and -5.25, with the three-factor interaction ABC resulting in -6.75. These values represent the average change in response when moving from the low level to the high level of each factor or combination.

<a id="cronbach_alpha"></a>

#### Cronbach's alpha (internal consistency): `cronbach_alpha`

Cronbach's alpha for a questionnaire or test: rows are respondents, columns are items.

$$
\alpha = \frac{k}{k-1}\left(1 - \frac{\sum s_i^2}{s_T^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `data` | in | scores: one row per respondent, one column per item | - |
| `alpha` | out | k/(k-1) (1 - sum of item variances / variance of totals) | - |
| `items` | out | k | - |
| `respondents` | out | n | - |

**Applies when:** at least 2 items and 2 respondents.

```shell
$ statistics solve cronbach_alpha "data=[[4, 5, 4], [3, 3, 4], [5, 5, 5], [2, 3, 2], [4, 4, 3]]"
alpha = 0.9130434783
items = 3
respondents = 5
  by: Cronbach's alpha (internal consistency)  (sample variances (n - 1) of each item and of the totals)
  assumes: at least 2 items and 2 respondents
```

**What the worked example means:** Cronbach's alpha measures how closely related a set of items are as a group, indicating the reliability of the scale. For this dataset, which includes 3 items completed by 5 respondents, the computed alpha value is 0.913043. A higher value suggests that the items consistently measure the same underlying construct, while a lower value indicates less coherence among the responses.

<a id="randomized_block_anova"></a>

#### Randomized complete block design (two-way ANOVA without replication), `randomized_block_anova`

Two-way ANOVA for a randomized complete block design: rows are blocks, columns are treatments, one observation per cell; F-tests for treatments and blocks.

$$
F_{\text{trt}} = \frac{SS_{\text{trt}}/(t-1)}{SS_E/((t-1)(b-1))}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `data` | in | table with one row per block and one column per treatment | - |
| `f_treatment` | out | F for treatments | - |
| `p_treatment` | out | p-value for treatments | - |
| `f_block` | out | F for blocks | - |
| `p_block` | out | p-value for blocks | - |
| `ss_treatment` | out | treatment sum of squares | - |
| `ss_block` | out | block sum of squares | - |
| `ss_error` | out | error sum of squares | - |
| `df_treatment` | out | t - 1 | - |
| `df_block` | out | b - 1 | - |
| `df_error` | out | (t - 1)(b - 1) | - |

**Applies when:** one observation per block and treatment; additive model (no block x treatment interaction); normal errors with constant variance.

```shell
$ statistics solve randomized_block_anova "data=[[12, 14, 15], [10, 13, 13], [11, 15, 16], [9, 12, 14]]"
f_treatment = 39
p_treatment = 0.0003644314869
f_block = 9.25
p_block = 0.01143989927
ss_treatment = 34.66666667
ss_block = 12.33333333
ss_error = 2.666666667
df_treatment = 2
df_block = 3
df_error = 6
  by: Randomized complete block design (two-way ANOVA without replication)  (sums of squares for treatments, blocks and the residual y_ij - row mean - column mean + grand mean)
  assumes: one observation per block and treatment; additive model (no block x treatment interaction); normal errors with constant variance
```

**What the worked example means:** This analysis computes the sum of squares for treatments, blocks, and error from the input data table. The treatment F-statistic, calculated as 39, compares the variance explained by treatments to the residual variance. With degrees of freedom 2 for treatments and 6 for error, the resulting p-value of 0.000364431 determines if the observed differences among treatment means are statistically significant after accounting for block effects.

### Hypothesis tests (14)

<a id="bartlett_test"></a>

#### Bartlett's test for equal variances: `bartlett_test`

Bartlett's test of equal variances across 2 to 5 groups (normal populations).

$$
T = \frac{(N-k)\ln s_p^2 - \sum (n_i - 1)\ln s_i^2}{1 + \frac{1}{3(k-1)}\left(\sum \frac{1}{n_i - 1} - \frac{1}{N-k}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | group 1 | - |
| `g2` | in | group 2 | - |
| `g3` | in | group 3 (optional) | - |
| `g4` | in | group 4 (optional) | - |
| `g5` | in | group 5 (optional) | - |
| `statistic` | out | Bartlett's chi-square statistic | - |
| `df` | out | k - 1 | - |
| `p_value` | out | p-value | - |

**Applies when:** normal populations (Bartlett's test is sensitive to non-normality); at least 2 values per group.

```shell
$ statistics solve bartlett_test "g1=[21, 24, 19, 26, 23]" "g2=[22, 23, 22, 24, 23]" "g3=[18, 28, 20, 30, 24]"
statistic = 8.618215556
df = 2
p_value = 0.01344554065
  by: Bartlett's test for equal variances  (log of the pooled variance against the group log variances, with Bartlett's correction)
  assumes: normal populations (Bartlett's test is sensitive to non-normality); at least 2 values per group
```

**What the worked example means:** This calculation determines whether the variances across the provided groups are statistically equal. It computes a chi-square statistic based on the natural logarithm of the pooled variance compared to the individual group variances, applying Bartlett's correction factor for bias. The resulting statistic of 8.61822 and p-value of 0.0134455 indicate the strength of evidence against the null hypothesis that all group variances are identical.

<a id="chi_square_goodness_of_fit"></a>

#### Chi-square goodness-of-fit test: `chi_square_goodness_of_fit`

The chi-square goodness-of-fit test of observed counts against expected proportions (equal if omitted).

$$
\chi^2 = \sum \frac{(O_i - E_i)^2}{E_i},\ \nu = k - 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `observed` | in | observed counts per category | - |
| `proportions` | in | expected proportions or relative weights per category (equal if omitted) (optional) | - |
| `chi_square` | out | sum (O - E)^2 / E | - |
| `df` | out | categories - 1 | - |
| `p_value` | out | upper-tail p-value | - |
| `expected` | out | expected counts | - |

**Applies when:** independent observations; expected counts at least about 5; proportions are rescaled to sum to 1.

```shell
$ statistics solve chi_square_goodness_of_fit "observed=[18, 22, 20, 25, 15]"
chi_square = 2.9
df = 4
p_value = 0.5746972058
expected = [20.0,20.0,20.0,20.0,20.0]
  by: Chi-square goodness-of-fit test  (expected counts n p_i; chi-square statistic with k - 1 df)
  assumes: independent observations; expected counts at least about 5; proportions are rescaled to sum to 1
```

**What the worked example means:** The chi-square goodness-of-fit test yielded a statistic of 2.9 with 4 degrees of freedom and a p-value of 0.574697.

<a id="chi_square_variance_test"></a>

#### Chi-square test for a variance: `chi_square_variance_test`

The chi-square test of H0: sigma = sigma0 for a normal population from sample data.

$$
\chi^2 = \frac{(n-1)s^2}{\sigma_0^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `sigma0` | in | hypothesised standard deviation | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `chi_square` | out | (n - 1) s^2 / sigma0^2 | - |
| `df` | out | n - 1 | - |
| `p_value` | out | p-value | - |
| `sample_variance` | out | s^2 | - |

**Applies when:** normal population (the test is sensitive to non-normality).

```shell
$ statistics solve chi_square_variance_test "x=[10.2, 9.8, 10.5, 10.1, 9.6, 10.4]" sigma0=0.3 alternative=two-sided
chi_square = 6.666666667
df = 5
p_value = 0.4932683044
sample_variance = 0.12
  by: Chi-square test for a variance  (chi-square statistic with n - 1 df; two-sided p-value = twice the smaller tail)
  sigma0 = 0.3
  alternative = two-sided
  assumes: normal population (the test is sensitive to non-normality)
```

**What the worked example means:** This calculation tests whether the population variance differs from the hypothesised value. Using the sample data, the sample variance is computed as 0.12. The test statistic, calculated as 6.66667, measures how far the observed variability deviates from the expected variability under the null hypothesis. With 5 degrees of freedom, the resulting p-value of 0.493268 indicates the probability of observing such a discrepancy if the null hypothesis were true.

<a id="chi_square_independence"></a>

#### Chi-square test of independence: `chi_square_independence`

The chi-square test of independence (or homogeneity) for an r x c contingency table of counts.

$$
\chi^2 = \sum_{i,j}\frac{(O_{ij} - E_{ij})^2}{E_{ij}},\ E_{ij} = \frac{R_i C_j}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `table` | in | contingency table of counts (rows x columns) | - |
| `chi_square` | out | Pearson chi-square statistic | - |
| `df` | out | (r - 1)(c - 1) | - |
| `p_value` | out | upper-tail p-value | - |
| `expected` | out | expected counts under independence | - |

**Applies when:** independent observations; expected counts at least about 5; no continuity correction.

```shell
$ statistics solve chi_square_independence "table=[[20, 30], [25, 25]]"
chi_square = 1.01010101
df = 1
p_value = 0.3148786413
expected = [[22.5,27.5],[22.5,27.5]]
  by: Chi-square test of independence  (expected counts row total x column total / n; Pearson chi-square with (r-1)(c-1) df)
  assumes: independent observations; expected counts at least about 5; no continuity correction
```

**What the worked example means:** This analysis evaluates the independence between two categorical variables by comparing observed counts in the contingency table against expected counts under the assumption of no association. The calculated Pearson chi-square statistic of 1.0101 reflects the total squared difference between observed and expected values, normalized by those expectations. With 1 degrees of freedom, the resulting upper-tail p-value of 0.314879 indicates the probability of observing such discrepancies if the variables were truly independent.

<a id="f_test_variances"></a>

#### F-test for equality of two variances: `f_test_variances`

The F-test of H0: sigma1^2 = sigma2^2 from two independent normal samples.

$$
F = \frac{s_1^2}{s_2^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `f` | out | s1^2 / s2^2 | - |
| `df1` | out | n1 - 1 | - |
| `df2` | out | n2 - 1 | - |
| `p_value` | out | p-value | - |

**Applies when:** independent samples from normal populations (sensitive to non-normality).

```shell
$ statistics solve f_test_variances "x=[21, 24, 19, 26, 23, 25]" "y=[22, 23, 22, 24, 23]" alternative=two-sided
f = 9.714285714
df1 = 5
df2 = 4
p_value = 0.04685703619
  by: F-test for equality of two variances  (variance ratio with (n1 - 1, n2 - 1) df; two-sided p-value = twice the smaller tail)
  alternative = two-sided
  assumes: independent samples from normal populations (sensitive to non-normality)
```

**What the worked example means:** This calculation compares the variance of the first sample to that of the second by dividing the squared standard deviation of the first group by that of the second, resulting in an F-ratio of 9.71429. The associated degrees of freedom are 5 for the numerator and 4 for the denominator. A p-value of 0.046857 indicates the probability of observing such a ratio if the true population variances were equal.

<a id="levene_test"></a>

#### Levene's test for equal variances: `levene_test`

Levene's test of equal variances across 2 to 5 groups, centred at the median (Brown-Forsythe) or the mean.

$$
W = \frac{(N-k)\sum n_i(\bar{z}_{i\cdot} - \bar{z})^2}{(k-1)\sum\sum (z_{ij} - \bar{z}_{i\cdot})^2},\ z_{ij} = |x_{ij} - \tilde{x}_i|
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | group 1 | - |
| `g2` | in | group 2 | - |
| `g3` | in | group 3 (optional) | - |
| `g4` | in | group 4 (optional) | - |
| `g5` | in | group 5 (optional) | - |
| `center` | in | median (Brown-Forsythe) or mean (Levene's original) | - |
| `w` | out | Levene statistic | - |
| `p_value` | out | p-value | - |
| `df1` | out | k - 1 | - |
| `df2` | out | N - k | - |

**Applies when:** independent groups, at least 2 values each.

```shell
$ statistics solve levene_test "g1=[21, 24, 19, 26, 23]" "g2=[22, 23, 22, 24, 23]" "g3=[18, 28, 20, 30, 24]" center=median
w = 4.977272727
p_value = 0.02666495019
df1 = 2
df2 = 12
  by: Levene's test for equal variances  (one-way ANOVA on the absolute deviations from each group's median (or mean))
  center = median
  assumes: independent groups, at least 2 values each
```

**What the worked example means:** This calculation determines if the variances across your groups are equal by performing an analysis of variance on the absolute deviations from each group's center. The resulting test statistic, denoted as 4.97727, follows an F-distribution with degrees of freedom 2 and 12. A low p-value of 0.026665 suggests that the assumption of equal variances may be violated, while a high p-value supports this assumption.

<a id="proportion_z_test"></a>

#### One-proportion z-test: `proportion_z_test`

The z-test of H0: p = p0 for a population proportion from x successes in n trials.

$$
z = \frac{\hat{p} - p_0}{\sqrt{p_0(1-p_0)/n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | number of successes | - |
| `n` | in | sample size | - |
| `p0` | in | hypothesised proportion | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `z` | out | (x/n - p0)/sqrt(p0(1 - p0)/n) | - |
| `p_value` | out | p-value | - |
| `proportion` | out | x/n | - |

**Applies when:** n p0 and n(1 - p0) at least about 10; independent trials.

```shell
$ statistics solve proportion_z_test x=58 n=100 p0=0.5 alternative=two-sided
z = 1.6
p_value = 0.1095985834
proportion = 0.58
  by: One-proportion z-test  (z statistic with the null standard error sqrt(p0(1-p0)/n))
  p0 = 0.5
  alternative = two-sided
  assumes: n p0 and n(1 - p0) at least about 10; independent trials
```

**What the worked example means:** This calculation determines if the observed success rate significantly differs from the hypothesized value. Using the null standard error gives the z-statistic as 1.6. The resulting p-value is 0.109599, which indicates the probability of seeing data this extreme if the true proportion were actually 0.5. The sample proportion itself is 0.58.

<a id="t_test_one_sample"></a>

#### One-sample t-test: `t_test_one_sample`

The one-sample t-test of H0: mu = mu0 from sample data: t statistic, degrees of freedom and p-value.

$$
t = \frac{\bar{x} - \mu_0}{s/\sqrt{n}},\ \nu = n - 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the data values (at least two) | - |
| `mu0` | in | hypothesised mean | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | (mean - mu0)/(s/sqrt(n)) | - |
| `df` | out | n - 1 | - |
| `p_value` | out | p-value | - |
| `mean` | out | sample mean | - |
| `standard_deviation` | out | sample standard deviation | - |

**Applies when:** random sample from a roughly normal population (or large n); the alternative must be stated for a one-sided claim.

```shell
$ statistics solve t_test_one_sample "x=[5.1, 4.9, 5.3, 5.5, 5, 5.2]" mu0=5 alternative=two-sided
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

**What the worked example means:** This calculation performs a one-sample t-test to determine if the sample mean differs from the hypothesized value. Using the input data, it computes the sample mean as 5.16667 and the standard deviation as 0.216025. The resulting t-statistic is 1.88982, which indicates how many standard errors the sample mean lies away from the null hypothesis mean. With 5 degrees of freedom, the associated p-value is 0.117387, quantifying the probability of observing such an extreme result if the null hypothesis were true.

<a id="z_test_mean"></a>

#### One-sample z-test for a mean (sigma known), `z_test_mean`

The z statistic and p-value for H0: mu = mu0 when the population standard deviation is known.

$$
z = \frac{\bar{x} - \mu_0}{\sigma/\sqrt{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `xbar` | in | sample mean | - |
| `mu0` | in | hypothesised mean | - |
| `sigma` | in | population standard deviation (known) | - |
| `n` | in | sample size | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `z` | out | (xbar - mu0) / (sigma / sqrt(n)) | - |
| `p_value` | out | p-value for the chosen alternative | - |

**Applies when:** sigma known; normal population or large n; the alternative must be stated for a one-sided claim.

```shell
$ statistics solve z_test_mean xbar=52 mu0=50 sigma=5 n=36 alternative=two-sided
z = 2.4
p_value = 0.01639507185
  by: One-sample z-test for a mean (sigma known)  (z statistic and its normal tail probability)
  xbar = 52
  mu0 = 50
  sigma = 5
  alternative = two-sided
  assumes: sigma known; normal population or large n; the alternative must be stated for a one-sided claim
```

**What the worked example means:** This calculation determines the significance of the difference between the observed sample mean and the hypothesized population mean. Using the known population standard deviation, it computes a z statistic of 2.4. This value indicates how many standard errors the sample mean lies away from the null hypothesis value. The resulting p-value of 0.0163951 represents the probability of observing such an extreme result if the null hypothesis were true, helping to decide whether to reject the claim that the true mean equals the hypothesized value.

<a id="paired_t_test"></a>

#### Paired t-test: `paired_t_test`

The paired (dependent samples) t-test on the differences x - y: t statistic, df and p-value.

$$
t = \frac{\bar{d}}{s_d/\sqrt{n}},\ \nu = n - 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first measurement of each pair | - |
| `y` | in | second measurement of each pair | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | mean difference / (s_d / sqrt(n)) | - |
| `df` | out | n - 1 | - |
| `p_value` | out | p-value | - |
| `mean_difference` | out | mean of x - y | - |

**Applies when:** pairs measured on the same units; differences roughly normal (or many pairs).

```shell
$ statistics solve paired_t_test "x=[200, 190, 210, 220, 205]" "y=[195, 185, 200, 214, 200]" alternative=two-sided
t = 6.394811727
df = 4
p_value = 0.003070152082
mean_difference = 6.2
  by: Paired t-test  (one-sample t-test on the differences)
  alternative = two-sided
  assumes: pairs measured on the same units; differences roughly normal (or many pairs)
```

**What the worked example means:** This analysis computes the significance of the difference between two related measurements by performing a one-sample t-test on their differences. The resulting t-statistic is 6.39481, derived from a mean difference of 6.2 and standard error calculations, leading to 4 degrees of freedom. The associated p-value of 0.00307015 indicates whether the observed difference is statistically significant compared to zero.

<a id="two_proportion_z_test"></a>

#### Two-proportion z-test: `two_proportion_z_test`

The pooled z-test of H0: p1 = p2 from x1 of n1 and x2 of n2 successes.

$$
z = \frac{\hat{p}_1 - \hat{p}_2}{\sqrt{\hat{p}(1-\hat{p})\left(\frac{1}{n_1} + \frac{1}{n_2}\right)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x1` | in | successes in sample 1 | - |
| `n1` | in | size of sample 1 | - |
| `x2` | in | successes in sample 2 | - |
| `n2` | in | size of sample 2 | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `z` | out | z statistic (pooled standard error) | - |
| `p_value` | out | p-value | - |
| `pooled_proportion` | out | (x1 + x2)/(n1 + n2) | - |
| `difference` | out | x1/n1 - x2/n2 | - |

**Applies when:** independent samples; at least about 10 successes and failures in each sample.

```shell
$ statistics solve two_proportion_z_test x1=45 n1=100 x2=30 n2=100 alternative=two-sided
z = 2.19089023
p_value = 0.02845973692
pooled_proportion = 0.375
difference = 0.15
  by: Two-proportion z-test  (pooled proportion, z = (p1 - p2)/sqrt(p(1-p)(1/n1 + 1/n2)))
  alternative = two-sided
  assumes: independent samples; at least about 10 successes and failures in each sample
```

**What the worked example means:** This calculation tests whether the proportion of successes in the first sample, based on 45 out of 100 trials, differs significantly from the proportion in the second sample, based on 30 out of 100 trials. The method uses a pooled proportion estimate to determine the standard error under the null hypothesis that both populations share the same true proportion. The resulting test statistic is 2.19089, which measures how many standard errors the observed difference of 0.15 lies from zero. The associated p-value is 0.0284597, representing the probability of observing such an extreme difference if the null hypothesis were true.

<a id="t_test_two_sample_pooled"></a>

#### Two-sample t-test (pooled variances): `t_test_two_sample_pooled`

Two-sample t-test (pooled variances) comparing the means of two independent samples: t statistic, degrees of freedom, p-value.

$$
t = \frac{\bar{x}_1 - \bar{x}_2}{s_p\sqrt{1/n_1 + 1/n_2}},\ \nu = n_1 + n_2 - 2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | t statistic | - |
| `df` | out | degrees of freedom | - |
| `p_value` | out | p-value | - |
| `difference` | out | mean(x) - mean(y) | - |

**Applies when:** independent random samples; roughly normal populations or large samples; equal population variances assumed.

```shell
$ statistics solve t_test_two_sample_pooled "x=[12, 15, 11, 18, 14]" "y=[10, 9, 13, 11, 8]" alternative=two-sided
t = 2.538981798
df = 8
p_value = 0.0347658419
difference = 3.8
  by: Two-sample t-test (pooled variances)  (pooled variance with n1 + n2 - 2 df)
  alternative = two-sided
  assumes: independent random samples; roughly normal populations or large samples; equal population variances assumed
```

**What the worked example means:** This calculation performs a two-sample t-test assuming equal variances. It compares the mean difference of 3.8 against the pooled standard error derived from samples x and y. The resulting t statistic of 2.53898 indicates how many standard errors separate the group means, while the p-value of 0.0347658 quantifies the probability of observing such a difference if the population means were actually equal.

<a id="t_test_summary_statistics"></a>

#### Two-sample t-test from summary statistics: `t_test_summary_statistics`

The two-sample t-test from the means, sample standard deviations and sizes of two groups (Welch or pooled).

$$
t = \frac{\bar{x}_1 - \bar{x}_2}{\sqrt{s_1^2/n_1 + s_2^2/n_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | mean of group 1 | - |
| `s1` | in | sample standard deviation of group 1 | - |
| `n1` | in | size of group 1 | - |
| `m2` | in | mean of group 2 | - |
| `s2` | in | sample standard deviation of group 2 | - |
| `n2` | in | size of group 2 | - |
| `variance` | in | welch (unequal variances) or pooled (equal variances) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | t statistic | - |
| `df` | out | degrees of freedom | - |
| `p_value` | out | p-value | - |
| `difference` | out | m1 - m2 | - |

**Applies when:** independent random samples; roughly normal populations or large samples.

```shell
$ statistics solve t_test_summary_statistics m1=20 s1=4 n1=15 m2=17 s2=5 n2=12 alternative=two-sided variance=pooled
t = 1.733785461
df = 25
p_value = 0.0952738393
difference = 3
  by: Two-sample t-test from summary statistics  (pooled or Welch standard error and degrees of freedom from the summaries)
  m1 = 20
  s1 = 4
  m2 = 17
  s2 = 5
  variance = pooled
  alternative = two-sided
  assumes: independent random samples; roughly normal populations or large samples
```

**What the worked example means:** This calculation performs a two-sample t-test using the provided summary statistics. The difference between the means is 3, and the test statistic is calculated as 1.73379. Based on the chosen variance assumption and sample sizes, the degrees of freedom are 25. The resulting p-value of 0.0952738 indicates the strength of evidence against the null hypothesis that the group means are equal.

<a id="welch_t_test"></a>

#### Welch's two-sample t-test (unequal variances), `welch_t_test`

Welch's two-sample t-test (unequal variances) comparing the means of two independent samples: t statistic, degrees of freedom, p-value.

$$
t = \frac{\bar{x}_1 - \bar{x}_2}{\sqrt{s_1^2/n_1 + s_2^2/n_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | t statistic | - |
| `df` | out | degrees of freedom | - |
| `p_value` | out | p-value | - |
| `difference` | out | mean(x) - mean(y) | - |

**Applies when:** independent random samples; roughly normal populations or large samples; Welch-Satterthwaite degrees of freedom.

```shell
$ statistics solve welch_t_test "x=[12, 15, 11, 18, 14]" "y=[10, 9, 13, 11, 8]" alternative=two-sided
t = 2.538981798
df = 7.174149271
p_value = 0.03794551168
difference = 3.8
  by: Welch's two-sample t-test (unequal variances)  (separate variances, Welch-Satterthwaite df)
  alternative = two-sided
  assumes: independent random samples; roughly normal populations or large samples; Welch-Satterthwaite degrees of freedom
```

**What the worked example means:** This calculation performs a Welch two-sample t-test to compare the means of two independent groups without assuming equal variances. The test statistic, denoted as 2.53898, measures the difference between the sample means relative to their standard error. The resulting degrees of freedom, 7.17415, are adjusted using the Welch-Satterthwaite equation to account for unequal variances and sample sizes. The associated p-value, 0.0379455, indicates the probability of observing such a difference if the true population means were actually equal.

### Nonparametric tests (7)

<a id="friedman_test"></a>

#### Friedman test: `friedman_test`

The Friedman rank test for k >= 3 related treatments measured on the same blocks (subjects): rows are blocks, columns are treatments.

$$
Q = \frac{12}{nk(k+1)}\sum_j R_j^2 - 3n(k+1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `data` | in | table with one row per block (subject) and one column per treatment | - |
| `chi_square` | out | Friedman statistic (tie-corrected) | - |
| `df` | out | k - 1 | - |
| `p_value` | out | chi-square p-value | - |

**Applies when:** at least 3 treatments and 2 blocks; chi-square approximation.

```shell
$ statistics solve friedman_test "data=[[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]"
chi_square = 0.4
df = 2
p_value = 0.8187307531
  by: Friedman test  (rank within each block; Q = 12/(nk(k+1)) sum R_j^2 - 3n(k+1), divided by the tie correction)
  assumes: at least 3 treatments and 2 blocks; chi-square approximation
```

**What the worked example means:** This calculation performs the Friedman rank test to determine if there are significant differences between three or more related treatments measured on the same blocks. The data table is processed by ranking values within each block, and the sum of squared ranks for each treatment is used to compute the statistic. The resulting chi-square value of 0.4, based on 2 degrees of freedom, produces a p-value of 0.818731, which indicates whether the observed differences in central tendency are statistically significant under the chi-square approximation.

<a id="kruskal_wallis"></a>

#### Kruskal-Wallis H test: `kruskal_wallis`

The Kruskal-Wallis rank test comparing 2 to 5 independent groups (nonparametric one-way ANOVA).

$$
H = \frac{12}{N(N+1)}\sum \frac{R_i^2}{n_i} - 3(N+1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | group 1 | - |
| `g2` | in | group 2 | - |
| `g3` | in | group 3 (optional) | - |
| `g4` | in | group 4 (optional) | - |
| `g5` | in | group 5 (optional) | - |
| `h` | out | H statistic (tie-corrected) | - |
| `df` | out | k - 1 | - |
| `p_value` | out | chi-square p-value | - |

**Applies when:** independent groups, ordinal or continuous data; chi-square approximation (about 5 or more per group).

```shell
$ statistics solve kruskal_wallis "g1=[27, 2, 4, 18, 7, 9]" "g2=[20, 8, 14, 36, 21, 22]" "g3=[34, 31, 3, 23, 30, 6]"
h = 2.85380117
df = 2
p_value = 0.2400517906
  by: Kruskal-Wallis H test  (rank all observations; H = 12/(N(N+1)) sum R_i^2/n_i - 3(N+1), divided by the tie correction)
  assumes: independent groups, ordinal or continuous data; chi-square approximation (about 5 or more per group)
```

**What the worked example means:** This calculation determines the Kruskal-Wallis H statistic by ranking all observations from the independent groups and comparing their rank sums. The resulting H value of 2.8538 is evaluated against a chi-square distribution with 2 degrees of freedom to produce a p-value of 0.240052. This process tests whether the distributions of the ordinal or continuous data across the groups are identical, serving as a nonparametric alternative to one-way ANOVA.

<a id="mann_whitney_exact"></a>

#### Mann-Whitney U test (exact): `mann_whitney_exact`

The Mann-Whitney U test (exact): compares two independent samples by ranks (statistic U of the first sample and the p-value).

$$
U_1 = R_1 - \frac{n_1(n_1+1)}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `u` | out | U statistic of the first sample (R1 - n1(n1+1)/2) | - |
| `p_value` | out | p-value | - |

**Applies when:** independent samples, ordinal or continuous data; no ties; samples of at most 30.

```shell
$ statistics solve mann_whitney_exact "x=[3.1, 5.2, 4.3, 6.4, 7.5]" "y=[1.5, 2.6, 2.2, 3.9]" alternative=two-sided
u = 19
p_value = 0.03174603175
  by: Mann-Whitney U test (exact)  (rank the combined sample; U from the rank sum; exact null distribution of U (Gaussian binomial coefficients))
  alternative = two-sided
  assumes: independent samples, ordinal or continuous data; no ties; samples of at most 30
```

**What the worked example means:** This analysis ranks all values from both samples together to compute the Mann-Whitney U statistic, which measures how much the ranks of the first sample exceed what would be expected by chance. The calculated U value is 19, derived from the rank sum of the first sample minus its minimum possible rank sum. The exact p-value is 0.031746, indicating the probability of observing such an extreme ranking under the null hypothesis that the two distributions are identical.

<a id="mann_whitney_u"></a>

#### Mann-Whitney U test (large-sample): `mann_whitney_u`

The Mann-Whitney U test (large-sample): compares two independent samples by ranks (statistic U of the first sample and the p-value).

$$
U_1 = R_1 - \frac{n_1(n_1+1)}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | first sample | - |
| `y` | in | second sample | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `u` | out | U statistic of the first sample (R1 - n1(n1+1)/2) | - |
| `p_value` | out | p-value | - |

**Applies when:** independent samples, ordinal or continuous data; normal approximation with tie correction and continuity correction.

```shell
$ statistics solve mann_whitney_u "x=[3, 5, 4, 6, 7]" "y=[1.5, 2.6, 2.2, 3.9]" alternative=two-sided
u = 19
p_value = 0.03733641592
  by: Mann-Whitney U test (large-sample)  (rank the combined sample; U from the rank sum; normal approximation)
  alternative = two-sided
  assumes: independent samples, ordinal or continuous data; normal approximation with tie correction and continuity correction
```

In the worked example, x = [3, 5, 4, 6, 7], y = [1.5, 2.6, 2.2, 3.9], alternative = two-sided go through the equation above and give u = 19, p_value = 0.03733641592. It applies only when independent samples, ordinal or continuous data; normal approximation with tie correction and continuity correction: outside that it says what is wrong instead of returning a number.

<a id="sign_test"></a>

#### Sign test: `sign_test`

The sign test for the median difference of paired data (or of differences given directly): exact binomial p-value.

$$
P = 2\min\left(P(K \le k), P(K \ge k)\right),\ K \sim \mathrm{Bin}(n, \tfrac12)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | differences, or the first measurement of each pair | - |
| `y` | in | second measurement of each pair (optional) (optional) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `positives` | out | number of positive differences | - |
| `n` | out | non-zero differences | - |
| `p_value` | out | exact binomial p-value | - |

**Applies when:** paired data; zero differences dropped.

```shell
$ statistics solve sign_test "x=[125, 115, 130, 140, 141, 115, 140, 125]" "y=[110, 122, 125, 120, 140, 124, 123, 137]" alternative=two-sided
positives = 5
n = 8
p_value = 0.7265625
  by: Sign test  (count positive signs; exact Binomial(n, 1/2) tail probabilities)
  alternative = two-sided
  assumes: paired data; zero differences dropped
```

**What the worked example means:** This calculation determines the exact binomial probability of observing at least as many positive signs as found in the data, assuming the median difference is zero. With 5 positive outcomes among 8 valid pairs, the resulting p-value of 0.726562 indicates how likely such an extreme split would be if there were no true difference between the paired measurements.

<a id="wilcoxon_signed_rank_exact"></a>

#### Wilcoxon signed-rank test (exact): `wilcoxon_signed_rank_exact`

The Wilcoxon signed-rank test (exact) for paired data (x - y) or for differences given directly (x alone); zero differences are dropped.

$$
R^+ = \sum_{d_i > 0} \operatorname{rank}(|d_i|)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | differences, or the first measurement of each pair | - |
| `y` | in | second measurement of each pair (optional) (optional) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `statistic` | out | min(R+, R-) for a two-sided test, R+ otherwise | - |
| `p_value` | out | p-value | - |
| `r_plus` | out | sum of ranks of positive differences | - |
| `n` | out | non-zero differences | - |

**Applies when:** paired data, symmetric distribution of differences; zero differences dropped; no zero and no tied absolute differences; at most 60 differences.

```shell
$ statistics solve wilcoxon_signed_rank_exact "x=[125, 115, 130, 140, 141, 115, 140, 125]" "y=[110, 122, 125, 120, 140, 124, 123, 137]" alternative=two-sided
statistic = 12
p_value = 0.4609375
r_plus = 24
n = 8
  by: Wilcoxon signed-rank test (exact)  (rank |d|; R+ = sum of ranks of positive d; exact null distribution by counting subsets)
  alternative = two-sided
  assumes: paired data, symmetric distribution of differences; zero differences dropped; no zero and no tied absolute differences; at most 60 differences
```

**What the worked example means:** This calculation performs the exact Wilcoxon signed-rank test on paired data or direct differences. It first removes any zero differences, leaving 8 observations. The absolute values of these remaining differences are ranked, and the sum of ranks for positive differences is calculated as 24. Depending on the alternative hypothesis, the test statistic is reported as 12, which compares this rank sum against the exact null distribution to determine the probability of observing such an extreme result if the median difference were truly zero. The resulting p-value is 0.460938.

<a id="wilcoxon_signed_rank"></a>

#### Wilcoxon signed-rank test (large-sample): `wilcoxon_signed_rank`

The Wilcoxon signed-rank test (large-sample) for paired data (x - y) or for differences given directly (x alone); zero differences are dropped.

$$
R^+ = \sum_{d_i > 0} \operatorname{rank}(|d_i|)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | differences, or the first measurement of each pair | - |
| `y` | in | second measurement of each pair (optional) (optional) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `statistic` | out | min(R+, R-) for a two-sided test, R+ otherwise | - |
| `p_value` | out | p-value | - |
| `r_plus` | out | sum of ranks of positive differences | - |
| `n` | out | non-zero differences | - |

**Applies when:** paired data, symmetric distribution of differences; zero differences dropped; normal approximation with tie correction, no continuity correction.

```shell
$ statistics solve wilcoxon_signed_rank "x=[125, 115, 130, 140, 141, 115, 140, 125]" "y=[110, 122, 125, 120, 140, 124, 123, 137]" alternative=two-sided
statistic = 12
p_value = 0.4008141694
r_plus = 24
n = 8
  by: Wilcoxon signed-rank test (large-sample)  (rank |d|; R+ = sum of ranks of positive d; normal approximation)
  alternative = two-sided
  assumes: paired data, symmetric distribution of differences; zero differences dropped; normal approximation with tie correction, no continuity correction
```

**What the worked example means:** This calculation determines whether the median difference between paired observations is significantly different from zero using a normal approximation. The absolute differences are ranked, and the sum of ranks for positive differences, denoted as R+, is computed as 24. The final test statistic, reported as 12, represents either R+ or its complement depending on the specified alternative hypothesis. With 8 valid data points after excluding zeros, the resulting p-value of 0.400814 indicates the probability of observing such an imbalance in ranks if the true median difference were actually zero.

### Power and sample size (5)

<a id="power_one_sample_z"></a>

#### Power of a one-sample z-test: `power_one_sample_z`

The power of a one-sample z-test for a standardized effect size d (Cohen's d), sample size n and significance level alpha.

$$
1 - \beta = \Phi\left(d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right) + \Phi\left(-d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | standardized effect size (difference / sigma) | - |
| `n` | in | sample size | - |
| `alpha` | in | significance level | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `power` | out | probability of rejecting H0 when the effect is d | - |

**Applies when:** normal approximation (sigma known); two-sided power includes both rejection tails.

```shell
$ statistics solve power_one_sample_z d=0.5 n=30 alpha=0.05 alternative=two-sided
power = 0.7819079987
  by: Power of a one-sample z-test  (power = P(Z > z_crit - d sqrt(n_eff)) (+ the opposite tail when two-sided), n_eff = n)
  d = 0.5
  n = 30
  alpha = 0.05
  alternative = two-sided
  assumes: normal approximation (sigma known); two-sided power includes both rejection tails
```

**What the worked example means:** This calculation determines the probability of correctly rejecting the null hypothesis in a one-sample z-test. It uses the standardized effect size 0.5, the total sample size 30, and the chosen significance level 0.05 to compute the statistical power, which is reported as 0.781908. A higher value indicates a greater likelihood of detecting the specified effect if it truly exists.

<a id="power_two_sample_z"></a>

#### Power of a two-sample z-test: `power_two_sample_z`

The power of a two-sample z-test for a standardized effect size d (Cohen's d), sample size n per group and significance level alpha.

$$
1 - \beta = \Phi\left(d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right) + \Phi\left(-d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | standardized effect size (difference / sigma) | - |
| `n` | in | sample size per group | - |
| `alpha` | in | significance level | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `power` | out | probability of rejecting H0 when the effect is d | - |

**Applies when:** normal approximation (sigma known); two-sided power includes both rejection tails.

```shell
$ statistics solve power_two_sample_z d=0.5 n=30 alpha=0.05 alternative=two-sided
power = 0.4906855677
  by: Power of a two-sample z-test  (power = P(Z > z_crit - d sqrt(n_eff)) (+ the opposite tail when two-sided), n_eff = n/2)
  d = 0.5
  n = 30
  alpha = 0.05
  alternative = two-sided
  assumes: normal approximation (sigma known); two-sided power includes both rejection tails
```

**What the worked example means:** This calculation determines the statistical power of a two-sample z-test given a standardized effect size of 0.5, a sample size per group of 30, and a significance level of 0.05. The resulting power value of 0.490686 represents the likelihood that the test will correctly reject the null hypothesis when the specified effect exists. Higher values indicate a greater ability to detect the difference between groups, while lower values suggest a higher risk of missing a true effect.

<a id="sample_size_two_means"></a>

#### Sample size per group to compare two means (z-test power), `sample_size_two_means`

The sample size per group for a two-sample z-test to reach a target power for a standardized effect size d.

$$
n \approx 2\left(\frac{z_{1-\alpha/2} + z_{1-\beta}}{d}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | standardized effect size (difference / sigma) | - |
| `alpha` | in | significance level | - |
| `power` | in | target power (e.g. 0.8) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `n_per_group` | out | required size per group (rounded up) | - |
| `n_exact` | out | solution before rounding | - |

**Applies when:** equal group sizes; normal approximation; two-sided power includes both tails.

```shell
$ statistics solve sample_size_two_means d=0.5 alpha=0.05 power=0.8 alternative=two-sided
n_per_group = 63
n_exact = 62.79088407
  by: Sample size per group to compare two means (z-test power)  (solve power(n) = target by bisection, round up)
  d = 0.5
  alpha = 0.05
  power = 0.8
  alternative = two-sided
  assumes: equal group sizes; normal approximation; two-sided power includes both tails
```

**What the worked example means:** This procedure determines the required sample size for each group in a two-sample comparison to reach a specified statistical power. Using a standardized effect size of 0.5 and a significance level of 0.05, the method solves for the necessary number of observations, resulting in an exact value of 62.7909 which is rounded up to 63. This ensures that if the true difference matches the assumed effect size, the test will correctly reject the null hypothesis with the desired probability.

<a id="sample_size_mean"></a>

#### Sample size to estimate a mean: `sample_size_mean`

The smallest n giving a z confidence interval for a mean with the required margin of error, sigma known.

$$
n = \left\lceil \left(\frac{z_{\alpha/2}\,\sigma}{E}\right)^2 \right\rceil
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | population standard deviation (planning value) | - |
| `margin` | in | margin of error | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `n` | out | required sample size (rounded up) | - |
| `n_exact` | out | (z sigma / E)^2 | - |

**Applies when:** sigma known or a planning estimate.

```shell
$ statistics solve sample_size_mean sigma=15 margin=2 confidence=0.95
n = 217
n_exact = 216.0820587
  by: Sample size to estimate a mean  (n = (z sigma / E)^2, rounded up)
  sigma = 15
  margin = 2
  confidence = 0.95
  assumes: sigma known or a planning estimate
```

**What the worked example means:** This calculation determines the minimum number of observations needed to estimate a population mean within a specified margin of error. Using a planning standard deviation of 15 and a target margin of error of 2, the exact computed value before rounding is 216.082. The final recommended sample size, rounded up to ensure the requirement is met, is 217. This ensures that a confidence interval for the mean will have the desired precision.

<a id="sample_size_proportion"></a>

#### Sample size to estimate a proportion: `sample_size_proportion`

The smallest n giving a confidence interval for a proportion with the required margin of error (p = 0.5 if no estimate).

$$
n = \left\lceil \frac{z_{\alpha/2}^2\,p(1-p)}{E^2} \right\rceil
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `margin` | in | margin of error (as a proportion, 0.03 for 3 points) | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `p` | in | planning estimate of the proportion (default 0.5) (optional) | - |
| `n` | out | required sample size (rounded up) | - |
| `n_exact` | out | z^2 p(1-p)/E^2 before rounding | - |

**Applies when:** large-sample (Wald) interval; simple random sample.

```shell
$ statistics solve sample_size_proportion margin=0.03 confidence=0.95
n = 1068
n_exact = 1067.071895
  by: Sample size to estimate a proportion  (n = z^2 p(1 - p)/E^2, rounded up)
  margin = 0.03
  confidence = 0.95
  assumes: large-sample (Wald) interval; simple random sample
```

**What the worked example means:** The required sample size is 1068 to estimate the proportion with a margin of error of 0.03 at 0.95 confidence.

### Probability (3)

<a id="two_way_table_probabilities"></a>

#### Joint, marginal and conditional probabilities from a two-way table, `two_way_table_probabilities`

From a two-way table of counts: the joint, marginal and conditional probabilities of every cell, and for a chosen row and column P(row and column), P(row), P(column), P(row | column), P(column | row) and P(row or column).

$$
P(R_i \mid C_j) = \frac{n_{ij}}{n_{\cdot j}},\quad P(C_j \mid R_i) = \frac{n_{ij}}{n_{i\cdot}},\quad P(R_i \cap C_j) = \frac{n_{ij}}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `table` | in | the two-way table of counts (rows x columns) | - |
| `row` | in | the row of interest (from 1) | - |
| `column` | in | the column of interest (from 1) | - |
| `p_row_given_column` | out | P(row \| column) = cell / column total | - |
| `p_column_given_row` | out | P(column \| row) = cell / row total | - |
| `p_row_and_column` | out | P(row and column) = cell / grand total | - |
| `p_row` | out | P(row) = row total / grand total | - |
| `p_column` | out | P(column) = column total / grand total | - |
| `p_row_or_column` | out | P(row or column) | - |
| `total` | out | grand total | - |
| `joint` | out | joint probabilities of every cell | - |
| `row_marginals` | out | marginal probabilities of the rows | - |
| `column_marginals` | out | marginal probabilities of the columns | - |
| `row_given_column` | out | P(row i \| column j) for every cell | - |
| `column_given_row` | out | P(column j \| row i) for every cell | - |

**Applies when:** counts >= 0; every row and column has a positive total; row and column numbered from 1.

```shell
$ statistics solve two_way_table_probabilities "table=[[30, 20], [10, 40]]" row=1 column=2
total = 100
joint = [[0.3,0.2],[0.1,0.4]]
row_marginals = [0.5,0.5]
column_marginals = [0.4,0.6]
row_given_column = [[0.75,0.3333333333333333],[0.25,0.6666666666666666]]
column_given_row = [[0.6,0.4],[0.2,0.8]]
p_row_and_column = 0.2
p_row = 0.5
p_column = 0.6
p_row_given_column = 0.3333333333
p_column_given_row = 0.4
p_row_or_column = 0.9
  by: Joint, marginal and conditional probabilities from a two-way table  (divide a cell by the grand total (joint), by its column total (P(row | column)) or by its row total (P(column | row)); margins are the row and column totals over the grand total)
  assumes: counts >= 0; every row and column has a positive total; row and column numbered from 1
```

In the worked example, table = [30 20; 10 40], row = 1, column = 2 go through the equation above and give total = 100, joint = [[0.3, 0.2], [0.1, 0.4]], row_marginals = [0.5, 0.5], column_marginals = [0.4, 0.6]. It applies only when counts >= 0; every row and column has a positive total; row and column numbered from 1: outside that it says what is wrong instead of returning a number.

<a id="probability_at_least_one"></a>

#### Probability of at least one occurrence: `probability_at_least_one`

The probability that an event of probability p occurs at least once in n independent trials.

$$
P(\text{at least one}) = 1 - (1 - p)^n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | probability of the event on one trial | - |
| `n` | in | number of independent trials | - |
| `probability` | out | 1 - (1 - p)^n | - |
| `none` | out | probability that it never occurs, (1 - p)^n | - |

**Applies when:** independent trials with the same probability p.

```shell
$ statistics solve probability_at_least_one p=0.2 n=5
probability = 0.67232
none = 0.32768
  by: Probability of at least one occurrence  (complement rule: 1 - (1 - p)^n)
  p = 0.2
  assumes: independent trials with the same probability p
```

**What the worked example means:** This calculation determines the likelihood that an event happens at least once across multiple independent attempts. Applying the complement rule subtracts the probability of the event never occurring from one. With a single trial success rate of 0.2 and a total of 5 trials, the chance of zero successes is 0.32768. Consequently, the final probability of seeing the event at least once is 0.67232.

<a id="probability_rules"></a>

#### Probability rules: complement, addition and multiplication, `probability_rules`

From P(A), P(B) and one statement about how A and B overlap (P(A and B), P(A or B), independent, or mutually exclusive): P(not A), P(not B), P(A and B), P(A or B), P(neither), P(A only), P(B only) and the conditional probabilities P(A | B) and P(B | A).

$$
P(A^c) = 1 - P(A),\quad P(A\cup B) = P(A) + P(B) - P(A\cap B),\quad P(A\cap B) \overset{\text{indep.}}{=} P(A)P(B)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p_a` | in | P(A) | - |
| `p_b` | in | P(B) | - |
| `p_a_and_b` | in | P(A and B) (optional) | - |
| `p_a_or_b` | in | P(A or B) (optional) | - |
| `relation` | in | independent or mutually exclusive (instead of P(A and B) or P(A or B)) (optional) | - |
| `p_not_a` | out | P(not A) = 1 - P(A) | - |
| `p_not_b` | out | P(not B) = 1 - P(B) | - |
| `p_a_and_b` | out | P(A and B) | - |
| `p_a_or_b` | out | P(A or B) = P(A) + P(B) - P(A and B) | - |
| `p_neither` | out | P(neither) = 1 - P(A or B) | - |
| `p_a_only` | out | P(A and not B) | - |
| `p_b_only` | out | P(B and not A) | - |
| `p_a_given_b` | out | P(A \| B) = P(A and B) / P(B) | - |
| `p_b_given_a` | out | P(B \| A) = P(A and B) / P(A) | - |

**Applies when:** probabilities between 0 and 1; exactly one statement about the overlap; max(0, P(A) + P(B) - 1) <= P(A and B) <= min(P(A), P(B)).

```shell
$ statistics solve probability_rules p_a=0.4 p_b=0.5 p_a_and_b=0.2
p_not_a = 0.6
p_not_b = 0.5
p_a_and_b = 0.2
p_a_or_b = 0.7
p_neither = 0.3
p_a_only = 0.2
p_b_only = 0.3
p_a_given_b = 0.4
p_b_given_a = 0.5
  by: Probability rules: complement, addition and multiplication  (complement rule 1 - P; multiplication rule P(A and B) = P(A) P(B) for independent events (0 for mutually exclusive ones); addition rule P(A or B) = P(A) + P(B) - P(A and B); conditional probability P(A | B) = P(A and B) / P(B))
  p_a = 0.4
  p_b = 0.5
  p_a_and_b = 0.2
  assumes: probabilities between 0 and 1; exactly one statement about the overlap; max(0, P(A) + P(B) - 1) <= P(A and B) <= min(P(A), P(B))
```

In the worked example, p_a = 0.4, p_b = 0.5, p_a_and_b = 0.2 go through the equation above and give p_not_a = 0.6, p_not_b = 0.5, p_a_and_b = 0.2, p_a_or_b = 0.7. It applies only when probabilities between 0 and 1; exactly one statement about the overlap; max(0, P(A) + P(B) - 1) <= P(A and B) <= min(P(A), P(B)): outside that it says what is wrong instead of returning a number.

### Probability distributions (17)

<a id="binomial_at_least"></a>

#### Binomial probability: at least k successes, `binomial_at_least`

The probability of at least k successes in n independent trials with success probability p.

$$
P(X \ge k) = \sum_{j=k}^{n} \binom{n}{j} p^j (1-p)^{n-j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of trials | - |
| `k` | in | number of successes | - |
| `p` | in | probability of success on each trial | - |
| `probability` | out | P(X >= k) | - |

**Applies when:** n independent trials, constant success probability 0 < p < 1; 0 <= k <= n.

```shell
$ statistics solve binomial_at_least n=10 k=3 p=0.4
probability = 0.8327102464
  by: Binomial probability: at least k successes  (exact binomial probabilities summed term by term (log-space binomial coefficients))
  p = 0.4
  assumes: n independent trials, constant success probability 0 < p < 1; 0 <= k <= n
```

**What the worked example means:** This calculation determines the likelihood of observing at least 3 successes across 10 independent trials, where each trial has a success probability of 0.4. The result, 0.83271, represents the cumulative sum of exact binomial probabilities from k up to n. This value indicates how likely it is to meet or exceed the specified threshold of successes given the underlying chance of success on any single attempt.

<a id="binomial_at_most"></a>

#### Binomial probability: at most k successes, `binomial_at_most`

The probability of at most k successes in n independent trials with success probability p.

$$
P(X \le k) = \sum_{j=0}^{k} \binom{n}{j} p^j (1-p)^{n-j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of trials | - |
| `k` | in | number of successes | - |
| `p` | in | probability of success on each trial | - |
| `probability` | out | P(X <= k) | - |

**Applies when:** n independent trials, constant success probability 0 < p < 1; 0 <= k <= n.

```shell
$ statistics solve binomial_at_most n=10 k=3 p=0.4
probability = 0.3822806016
  by: Binomial probability: at most k successes  (exact binomial probabilities summed term by term (log-space binomial coefficients))
  p = 0.4
  assumes: n independent trials, constant success probability 0 < p < 1; 0 <= k <= n
```

**What the worked example means:** This calculation determines the cumulative probability that a binomial random variable takes a value less than or equal to 3. It sums the exact probabilities for each possible number of successes from zero up to 3, using 10 as the total number of independent trials and 0.4 as the constant probability of success on each trial. The resulting value, 0.382281, represents the likelihood of observing this many or fewer successes under these conditions.

<a id="binomial_exactly"></a>

#### Binomial probability: exactly k successes, `binomial_exactly`

The probability of exactly k successes in n independent trials with success probability p.

$$
P(X = k) = \binom{n}{k} p^k (1-p)^{n-k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of trials | - |
| `k` | in | number of successes | - |
| `p` | in | probability of success on each trial | - |
| `probability` | out | P(X = k) | - |

**Applies when:** n independent trials, constant success probability 0 < p < 1; 0 <= k <= n.

```shell
$ statistics solve binomial_exactly n=10 k=3 p=0.4
probability = 0.214990848
  by: Binomial probability: exactly k successes  (exact binomial probabilities summed term by term (log-space binomial coefficients))
  p = 0.4
  assumes: n independent trials, constant success probability 0 < p < 1; 0 <= k <= n
```

**What the worked example means:** This calculation determines the likelihood of observing exactly 3 successes when performing 10 independent trials, where each trial has a constant success probability of 0.4. The result, 0.214991, represents the specific chance of this exact outcome occurring under these conditions.

<a id="discrete_random_variable"></a>

#### Expected value and variance of a discrete random variable, `discrete_random_variable`

The expected value, variance and standard deviation of a discrete random variable given its values and probabilities.

$$
E[X] = \sum x_i p_i,\quad \operatorname{Var}(X) = \sum (x_i - \mu)^2 p_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the values of the random variable | - |
| `p` | in | their probabilities (summing to 1) | - |
| `expected_value` | out | E[X] = sum x p | - |
| `variance` | out | Var[X] = sum (x - E[X])^2 p | - |
| `standard_deviation` | out | sqrt(Var[X]) | - |

**Applies when:** probabilities non-negative, summing to 1 (within 1e-9).

```shell
$ statistics solve discrete_random_variable "x=[0, 1, 2, 3]" "p=[0.1, 0.3, 0.4, 0.2]"
expected_value = 1.7
variance = 0.81
standard_deviation = 0.9
  by: Expected value and variance of a discrete random variable  (E[X] = sum x p; Var[X] = sum p (x - E[X])^2)
  assumes: probabilities non-negative, summing to 1 (within 1e-9)
```

**What the worked example means:** This calculation determines the long-run average outcome, known as the expected value, which equals 1.7. It also measures the spread of the distribution by computing the variance, resulting in 0.81, and its square root, the standard deviation, which is 0.9. These metrics summarize the central tendency and dispersion of the discrete random variable based on its possible values [0, 1, 2, 3] and their corresponding probabilities [0.1, 0.3, 0.4, 0.2].

<a id="exponential_probability_above"></a>

#### Exponential probability above a value: `exponential_probability_above`

P(X > x) for an exponential distribution given its rate lambda or its mean.

$$
P(X > x) = e^{-\lambda x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value (>= 0) | - |
| `rate` | in | rate lambda (events per unit) (optional) | - |
| `mean` | in | mean 1/lambda (instead of the rate) (optional) | - |
| `probability` | out | P(X > x) | - |

**Applies when:** exponential (memoryless) waiting time; x >= 0; exactly one of rate or mean.

```shell
$ statistics solve exponential_probability_above rate=0.5 x=3
probability = 0.2231301601
  by: Exponential probability above a value  (1 - exp(-lambda x) or exp(-lambda x))
  rate = 0.5
  x = 3
  assumes: exponential (memoryless) waiting time; x >= 0; exactly one of rate or mean
```

**What the worked example means:** This calculation determines the likelihood that an event occurs after a duration of 3, based on a constant rate or mean. The resulting survival probability is 0.22313, indicating how often the system remains operational beyond this specific time point under exponential assumptions.

<a id="exponential_probability_below"></a>

#### Exponential probability below a value: `exponential_probability_below`

P(X < x) for an exponential distribution given its rate lambda or its mean.

$$
P(X < x) = 1 - e^{-\lambda x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value (>= 0) | - |
| `rate` | in | rate lambda (events per unit) (optional) | - |
| `mean` | in | mean 1/lambda (instead of the rate) (optional) | - |
| `probability` | out | P(X < x) | - |

**Applies when:** exponential (memoryless) waiting time; x >= 0; exactly one of rate or mean.

```shell
$ statistics solve exponential_probability_below rate=0.5 x=3
probability = 0.7768698399
  by: Exponential probability below a value  (1 - exp(-lambda x) or exp(-lambda x))
  rate = 0.5
  x = 3
  assumes: exponential (memoryless) waiting time; x >= 0; exactly one of rate or mean
```

**What the worked example means:** This calculation determines the likelihood that an exponential event occurs before the specified time point. Using the rate of occurrence, the cumulative distribution function yields a result of 0.77687 for the input value of 3, indicating the proportion of cases expected to happen within this timeframe.

<a id="geometric_probability"></a>

#### Geometric distribution: first success on trial k, `geometric_probability`

The probability that the first success occurs on trial k, and that it occurs on or before trial k.

$$
P(X = k) = (1-p)^{k-1}p,\quad P(X \le k) = 1 - (1-p)^k
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | probability of success on each trial | - |
| `k` | in | the trial number (1, 2, ...) | - |
| `probability` | out | P(X = k) = (1 - p)^(k-1) p | - |
| `cumulative` | out | P(X <= k) = 1 - (1 - p)^k | - |

**Applies when:** independent trials, constant p; X counts trials up to and including the first success.

```shell
$ statistics solve geometric_probability p=0.2 k=4
probability = 0.1024
cumulative = 0.5904
  by: Geometric distribution: first success on trial k  ((1 - p)^(k - 1) p, and the complement of k failures)
  p = 0.2
  assumes: independent trials, constant p; X counts trials up to and including the first success
```

**What the worked example means:** This calculation determines the likelihood of waiting until a specific trial for the first success. Using the input probability of success 0.2 and the target trial number 4, the model computes the exact probability of the first success occurring on that trial, resulting in 0.1024. It also calculates the cumulative probability that the first success happens on or before that trial, yielding 0.5904. These values help quantify the risk of extended delays in independent binary processes.

<a id="hypergeometric_probability"></a>

#### Hypergeometric probability (sampling without replacement), `hypergeometric_probability`

The probability of exactly k successes, and of at most k, in n draws without replacement from a population of N items of which K are successes.

$$
P(X = k) = \frac{\binom{K}{k}\binom{N-K}{n-k}}{\binom{N}{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | population size | - |
| `K` | in | number of successes in the population | - |
| `n` | in | number of draws | - |
| `k` | in | number of successes drawn | - |
| `probability` | out | P(X = k) | - |
| `cumulative` | out | P(X <= k) | - |

**Applies when:** draws without replacement; K <= N, n <= N, k <= n.

```shell
$ statistics solve hypergeometric_probability N=50 K=5 n=10 k=2
probability = 0.2098397176
cumulative = 0.9517396968
  by: Hypergeometric probability (sampling without replacement)  (exact rational arithmetic: C(K, k) C(N - K, n - k) / C(N, n))
  assumes: draws without replacement; K <= N, n <= N, k <= n
```

**What the worked example means:** This calculation determines the likelihood of observing a specific count of successes when sampling without replacement. Given a population of 50 items containing 5 successes, drawing 10 items gives a probability of getting exactly 2 successes of 0.20984. The result represents the precise ratio of favorable combinations to all possible sample combinations, assuming every item has an equal chance of selection.

<a id="inverse_normal"></a>

#### Inverse normal (normal percentile): `inverse_normal`

The value x with P(X < x) = p for a normal distribution (percentiles, cut-offs, top and bottom percentages).

$$
x = \mu + \sigma\,\Phi^{-1}(p)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | cumulative probability to the left of x (0 < p < 1) | - |
| `mu` | in | the mean | - |
| `sigma` | in | the standard deviation (> 0) | - |
| `x` | out | the value with P(X < x) = p | - |
| `z` | out | the standard normal quantile | - |

**Applies when:** 0 < p < 1; sigma > 0.

```shell
$ statistics solve inverse_normal mu=100 sigma=15 p=0.9
x = 119.2232735
z = 1.281551566
  by: Inverse normal (normal percentile)  (standard normal quantile (rational start, Halley refinement on erfc), x = mu + z sigma)
  mu = 100
  sigma = 15
  p = 0.9
  assumes: 0 < p < 1; sigma > 0
```

**What the worked example means:** This calculation finds the specific data point where the area under the normal curve to its left equals the input probability. Using the mean 100 and standard deviation 15, the method first determines the standard normal quantile z, which is 1.28155. This z-score is then scaled by the standard deviation and shifted by the mean to produce the final result x, equal to 119.223. This process allows for the precise identification of percentiles, such as setting cut-off scores or warranty limits based on desired probabilities.

<a id="binomial_mean_sd"></a>

#### Mean and standard deviation of a binomial distribution, `binomial_mean_sd`

The mean np, variance np(1-p) and standard deviation of a binomial random variable.

$$
\mu = np,\quad \sigma = \sqrt{np(1-p)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of trials | - |
| `p` | in | probability of success | - |
| `mean` | out | n p | - |
| `variance` | out | n p (1 - p) | - |
| `standard_deviation` | out | sqrt(n p (1 - p)) | - |

**Applies when:** 0 < p < 1.

```shell
$ statistics solve binomial_mean_sd n=50 p=0.3
mean = 15
variance = 10.5
standard_deviation = 3.240370349
  by: Mean and standard deviation of a binomial distribution  (mean = np, variance = np(1 - p))
  p = 0.3
  assumes: 0 < p < 1
```

**What the worked example means:** For a binomial distribution defined by 50 trials with success probability 0.3, the expected number of successes is 15 and the spread of possible outcomes is 3.24037. These values summarize the central tendency and variability without needing to calculate every individual probability.

<a id="negative_binomial_probability"></a>

#### Negative binomial: r-th success on trial n, `negative_binomial_probability`

The probability that the r-th success occurs on trial n (independent trials with success probability p).

$$
P(N = n) = \binom{n-1}{r-1} p^r (1-p)^{n-r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | number of successes required | - |
| `p` | in | probability of success | - |
| `n` | in | the trial number | - |
| `probability` | out | C(n-1, r-1) p^r (1-p)^(n-r) | - |

**Applies when:** independent trials, constant p; n >= r.

```shell
$ statistics solve negative_binomial_probability r=3 p=0.4 n=7
probability = 0.124416
  by: Negative binomial: r-th success on trial n  (C(n - 1, r - 1) p^r (1 - p)^(n - r))
  p = 0.4
  assumes: independent trials, constant p; n >= r
```

**What the worked example means:** This calculation determines the likelihood that the final required success happens on a specific trial number, given independent trials with a constant success probability. Using the inputs of 3 successes needed, a success probability of 0.4, and targeting trial 7, the result shows that the probability of this exact outcome is 0.124416. This helps practitioners understand waiting times until a set number of events occur.

<a id="normal_probability_above"></a>

#### Normal probability above a value: `normal_probability_above`

P(X > x) for a normal distribution with mean mu and standard deviation sigma (the area to the right).

$$
P(X > x) = 1 - \Phi\left(\frac{x - \mu}{\sigma}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value | - |
| `mu` | in | the mean | - |
| `sigma` | in | the standard deviation (> 0) | - |
| `probability` | out | P(X > x) | - |
| `z` | out | the z-score of x | - |

**Applies when:** X normally distributed; sigma > 0.

```shell
$ statistics solve normal_probability_above mu=100 sigma=15 x=130
probability = 0.02275013195
z = 2
  by: Normal probability above a value  (z = (x - mu)/sigma; 1 - Phi(z) computed as erfc(z/sqrt 2)/2)
  mu = 100
  sigma = 15
  x = 130
  assumes: X normally distributed; sigma > 0
```

**What the worked example means:** This calculation determines the area under the normal curve to the right of the value 130. Given a distribution with a mean of 100 and a standard deviation of 15, the standardized score, or z-score, is found to be 2. The resulting tail probability, representing the chance of observing a value greater than 130, is 0.0227501. This metric is essential for understanding upper-tail risks and performing one-sided hypothesis tests.

<a id="normal_probability_below"></a>

#### Normal probability below a value: `normal_probability_below`

P(X < x) for a normal distribution with mean mu and standard deviation sigma (the area to the left).

$$
P(X < x) = \Phi\left(\frac{x - \mu}{\sigma}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value | - |
| `mu` | in | the mean | - |
| `sigma` | in | the standard deviation (> 0) | - |
| `probability` | out | P(X < x) | - |
| `z` | out | the z-score of x | - |

**Applies when:** X normally distributed; sigma > 0.

```shell
$ statistics solve normal_probability_below mu=100 sigma=15 x=120
probability = 0.9087887803
z = 1.333333333
  by: Normal probability below a value  (z = (x - mu)/sigma; Phi(z) from the complementary error function)
  mu = 100
  sigma = 15
  x = 120
  assumes: X normally distributed; sigma > 0
```

**What the worked example means:** This calculation determines the cumulative area under the normal distribution curve to the left of the specific value 120. Given a mean of 100 and a standard deviation of 15, the result indicates how common or rare an observation at this level is within the dataset. The computed probability is 0.908789, which represents the proportion of the total distribution falling below this threshold, while the associated z-score of 1.33333 quantifies how many standard deviations the value lies from the mean.

<a id="normal_probability_between"></a>

#### Normal probability between two values: `normal_probability_between`

P(a < X < b) for a normal distribution with mean mu and standard deviation sigma.

$$
P(a < X < b) = \Phi\left(\frac{b-\mu}{\sigma}\right) - \Phi\left(\frac{a-\mu}{\sigma}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lower bound | - |
| `b` | in | upper bound | - |
| `mu` | in | the mean | - |
| `sigma` | in | the standard deviation (> 0) | - |
| `probability` | out | P(a < X < b) | - |
| `z_a` | out | z-score of a | - |
| `z_b` | out | z-score of b | - |

**Applies when:** X normally distributed; sigma > 0; a < b.

```shell
$ statistics solve normal_probability_between mu=100 sigma=15 a=85 b=115
probability = 0.6826894921
z_a = -1
z_b = 1
  by: Normal probability between two values  (Phi(z_b) - Phi(z_a), using upper tails when both z-scores are positive (no cancellation))
  mu = 100
  sigma = 15
  a = 85
  b = 115
  assumes: X normally distributed; sigma > 0; a < b
```

**What the worked example means:** This calculation determines the likelihood that a normally distributed variable lies within a specific range. Converting the lower bound 85 and upper bound 115 into z-scores relative to the mean 100 and standard deviation 15 gives the area under the curve between these points. The resulting probability of 0.682689 indicates how often values fall in this interval, with intermediate z-scores calculated as -1 and 1.

<a id="poisson_at_least"></a>

#### Poisson probability: at least k events, `poisson_at_least`

The probability of at least k events when events occur independently at mean rate lambda per interval.

$$
P(X \ge k) = 1 - \sum_{j=0}^{k-1} \frac{e^{-\lambda}\lambda^j}{j!}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | the mean number of events (lambda > 0) | - |
| `k` | in | number of events | - |
| `probability` | out | the Poisson probability | - |

**Applies when:** events independent, at a constant mean rate; lambda > 0, k a whole number.

```shell
$ statistics solve poisson_at_least lam=4 k=2
probability = 0.9084218056
  by: Poisson probability: at least k events  (Poisson probabilities summed term by term)
  lam = 4
  assumes: events independent, at a constant mean rate; lambda > 0, k a whole number
```

**What the worked example means:** This calculation determines the likelihood of seeing a minimum number of occurrences, specifically at least 2, within a fixed interval where events happen independently at an average rate of 4. The result, expressed as 0.908422, quantifies the risk of exceeding this threshold. It is derived by subtracting the cumulative probability of all counts below 2 from one, providing a direct measure of tail risk for capacity planning or anomaly detection.

<a id="poisson_at_most"></a>

#### Poisson probability: at most k events, `poisson_at_most`

The probability of at most k events when events occur independently at mean rate lambda per interval.

$$
P(X \le k) = \sum_{j=0}^{k} \frac{e^{-\lambda}\lambda^j}{j!}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | the mean number of events (lambda > 0) | - |
| `k` | in | number of events | - |
| `probability` | out | the Poisson probability | - |

**Applies when:** events independent, at a constant mean rate; lambda > 0, k a whole number.

```shell
$ statistics solve poisson_at_most lam=4 k=2
probability = 0.2381033056
  by: Poisson probability: at most k events  (Poisson probabilities summed term by term)
  lam = 4
  assumes: events independent, at a constant mean rate; lambda > 0, k a whole number
```

**What the worked example means:** This calculation determines the cumulative likelihood that a count of independent events does not exceed a specific limit. Summing the individual probabilities for zero through 2 occurrences, given a constant mean rate of 4, shows that the total chance of staying within this threshold is 0.238103. This result helps practitioners assess risk or capacity constraints in systems where events happen randomly but at a predictable average speed.

<a id="poisson_exactly"></a>

#### Poisson probability: exactly k events, `poisson_exactly`

The probability of exactly k events when events occur independently at mean rate lambda per interval.

$$
P(X = k) = \frac{e^{-\lambda}\lambda^k}{k!}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | the mean number of events (lambda > 0) | - |
| `k` | in | number of events | - |
| `probability` | out | the Poisson probability | - |

**Applies when:** events independent, at a constant mean rate; lambda > 0, k a whole number.

```shell
$ statistics solve poisson_exactly lam=4 k=2
probability = 0.1465251111
  by: Poisson probability: exactly k events  (Poisson probabilities summed term by term)
  lam = 4
  assumes: events independent, at a constant mean rate; lambda > 0, k a whole number
```

**What the worked example means:** This calculation determines the likelihood of observing precisely 2 events in a fixed interval, given that the average rate is 4. The resulting probability is 0.146525, which quantifies how often this specific count occurs relative to all possible counts under these conditions.

### Quality control (6)

<a id="c_chart"></a>

#### c chart limits (defects per unit), `c_chart`

Centre line and 3-sigma limits of a c chart from counts of defects per inspection unit.

$$
\bar{c} \pm 3\sqrt{\bar{c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `counts` | in | defects counted in each unit | - |
| `center` | out | c-bar | - |
| `ucl` | out | c-bar + 3 sqrt(c-bar) | - |
| `lcl` | out | max(0, c-bar - 3 sqrt(c-bar)) | - |

**Applies when:** constant inspection unit; Poisson-distributed counts.

```shell
$ statistics solve c_chart "counts=[3, 5, 2, 6, 4, 7, 3, 4]"
center = 4.25
ucl = 10.43465844
lcl = 0
  by: c chart limits (defects per unit)  (c-bar +/- 3 sqrt(c-bar))
  assumes: constant inspection unit; Poisson-distributed counts
```

**What the worked example means:** This calculation determines the control limits for a c chart using the average number of defects per unit. The center line represents the mean count, while the upper and lower limits mark the boundaries within which most variation should fall under stable conditions. Specifically, the upper control limit is set at three standard deviations above the mean, calculated as 10.4347, and the lower limit is 0. These values help identify when the process might be out of statistical control based on observed defect rates.

<a id="individuals_moving_range_chart"></a>

#### Individuals and moving range (I-MR) chart limits, `individuals_moving_range_chart`

Control limits of the individuals (X) and moving range charts from individual measurements in time order.

$$
UCL_x = \bar{x} + 2.66\,\overline{MR}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | individual measurements in time order | - |
| `center` | out | mean | - |
| `mean_moving_range` | out | average moving range of span 2 | - |
| `x_ucl` | out | mean + 2.66 MR-bar | - |
| `x_lcl` | out | mean - 2.66 MR-bar | - |
| `mr_ucl` | out | 3.267 MR-bar | - |
| `mr_lcl` | out | 0 | - |

**Applies when:** measurements in time order; constants E2 = 2.66 (3/d2) and D4 = 3.267 for span 2.

```shell
$ statistics solve individuals_moving_range_chart "x=[10.2, 9.8, 10.5, 10.1, 9.7, 10.3, 10]"
center = 10.08571429
mean_moving_range = 0.4666666667
x_ucl = 11.32704762
x_lcl = 8.844380952
mr_ucl = 1.5246
mr_lcl = 0
  by: Individuals and moving range (I-MR) chart limits  (moving ranges |x_i - x_(i-1)|; limits mean +/- 2.66 MR-bar)
  assumes: measurements in time order; constants E2 = 2.66 (3/d2) and D4 = 3.267 for span 2
```

**What the worked example means:** This calculation determines the control limits for an Individuals and Moving Range chart based on sequential measurements. The center line for the individuals chart is the mean of all observations, calculated as 10.0857, while the average moving range between consecutive points is 0.466667. Using these values, the upper control limit for the individuals chart is found to be 11.327 and the lower control limit is 8.84438. For the moving range chart itself, the upper control limit is 1.5246 and the lower control limit is fixed at zero.

<a id="p_chart"></a>

#### p chart limits (fraction defective), `p_chart`

Centre line and 3-sigma limits of a p chart from counts of defective items in samples of constant size n.

$$
\bar{p} \pm 3\sqrt{\frac{\bar{p}(1-\bar{p})}{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `defectives` | in | number of defective items in each sample | - |
| `n` | in | sample size (constant) | - |
| `center` | out | p-bar | - |
| `ucl` | out | upper control limit (at most 1) | - |
| `lcl` | out | lower control limit (at least 0) | - |
| `sigma` | out | sqrt(p-bar (1 - p-bar)/n) | - |

**Applies when:** constant sample size; LCL truncated at 0.

```shell
$ statistics solve p_chart "defectives=[4, 6, 3, 5, 7, 2, 5]" n=100
center = 0.04571428571
ucl = 0.108373715
lcl = 0
sigma = 0.02088647644
  by: p chart limits (fraction defective)  (p-bar +/- 3 sqrt(p-bar(1 - p-bar)/n))
  assumes: constant sample size; LCL truncated at 0
```

**What the worked example means:** This calculation determines the upper and lower control limits for a p chart, which monitors the proportion of defective items in samples of constant size. The center line represents the average fraction defective, denoted here as 0.0457143. The limits are derived by adding and subtracting three standard deviations from this mean. The resulting upper control limit is 0.108374, and the lower control limit is 0. These boundaries help practitioners distinguish between common cause variation and special cause variation in the production process.

<a id="process_capability"></a>

#### Process capability (Cp, Cpk) and expected nonconforming, `process_capability`

Process capability indices Cp, Cpk (with Cpu, Cpl) and the expected parts per million outside the specification for a normal process with mean mu and standard deviation sigma.

$$
C_p = \frac{USL - LSL}{6\sigma},\quad C_{pk} = \min\left(\frac{USL - \mu}{3\sigma}, \frac{\mu - LSL}{3\sigma}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | process mean | - |
| `sigma` | in | process standard deviation | - |
| `lsl` | in | lower specification limit | - |
| `usl` | in | upper specification limit | - |
| `cp` | out | (USL - LSL)/(6 sigma) | - |
| `cpk` | out | min(Cpu, Cpl) | - |
| `cpu` | out | (USL - mu)/(3 sigma) | - |
| `cpl` | out | (mu - LSL)/(3 sigma) | - |
| `ppm_out` | out | expected nonconforming parts per million | - |

**Applies when:** normal, stable (in-control) process; USL > LSL.

```shell
$ statistics solve process_capability mu=10.2 sigma=0.1 lsl=9.8 usl=10.5
cp = 1.166666667
cpk = 1
cpu = 1
cpl = 1.333333333
ppm_out = 1381.569273
  by: Process capability (Cp, Cpk) and expected nonconforming  (capability ratios; normal tail areas for the ppm)
  mu = 10.2
  sigma = 0.1
  lsl = 9.8
  usl = 10.5
  assumes: normal, stable (in-control) process; USL > LSL
```

**What the worked example means:** This calculation determines how well a stable process meets its specification limits. The Cp index of 1.16667 measures potential capability based on the spread between the upper and lower specification limits divided by six times the standard deviation. The Cpk index of 1 accounts for centering by taking the minimum of the upper and lower capability indices, reflecting the actual performance given the mean of 10.2. With a standard deviation of 0.1, the expected number of defective parts per million is calculated as 1381.57, assuming a normal distribution.

<a id="xbar_r_chart"></a>

#### X-bar and R control chart limits: `xbar_r_chart`

Centre lines and 3-sigma control limits of X-bar and R charts from subgroups (rows) of size 2 to 10.

$$
UCL_{\bar{x}} = \bar{\bar{x}} + A_2\bar{R},\quad UCL_R = D_4\bar{R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `data` | in | one row per subgroup, one column per observation (subgroup size 2-10) | - |
| `center_xbar` | out | grand mean | - |
| `mean_range` | out | average range | - |
| `xbar_ucl` | out | X-bar UCL | - |
| `xbar_lcl` | out | X-bar LCL | - |
| `r_ucl` | out | R chart UCL | - |
| `r_lcl` | out | R chart LCL | - |
| `A2` | out | A2 | - |
| `D3` | out | D3 | - |
| `D4` | out | D4 | - |

**Applies when:** equal subgroup sizes 2 to 10; standard 3-decimal control chart constants (A2, D3, D4).

```shell
$ statistics solve xbar_r_chart "data=[[10.2, 9.9, 10.1, 10], [10.3, 10.1, 9.8, 10.2], [9.9, 10, 10.1, 9.7]]"
center_xbar = 10.025
mean_range = 0.4
xbar_ucl = 10.3166
xbar_lcl = 9.7334
r_ucl = 0.9128
r_lcl = 0
A2 = 0.729
D3 = 0
D4 = 2.282
  by: X-bar and R control chart limits  (grand mean +/- A2 R-bar; R chart limits D3 R-bar and D4 R-bar)
  assumes: equal subgroup sizes 2 to 10; standard 3-decimal control chart constants (A2, D3, D4)
```

**What the worked example means:** This calculation determines the upper and lower control limits for both the X-bar chart and the R chart based on subgroup data. The center line for the mean chart is the grand mean, which equals 10.025. The average range across all subgroups is 0.4. Using standard constants A2, D3, and D4, the upper control limit for the mean is 10.3166 and the lower is 9.7334. For the range chart, the upper limit is 0.9128 and the lower limit is 0. These boundaries help distinguish between common cause variation and special cause signals in the process.

<a id="xbar_s_chart"></a>

#### X-bar and S control chart limits: `xbar_s_chart`

Centre lines and 3-sigma control limits of X-bar and S charts from subgroups (rows) of size 2 to 10.

$$
UCL_{\bar{x}} = \bar{\bar{x}} + A_3\bar{s},\quad UCL_s = B_4\bar{s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `data` | in | one row per subgroup, one column per observation (subgroup size 2-10) | - |
| `center_xbar` | out | grand mean | - |
| `mean_sd` | out | average subgroup standard deviation | - |
| `xbar_ucl` | out | X-bar UCL | - |
| `xbar_lcl` | out | X-bar LCL | - |
| `s_ucl` | out | S chart UCL | - |
| `s_lcl` | out | S chart LCL | - |
| `A3` | out | A3 | - |
| `B3` | out | B3 | - |
| `B4` | out | B4 | - |

**Applies when:** equal subgroup sizes 2 to 10; standard 3-decimal constants (A3, B3, B4).

```shell
$ statistics solve xbar_s_chart "data=[[10.2, 9.9, 10.1, 10], [10.3, 10.1, 9.8, 10.2], [9.9, 10, 10.1, 9.7]]"
center_xbar = 10.025
mean_sd = 0.1719688825
xbar_ucl = 10.30496534
xbar_lcl = 9.745034659
s_ucl = 0.3896814878
s_lcl = 0
A3 = 1.628
B3 = 0
B4 = 2.266
  by: X-bar and S control chart limits  (grand mean +/- A3 s-bar; S chart limits B3 s-bar and B4 s-bar)
  assumes: equal subgroup sizes 2 to 10; standard 3-decimal constants (A3, B3, B4)
```

**What the worked example means:** This calculation determines the center lines and three-sigma control limits for an X-bar and S chart based on subgroup data. The grand mean, which serves as the center line for the X-bar chart, is calculated as 10.025. The average standard deviation of the subgroups, acting as the center line for the S chart, is 0.171969. Using the appropriate constants for the subgroup size, the upper and lower control limits for the means are derived as 10.305 and 9.74503, respectively. Similarly, the limits for the variability chart are computed as 0.389681 and 0. These boundaries help identify whether the process mean and variation are within statistical control.

### Regression and correlation (10)

<a id="correlation_confidence_interval"></a>

#### Confidence interval for a correlation (Fisher z), `correlation_confidence_interval`

The Fisher z-transformation confidence interval for the population correlation from paired data.

$$
\tanh\left(\operatorname{atanh}(r) \pm \frac{z_{\alpha/2}}{\sqrt{n-3}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `confidence` | in | confidence level as a fraction (0.95 for 95%) | - |
| `r` | out | sample correlation | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |

**Applies when:** bivariate normal data; at least 4 pairs; |r| < 1.

```shell
$ statistics solve correlation_confidence_interval "x=[1, 2, 3, 4, 5, 6]" "y=[2, 3, 5, 4, 6, 7]" confidence=0.95
r = 0.9428571429
lower = 0.5591491951
upper = 0.9938998356
  by: Confidence interval for a correlation (Fisher z)  (tanh(atanh(r) +/- z_(alpha/2) / sqrt(n - 3)))
  confidence = 0.95
  assumes: bivariate normal data; at least 4 pairs; |r| < 1
```

**What the worked example means:** This calculation determines the confidence interval for the population correlation based on paired data. The sample correlation, denoted as 0.942857, serves as the central estimate. Applying the Fisher z-transformation accounts for the non-normal distribution of correlation coefficients. The resulting lower and upper bounds, 0.559149 and 0.9939, define the range where the true population correlation likely resides given the specified confidence level.

<a id="covariance"></a>

#### Covariance: `covariance`

The sample (n - 1) and population (n) covariance of paired data.

$$
s_{xy} = \frac{1}{n-1}\sum (x_i - \bar{x})(y_i - \bar{y})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `sample_covariance` | out | divisor n - 1 | - |
| `population_covariance` | out | divisor n | - |

```shell
$ statistics solve covariance "x=[2, 4, 6, 8]" "y=[1, 3, 2, 5]"
sample_covariance = 3.666666667
population_covariance = 2.75
  by: Covariance  (sum of products of deviations divided by n - 1 (sample) or n (population))
```

**What the worked example means:** This calculation determines how two variables, x and y, change together. The sample covariance, 3.66667, uses a divisor of n minus one to estimate the relationship from observed data pairs. The population covariance, 2.75, uses a divisor of n, representing the exact variance for the entire dataset. A positive value indicates that higher values of x tend to accompany higher values of y, while a negative value suggests an inverse relationship where high x values pair with low y values.

<a id="durbin_watson"></a>

#### Durbin-Watson statistic: `durbin_watson`

The Durbin-Watson statistic of regression residuals (about 2: no first-order autocorrelation; towards 0: positive; towards 4: negative).

$$
d = \frac{\sum_{t=2}^{n} (e_t - e_{t-1})^2}{\sum_{t=1}^{n} e_t^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `residuals` | in | regression residuals in time order | - |
| `durbin_watson` | out | sum of squared successive differences / sum of squares | - |

**Applies when:** residuals in time order.

```shell
$ statistics solve durbin_watson "residuals=[0.5, -0.2, 0.3, -0.4, 0.1, 0.6, -0.5]"
durbin_watson = 2.534482759
  by: Durbin-Watson statistic  (d = sum (e_t - e_(t-1))^2 / sum e_t^2)
  assumes: residuals in time order
```

**What the worked example means:** This calculation determines the Durbin-Watson statistic by comparing the sum of squared differences between consecutive regression residuals to the total sum of squared residuals. The resulting value, 2.53448, helps assess whether errors are correlated over time. A result near two suggests no significant first-order autocorrelation, while values approaching zero indicate positive correlation and those nearing four suggest negative correlation. This metric relies on the input residuals being strictly ordered in time sequence to ensure validity.

<a id="kendall_tau"></a>

#### Kendall's tau-b rank correlation: `kendall_tau`

Kendall's tau-b (tie-adjusted) between paired data with its two-sided large-sample p-value.

$$
\tau_b = \frac{C - D}{\sqrt{(n_0 - n_1)(n_0 - n_2)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `tau` | out | Kendall's tau-b | - |
| `p_value` | out | two-sided p-value (normal approximation with tie correction) | - |
| `concordant` | out | concordant pairs | - |
| `discordant` | out | discordant pairs | - |

**Applies when:** paired ordinal or continuous data; large-sample normal approximation for the p-value.

```shell
$ statistics solve kendall_tau "x=[1, 2, 3, 4, 5, 6]" "y=[2, 1, 4, 3, 6, 5]"
tau = 0.6
p_value = 0.09087393999
concordant = 12
discordant = 3
  by: Kendall's tau-b rank correlation  (count concordant and discordant pairs; tau-b = (C - D)/sqrt((n0 - n1)(n0 - n2)); tie-corrected variance of C - D)
  assumes: paired ordinal or continuous data; large-sample normal approximation for the p-value
```

**What the worked example means:** This calculation determines the Kendall's tau-b coefficient, a measure of association for paired ordinal or continuous data. It counts the number of concordant pairs, where both variables increase together, and discordant pairs, where one increases while the other decreases. The result, 0.6, reflects the difference between these counts normalized by the total possible pairs adjusted for ties. A value close to one indicates strong positive association, while a value near negative one indicates strong negative association. The associated two-sided p-value is 0.0908739.

<a id="multiple_linear_regression"></a>

#### Multiple linear regression: `multiple_linear_regression`

Ordinary least squares with several predictors: coefficients (intercept first), R squared, adjusted R squared and the overall F-test.

$$
\hat{\beta} = (X^\top X)^{-1}X^\top y,\quad R^2 = 1 - \frac{SSE}{SST}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X` | in | predictor matrix: one row per observation, one column per predictor | - |
| `y` | in | response values | - |
| `coefficients` | out | [intercept, b1, ..., bk] | - |
| `r_squared` | out | R squared | - |
| `adj_r_squared` | out | adjusted R squared | - |
| `f` | out | overall F statistic | - |
| `f_p_value` | out | p-value of the F test | - |
| `residual_se` | out | residual standard error | - |

**Applies when:** n > k + 1; predictors not collinear; inference assumes independent normal errors.

```shell
$ statistics solve multiple_linear_regression "X=[[1, 2], [2, 1], [3, 4], [4, 3], [5, 6], [6, 5]]" "y=[10, 12, 15, 19, 22, 25]"
coefficients = [6.229166666666668,3.0625,0.06249999999999978]
r_squared = 0.9936585366
adj_r_squared = 0.9894308943
f = 235.0384615
f_p_value = 0.0005049916962
residual_se = 0.6009252126
  by: Multiple linear regression  (centred normal equations solved by Gaussian elimination with partial pivoting; F = (SSR/k)/(SSE/(n-k-1)))
  assumes: n > k + 1; predictors not collinear; inference assumes independent normal errors
```

**What the worked example means:** This analysis computes ordinary least squares estimates for multiple linear regression using the predictor matrix and response values. The resulting coefficients indicate the estimated effect of each predictor while holding others constant. The R-squared value of 0.993659 represents the proportion of total variation in the response explained by the model, while the adjusted R-squared of 0.989431 accounts for the number of predictors used. The overall F-test statistic is 235.038, with a corresponding p-value of 0.000504992, assessing whether the set of predictors collectively has a significant relationship with the response.

<a id="pearson_correlation"></a>

#### Pearson correlation coefficient: `pearson_correlation`

Pearson's correlation coefficient r between paired data, r squared, and the two-sided p-value for H0: rho = 0.

$$
r = \frac{\sum (x_i - \bar{x})(y_i - \bar{y})}{\sqrt{\sum (x_i - \bar{x})^2 \sum (y_i - \bar{y})^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `r` | out | correlation coefficient | - |
| `r_squared` | out | coefficient of determination | - |
| `p_value` | out | two-sided p-value (t with n - 2 df) | - |
| `n` | out | number of pairs | - |

**Applies when:** paired observations; linear relationship; the p-value assumes bivariate normality.

```shell
$ statistics solve pearson_correlation "x=[1, 2, 3, 4, 5]" "y=[2, 4, 5, 4, 5]"
r = 0.7745966692
r_squared = 0.6
p_value = 0.1240270627
n = 5
  by: Pearson correlation coefficient  (r = Sxy / sqrt(Sxx Syy); t = r sqrt((n - 2)/(1 - r^2)))
  assumes: paired observations; linear relationship; the p-value assumes bivariate normality
```

**What the worked example means:** This calculation determines the Pearson correlation coefficient, denoted as r, which measures the strength and direction of the linear relationship between the paired observations of variable x and variable y. The resulting value of 0.774597 indicates how closely the data points align along a straight line, ranging from negative one to positive one. Additionally, the square of this coefficient, 0.6, represents the proportion of variance in one variable that is predictable from the other. A statistical test was performed using the number of pairs, 5, to derive a two-sided p-value of 0.124027, which assesses whether the observed correlation is significantly different from zero under the assumption of bivariate normality.

<a id="regression_prediction"></a>

#### Prediction from a least squares line: `regression_prediction`

The predicted y at a given x0 from the least-squares line fitted to paired data.

$$
\hat{y}_0 = b_0 + b_1 x_0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `x0` | in | the x value to predict at | - |
| `y_hat` | out | b0 + b1 x0 | - |
| `slope` | out | slope | - |
| `intercept` | out | intercept | - |

**Applies when:** x0 within the range of the data (extrapolation is unreliable).

```shell
$ statistics solve regression_prediction "x=[1, 2, 3, 4, 5]" "y=[2.2, 4.1, 6.1, 7.9, 10.2]" x0=3.5
y_hat = 7.09
slope = 1.98
intercept = 0.16
  by: Prediction from a least squares line  (fit by least squares, evaluate at x0)
  x0 = 3.5
  assumes: x0 within the range of the data (extrapolation is unreliable)
```

**What the worked example means:** This calculation determines the estimated response for a specific input by applying the linear model derived from paired data. Using the fitted line, the prediction at the given point is computed as 7.09, which relies on the established slope of 1.98 and the vertical intercept of 0.16. This result represents the most likely outcome within the observed range, assuming a consistent linear trend between the variables.

<a id="correlation_t_test"></a>

#### Significance test for a correlation coefficient, `correlation_t_test`

The t-test of H0: rho = 0 from a sample correlation r and the number of pairs n.

$$
t = r\sqrt{\frac{n-2}{1-r^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | sample correlation coefficient | - |
| `n` | in | number of pairs (>= 3) | - |
| `alternative` | in | alternative hypothesis: two-sided (default), greater or less (optional) | - |
| `t` | out | r sqrt((n - 2)/(1 - r^2)) | - |
| `df` | out | n - 2 | - |
| `p_value` | out | p-value | - |

**Applies when:** -1 < r < 1; bivariate normal data.

```shell
$ statistics solve correlation_t_test r=0.45 n=30 alternative=two-sided
t = 2.666405421
df = 28
p_value = 0.01259107128
  by: Significance test for a correlation coefficient  (t statistic with n - 2 degrees of freedom)
  r = 0.45
  alternative = two-sided
  assumes: -1 < r < 1; bivariate normal data
```

**What the worked example means:** This procedure tests whether the observed sample correlation is significantly different from zero. Using the input correlation coefficient 0.45 and the number of pairs 30, it calculates a t statistic 2.66641 based on the formula involving the square root of the ratio of degrees of freedom to one minus the squared correlation. The resulting degrees of freedom are 28, and the probability of observing such an extreme result by chance under the null hypothesis is given by the p-value 0.0125911. A small p-value suggests evidence against the null hypothesis of no linear relationship.

<a id="simple_linear_regression"></a>

#### Simple linear regression (least squares line), `simple_linear_regression`

The least-squares line y = intercept + slope x: coefficients, r, r squared, standard errors and the slope's p-value.

$$
\hat{y} = b_0 + b_1 x,\quad b_1 = \frac{S_{xy}}{S_{xx}},\quad b_0 = \bar{y} - b_1\bar{x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `slope` | out | b1 = Sxy/Sxx | - |
| `intercept` | out | b0 = ybar - b1 xbar | - |
| `r` | out | correlation | - |
| `r_squared` | out | coefficient of determination | - |
| `p_value` | out | two-sided p-value for H0: slope = 0 | - |
| `slope_se` | out | standard error of the slope | - |
| `intercept_se` | out | standard error of the intercept | - |
| `residual_se` | out | residual standard error | - |

**Applies when:** at least 3 pairs, x not all equal; inference assumes independent normal errors with constant variance.

```shell
$ statistics solve simple_linear_regression "x=[1, 2, 3, 4, 5]" "y=[2.2, 4.1, 6.1, 7.9, 10.2]"
slope = 1.98
intercept = 0.16
r = 0.9992865514
r_squared = 0.9985736118
p_value = 2.287343586e-05
slope_se = 0.04320493799
intercept_se = 0.1432945684
residual_se = 0.1366260102
  by: Simple linear regression (least squares line)  (ordinary least squares; t test on the slope with n - 2 df)
  assumes: at least 3 pairs, x not all equal; inference assumes independent normal errors with constant variance
```

**What the worked example means:** This calculation determines the best-fit straight line through your paired data by minimizing the sum of squared vertical distances. The resulting equation uses an intercept of 0.16 and a slope of 1.98. The correlation coefficient is 0.999287, indicating how strongly the variables move together, while r-squared is 0.998574, showing the proportion of variation in y explained by x. To assess if this linear relationship is statistically significant, the test checks whether the true slope differs from zero. With a standard error of 0.0432049 for the slope, the two-sided p-value is 2.28734e-05. A small p-value suggests strong evidence against the null hypothesis that there is no linear association between the variables.

<a id="spearman_correlation"></a>

#### Spearman rank correlation: `spearman_correlation`

Spearman's rank correlation coefficient (ties given average ranks) with its two-sided p-value.

$$
\rho_s = r\left(\operatorname{rank}(x), \operatorname{rank}(y)\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | values of the first variable (x) | - |
| `y` | in | paired values of the second variable (y) | - |
| `rho` | out | Spearman's rho | - |
| `p_value` | out | two-sided p-value (t approximation, n - 2 df) | - |
| `n` | out | pairs | - |

**Applies when:** paired ordinal or continuous data; p-value by the t approximation.

```shell
$ statistics solve spearman_correlation "x=[10, 20, 30, 40, 50]" "y=[12, 25, 22, 41, 48]"
rho = 0.9
p_value = 0.03738607347
n = 5
  by: Spearman rank correlation  (Pearson correlation of the average ranks)
  assumes: paired ordinal or continuous data; p-value by the t approximation
```

**What the worked example means:** This calculation determines the Spearman rank correlation coefficient, which assesses the strength and direction of a monotonic relationship between two paired variables using their average ranks. The resulting value of 0.9 suggests how closely the ranked order of one variable matches the ranked order of the other. Based on the sample size of 5 pairs, the associated two-sided p-value is 0.0373861, which helps determine if this observed association is statistically significant under the assumption of independent observations.

### Reliability and survival (7)

<a id="exponential_reliability"></a>

#### Exponential reliability (constant failure rate), `exponential_reliability`

Reliability R(t) = exp(-lambda t), unreliability and MTTF for a constant failure rate (given the rate or the MTBF).

$$
R(t) = e^{-\lambda t},\quad MTTF = 1/\lambda
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | mission time | - |
| `rate` | in | failure rate lambda (optional) | - |
| `mtbf` | in | mean time between failures (instead of the rate) (optional) | - |
| `reliability` | out | exp(-lambda t) | - |
| `unreliability` | out | 1 - R(t) | - |
| `mttf` | out | 1/lambda | - |
| `failure_rate` | out | lambda | - |

**Applies when:** constant failure rate (useful-life period); exactly one of rate or MTBF.

```shell
$ statistics solve exponential_reliability t=1000 rate=0.0002
reliability = 0.8187307531
unreliability = 0.1812692469
mttf = 5000
failure_rate = 0.0002
  by: Exponential reliability (constant failure rate)  (R(t) = exp(-lambda t))
  t = 1000
  rate = 0.0002
  assumes: constant failure rate (useful-life period); exactly one of rate or MTBF
```

**What the worked example means:** This calculation determines the probability that the component survives without failure for the specified duration, assuming a constant failure rate during its useful life. Using the provided parameters, the resulting reliability is 0.818731, which means there is a 0.181269 chance of failure within that period. The mean time to failure is derived as 5000, representing the expected operational duration before a failure occurs under these steady-state conditions.

<a id="k_out_of_n_reliability"></a>

#### k-out-of-n system reliability, `k_out_of_n_reliability`

The reliability of a system that works when at least k of n identical independent components work.

$$
R = \sum_{j=k}^{n}\binom{n}{j} r^j (1-r)^{n-j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of components | - |
| `k` | in | number that must work | - |
| `r` | in | reliability of one component | - |
| `reliability` | out | P(at least k of n work) | - |

**Applies when:** identical, independent components.

```shell
$ statistics solve k_out_of_n_reliability n=3 k=2 r=0.9
reliability = 0.972
  by: k-out-of-n system reliability  (binomial upper tail)
  r = 0.9
  assumes: identical, independent components
```

**What the worked example means:** This calculation determines the probability that a k-out-of-n system operates successfully. It uses the binomial upper tail method to sum the probabilities that at least 2 out of 3 identical, independent components function, given a single component reliability of 0.9. The result, 0.972, represents the overall chance that the system meets its operational threshold.

<a id="kaplan_meier"></a>

#### Kaplan-Meier survival estimate: `kaplan_meier`

The Kaplan-Meier (product-limit) estimate of the survival function from times with event/censoring indicators, evaluated at time t, and the whole step curve.

$$
\hat{S}(t) = \prod_{t_i \le t}\left(1 - \frac{d_i}{n_i}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times | - |
| `events` | in | 1 = event observed, 0 = censored | - |
| `t` | in | time at which to report S(t) | - |
| `survival` | out | S(t) | - |
| `event_times` | out | distinct event times | - |
| `survival_curve` | out | S just after each event time | - |

**Applies when:** non-informative censoring; a censoring at an event time counts as still at risk.

```shell
$ statistics solve kaplan_meier "times=[6, 6, 6, 7, 10, 13, 16, 22, 23]" "events=[1, 1, 1, 0, 1, 1, 1, 1, 1]" t=12
survival = 0.5333333333
event_times = [6.0,10.0,13.0,16.0,22.0,23.0]
survival_curve = [0.6666666666666667,0.5333333333333334,0.4000000000000001,0.2666666666666668,0.1333333333333334,0.0]
  by: Kaplan-Meier survival estimate  (product over event times of (1 - deaths / number at risk))
  t = 12
  assumes: non-informative censoring; a censoring at an event time counts as still at risk
```

**What the worked example means:** This calculation determines the Kaplan-Meier estimate of the survival function, which represents the probability that a subject survives beyond a specific time point. Using follow-up times and event indicators, the method computes a product limit over distinct event times up to 12. The resulting survival probability is 0.533333, indicating the proportion of the initial cohort expected to remain event-free at that moment.

<a id="log_rank_test"></a>

#### Log-rank test comparing two survival curves, `log_rank_test`

The log-rank (Mantel-Cox) test of equal survival in two groups from times and event indicators.

$$
\chi^2 = \frac{\left(\sum (d_{1i} - e_{1i})\right)^2}{\sum v_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times1` | in | times, group 1 | - |
| `events1` | in | events (1/0), group 1 | - |
| `times2` | in | times, group 2 | - |
| `events2` | in | events (1/0), group 2 | - |
| `chi_square` | out | (O1 - E1)^2 / V | - |
| `p_value` | out | chi-square p-value (1 df) | - |
| `observed_1` | out | events in group 1 | - |
| `expected_1` | out | expected events in group 1 | - |

**Applies when:** non-informative censoring; proportional hazards gives the test its best power.

```shell
$ statistics solve log_rank_test "times1=[6, 6, 6, 7, 10, 13, 16]" "events1=[1, 1, 1, 0, 1, 1, 1]" "times2=[1, 1, 2, 2, 3, 4, 4]" "events2=[1, 1, 1, 1, 1, 1, 1]"
chi_square = 13.81148343
p_value = 0.0002020972232
observed_1 = 6
expected_1 = 10.42222222
  by: Log-rank test comparing two survival curves  (at each event time: expected events and hypergeometric variance; sum over times)
  assumes: non-informative censoring; proportional hazards gives the test its best power
```

**What the worked example means:** This calculation assesses whether the survival distributions of two groups differ significantly. It aggregates the difference between observed and expected event counts across all unique event times, using hypergeometric variance for each step. The resulting chi-square statistic of 13.8115 is evaluated against a distribution with one degree of freedom to produce a p-value of 0.000202097. A small p-value suggests that the probability of observing such a discrepancy by chance is low, indicating distinct survival curves.

<a id="parallel_system_reliability"></a>

#### Reliability of a parallel system: `parallel_system_reliability`

The reliability of independent components in parallel (at least one must work).

$$
R_p = 1 - \prod (1 - R_i)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | component reliabilities (0 to 1) | - |
| `reliability` | out | system reliability | - |

**Applies when:** independent components.

```shell
$ statistics solve parallel_system_reliability "r=[0.95, 0.9, 0.98]"
reliability = 0.9999
  by: Reliability of a parallel system  (product rule on the unreliabilities)
  assumes: independent components
```

**What the worked example means:** This calculation determines the overall reliability of a parallel system, where the system functions if at least one component works. It uses the product rule on unreliabilities, assuming independent components. Given the input component reliabilities [0.95, 0.9, 0.98], the resulting system reliability is 0.9999. This value represents the probability that the redundant configuration will successfully operate.

<a id="series_system_reliability"></a>

#### Reliability of a series system: `series_system_reliability`

The reliability of independent components in series (all must work).

$$
R_s = \prod R_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | component reliabilities (0 to 1) | - |
| `reliability` | out | system reliability | - |

**Applies when:** independent components.

```shell
$ statistics solve series_system_reliability "r=[0.95, 0.9, 0.98]"
reliability = 0.8379
  by: Reliability of a series system  (product rule)
  assumes: independent components
```

**What the worked example means:** This calculation determines the overall reliability of a series system by multiplying the reliabilities of all independent components. The result, 0.8379, represents the probability that every component functions successfully simultaneously. Since each component must work for the system to operate, the total reliability is always less than or equal to the lowest individual component reliability, illustrating how adding more components in series reduces the overall chance of success.

<a id="weibull_reliability"></a>

#### Weibull reliability, hazard rate and MTTF: `weibull_reliability`

Reliability R(t), unreliability, hazard rate and mean time to failure of a two-parameter Weibull distribution.

$$
R(t) = e^{-(t/\eta)^\beta},\quad h(t) = \frac{\beta}{\eta}\left(\frac{t}{\eta}\right)^{\beta-1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | shape parameter beta | - |
| `eta` | in | scale parameter eta (characteristic life) | - |
| `t` | in | time | - |
| `reliability` | out | exp(-(t/eta)^beta) | - |
| `unreliability` | out | 1 - R(t) | - |
| `hazard_rate` | out | (beta/eta)(t/eta)^(beta-1) | - |
| `mttf` | out | eta Gamma(1 + 1/beta) | - |

**Applies when:** two-parameter Weibull (no location shift); t >= 0.

```shell
$ statistics solve weibull_reliability beta=2 eta=1000 t=500
reliability = 0.7788007831
unreliability = 0.2211992169
hazard_rate = 0.001
mttf = 886.2269255
  by: Weibull reliability, hazard rate and MTTF  (closed forms of the Weibull survival, hazard and mean)
  beta = 2
  eta = 1000
  t = 500
  assumes: two-parameter Weibull (no location shift); t >= 0
```

**What the worked example means:** This calculation determines the probability that a system survives beyond time 500, known as reliability, which is 0.778801. The corresponding unreliability, or cumulative distribution function, is 0.221199. Additionally, the instantaneous hazard rate at this specific time is computed as 0.001. The long-term average lifespan, referred to as the mean time to failure, is estimated to be 886.227 based on the provided shape and scale parameters.

### Sampling distributions (2)

<a id="sample_mean_probability_above"></a>

#### Probability that a sample mean is above a value, `sample_mean_probability_above`

P(sample mean > x) for samples of size n from a population with mean mu and standard deviation sigma (central limit theorem).

$$
P(\bar{X} < x) = \Phi\left(\frac{x - \mu}{\sigma/\sqrt{n}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value for the sample mean | - |
| `mu` | in | population mean | - |
| `sigma` | in | population standard deviation | - |
| `n` | in | sample size | - |
| `probability` | out | the probability | - |
| `z` | out | z = (x - mu)/(sigma/sqrt(n)) | - |
| `standard_error` | out | sigma/sqrt(n) | - |

**Applies when:** population normal, or n large enough for the central limit theorem; independent observations.

```shell
$ statistics solve sample_mean_probability_above mu=50 sigma=10 n=25 x=52
probability = 0.1586552539
z = 1
standard_error = 2
  by: Probability that a sample mean is above a value  (standard error sigma/sqrt(n), then the normal probability of the z-score)
  mu = 50
  sigma = 10
  x = 52
  assumes: population normal, or n large enough for the central limit theorem; independent observations
```

**What the worked example means:** This calculation finds the chance that an average from a sample of size 25 drawn from a population with mean 50 and standard deviation 10 will be greater than 52. It first determines the standard error, which is 2, then converts the difference between the target value and the population mean into a z-score of 1. The final result, 0.158655, represents the area under the normal curve to the right of this z-score.

<a id="sample_mean_probability_below"></a>

#### Probability that a sample mean is below a value, `sample_mean_probability_below`

P(sample mean < x) for samples of size n from a population with mean mu and standard deviation sigma (central limit theorem).

$$
P(\bar{X} < x) = \Phi\left(\frac{x - \mu}{\sigma/\sqrt{n}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | the value for the sample mean | - |
| `mu` | in | population mean | - |
| `sigma` | in | population standard deviation | - |
| `n` | in | sample size | - |
| `probability` | out | the probability | - |
| `z` | out | z = (x - mu)/(sigma/sqrt(n)) | - |
| `standard_error` | out | sigma/sqrt(n) | - |

**Applies when:** population normal, or n large enough for the central limit theorem; independent observations.

```shell
$ statistics solve sample_mean_probability_below mu=50 sigma=10 n=25 x=52
probability = 0.8413447461
z = 1
standard_error = 2
  by: Probability that a sample mean is below a value  (standard error sigma/sqrt(n), then the normal probability of the z-score)
  mu = 50
  sigma = 10
  x = 52
  assumes: population normal, or n large enough for the central limit theorem; independent observations
```

**What the worked example means:** This calculation determines the likelihood that the average of a random sample of size 25 falls below the value 52. It uses the population mean 50 and standard deviation 10 to find the standard error, which is 2. The resulting z-score of 1 maps to a cumulative normal probability of 0.841345, indicating how extreme or common such a sample mean would be under these conditions.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
