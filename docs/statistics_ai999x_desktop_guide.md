# Statistics: the desktop app, user guide

**Statistics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Statistics covers descriptive statistics, probability distributions, confidence intervals, hypothesis tests, regression and correlation, experimental design, quality control and reliability. Its desktop app holds all 112 calculations in 16 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-statistics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-statistics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-statistics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-statistics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-statistics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-statistics-1.6.5-linux-x86_64.tar.gz
./ai999x-statistics-1.6.5-linux-x86_64/ai999x-statistics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Statistics app, opened on One-sample t-test with its worked example.](https://www.ai999x.com/?ax_shot=statistics-ai999x/hero.webp)

*The Statistics app, opened on One-sample t-test with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Hypothesis tests › One-sample t-test (or type part of its name in the search box). It opens with a worked example filled in: “Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.” The answer: t = 1.889822365, df = 5, p_value = 0.1173867951, mean = 5.166666667.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![One-sample t-test: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-1.webp)

*One-sample t-test: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: One-sample t-test
t = 1.889822365
df = 5
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Normal probability below a value](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-2.webp)

*Normal probability below a value*

![Confidence interval for a mean from data (t-interval)](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-3.webp)

*Confidence interval for a mean from data (t-interval)*

![Weibull reliability, hazard rate and MTTF](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-4.webp)

*Weibull reliability, hazard rate and MTTF*

![One-way ANOVA](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-5.webp)

*One-way ANOVA*

![Fisher's exact test (2 x 2)](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-6.webp)

*Fisher's exact test (2 x 2)*

![Critical value of chi-square](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-7.webp)

*Critical value of chi-square*

![Arithmetic mean](https://www.ai999x.com/?ax_shot=statistics-ai999x/scene-8.webp)

*Arithmetic mean*

![One-sample t-test: a chart, enlarged.](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-1.webp)

*One-sample t-test: a chart, enlarged.*

![Normal probability below a value: a chart, enlarged.](https://www.ai999x.com/?ax_shot=statistics-ai999x/chart-2.webp)

*Normal probability below a value: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Statistics app in the dark theme.](https://www.ai999x.com/?ax_shot=statistics-ai999x/hero-dark.webp)

*The Statistics app in the dark theme.*

## 7. Every calculation

112 calculations in 16 topics; each has its own entry on the model's page (https://www.ai999x.com/models/statistics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Analysis of variance | One-way ANOVA |
| Categorical data | Fisher's exact test (2 x 2), McNemar's test for paired proportions |
| Confidence intervals | Confidence interval for a mean (sigma known, z-interval), Confidence interval for a mean from data (t-interval), Confidence interval for a mean from summary statistics (t-interval), Confidence interval for a proportion (Wald), Confidence interval for a variance and standard deviation, Confidence interval for the difference of two means, Wilson score interval for a proportion |
| Confidence intervals and tests | Critical value of chi-square, Critical value of F, Critical value of Student's t, Critical value of z |
| Descriptive statistics | Arithmetic mean, Coefficient of variation, Five-number summary and interquartile range, Geometric mean, Harmonic mean, Mean absolute deviation, Mean and standard deviation of a frequency table, Median, Median absolute deviation, Mode, Outliers by the 1.5 IQR rule, Percentile of a data set, Population standard deviation, Population variance, Range, Sample standard deviation, Sample variance, Skewness and kurtosis, Standard error of the mean, Trimmed mean, Weighted mean, z-score of a value |
| Effect sizes | Cohen's d and Hedges' g |
| Experimental design | 2^2 factorial design: main effects and interaction, 2^3 factorial design: main effects and interactions, Cronbach's alpha (internal consistency), Randomized complete block design (two-way ANOVA without replication) |
| Hypothesis tests | Bartlett's test for equal variances, Chi-square goodness-of-fit test, Chi-square test for a variance, Chi-square test of independence, F-test for equality of two variances, Levene's test for equal variances, One-proportion z-test, One-sample t-test, One-sample z-test for a mean (sigma known), Paired t-test, Two-proportion z-test, Two-sample t-test (pooled variances), Two-sample t-test from summary statistics, Welch's two-sample t-test (unequal variances) |
| Nonparametric tests | Friedman test, Kruskal-Wallis H test, Mann-Whitney U test (exact), Mann-Whitney U test (large-sample), Sign test, Wilcoxon signed-rank test (exact), Wilcoxon signed-rank test (large-sample) |
| Power and sample size | Power of a one-sample z-test, Power of a two-sample z-test, Sample size per group to compare two means (z-test power), Sample size to estimate a mean, Sample size to estimate a proportion |
| Probability | Joint, marginal and conditional probabilities from a two-way table, Probability of at least one occurrence, Probability rules: complement, addition and multiplication |
| Probability distributions | Binomial probability: at least k successes, Binomial probability: at most k successes, Binomial probability: exactly k successes, Expected value and variance of a discrete random variable, Exponential probability above a value, Exponential probability below a value, Geometric distribution: first success on trial k, Hypergeometric probability (sampling without replacement), Inverse normal (normal percentile), Mean and standard deviation of a binomial distribution, Negative binomial: r-th success on trial n, Normal probability above a value, Normal probability below a value, Normal probability between two values, Poisson probability: at least k events, Poisson probability: at most k events, Poisson probability: exactly k events |
| Quality control | c chart limits (defects per unit), Individuals and moving range (I-MR) chart limits, p chart limits (fraction defective), Process capability (Cp, Cpk) and expected nonconforming, X-bar and R control chart limits, X-bar and S control chart limits |
| Regression and correlation | Confidence interval for a correlation (Fisher z), Covariance, Durbin-Watson statistic, Kendall's tau-b rank correlation, Multiple linear regression, Pearson correlation coefficient, Prediction from a least squares line, Significance test for a correlation coefficient, Simple linear regression (least squares line), Spearman rank correlation |
| Reliability and survival | Exponential reliability (constant failure rate), k-out-of-n system reliability, Kaplan-Meier survival estimate, Log-rank test comparing two survival curves, Reliability of a parallel system, Reliability of a series system, Weibull reliability, hazard rate and MTTF |
| Sampling distributions | Probability that a sample mean is above a value, Probability that a sample mean is below a value |

## 8. Other ways to use it

The same engine runs on the command line (`statistics`, its own guide), in Python (`pip install statistics-ai999x`, `import statistics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/statistics-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/statistics-ai999x/
