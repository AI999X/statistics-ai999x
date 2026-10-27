"""statistics-ai999x — offline statistics answers with their formula, steps and verification.

    import statistics_ai999x as m
    m.ask("<a question in words>").show()
    m.formulas()                       # every certified capability

110 certified capabilities, school level to graduate level:
- descriptive statistics: mean, median, mode, range, variance and standard deviation, quartiles, percentiles,
  z-scores, outliers, skewness and kurtosis, and other measures
- probability distributions: binomial, Poisson, geometric, hypergeometric, negative binomial, normal, exponential,
  sampling distributions, and critical values of z, t, chi-square and F
- confidence intervals: means, proportions (Wald and Wilson), differences of means, variances, correlations
- hypothesis tests: z, one-sample, two-sample, Welch, paired and summary t-tests, proportion tests, chi-square
  tests (goodness of fit, independence, variance), F, Levene and Bartlett tests, Fisher's exact and McNemar tests
- nonparametric tests: Mann-Whitney, Wilcoxon signed-rank (large-sample and exact), Kruskal-Wallis, Friedman, sign test
- regression and correlation: Pearson, Spearman, Kendall, simple and multiple regression, prediction, Durbin-Watson
- experimental design: one-way ANOVA, randomized block ANOVA, 2^2 and 2^3 factorial effects, Cronbach's alpha,
  Cohen's d, power and sample size
- quality control and reliability: X-bar/R, X-bar/S, I-MR, p and c chart limits, Cp and Cpk, Weibull and
  exponential reliability, series, parallel and k-out-of-n systems, Kaplan-Meier and the log-rank test

Examples (both run as shown):

    m.ask("Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.").values
    # {'t': 1.8898..., 'df': 5, 'p_value': 0.11738..., ...}
    m.solve("weibull_reliability", beta=2, eta=1000, t=500).values
    # {'reliability': 0.77880..., 'unreliability': 0.22119..., 'hazard_rate': 0.001, 'mttf': 886.22...}

Built by the AI999X model machine from the same engine as every domain. Every capability was checked against
independent statistics libraries during the build. A question that is not fully determined is refused, not guessed.
For example, a one-sided claim needs the tail stated, and a two-sample t-test needs pooled or Welch stated.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.1"
_P = Product("statistics", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
