"""Statistics procedures: deterministic algorithms with typed inputs (data samples as vectors, tables as matrices)
and several outputs. Domain code, loaded by the pack; a procedure is exposed only once tools/certify_procedures.py
has certified it.

Every procedure states its inputs, outputs, conditions, method and LaTeX, the documented question forms it reads,
worked examples, an INDEPENDENT reference computation (scipy, statsmodels, pingouin, lifelines, numpy; imported inside
the reference only, used by certification only) and a deliberately wrong variant that certification shows the
checks reject. The shipped code is plain Python: distribution functions (normal, t, chi-square, F, binomial, Poisson,
...) are computed here from the regularized incomplete beta and gamma functions (continued fractions and series) and
math.erfc, and certified against scipy.
"""
from __future__ import annotations

import math
import re
from fractions import Fraction
from itertools import product as _iproduct

PROCEDURES: dict = {}
LEVELS: dict = {}

# ============================================================================================ special functions ====
_TINY = 1e-300
_SQRT2 = math.sqrt(2.0)


def _bcf(a, b, x):
    """Continued fraction of the regularized incomplete beta function (modified Lentz)."""
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    if abs(d) < _TINY:
        d = _TINY
    d = 1.0 / d
    h = d
    for m in range(1, 300000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = _TINY if abs(d) < _TINY else d
        c = 1.0 + aa / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = _TINY if abs(d) < _TINY else d
        c = 1.0 + aa / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 4e-16:
            return h
    raise ArithmeticError("the incomplete beta function did not converge")


def _ibeta(a, b, x, y):
    """(I_x(a, b), 1 - I_x(a, b)); y = 1 - x is passed separately so that neither tail loses precision."""
    if x <= 0.0:
        return 0.0, 1.0
    if y <= 0.0:
        return 1.0, 0.0
    lf = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(y)
    if x < (a + 1.0) / (a + b + 2.0):
        v = math.exp(lf) * _bcf(a, b, x) / a
        return v, 1.0 - v
    v = math.exp(lf) * _bcf(b, a, y) / b
    return 1.0 - v, v


def _igamma(a, x):
    """(P(a, x), Q(a, x)): the regularized lower and upper incomplete gamma functions."""
    if x <= 0.0:
        return 0.0, 1.0
    lf = a * math.log(x) - x - math.lgamma(a)
    if x < a + 1.0:
        ap, s = a, 1.0 / a
        d = s
        for _ in range(300000):
            ap += 1.0
            d *= x / ap
            s += d
            if abs(d) < abs(s) * 1e-17:
                break
        p = s * math.exp(lf)
        return p, 1.0 - p
    b = x + 1.0 - a
    c, d = 1.0 / _TINY, 1.0 / b
    h = d
    for i in range(1, 300000):
        an = -i * (i - a)
        b += 2.0
        d = an * d + b
        d = _TINY if abs(d) < _TINY else d
        c = b + an / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 4e-16:
            break
    q = math.exp(lf) * h
    return 1.0 - q, q


def _phi(z):
    """Standard normal CDF."""
    return 0.5 * math.erfc(-z / _SQRT2)


def _phic(z):
    """Standard normal upper tail 1 - Phi(z), without cancellation."""
    return 0.5 * math.erfc(z / _SQRT2)


def _z(p):
    """Standard normal quantile (Acklam's rational start, refined by Halley steps on the exact CDF)."""
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    if p > 0.5:
        return -_z(1.0 - p)                       # 1 - p is exact for p in [0.5, 1)
    a = (-3.969683028665376e+01, 2.209460984245205e+02, -2.759285104469687e+02, 1.383577518672690e+02,
         -3.066479806614716e+01, 2.506628277459239e+00)
    b = (-5.447609879822406e+01, 1.615858368580409e+02, -1.556989798598866e+02, 6.680131188771972e+01,
         -1.328068155288572e+01)
    c = (-7.784894002430293e-03, -3.223964580411365e-01, -2.400758277161838e+00, -2.549732539343734e+00,
         4.374664141464968e+00, 2.938163982698783e+00)
    d = (7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e+00, 3.754408661907416e+00)
    if p < 0.02425:
        q = math.sqrt(-2.0 * math.log(p))
        x = (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / \
            ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    else:
        q = p - 0.5
        r = q * q
        x = (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q / \
            (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    for _ in range(3):
        e = _phi(x) - p
        u = e * math.sqrt(2.0 * math.pi) * math.exp(x * x / 2.0)
        x = x - u / (1.0 + x * u / 2.0)
    return x


def _t_two_sided(t, df):
    """P(|T| >= |t|) for Student's t with df degrees of freedom."""
    t2 = t * t
    return _ibeta(df / 2.0, 0.5, df / (df + t2), t2 / (df + t2))[0]


def _t_sf(t, df):
    """P(T > t)."""
    if t == 0:
        return 0.5
    half = 0.5 * _t_two_sided(t, df)
    return half if t > 0 else 1.0 - half


def _t_cdf(t, df):
    if t == 0:
        return 0.5
    half = 0.5 * _t_two_sided(t, df)
    return 1.0 - half if t > 0 else half


def _chi2_cdf(x, k):
    return _igamma(k / 2.0, x / 2.0)[0]


def _chi2_sf(x, k):
    return _igamma(k / 2.0, x / 2.0)[1]


def _f_tails(f, d1, d2):
    """(P(F <= f), P(F > f)) for the F distribution with (d1, d2) degrees of freedom."""
    if f <= 0:
        return 0.0, 1.0
    den = d1 * f + d2
    return _ibeta(d1 / 2.0, d2 / 2.0, d1 * f / den, d2 / den)


def _bisect_decreasing(sf, q, lo=0.0, hi=1.0):
    """x > lo with sf(x) = q for a decreasing sf: bracket, then bisect to the resolution of a float."""
    while sf(hi) > q:
        lo, hi = hi, hi * 2.0
        if hi > 1e300:
            raise ArithmeticError("quantile out of range")
    for _ in range(3000):
        mid = 0.5 * (lo + hi)
        if mid <= lo or mid >= hi:
            break
        if sf(mid) > q:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def _bisect_increasing(cdf, p, lo=0.0, hi=1.0):
    while cdf(hi) < p:
        lo, hi = hi, hi * 2.0
        if hi > 1e300:
            raise ArithmeticError("quantile out of range")
    for _ in range(3000):
        mid = 0.5 * (lo + hi)
        if mid <= lo or mid >= hi:
            break
        if cdf(mid) < p:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def _t_ppf(p, df):
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    if p == 0.5:
        return 0.0
    if p < 0.5:
        return -_t_ppf(1.0 - p, df)
    q = 1.0 - p
    return _bisect_decreasing(lambda t: 0.5 * _t_two_sided(t, df), q)


def _chi2_ppf(p, k):
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    if p <= 0.5:
        return _bisect_increasing(lambda x: _chi2_cdf(x, k), p)
    return _bisect_decreasing(lambda x: _chi2_sf(x, k), 1.0 - p)


def _f_ppf(p, d1, d2):
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    if p <= 0.5:
        return _bisect_increasing(lambda x: _f_tails(x, d1, d2)[0], p)
    return _bisect_decreasing(lambda x: _f_tails(x, d1, d2)[1], 1.0 - p)


def _binom_pmf(k, n, p):
    if k < 0 or k > n:
        return 0.0
    return math.exp(math.log(math.comb(n, k)) + k * math.log(p) + (n - k) * math.log1p(-p))


def _binom_cdf(k, n, p):
    return min(1.0, math.fsum(_binom_pmf(j, n, p) for j in range(0, min(k, n) + 1))) if k >= 0 else 0.0


def _binom_sf_ge(k, n, p):
    """P(X >= k)."""
    return min(1.0, math.fsum(_binom_pmf(j, n, p) for j in range(max(k, 0), n + 1))) if k <= n else 0.0


def _pois_pmf(k, lam):
    return math.exp(k * math.log(lam) - lam - math.lgamma(k + 1)) if k >= 0 else 0.0


def _pois_sf_ge(k, lam):
    """P(X >= k) without cancellation: in the upper tail (k > lambda) the terms are summed directly, upward until they
    no longer change the sum (1 - cdf lost every digit there: lambda 39, k 77; 2026-10-23)."""
    if k <= 0:
        return 1.0
    if k <= lam:
        return max(0.0, 1.0 - _pois_cdf(k - 1, lam))
    terms, j = [], k
    while True:
        t = _pois_pmf(j, lam)
        terms.append(t)
        if t < 1e-18 * math.fsum(terms) or t == 0.0:
            break
        j += 1
    return min(1.0, math.fsum(terms))


def _pois_cdf(k, lam):
    return min(1.0, math.fsum(_pois_pmf(j, lam) for j in range(0, k + 1))) if k >= 0 else 0.0


# ================================================================================================ data helpers ====
def _finite(x, name="data"):
    for v in x:
        if not (isinstance(v, (int, float)) and math.isfinite(v)):
            raise ValueError(f"{name} must contain finite numbers only")
    return [float(v) for v in x]


def _need(x, n, name="the sample"):
    if len(x) < n:
        raise ValueError(f"{name} needs at least {n} values")


def _mean(x):
    return math.fsum(x) / len(x)


def _ss(x):
    m = _mean(x)
    return math.fsum((v - m) ** 2 for v in x)


def _var(x, ddof=1):
    return _ss(x) / (len(x) - ddof)


def _sd(x, ddof=1):
    return math.sqrt(_var(x, ddof))


def _median(x):
    s = sorted(x)
    n = len(s)
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2.0


def _quantile(x, p, method="linear"):
    """Sample quantile, 0 <= p <= 1: 'linear' (Hyndman-Fan type 7, the spreadsheet/numpy default) or 'weibull'
    (type 6, position p(n + 1))."""
    s = sorted(x)
    n = len(s)
    h = (n - 1) * p if method == "linear" else (n + 1) * p - 1.0
    if h <= 0:
        return s[0]
    if h >= n - 1:
        return s[-1]
    lo = int(math.floor(h))
    return s[lo] + (h - lo) * (s[lo + 1] - s[lo])


def _ranks(x):
    """Average ranks (1-based) with ties sharing the mean of their positions."""
    idx = sorted(range(len(x)), key=lambda i: x[i])
    r = [0.0] * len(x)
    i = 0
    while i < len(x):
        j = i
        while j + 1 < len(x) and x[idx[j + 1]] == x[idx[i]]:
            j += 1
        for k in range(i, j + 1):
            r[idx[k]] = (i + j) / 2.0 + 1.0
        i = j + 1
    return r


def _tie_counts(x):
    c = {}
    for v in x:
        c[v] = c.get(v, 0) + 1
    return [t for t in c.values() if t > 1]


def _pearson(x, y):
    mx, my = _mean(x), _mean(y)
    sxy = math.fsum((a - mx) * (b - my) for a, b in zip(x, y))
    sxx = math.fsum((a - mx) ** 2 for a in x)
    syy = math.fsum((b - my) ** 2 for b in y)
    if sxx == 0 or syy == 0:
        raise ValueError("a variable is constant: the correlation is undefined")
    r = sxy / math.sqrt(sxx * syy)
    return max(-1.0, min(1.0, r))


def _pair(x, y, n=2):
    x, y = _finite(x, "x"), _finite(y, "y")
    if len(x) != len(y):
        raise ValueError("x and y must have the same number of values (paired data)")
    _need(x, n, "the paired data")
    return x, y


def _groups(*gs):
    out = [_finite(g, "each group") for g in gs if g is not None]
    if len(out) < 2:
        raise ValueError("at least two groups are required")
    return out


def _alt_p(z_or_t, alternative, sf, cdf):
    if alternative == "two-sided":
        return min(1.0, 2.0 * min(sf(z_or_t), cdf(z_or_t)))
    return sf(z_or_t) if alternative == "greater" else cdf(z_or_t)


def _check_alt(alternative):
    if alternative not in ("two-sided", "greater", "less"):
        raise ValueError("alternative must be two-sided, greater or less")


def _prob(p, name="p", open_=True):
    if not isinstance(p, (int, float)) or not math.isfinite(p) or (not 0 < p < 1 if open_ else not 0 <= p <= 1):
        raise ValueError(f"{name} must be a probability {'strictly ' if open_ else ''}between 0 and 1")
    return float(p)


def _conf(c):
    if not isinstance(c, (int, float)) or not 0 < c < 1:
        raise ValueError("the confidence level must be strictly between 0 and 1 (for example 0.95)")
    return float(c)


def _pos(v, name):
    if not isinstance(v, (int, float)) or not math.isfinite(v) or v <= 0:
        raise ValueError(f"{name} must be a positive number")
    return float(v)


def _whole(v, name, lo=0):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or float(v) != int(float(v)) or int(v) < lo:
        raise ValueError(f"{name} must be a whole number >= {lo}")
    return int(v)


def _matrix_ok(m, name="the table", min_rows=2, min_cols=2):
    if not m or any(len(r) != len(m[0]) for r in m):
        raise ValueError(f"{name} must be rectangular")
    if len(m) < min_rows or len(m[0]) < min_cols:
        raise ValueError(f"{name} needs at least {min_rows} rows and {min_cols} columns")
    return [_finite(r, name) for r in m]


# ===================================================================================== comparison (verify) ====
def _diff(a, b):
    """Largest relative difference between two outputs (numbers, lists, dicts, text, None)."""
    if isinstance(a, dict) or isinstance(b, dict):
        if not (isinstance(a, dict) and isinstance(b, dict)) or a.keys() != b.keys():
            return 1.0
        return max((_diff(a[k], b[k]) for k in a), default=0.0)
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max((_diff(u, v) for u, v in zip(a, b)), default=0.0)
    if a is None or b is None:
        return 0.0 if a is b else 1.0
    if isinstance(a, str) or isinstance(b, str):
        return 0.0 if a == b else 1.0
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return 1.0
    if not (math.isfinite(a) and math.isfinite(b)):
        return 0.0 if a == b else 1.0
    # relative, with the same tiny floor as certification: with max(1, |a|, |b|) a wrong tail probability of 1e-12
    # "agreed" with a right one of 2e-12, so verify could not reject the wrong variant (2026-10-23)
    d = abs(a - b)
    return 0.0 if d <= 1e-15 else d / max(abs(a), abs(b))


# ================================================================================ reading documented questions ====
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"
_MATRIX = re.compile(r"\[\s*\[.*?\]\s*\]")
_LIST = re.compile(r"\[([^\[\]]*)\]")
_ROUTES: list = []          # (key, required patterns, avoided patterns, priority)

_DESCRIPTIVE_AVOID = (r"\btest\b|hypothes|confidence|interval|p-value|signific|\bdiffer|compare|probabilit|regression|"
                      r"correlat|anova|chart|control limit|distribution|binomial|poisson|normal|critical|reliab|surviv")


def _norm(question: str) -> str:
    q = question.lower()
    for a, b in (("−", "-"), ("–", "-"), ("σ", "sigma "), ("μ", "mu "), ("≤", "<="), ("≥", ">="), ("α", "alpha "),
                 ("λ", "lambda "), ("β", "beta "), ("η", "eta "), ("×", "x")):
        q = q.replace(a, b)
    return q


def _route(q: str) -> list:
    hits = [(prio, key) for key, need, avoid, prio in _ROUTES
            if all(re.search(p, q) for p in need) and not any(re.search(p, q) for p in avoid)]
    if not hits:
        return []
    best = max(p for p, _k in hits)
    return [k for p, k in hits if p == best]


def _value(q, alias):
    m = re.search(r"(?<![a-z0-9_])(?:" + alias + r")\s*(?:=|:|\bis\b|\bof\b)\s*(" + _NUM + r")(?!\.?\d)", q)
    return float(m.group(1)) if m else None


def _percent_or_fraction(text):
    v = float(text.rstrip("%").strip())
    return v / 100.0 if text.strip().endswith("%") or v > 1 else v


def _read_confidence(q):
    m = re.search(r"(\d+(?:\.\d+)?)\s*%\s*(?:[a-z-]+\s+){0,3}?(?:confidence|c\.?i\b|interval)", q)
    if m:
        return float(m.group(1)) / 100.0
    m = re.search(r"confidence(?: level)?\s*(?:=|:|of|is)?\s*(\d+(?:\.\d+)?|\.\d+)\s*(%?)", q)
    if m:
        v = float(m.group(1))
        return v / 100.0 if m.group(2) or v > 1 else v
    return None


def _read_alpha(q):
    m = re.search(r"(?:alpha|significance level|level of significance)\s*(?:=|:|of|is)?\s*(" + _NUM + r"\s*%?)", q)
    if m:
        return _percent_or_fraction(m.group(1))
    m = re.search(r"(" + _NUM + r")\s*%\s*(?:level of significance|significance|level)", q)
    return float(m.group(1)) / 100.0 if m else None


_DIRECTIONAL = r"\b(greater|less|more|fewer|exceeds?|larger|smaller|higher|lower|increase[sd]?|decrease[sd]?|above|below|at least|at most)\b|[<>]"


def _read_alternative(q):
    """The alternative hypothesis, only when it is stated explicitly; a directional claim without an explicit tail is
    refused (a two-sided p-value for a one-sided question would be wrong by a factor of two)."""
    if re.search(r"right[- ]tailed|upper[- ]tailed|alternative\s*(?:=|:|is)?\s*greater|one-sided \(greater\)", q):
        return "greater"
    if re.search(r"left[- ]tailed|lower[- ]tailed|alternative\s*(?:=|:|is)?\s*less|one-sided \(less\)", q):
        return "less"
    if re.search(r"two[- ]sided|two[- ]tailed|alternative\s*(?:=|:|is)?\s*two", q):
        return "two-sided"
    if re.search(r"one[- ](?:sided|tailed)", q):
        return {"_out_of_scope": "a one-sided test without a direction: say right-tailed or left-tailed"}
    stripped = re.sub(r"\[[^\]]*\]", " ", q)
    stripped = re.sub(r"(?:alpha|significance level|level of significance)\s*(?:=|:|of|is)?\s*" + _NUM, " ", stripped)
    if re.search(_DIRECTIONAL, stripped):
        return {"_out_of_scope": "a one-sided claim: say right-tailed, left-tailed or two-sided"}
    return None


def _make_parse(key, inputs, aliases, extract):
    def parse(question):
        if not isinstance(question, str):
            return None
        q = _norm(question)
        if key not in _route(q):
            return None
        d = {}
        mats = [i for i in inputs if i.get("type") == "matrix"]
        if mats:
            m = _MATRIX.search(q)
            if m:
                d[mats[0]["symbol"]] = m.group(0)
        rest = _MATRIX.sub(" ", q)
        found = list(_LIST.finditer(rest))
        vecs = [i for i in inputs if i.get("type") == "vector"]
        if len(found) > len(vecs):
            return {"_out_of_scope": f"{len(found)} data lists were given; this capability takes at most {len(vecs)}"}
        names = {i["symbol"].lower(): i["symbol"] for i in vecs}
        loose = []
        for m in found:                                   # 'y = [..]' names its input; other lists fill the rest in order
            label = re.search(r"\b([a-z][a-z0-9_]*)\s*(?:=|:)\s*$", rest[:m.start()])
            sym = names.get(label.group(1)) if label else None
            if sym and sym not in d:
                d[sym] = m.group(1)
            else:
                loose.append(m.group(1))
        for i in vecs:
            if i["symbol"] not in d and loose:
                d[i["symbol"]] = loose.pop(0)
        scalar_text = _LIST.sub(" ", rest)
        for i in inputs:
            if i.get("type", "number") in ("number", "integer") and i["symbol"] not in d:
                v = _value(scalar_text, aliases.get(i["symbol"], re.escape(i["symbol"])))
                if v is not None:
                    d[i["symbol"]] = v
        if any(i["symbol"] == "df" for i in inputs) and "df" not in d:
            m = re.search(r"(\d+(?:\.\d+)?)\s+degrees of freedom", scalar_text)
            if m:
                d["df"] = float(m.group(1))
        if extract is not None:
            extra = extract(scalar_text, d)
            if isinstance(extra, dict) and "_out_of_scope" in extra:
                return extra
        return d
    return parse


# ================================================================================================= the builder ====
def X(inputs, outputs=None):
    """A worked example: inputs as a user writes them, and the outputs (filled from a run checked against the
    reference)."""
    return {"inputs": inputs, "outputs": outputs or {}}


def Q(text, outputs=None, outcome=None):
    return {"q": text, "outcome": outcome} if outcome else {"q": text, "outputs": outputs or {}}


def _cap(key, *, level, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong,
         rnd, need, templates, examples, questions, conditions=(), avoid=(), prio=5, aliases=None, extract=None,
         keywords=()):
    aliases = aliases or {}
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": (lambda i, o, _r=run: _diff(_r(**i), {k: v for k, v in o.items()})),
        "parse": _make_parse(key, inputs, aliases, extract), "reference": reference, "wrong": wrong,
        "random_inputs": rnd, "templates": list(templates), "examples": list(examples), "questions": list(questions),
        "keywords": list(keywords)}
    LEVELS[key] = level
    _ROUTES.append((key, [re.compile(p) for p in ([need] if isinstance(need, str) else need)],
                    [re.compile(p) for p in ([avoid] if isinstance(avoid, str) else avoid)], prio))


def V(sym, meaning, min_length=2, required=True, max_length=None):
    d = {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
         "min_length": min_length}
    if max_length:
        d["max_length"] = max_length
    return d


def N(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}


def I(sym, meaning, required=True, lo=0):
    return {"symbol": sym, "type": "integer", "kind": "number", "meaning": meaning, "required": required, "min": lo}


def M(sym, meaning):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": True}


def C(sym, meaning, choices, required=False):
    return {"symbol": sym, "type": "choice", "kind": "number", "meaning": meaning, "required": required,
            "choices": list(choices)}


def O(sym, meaning):
    return {"symbol": sym, "kind": "number", "meaning": meaning}


ALT = C("alternative", "alternative hypothesis: two-sided (default), greater or less", ["two-sided", "greater", "less"])
CONF = N("confidence", "confidence level as a fraction (0.95 for 95%)")
_CONF_ALIAS = {"confidence": r"confidence level|confidence"}


def _x_conf(q, d):
    c = _read_confidence(q)
    if c is not None:
        d["confidence"] = c


def _x_alt(q, d):
    a = _read_alternative(q)
    if isinstance(a, dict):
        return a
    if a is not None:
        d["alternative"] = a


def _x_alpha(q, d):
    a = _read_alpha(q)
    if a is not None:
        d["alpha"] = a


def _chain(*fs):
    def f(q, d):
        for g in fs:
            r = g(q, d)
            if isinstance(r, dict):
                return r
    return f


# ---------------------------------------------------------------------------------------- random samples ----
def _sample(rng, n, mu=50.0, sd=10.0, dec=1):
    return [round(rng.gauss(mu, sd), dec) for _ in range(n)]


def _distinct(rng, n, lo=1, hi=999, scale=10.0):
    return [v / scale for v in rng.sample(range(lo, hi), n)]


# ======================================================================================= descriptive statistics ====
_D = "descriptive statistics"
_AV = _DESCRIPTIVE_AVOID
_OTHER_DESC = (r"geometric|harmonic|trimm|weighted|absolute|standard error|frequenc|deviation|variance|\bmedian\b|"
               r"\bmodes?\b|\brange\b|percentile|quartile|expected|skew|kurt|coefficient")
_X = V("x", "the data values", min_length=1)
_X2 = V("x", "the data values (at least two)", min_length=2)


def _rnd_x(rng, lo=3, hi=30):
    return {"x": _sample(rng, rng.randint(lo, hi), rng.uniform(-20, 80), rng.uniform(0.5, 20), rng.choice([0, 1, 2]))}


def run_mean(x):
    x = _finite(x)
    _need(x, 1)
    return {"mean": _mean(x), "n": len(x), "sum": math.fsum(x)}


_cap("mean", level="beginner", name="Arithmetic mean", topic=_D,
     purpose="The arithmetic mean (average) of a list of numbers.",
     inputs=[_X], outputs=[O("mean", "arithmetic mean"), O("n", "number of values"), O("sum", "sum of the values")],
     primary="mean", method="sum of the values divided by their number (compensated summation)",
     latex=r"\bar{x} = \frac{1}{n}\sum_{i=1}^{n} x_i", run=run_mean,
     reference=lambda x: {"mean": float(__import__("numpy").mean(x)), "n": len(x)},
     wrong=lambda x: {"mean": sum(x) / max(1, len(x) - 1), "n": len(x)},
     rnd=_rnd_x, need=r"\b(?:arithmetic )?mean\b|\baverage\b", avoid=[_AV, _OTHER_DESC], prio=1,
     templates=["Find the mean of [2, 4, 9].", "What is the average of [12, 15, 11, 18, 14]?"],
     examples=[X({"x": "2, 4, 9"}), X({"x": [12, 15, 11, 18, 14]}), X({"x": "-3.5, 0, 7.25, 10"})],
     questions=[Q("Find the mean of [2, 4, 9]."), Q("What is the average of [12, 15, 11, 18, 14]?"),
                Q("Compute the arithmetic mean of [1.5, 2.5, 3.5, 10]."),
                Q("Find the mean of [1, two, 3].", outcome="INVALID_INPUT"),
                Q("Find the mean.", outcome="NEEDS_INPUT")])


def run_median(x):
    x = _finite(x)
    _need(x, 1)
    return {"median": _median(x), "n": len(x)}


_cap("median", level="beginner", name="Median", topic=_D,
     purpose="The median (middle value) of a list of numbers; the mean of the two middle values when n is even.",
     inputs=[_X], outputs=[O("median", "median"), O("n", "number of values")], primary="median",
     method="sort the values; take the middle one (odd n) or the mean of the two middle ones (even n)",
     latex=r"\tilde{x} = x_{((n+1)/2)} \ \text{or}\ \tfrac12\left(x_{(n/2)} + x_{(n/2+1)}\right)", run=run_median,
     reference=lambda x: {"median": float(__import__("numpy").median(x))},
     wrong=lambda x: {"median": x[len(x) // 2]},
     rnd=_rnd_x, need=r"\bmedian\b", avoid=[_AV, r"\bmean\b|average|\bmodes?\b|absolute deviation|quartile|five|percentile"],
     prio=1, templates=["Find the median of [7, 1, 5, 3, 9].", "What is the median of [4, 8, 15, 16, 23, 42]?"],
     examples=[X({"x": "7, 1, 5, 3, 9"}), X({"x": [4, 8, 15, 16, 23, 42]}), X({"x": "3.2"})],
     questions=[Q("Find the median of [7, 1, 5, 3, 9]."), Q("What is the median of [4, 8, 15, 16, 23, 42]?"),
                Q("Find the median.", outcome="NEEDS_INPUT")])


def run_mode(x):
    x = _finite(x)
    _need(x, 1)
    c = {}
    for v in x:
        c[v] = c.get(v, 0) + 1
    top = max(c.values())
    modes = sorted(v for v, k in c.items() if k == top)
    if len(c) > 1 and min(c.values()) == top:
        modes = []                                  # every value occurs equally often: no mode
    return {"modes": modes, "frequency": top, "has_mode": "yes" if modes else "no"}


def ref_mode(x):
    import numpy as np
    vals, counts = np.unique(np.asarray(x, dtype=float), return_counts=True)
    top = int(counts.max())
    modes = [float(v) for v, k in zip(vals, counts) if k == top]
    if len(vals) > 1 and counts.min() == top:
        modes = []
    return {"modes": modes, "frequency": top}


_cap("mode", level="beginner", name="Mode", topic=_D,
     purpose="The mode(s) of a list of numbers: every value that occurs most often (no mode when all values occur "
             "equally often).",
     inputs=[_X], outputs=[O("modes", "the most frequent value(s), ascending; empty if there is no mode"),
                           O("frequency", "how often the mode occurs"), O("has_mode", "yes or no")],
     primary="frequency", conditions=["no mode is reported when every distinct value occurs equally often"],
     method="count each value; report all values with the largest count",
     latex=r"\operatorname{mode} = \arg\max_v \#\{i : x_i = v\}", run=run_mode, reference=ref_mode,
     wrong=lambda x: {"modes": [_median(x)], "frequency": max(x.count(v) for v in x)},
     rnd=lambda rng: {"x": [float(rng.randint(1, rng.randint(3, 12))) for _ in range(rng.randint(4, 25))]},
     need=r"\bmodes?\b", avoid=[_AV, r"\bmean\b|\bmedian\b|average"], prio=1,
     templates=["Find the mode of [3, 7, 7, 2, 9, 7, 3].", "What is the mode of [1, 2, 2, 3, 3, 4]?"],
     examples=[X({"x": "3, 7, 7, 2, 9, 7, 3"}), X({"x": [1, 2, 2, 3, 3, 4]}), X({"x": "1, 2, 3"})],
     questions=[Q("Find the mode of [3, 7, 7, 2, 9, 7, 3]."), Q("What is the mode of [1, 2, 2, 3, 3, 4]?"),
                Q("Find the mode of [5, 6, 7].")])


def run_range(x):
    x = _finite(x)
    _need(x, 1)
    return {"range": max(x) - min(x), "minimum": min(x), "maximum": max(x)}


_cap("range", level="beginner", name="Range", topic=_D,
     purpose="The range (largest minus smallest value) of a list of numbers.",
     inputs=[_X], outputs=[O("range", "maximum minus minimum"), O("minimum", "smallest value"), O("maximum", "largest value")],
     primary="range", method="maximum minus minimum", latex=r"R = x_{\max} - x_{\min}", run=run_range,
     reference=lambda x: {"range": float(__import__("numpy").ptp(x)), "minimum": float(min(x)), "maximum": float(max(x))},
     wrong=lambda x: {"range": x[-1] - x[0]}, rnd=_rnd_x,
     need=r"\brange\b", avoid=[_AV, r"interquartile|\bmean\b|\bmedian\b|average|mid-?range"], prio=1,
     templates=["Find the range of [12, 4, 19, 7].", "What is the range of the data [3.5, 9.25, 1, 6]?"],
     examples=[X({"x": "12, 4, 19, 7"}), X({"x": [3.5, 9.25, 1, 6]})],
     questions=[Q("Find the range of [12, 4, 19, 7]."), Q("What is the range of the data [3.5, 9.25, 1, 6]?")])


def _var_run(ddof, sd):
    def run(x):
        x = _finite(x)
        _need(x, 1 + ddof)
        v = _var(x, ddof)
        out = {"mean": _mean(x), "n": len(x)}
        out.update({"standard_deviation": math.sqrt(v), "variance": v} if sd else {"variance": v, "standard_deviation": math.sqrt(v)})
        return out
    return run


def _var_ref(ddof, sd):
    def ref(x):
        import numpy as np
        return ({"standard_deviation": float(np.std(x, ddof=ddof))} if sd else {"variance": float(np.var(x, ddof=ddof))})
    return ref


def _var_wrong(ddof, sd):
    def w(x):
        v = _var(x, 1 - ddof) if len(x) > 1 else 0.0
        return {"standard_deviation": math.sqrt(v)} if sd else {"variance": v}
    return w


for _key, _ddof, _sdq in (("population_variance", 0, False), ("sample_variance", 1, False),
                          ("population_standard_deviation", 0, True), ("sample_standard_deviation", 1, True)):
    _who = "population" if _ddof == 0 else "sample"
    _what = "standard deviation" if _sdq else "variance"
    _cap(_key, level="beginner", name=f"{_who.capitalize()} {_what}", topic=_D,
         purpose=f"The {_who} {_what} of a list of numbers (divisor {'n' if _ddof == 0 else 'n - 1'}).",
         inputs=[_X if _ddof == 0 else _X2],
         outputs=[O("standard_deviation" if _sdq else "variance", f"{_who} {_what}"), O("mean", "arithmetic mean"),
                  O("n", "number of values"), O("variance" if _sdq else "standard_deviation", "the companion measure")],
         primary="standard_deviation" if _sdq else "variance",
         conditions=["the data are the whole population (divisor n)" if _ddof == 0 else
                     "the data are a sample (divisor n - 1, Bessel's correction); at least two values"],
         method=f"two-pass: mean first, then the sum of squared deviations divided by {'n' if _ddof == 0 else 'n - 1'}",
         latex=(r"\sigma^2 = \frac{1}{n}\sum (x_i - \mu)^2" if _ddof == 0 else r"s^2 = \frac{1}{n-1}\sum (x_i - \bar{x})^2")
         if not _sdq else (r"\sigma = \sqrt{\frac{1}{n}\sum (x_i - \mu)^2}" if _ddof == 0 else
                           r"s = \sqrt{\frac{1}{n-1}\sum (x_i - \bar{x})^2}"),
         run=_var_run(_ddof, _sdq), reference=_var_ref(_ddof, _sdq), wrong=_var_wrong(_ddof, _sdq), rnd=_rnd_x,
         need=([r"\bvariance\b"] if not _sdq else [r"standard deviation|\bstd\b|\bsd\b"]) +
              ([r"\bpopulation\b"] if _ddof == 0 else []),
         avoid=[_AV, r"binomial|random variable|expected|frequenc|coefficient|standard error|pooled|relative"]
               + ([r"\bsample\b"] if _ddof == 0 else [r"\bpopulation\b"])
               + ([r"standard deviation|\bstd\b|\bsd\b"] if not _sdq else []),
         prio=2,
         templates=[f"Find the {_who} {_what} of [2, 4, 4, 4, 5, 5, 7, 9].",
                    f"Compute the {_who} {_what} of [10, 12, 23, 23, 16, 23, 21, 16]."],
         examples=[X({"x": "2, 4, 4, 4, 5, 5, 7, 9"}), X({"x": [10, 12, 23, 23, 16, 23, 21, 16]}), X({"x": "1.5, 2.5"})],
         questions=[Q(f"Find the {_who} {_what} of [2, 4, 4, 4, 5, 5, 7, 9]."),
                    Q(f"Compute the {_who} {_what} of [10, 12, 23, 23, 16, 23, 21, 16]."),
                    Q(f"What is the {_who} {_what} of [600, 470, 170, 430, 300]?"),
                    Q(f"Find the {_who} {_what}.", outcome="NEEDS_INPUT")])


def run_zscore(x, mu, sigma):
    sigma = _pos(sigma, "the standard deviation")
    return {"z": (x - mu) / sigma}


def ref_zscore(x, mu, sigma):
    from scipy.stats import norm
    return {"z": float(norm.isf(norm.sf(x, mu, sigma)) if x >= mu else norm.ppf(norm.cdf(x, mu, sigma)))}


_cap("z_score", level="beginner", name="z-score of a value", topic=_D,
     purpose="The z-score (standard score) of a value x in a distribution with mean mu and standard deviation sigma.",
     inputs=[N("x", "the value"), N("mu", "the mean"), N("sigma", "the standard deviation (> 0)")],
     outputs=[O("z", "number of standard deviations x lies above (+) or below (-) the mean")], primary="z",
     conditions=["sigma > 0"], method="z = (x - mu) / sigma", latex=r"z = \frac{x - \mu}{\sigma}", run=run_zscore,
     reference=ref_zscore, wrong=lambda x, mu, sigma: {"z": (mu - x) / sigma},
     rnd=lambda rng: (lambda mu, s: {"mu": mu, "sigma": s, "x": round(mu + s * rng.uniform(-4, 4), 2)})(
         round(rng.uniform(-50, 150), 1), round(rng.uniform(0.5, 30), 2)),
     need=r"z[- ]?score|standard score|standardi[sz]ed (?:value|score)", avoid=r"\[|probabilit|area|percent", prio=5,
     aliases={"x": r"x|value|score|observation", "mu": r"mean|mu", "sigma": r"standard deviation|sd|sigma"},
     templates=["Find the z-score of x = 85 when the mean = 70 and the standard deviation = 10.",
                "What is the z-score for a value of 42 with mu = 50 and sigma = 4?"],
     examples=[X({"x": 85, "mu": 70, "sigma": 10}), X({"x": 42, "mu": 50, "sigma": 4})],
     questions=[Q("Find the z-score of x = 85 when the mean = 70 and the standard deviation = 10."),
                Q("What is the z-score for a value of 42 with mu = 50 and sigma = 4?"),
                Q("Find the z-score of x = 85 when the mean = 70 and the standard deviation = 0.", outcome="INVALID_INPUT"),
                Q("Find the z-score of x = 85 when the mean = 70.", outcome="NEEDS_INPUT")])


def run_weighted_mean(x, w):
    x, w = _finite(x, "values"), _finite(w, "weights")
    if len(x) != len(w):
        raise ValueError("values and weights must have the same length")
    if any(v < 0 for v in w) or math.fsum(w) <= 0:
        raise ValueError("weights must be non-negative with a positive sum")
    return {"weighted_mean": math.fsum(a * b for a, b in zip(x, w)) / math.fsum(w), "total_weight": math.fsum(w)}


_cap("weighted_mean", level="beginner", name="Weighted mean", topic=_D,
     purpose="The weighted mean of values x with weights w (grade-point averages, portfolio averages).",
     inputs=[V("x", "the values", 1), V("w", "the weights (non-negative, same length)", 1)],
     outputs=[O("weighted_mean", "sum(w x) / sum(w)"), O("total_weight", "sum of the weights")], primary="weighted_mean",
     conditions=["same number of values and weights", "weights >= 0 with a positive sum"],
     method="sum of weight times value divided by the sum of weights", latex=r"\bar{x}_w = \frac{\sum w_i x_i}{\sum w_i}",
     run=run_weighted_mean, reference=lambda x, w: {"weighted_mean": float(__import__("numpy").average(x, weights=w))},
     wrong=lambda x, w: {"weighted_mean": _mean(x)},
     rnd=lambda rng: (lambda n: {"x": _sample(rng, n, 70, 15, 1), "w": [float(rng.randint(1, 6)) for _ in range(n)]})(rng.randint(2, 12)),
     need=r"weighted (?:mean|average)|grade point average|\bgpa\b", prio=5,
     templates=["Find the weighted mean of values [90, 80, 70] with weights [3, 2, 1].",
                "Compute the weighted average of [3.7, 3.0, 4.0] with weights [4, 3, 2]."],
     examples=[X({"x": "90, 80, 70", "w": "3, 2, 1"}), X({"x": [3.7, 3.0, 4.0], "w": [4, 3, 2]})],
     questions=[Q("Find the weighted mean of values [90, 80, 70] with weights [3, 2, 1]."),
                Q("Compute the weighted average of [3.7, 3.0, 4.0] with weights [4, 3, 2]."),
                Q("Find the weighted mean of values [90, 80, 70] with weights [3, 2].", outcome="INVALID_INPUT"),
                Q("Find the weighted mean of values [90, 80, 70].", outcome="NEEDS_INPUT")])


_QMETHOD = C("method", "quantile method: linear (type 7, spreadsheet/numpy default) or weibull (type 6, p(n+1))",
             ["linear", "weibull"])


def _x_qmethod(q, d):
    if re.search(r"weibull|type 6|\(n ?\+ ?1\)", q):
        d["method"] = "weibull"
    elif re.search(r"linear|type 7|inclusive", q):
        d["method"] = "linear"


def run_five_number(x, method="linear"):
    x = _finite(x)
    _need(x, 2)
    q1, q2, q3 = (_quantile(x, p, method) for p in (0.25, 0.5, 0.75))
    return {"minimum": min(x), "q1": q1, "median": q2, "q3": q3, "maximum": max(x), "iqr": q3 - q1}


def ref_five_number(x, method="linear"):
    import numpy as np
    v = np.percentile(np.asarray(x, dtype=float), [0, 25, 50, 75, 100], method=method)
    return {"minimum": float(v[0]), "q1": float(v[1]), "median": float(v[2]), "q3": float(v[3]), "maximum": float(v[4]),
            "iqr": float(v[3] - v[1])}


_cap("five_number_summary", level="beginner", name="Five-number summary and interquartile range", topic=_D,
     purpose="Minimum, first quartile, median, third quartile, maximum and the interquartile range of a data set.",
     inputs=[_X2, _QMETHOD],
     outputs=[O("minimum", "smallest value"), O("q1", "first quartile"), O("median", "median"), O("q3", "third quartile"),
              O("maximum", "largest value"), O("iqr", "interquartile range Q3 - Q1")], primary="iqr",
     conditions=["quartiles by linear interpolation between order statistics (Hyndman-Fan type 7, as in spreadsheets "
                 "and numpy) unless method = weibull (type 6, as in Minitab and SPSS); textbooks that take medians of "
                 "the halves can differ"],
     method="sort; quartile at position 1 + (n - 1)p (linear) or p(n + 1) (weibull), interpolating between neighbours",
     latex=r"Q_p = x_{(\lfloor h \rfloor)} + (h - \lfloor h \rfloor)\,(x_{(\lfloor h \rfloor + 1)} - x_{(\lfloor h \rfloor)}),\ h = 1 + (n-1)p",
     run=run_five_number, reference=ref_five_number,
     wrong=lambda x, method="linear": (lambda s: {"q1": s[int(0.25 * len(s))], "q3": s[min(len(s) - 1, int(0.75 * len(s)))]})(sorted(x)),
     rnd=lambda rng: dict(_rnd_x(rng, 4, 30), method=rng.choice(["linear", "weibull"])),
     need=r"five[- ]number|quartiles|\biqr\b|interquartile range|box ?plot", avoid=[_AV, r"outlier|fence"], prio=3,
     extract=_x_qmethod,
     templates=["Find the five-number summary of [1, 3, 5, 7, 9, 11, 13, 15].",
                "Find the quartiles and IQR of [6, 47, 49, 15, 42, 41, 7, 39, 43, 40, 36] using the weibull method."],
     examples=[X({"x": "1, 3, 5, 7, 9, 11, 13, 15"}), X({"x": [6, 47, 49, 15, 42, 41, 7, 39, 43, 40, 36], "method": "weibull"}),
               X({"x": [6, 47, 49, 15, 42, 41, 7, 39, 43, 40, 36]})],
     questions=[Q("Find the five-number summary of [1, 3, 5, 7, 9, 11, 13, 15]."),
                Q("Find the quartiles and IQR of [6, 47, 49, 15, 42, 41, 7, 39, 43, 40, 36] using the weibull method."),
                Q("What is the interquartile range of [2, 4, 4, 5, 6, 7, 8]?")])


def run_percentile(x, p, method="linear"):
    x = _finite(x)
    _need(x, 2)
    if not 0 <= p <= 100:
        raise ValueError("the percentile must be between 0 and 100")
    return {"percentile": _quantile(x, p / 100.0, method)}


_cap("percentile", level="beginner", name="Percentile of a data set", topic=_D,
     purpose="The p-th percentile of a data set (linear interpolation, or the weibull/p(n+1) method).",
     inputs=[_X2, N("p", "the percentile, 0 to 100"), _QMETHOD], outputs=[O("percentile", "the p-th percentile")],
     primary="percentile", conditions=["0 <= p <= 100", "linear interpolation (type 7) unless method = weibull (type 6)"],
     method="sort; position h = (n - 1)p/100 (0-based, linear) or (n + 1)p/100 - 1 (weibull); interpolate",
     latex=r"P_p = x_{(\lfloor h \rfloor)} + (h - \lfloor h \rfloor)(x_{(\lfloor h \rfloor+1)} - x_{(\lfloor h \rfloor)})",
     run=run_percentile,
     reference=lambda x, p, method="linear": {"percentile": float(__import__("numpy").percentile(x, p, method=method))},
     wrong=lambda x, p, method="linear": {"percentile": sorted(x)[min(len(x) - 1, int(round(p / 100 * len(x))))]},
     rnd=lambda rng: dict(_rnd_x(rng, 4, 30), p=float(rng.choice([rng.randint(1, 99), round(rng.uniform(0, 100), 1)])),
                          method=rng.choice(["linear", "weibull"])),
     need=r"percentile", avoid=[_AV, r"five|quartiles"], prio=3, aliases={"p": r"p|percentile"},
     extract=_chain(_x_qmethod, lambda q, d: d.update(p=float(re.search(r"(\d+(?:\.\d+)?)\s*(?:st|nd|rd|th)\s+percentile", q).group(1)))
                    if re.search(r"(\d+(?:\.\d+)?)\s*(?:st|nd|rd|th)\s+percentile", q) else None),
     templates=["Find the 90th percentile of [15, 20, 35, 40, 50].", "What is the 25th percentile of [3, 6, 7, 8, 8, 10, 13, 15, 16, 20]?"],
     examples=[X({"x": "15, 20, 35, 40, 50", "p": 90}), X({"x": [3, 6, 7, 8, 8, 10, 13, 15, 16, 20], "p": 25}),
               X({"x": [3, 6, 7, 8, 8, 10, 13, 15, 16, 20], "p": 25, "method": "weibull"})],
     questions=[Q("Find the 90th percentile of [15, 20, 35, 40, 50]."),
                Q("What is the 25th percentile of [3, 6, 7, 8, 8, 10, 13, 15, 16, 20]?"),
                Q("Find the 150th percentile of [1, 2, 3].", outcome="INVALID_INPUT")])


def run_cv(x):
    x = _finite(x)
    _need(x, 2)
    m = _mean(x)
    if m <= 0:
        raise ValueError("the coefficient of variation needs a positive mean (ratio-scale data)")
    s = _sd(x, 1)
    return {"cv": s / m, "cv_percent": 100.0 * s / m, "mean": m, "standard_deviation": s}


_cap("coefficient_of_variation", level="beginner", name="Coefficient of variation", topic=_D,
     purpose="The coefficient of variation (sample standard deviation divided by the mean), as a ratio and a percentage.",
     inputs=[_X2], outputs=[O("cv", "s / mean"), O("cv_percent", "100 s / mean"), O("mean", "mean"),
                            O("standard_deviation", "sample standard deviation")], primary="cv",
     conditions=["positive mean", "sample standard deviation (n - 1)"], method="CV = s / mean",
     latex=r"CV = \frac{s}{\bar{x}}", run=run_cv,
     reference=lambda x: {"cv": float(__import__("scipy.stats", fromlist=["variation"]).variation(x, ddof=1))},
     wrong=lambda x: {"cv": _sd(x, 0) / _mean(x)},
     rnd=lambda rng: {"x": [round(abs(v) + 1, 2) for v in _sample(rng, rng.randint(3, 25), rng.uniform(10, 200), rng.uniform(1, 30), 2)]},
     need=r"coefficient of variation|\bcv\b|relative standard deviation", avoid=_AV, prio=4,
     templates=["Find the coefficient of variation of [10, 12, 14, 16, 18].", "What is the CV of [52, 48, 50, 55, 45]?"],
     examples=[X({"x": "10, 12, 14, 16, 18"}), X({"x": [52, 48, 50, 55, 45]})],
     questions=[Q("Find the coefficient of variation of [10, 12, 14, 16, 18]."), Q("What is the CV of [52, 48, 50, 55, 45]?"),
                Q("Find the coefficient of variation of [-5, -3, -1].", outcome="INVALID_INPUT")])


def run_at_least_one(p, n):
    p = _prob(p, "the probability of the event on one trial", open_=False)
    n = _whole(n, "the number of trials", 1)
    return {"probability": -math.expm1(n * math.log1p(-p)) if p < 1 else 1.0, "none": (1.0 - p) ** n}


_cap("probability_at_least_one", level="beginner", name="Probability of at least one occurrence", topic="probability",
     purpose="The probability that an event of probability p occurs at least once in n independent trials.",
     inputs=[N("p", "probability of the event on one trial"), I("n", "number of independent trials", lo=1)],
     outputs=[O("probability", "1 - (1 - p)^n"), O("none", "probability that it never occurs, (1 - p)^n")],
     primary="probability", conditions=["independent trials with the same probability p"],
     method="complement rule: 1 - (1 - p)^n", latex=r"P(\text{at least one}) = 1 - (1 - p)^n", run=run_at_least_one,
     reference=lambda p, n: {"probability": float(__import__("scipy.stats", fromlist=["binom"]).binom.sf(0, int(n), p))},
     wrong=lambda p, n: {"probability": n * p},
     rnd=lambda rng: {"p": round(rng.uniform(0.01, 0.6), 3), "n": rng.randint(2, 40)},
     need=[r"at least one|at least once"], avoid=r"\[|binomial", prio=6,
     aliases={"p": r"probability|p", "n": r"n|trials|number of trials"},
     templates=["What is the probability of at least one success in n = 5 independent trials with probability = 0.2?",
                "Find the probability of at least one occurrence with p = 0.1 over n = 10 independent trials."],
     examples=[X({"p": 0.2, "n": 5}), X({"p": 0.1, "n": 10}), X({"p": 1 / 6, "n": 4})],
     questions=[Q("What is the probability of at least one success in n = 5 independent trials with probability = 0.2?"),
                Q("Find the probability of at least one occurrence with p = 0.1 over n = 10 independent trials."),
                Q("Find the probability of at least one occurrence with p = 1.5 over n = 10 independent trials.", outcome="INVALID_INPUT"),
                Q("What is the probability of at least one success with p = 0.3?", outcome="NEEDS_INPUT")])


def _positive_x(x):
    x = _finite(x)
    if any(v <= 0 for v in x):
        raise ValueError("all values must be positive")
    return x


def run_gmean(x):
    x = _positive_x(x)
    _need(x, 1)
    return {"geometric_mean": math.exp(math.fsum(math.log(v) for v in x) / len(x))}


def run_hmean(x):
    x = _positive_x(x)
    _need(x, 1)
    return {"harmonic_mean": len(x) / math.fsum(1.0 / v for v in x)}


_rnd_pos = lambda rng: {"x": [round(rng.uniform(0.5, 100), rng.choice([0, 1, 2])) or 1.0 for _ in range(rng.randint(2, 25))]}
_cap("geometric_mean", level="beginner", name="Geometric mean", topic=_D,
     purpose="The geometric mean of positive numbers (average growth factors, ratios).",
     inputs=[V("x", "positive values", 1)], outputs=[O("geometric_mean", "n-th root of the product")],
     primary="geometric_mean", conditions=["all values positive"], method="exp of the mean of the logarithms",
     latex=r"G = \left(\prod x_i\right)^{1/n}", run=run_gmean,
     reference=lambda x: {"geometric_mean": float(__import__("scipy.stats", fromlist=["gmean"]).gmean(x))},
     wrong=lambda x: {"geometric_mean": _mean(x)}, rnd=_rnd_pos, need=r"geometric mean", prio=5,
     templates=["Find the geometric mean of [2, 8].", "What is the geometric mean of [1.05, 1.10, 0.95, 1.20]?"],
     examples=[X({"x": "2, 8"}), X({"x": [1.05, 1.10, 0.95, 1.20]}), X({"x": "4, 1, 0.03125"})],
     questions=[Q("Find the geometric mean of [2, 8]."), Q("What is the geometric mean of [1.05, 1.10, 0.95, 1.20]?"),
                Q("Find the geometric mean of [2, -8].", outcome="INVALID_INPUT")])

_cap("harmonic_mean", level="beginner", name="Harmonic mean", topic=_D,
     purpose="The harmonic mean of positive numbers (average speeds over equal distances, average rates).",
     inputs=[V("x", "positive values", 1)], outputs=[O("harmonic_mean", "n divided by the sum of reciprocals")],
     primary="harmonic_mean", conditions=["all values positive"], method="n / sum(1/x)",
     latex=r"H = \frac{n}{\sum 1/x_i}", run=run_hmean,
     reference=lambda x: {"harmonic_mean": float(__import__("scipy.stats", fromlist=["hmean"]).hmean(x))},
     wrong=lambda x: {"harmonic_mean": 1.0 / _mean(x)}, rnd=_rnd_pos, need=r"harmonic mean", prio=5,
     templates=["Find the harmonic mean of [40, 60].", "What is the harmonic mean of [1, 2, 4]?"],
     examples=[X({"x": "40, 60"}), X({"x": [1, 2, 4]})],
     questions=[Q("Find the harmonic mean of [40, 60]."), Q("What is the harmonic mean of [1, 2, 4]?"),
                Q("Find the harmonic mean of [0, 2].", outcome="INVALID_INPUT")])


def run_mad(x):
    x = _finite(x)
    _need(x, 1)
    m = _mean(x)
    return {"mean_absolute_deviation": math.fsum(abs(v - m) for v in x) / len(x), "mean": m}


_cap("mean_absolute_deviation", level="beginner", name="Mean absolute deviation", topic=_D,
     purpose="The mean absolute deviation about the mean of a list of numbers.",
     inputs=[_X], outputs=[O("mean_absolute_deviation", "average distance from the mean"), O("mean", "mean")],
     primary="mean_absolute_deviation", method="mean of |x - mean|", latex=r"MAD = \frac{1}{n}\sum |x_i - \bar{x}|",
     run=run_mad,
     reference=lambda x: (lambda np: {"mean_absolute_deviation": float(np.mean(np.abs(np.asarray(x) - np.mean(x))))})(__import__("numpy")),
     wrong=lambda x: {"mean_absolute_deviation": math.fsum(abs(v - _mean(x)) for v in x) / max(1, len(x) - 1)},
     rnd=lambda rng: _rnd_x(rng, 2, 30), need=r"mean absolute deviation|average absolute deviation", prio=5,
     templates=["Find the mean absolute deviation of [2, 4, 6, 8].", "What is the mean absolute deviation of [10, 15, 20, 30, 50]?"],
     examples=[X({"x": "2, 4, 6, 8"}), X({"x": [10, 15, 20, 30, 50]})],
     questions=[Q("Find the mean absolute deviation of [2, 4, 6, 8]."),
                Q("What is the mean absolute deviation of [10, 15, 20, 30, 50]?")])


def run_mad_median(x):
    x = _finite(x)
    _need(x, 1)
    m = _median(x)
    return {"median_absolute_deviation": _median([abs(v - m) for v in x]), "median": m}


_cap("median_absolute_deviation", level="intermediate", name="Median absolute deviation", topic=_D,
     purpose="The median absolute deviation (MAD) about the median, a robust measure of spread (unscaled).",
     inputs=[_X], outputs=[O("median_absolute_deviation", "median of |x - median|"), O("median", "median")],
     primary="median_absolute_deviation", conditions=["unscaled (multiply by 1.4826 to estimate a normal sigma)"],
     method="median of the absolute deviations from the median", latex=r"MAD = \operatorname{median}\left(|x_i - \tilde{x}|\right)",
     run=run_mad_median,
     reference=lambda x: {"median_absolute_deviation": float(__import__("scipy.stats", fromlist=["median_abs_deviation"]).median_abs_deviation(x))},
     wrong=lambda x: {"median_absolute_deviation": _median([abs(v - _mean(x)) for v in x])},
     rnd=lambda rng: _rnd_x(rng, 3, 30), need=r"median absolute deviation", prio=6,
     templates=["Find the median absolute deviation of [1, 1, 2, 2, 4, 6, 9].",
                "What is the median absolute deviation of [10, 12, 13, 15, 40]?"],
     examples=[X({"x": "1, 1, 2, 2, 4, 6, 9"}), X({"x": [10, 12, 13, 15, 40]})],
     questions=[Q("Find the median absolute deviation of [1, 1, 2, 2, 4, 6, 9]."),
                Q("What is the median absolute deviation of [10, 12, 13, 15, 40]?")])


def run_sem(x):
    x = _finite(x)
    _need(x, 2)
    s = _sd(x, 1)
    return {"standard_error": s / math.sqrt(len(x)), "standard_deviation": s, "n": len(x)}


_cap("standard_error_of_mean", level="beginner", name="Standard error of the mean", topic=_D,
     purpose="The standard error of the mean of a sample: s / sqrt(n).",
     inputs=[_X2], outputs=[O("standard_error", "s / sqrt(n)"), O("standard_deviation", "sample standard deviation"),
                            O("n", "sample size")], primary="standard_error",
     method="sample standard deviation divided by the square root of n", latex=r"SE_{\bar{x}} = \frac{s}{\sqrt{n}}",
     run=run_sem, reference=lambda x: {"standard_error": float(__import__("scipy.stats", fromlist=["sem"]).sem(x))},
     wrong=lambda x: {"standard_error": _sd(x, 1) / len(x)}, rnd=lambda rng: _rnd_x(rng, 2, 30),
     need=r"standard error", avoid=r"regression|slope|proportion|difference|intercept|estimate of", prio=4,
     templates=["Find the standard error of the mean of [12, 15, 11, 18, 14].",
                "What is the standard error of [2.1, 2.5, 2.2, 2.8, 2.4, 2.6]?"],
     examples=[X({"x": "12, 15, 11, 18, 14"}), X({"x": [2.1, 2.5, 2.2, 2.8, 2.4, 2.6]})],
     questions=[Q("Find the standard error of the mean of [12, 15, 11, 18, 14]."),
                Q("What is the standard error of [2.1, 2.5, 2.2, 2.8, 2.4, 2.6]?"),
                Q("Find the standard error of the mean of [12].", outcome="INVALID_INPUT")])


def run_frequency(x, f):
    x, f = _finite(x, "values"), _finite(f, "frequencies")
    if len(x) != len(f):
        raise ValueError("values and frequencies must have the same length")
    if any(v < 0 or v != int(v) for v in f):
        raise ValueError("frequencies must be whole numbers >= 0")
    n = int(math.fsum(f))
    if n < 2:
        raise ValueError("the table needs a total frequency of at least 2")
    m = math.fsum(a * b for a, b in zip(x, f)) / n
    ss = math.fsum(b * (a - m) ** 2 for a, b in zip(x, f))
    return {"mean": m, "sample_standard_deviation": math.sqrt(ss / (n - 1)),
            "population_standard_deviation": math.sqrt(ss / n), "n": n}


def ref_frequency(x, f):
    import numpy as np
    d = np.repeat(np.asarray(x, dtype=float), np.asarray(f, dtype=int))
    return {"mean": float(d.mean()), "sample_standard_deviation": float(d.std(ddof=1)),
            "population_standard_deviation": float(d.std(ddof=0)), "n": int(d.size)}


_cap("frequency_table_statistics", level="beginner", name="Mean and standard deviation of a frequency table", topic=_D,
     purpose="The mean and the sample and population standard deviations of data given as values with frequencies.",
     inputs=[V("x", "the distinct values", 1), V("f", "their frequencies (whole numbers)", 1)],
     outputs=[O("mean", "sum(f x) / n"), O("sample_standard_deviation", "divisor n - 1"),
              O("population_standard_deviation", "divisor n"), O("n", "total frequency")], primary="mean",
     conditions=["frequencies are whole numbers", "total frequency >= 2"],
     method="weighted sums: mean = sum(f x)/n, then sum(f (x - mean)^2) over n - 1 or n",
     latex=r"\bar{x} = \frac{\sum f_i x_i}{\sum f_i},\quad s = \sqrt{\frac{\sum f_i (x_i - \bar{x})^2}{n - 1}}",
     run=run_frequency, reference=ref_frequency, wrong=lambda x, f: {"mean": _mean(x), "sample_standard_deviation": _sd(x, 1) if len(x) > 1 else 0.0},
     rnd=lambda rng: (lambda k: {"x": [float(v) for v in sorted(rng.sample(range(0, 50), k))],
                                 "f": [float(rng.randint(1, 12)) for _ in range(k)]})(rng.randint(2, 8)),
     need=[r"frequenc", r"mean|average|standard deviation"], prio=5,
     templates=["Find the mean and standard deviation of the frequency table with values [1, 2, 3, 4] and frequencies [3, 5, 8, 4].",
                "Find the mean of values [10, 20, 30] with frequencies [2, 5, 3]."],
     examples=[X({"x": "1, 2, 3, 4", "f": "3, 5, 8, 4"}), X({"x": [10, 20, 30], "f": [2, 5, 3]})],
     questions=[Q("Find the mean and standard deviation of the frequency table with values [1, 2, 3, 4] and frequencies [3, 5, 8, 4]."),
                Q("Find the mean of values [10, 20, 30] with frequencies [2, 5, 3]."),
                Q("Find the mean of values [10, 20, 30] with frequencies [2, 5.5, 3].", outcome="INVALID_INPUT")])


def run_trimmed(x, proportion):
    x = _finite(x)
    _need(x, 1)
    if not 0 <= proportion < 0.5:
        raise ValueError("the proportion cut from each end must be at least 0 and below 0.5")
    k = int(proportion * len(x) + 1e-12)
    s = sorted(x)[k:len(x) - k]
    return {"trimmed_mean": _mean(s), "removed_each_end": k}


def ref_trimmed(x, proportion):
    from scipy.stats import trim_mean
    return {"trimmed_mean": float(trim_mean(x, proportion))}


def _x_trim(q, d):
    m = re.search(r"(\d+(?:\.\d+)?)\s*%\s*trimmed", q)
    if m:
        d["proportion"] = float(m.group(1)) / 100.0


_cap("trimmed_mean", level="beginner", name="Trimmed mean", topic=_D,
     purpose="The trimmed mean: the mean after removing the given proportion of values from EACH end.",
     inputs=[_X, N("proportion", "proportion cut from each end (0.1 for a 10% trimmed mean)")],
     outputs=[O("trimmed_mean", "mean of the remaining values"), O("removed_each_end", "values removed at each end")],
     primary="trimmed_mean", conditions=["0 <= proportion < 0.5", "floor(proportion * n) values removed from each end"],
     method="sort, drop floor(proportion * n) values at each end, average the rest",
     latex=r"\bar{x}_{\alpha} = \frac{1}{n - 2k}\sum_{i=k+1}^{n-k} x_{(i)},\ k = \lfloor \alpha n \rfloor", run=run_trimmed,
     reference=ref_trimmed, wrong=lambda x, proportion: {"trimmed_mean": _mean(sorted(x)[int(proportion * len(x) + 1e-12):])},
     rnd=lambda rng: dict(_rnd_x(rng, 5, 30), proportion=rng.choice([0.05, 0.1, 0.2, 0.25, 0.3, 0.15])),
     need=r"trimmed mean", prio=5, aliases={"proportion": r"proportion|trim"}, extract=_x_trim,
     templates=["Find the 10% trimmed mean of [2, 4, 5, 6, 7, 8, 9, 10, 12, 50].",
                "Find the trimmed mean of [3, 8, 1, 9, 4, 6, 100, 5] with proportion = 0.25."],
     examples=[X({"x": "2, 4, 5, 6, 7, 8, 9, 10, 12, 50", "proportion": 0.1}),
               X({"x": [3, 8, 1, 9, 4, 6, 100, 5], "proportion": 0.25})],
     questions=[Q("Find the 10% trimmed mean of [2, 4, 5, 6, 7, 8, 9, 10, 12, 50]."),
                Q("Find the trimmed mean of [3, 8, 1, 9, 4, 6, 100, 5] with proportion = 0.25."),
                Q("Find the 60% trimmed mean of [1, 2, 3, 4].", outcome="INVALID_INPUT"),
                Q("Find the trimmed mean of [1, 2, 3, 4].", outcome="NEEDS_INPUT")])


def run_outliers(x):
    x = _finite(x)
    _need(x, 4)
    q1, q3 = _quantile(x, 0.25), _quantile(x, 0.75)
    iqr = q3 - q1
    lo, hi = q1 - 1.5 * iqr, q3 + 1.5 * iqr
    return {"lower_fence": lo, "upper_fence": hi, "outliers": sorted(v for v in x if v < lo or v > hi),
            "q1": q1, "q3": q3, "iqr": iqr}


def ref_outliers(x):
    import numpy as np
    a = np.asarray(x, dtype=float)
    q1, q3 = np.percentile(a, [25, 75])
    lo, hi = q1 - 1.5 * (q3 - q1), q3 + 1.5 * (q3 - q1)
    return {"lower_fence": float(lo), "upper_fence": float(hi), "outliers": [float(v) for v in np.sort(a[(a < lo) | (a > hi)])]}


_cap("iqr_outliers", level="beginner", name="Outliers by the 1.5 IQR rule", topic=_D,
     purpose="The lower and upper fences Q1 - 1.5 IQR and Q3 + 1.5 IQR and the values outside them.",
     inputs=[V("x", "the data values (at least four)", 4)],
     outputs=[O("lower_fence", "Q1 - 1.5 IQR"), O("upper_fence", "Q3 + 1.5 IQR"), O("outliers", "values outside the fences"),
              O("q1", "first quartile"), O("q3", "third quartile"), O("iqr", "interquartile range")], primary="upper_fence",
     conditions=["quartiles by linear interpolation (type 7)"], method="Tukey's fences from the quartiles",
     latex=r"[Q_1 - 1.5\,IQR,\ Q_3 + 1.5\,IQR]", run=run_outliers, reference=ref_outliers,
     wrong=lambda x: (lambda m, i: {"lower_fence": m - 1.5 * i, "upper_fence": m + 1.5 * i})(
         _median(x), _quantile(x, 0.75) - _quantile(x, 0.25)),
     rnd=lambda rng: {"x": _sample(rng, rng.randint(6, 30), 50, 10, 1) + [round(rng.uniform(80, 120), 1)] * rng.randint(0, 2)},
     need=r"outlier|fences?\b", avoid=[r"chart|control|regression|grubbs|dixon"], prio=4,
     templates=["Find the outliers in [10, 12, 12, 13, 12, 11, 14, 13, 15, 102].",
                "Use the 1.5 IQR rule to find outliers in [1, 3, 4, 5, 5, 6, 7, 30]."],
     examples=[X({"x": "10, 12, 12, 13, 12, 11, 14, 13, 15, 102"}), X({"x": [1, 3, 4, 5, 5, 6, 7, 30]})],
     questions=[Q("Find the outliers in [10, 12, 12, 13, 12, 11, 14, 13, 15, 102]."),
                Q("Use the 1.5 IQR rule to find outliers in [1, 3, 4, 5, 5, 6, 7, 30]."),
                Q("Find the outliers in [1, 2, 3].", outcome="INVALID_INPUT")])


def run_discrete_rv(x, p):
    x, p = _finite(x, "values"), _finite(p, "probabilities")
    if len(x) != len(p):
        raise ValueError("values and probabilities must have the same length")
    if any(v < 0 for v in p) or abs(math.fsum(p) - 1.0) > 1e-9:
        raise ValueError("probabilities must be non-negative and sum to 1")
    m = math.fsum(a * b for a, b in zip(x, p))
    var = math.fsum(b * (a - m) ** 2 for a, b in zip(x, p))
    return {"expected_value": m, "variance": var, "standard_deviation": math.sqrt(var)}


def ref_discrete_rv(x, p):
    import numpy as np
    x, p = np.asarray(x, dtype=float), np.asarray(p, dtype=float)
    m = float(np.dot(x, p))
    v = float(np.dot(x * x, p) - m * m)
    return {"expected_value": m, "variance": v}


def _rnd_drv(rng):
    k = rng.randint(2, 7)
    w = [rng.randint(1, 20) for _ in range(k)]
    t = sum(w)
    p = [Fraction(v, t) for v in w]
    pf = [float(v) for v in p]
    pf[-1] = float(1 - sum(Fraction(v) for v in pf[:-1]))
    return {"x": [float(v) for v in sorted(rng.sample(range(-10, 30), k))], "p": pf}


_cap("discrete_random_variable", level="beginner", name="Expected value and variance of a discrete random variable",
     topic="probability distributions",
     purpose="The expected value, variance and standard deviation of a discrete random variable given its values and "
             "probabilities.",
     inputs=[V("x", "the values of the random variable", 1), V("p", "their probabilities (summing to 1)", 1)],
     outputs=[O("expected_value", "E[X] = sum x p"), O("variance", "Var[X] = sum (x - E[X])^2 p"),
              O("standard_deviation", "sqrt(Var[X])")], primary="expected_value",
     conditions=["probabilities non-negative, summing to 1 (within 1e-9)"],
     method="E[X] = sum x p; Var[X] = sum p (x - E[X])^2", latex=r"E[X] = \sum x_i p_i,\quad \operatorname{Var}(X) = \sum (x_i - \mu)^2 p_i",
     run=run_discrete_rv, reference=ref_discrete_rv,
     wrong=lambda x, p: {"expected_value": math.fsum(a * b for a, b in zip(x, p)), "variance": math.fsum(a * a * b for a, b in zip(x, p))},
     rnd=_rnd_drv, need=r"expected value|random variable|probability distribution", avoid=r"binomial|poisson|normal|geometric|uniform", prio=5,
     templates=["Find the expected value of a random variable with values [0, 1, 2, 3] and probabilities [0.1, 0.3, 0.4, 0.2].",
                "Find the expected value and variance of the probability distribution with values [10, 20] and probabilities [0.25, 0.75]."],
     examples=[X({"x": "0, 1, 2, 3", "p": "0.1, 0.3, 0.4, 0.2"}), X({"x": [10, 20], "p": [0.25, 0.75]})],
     questions=[Q("Find the expected value of a random variable with values [0, 1, 2, 3] and probabilities [0.1, 0.3, 0.4, 0.2]."),
                Q("Find the expected value and variance of the probability distribution with values [10, 20] and probabilities [0.25, 0.75]."),
                Q("Find the expected value of a random variable with values [0, 1] and probabilities [0.5, 0.6].", outcome="INVALID_INPUT")])


def run_skew_kurt(x):
    x = _finite(x)
    _need(x, 4)
    n = len(x)
    m = _mean(x)
    m2 = math.fsum((v - m) ** 2 for v in x) / n
    m3 = math.fsum((v - m) ** 3 for v in x) / n
    m4 = math.fsum((v - m) ** 4 for v in x) / n
    if m2 == 0:
        raise ValueError("all values are equal: skewness and kurtosis are undefined")
    g1, g2 = m3 / m2 ** 1.5, m4 / m2 ** 2 - 3.0
    G1 = g1 * math.sqrt(n * (n - 1)) / (n - 2)
    G2 = ((n + 1) * g2 + 6.0) * (n - 1) / ((n - 2) * (n - 3))
    return {"skewness": G1, "excess_kurtosis": G2, "skewness_biased": g1, "excess_kurtosis_biased": g2}


def ref_skew_kurt(x):
    from scipy.stats import kurtosis, skew
    return {"skewness": float(skew(x, bias=False)), "excess_kurtosis": float(kurtosis(x, bias=False)),
            "skewness_biased": float(skew(x)), "excess_kurtosis_biased": float(kurtosis(x))}


_cap("skewness_kurtosis", level="intermediate", name="Skewness and kurtosis", topic=_D,
     purpose="Sample skewness and excess kurtosis: the adjusted Fisher-Pearson estimates (G1, G2, as in spreadsheets "
             "and most software) and the moment estimates (g1, g2).",
     inputs=[V("x", "the data values (at least four)", 4)],
     outputs=[O("skewness", "adjusted skewness G1"), O("excess_kurtosis", "adjusted excess kurtosis G2"),
              O("skewness_biased", "moment skewness g1"), O("excess_kurtosis_biased", "moment excess kurtosis g2")],
     primary="skewness", conditions=["at least four values, not all equal"],
     method="central moments m2, m3, m4; g1 = m3/m2^1.5, g2 = m4/m2^2 - 3; small-sample adjustments for G1 and G2",
     latex=r"G_1 = \frac{\sqrt{n(n-1)}}{n-2}\,\frac{m_3}{m_2^{3/2}},\quad G_2 = \frac{(n-1)\left((n+1)g_2 + 6\right)}{(n-2)(n-3)}",
     run=run_skew_kurt, reference=ref_skew_kurt,
     wrong=lambda x: {"skewness": run_skew_kurt(x)["skewness_biased"]},
     rnd=lambda rng: {"x": [round(rng.expovariate(0.2), 2) for _ in range(rng.randint(5, 40))]},
     need=r"skew|kurtosis", avoid=r"test", prio=5,
     templates=["Find the skewness and kurtosis of [2, 3, 3, 4, 8, 12, 20].", "What is the skewness of [1, 2, 2, 3, 3, 3, 4, 10]?"],
     examples=[X({"x": "2, 3, 3, 4, 8, 12, 20"}), X({"x": [1, 2, 2, 3, 3, 3, 4, 10]})],
     questions=[Q("Find the skewness and kurtosis of [2, 3, 3, 4, 8, 12, 20]."),
                Q("What is the skewness of [1, 2, 2, 3, 3, 3, 4, 10]?"),
                Q("Find the skewness of [1, 2, 3].", outcome="INVALID_INPUT")])

# =================================================================================== probability distributions ====
_PD = "probability distributions"
_BIN_IN = [I("n", "number of trials", lo=1), I("k", "number of successes", lo=0),
           N("p", "probability of success on each trial")]
_BIN_ALIAS = {"n": r"n|trials|number of trials", "k": r"k|x|successes|number of successes",
              "p": r"p|probability of success|success probability|probability"}


def _bin_args(n, k, p):
    n, k = _whole(n, "n", 1), _whole(k, "k", 0)
    if k > n:
        raise ValueError("k cannot exceed the number of trials n")
    return n, k, _prob(p, "p")


def run_binom_exact(n, k, p):
    n, k, p = _bin_args(n, k, p)
    return {"probability": _binom_pmf(k, n, p)}


def run_binom_at_most(n, k, p):
    n, k, p = _bin_args(n, k, p)
    return {"probability": _binom_cdf(k, n, p)}


def run_binom_at_least(n, k, p):
    n, k, p = _bin_args(n, k, p)
    return {"probability": _binom_sf_ge(k, n, p)}


def _rnd_bin(rng):
    n = rng.choice([rng.randint(1, 20), rng.randint(1, 200)])
    return {"n": n, "k": rng.randint(0, n), "p": round(rng.uniform(0.01, 0.99), rng.choice([2, 3]))}


_BIN_AVOID = r"poisson|hypergeom|geometric|normal|negative|at least one\b|at least once"
for _key, _word, _need_w, _run, _refname, _wrong, _lx in (
        ("binomial_exactly", "exactly k", r"\bexactly\b", run_binom_exact, "pmf",
         lambda n, k, p: {"probability": math.comb(int(n), int(k)) * p ** k}, r"P(X = k) = \binom{n}{k} p^k (1-p)^{n-k}"),
        ("binomial_at_most", "at most k", r"at most|or fewer|no more than|or less\b", run_binom_at_most, "cdf",
         lambda n, k, p: {"probability": _binom_cdf(int(k) - 1, int(n), p)}, r"P(X \le k) = \sum_{j=0}^{k} \binom{n}{j} p^j (1-p)^{n-j}"),
        ("binomial_at_least", "at least k", r"at least|or more\b|no fewer than", run_binom_at_least, "sf",
         lambda n, k, p: {"probability": _binom_sf_ge(int(k) + 1, int(n), p)}, r"P(X \ge k) = \sum_{j=k}^{n} \binom{n}{j} p^j (1-p)^{n-j}")):
    _cap(_key, level="intermediate", name=f"Binomial probability: {_word} successes", topic=_PD,
         purpose=f"The probability of {_word} successes in n independent trials with success probability p.",
         inputs=_BIN_IN, outputs=[O("probability", f"P(X {'=' if 'exactly' in _word else ('<=' if 'most' in _word else '>=')} k)")],
         primary="probability", conditions=["n independent trials, constant success probability 0 < p < 1", "0 <= k <= n"],
         method="exact binomial probabilities summed term by term (log-space binomial coefficients)", latex=_lx, run=_run,
         reference=(lambda f: (lambda n, k, p: {"probability": float(f(int(n), int(k), p))}))(
             {"pmf": lambda n, k, p: __import__("scipy.stats", fromlist=["binom"]).binom.pmf(k, n, p),
              "cdf": lambda n, k, p: __import__("scipy.stats", fromlist=["binom"]).binom.cdf(k, n, p),
              "sf": lambda n, k, p: __import__("scipy.stats", fromlist=["binom"]).binom.sf(k - 1, n, p)}[_refname]),
         wrong=_wrong, rnd=_rnd_bin, need=[r"binomial|trials|flips|tosses", _need_w], avoid=_BIN_AVOID, aliases=_BIN_ALIAS,
         templates=[f"Find the binomial probability of {_word.replace('k', 'k = 3')} successes in n = 10 trials with p = 0.4.",
                    f"In n = 20 trials with probability of success = 0.25, what is the probability of {_word.replace('k', 'k = 5')} successes?"],
         examples=[X({"n": 10, "k": 3, "p": 0.4}), X({"n": 20, "k": 5, "p": 0.25}), X({"n": 6, "k": 6, "p": 0.5})],
         questions=[Q(f"Find the binomial probability of {_word.replace('k', 'k = 3')} successes in n = 10 trials with p = 0.4."),
                    Q(f"In n = 20 trials with probability of success = 0.25, what is the probability of {_word.replace('k', 'k = 5')} successes?"),
                    Q(f"Find the binomial probability of {_word.replace('k', 'k = 12')} successes in n = 10 trials with p = 0.4.", outcome="INVALID_INPUT"),
                    Q(f"Find the binomial probability of {_word.replace('k', 'k = 3')} successes in n = 10 trials.", outcome="NEEDS_INPUT")])


def run_binom_moments(n, p):
    n, p = _whole(n, "n", 1), _prob(p, "p")
    v = n * p * (1 - p)
    return {"mean": n * p, "variance": v, "standard_deviation": math.sqrt(v)}


def ref_binom_moments(n, p):
    from scipy.stats import binom
    m, v = binom.stats(int(n), p, moments="mv")
    return {"mean": float(m), "variance": float(v), "standard_deviation": float(binom.std(int(n), p))}


_cap("binomial_mean_sd", level="intermediate", name="Mean and standard deviation of a binomial distribution", topic=_PD,
     purpose="The mean np, variance np(1-p) and standard deviation of a binomial random variable.",
     inputs=[I("n", "number of trials", lo=1), N("p", "probability of success")],
     outputs=[O("mean", "n p"), O("variance", "n p (1 - p)"), O("standard_deviation", "sqrt(n p (1 - p))")], primary="mean",
     conditions=["0 < p < 1"], method="mean = np, variance = np(1 - p)", latex=r"\mu = np,\quad \sigma = \sqrt{np(1-p)}",
     run=run_binom_moments, reference=ref_binom_moments,
     wrong=lambda n, p: {"mean": n * p, "standard_deviation": math.sqrt(n * p)},
     rnd=lambda rng: {"n": rng.randint(1, 500), "p": round(rng.uniform(0.01, 0.99), 3)},
     need=[r"binomial", r"\bmean\b|expected|standard deviation|variance"], avoid=r"negative|normal approx", aliases=_BIN_ALIAS,
     templates=["Find the mean and standard deviation of a binomial distribution with n = 50 and p = 0.3.",
                "What is the expected value and variance of a binomial random variable with n = 12 trials and p = 0.5?"],
     examples=[X({"n": 50, "p": 0.3}), X({"n": 12, "p": 0.5})],
     questions=[Q("Find the mean and standard deviation of a binomial distribution with n = 50 and p = 0.3."),
                Q("What is the expected value and variance of a binomial random variable with n = 12 trials and p = 0.5?"),
                Q("Find the mean of a binomial distribution with n = 50 and p = 1.3.", outcome="INVALID_INPUT")])


_POIS_IN = [N("lam", "the mean number of events (lambda > 0)"), I("k", "number of events", lo=0)]
_POIS_ALIAS = {"lam": r"lambda|mean|rate|average|mu", "k": r"k|x|events"}


def _pois_args(lam, k):
    return _pos(lam, "lambda"), _whole(k, "k", 0)


for _key, _word, _need_w, _f, _refname, _wrong, _lx in (
        ("poisson_exactly", "exactly k", r"\bexactly\b", lambda lam, k: _pois_pmf(k, lam), "pmf",
         lambda lam, k: {"probability": lam ** k * math.exp(-lam)}, r"P(X = k) = \frac{e^{-\lambda}\lambda^k}{k!}"),
        ("poisson_at_most", "at most k", r"at most|or fewer|no more than|or less\b", lambda lam, k: _pois_cdf(k, lam), "cdf",
         lambda lam, k: {"probability": _pois_cdf(int(k) - 1, lam)}, r"P(X \le k) = \sum_{j=0}^{k} \frac{e^{-\lambda}\lambda^j}{j!}"),
        ("poisson_at_least", "at least k", r"at least|or more\b|no fewer than", lambda lam, k: _pois_sf_ge(k, lam), "sf",
         lambda lam, k: {"probability": 1.0 - _pois_cdf(int(k), lam)}, r"P(X \ge k) = 1 - \sum_{j=0}^{k-1} \frac{e^{-\lambda}\lambda^j}{j!}")):
    _cap(_key, level="intermediate", name=f"Poisson probability: {_word} events", topic=_PD,
         purpose=f"The probability of {_word} events when events occur independently at mean rate lambda per interval.",
         inputs=_POIS_IN, outputs=[O("probability", "the Poisson probability")], primary="probability",
         conditions=["events independent, at a constant mean rate", "lambda > 0, k a whole number"],
         method="Poisson probabilities summed term by term", latex=_lx,
         run=(lambda f: (lambda lam, k: (lambda a: {"probability": f(*a)})(_pois_args(lam, k))))(_f),
         reference=(lambda name: (lambda lam, k: {"probability": float(
             {"pmf": lambda: __import__("scipy.stats", fromlist=["poisson"]).poisson.pmf(int(k), lam),
              "cdf": lambda: __import__("scipy.stats", fromlist=["poisson"]).poisson.cdf(int(k), lam),
              "sf": lambda: __import__("scipy.stats", fromlist=["poisson"]).poisson.sf(int(k) - 1, lam)}[name]())}))(_refname),
         wrong=_wrong,
         rnd=lambda rng: (lambda lam: {"lam": lam, "k": rng.randint(0, int(lam * 2 + 6))})(round(rng.uniform(0.1, 40), 2)),
         need=[r"poisson", _need_w], avoid=r"binomial|normal approx", aliases=_POIS_ALIAS,
         templates=[f"For a Poisson distribution with lambda = 4, find the probability of {_word.replace('k', 'k = 2')} events.",
                    f"Calls arrive as a Poisson process with mean = 3.5 per hour; what is the probability of {_word.replace('k', 'k = 6')} calls?"],
         examples=[X({"lam": 4, "k": 2}), X({"lam": 3.5, "k": 6}), X({"lam": 0.5, "k": 0})],
         questions=[Q(f"For a Poisson distribution with lambda = 4, find the probability of {_word.replace('k', 'k = 2')} events."),
                    Q(f"Calls arrive as a Poisson process with mean = 3.5 per hour; what is the probability of {_word.replace('k', 'k = 6')} calls?"),
                    Q(f"For a Poisson distribution with lambda = -4, find the probability of {_word.replace('k', 'k = 2')} events.", outcome="INVALID_INPUT"),
                    Q(f"For a Poisson distribution, find the probability of {_word.replace('k', 'k = 2')} events.", outcome="NEEDS_INPUT")])


def run_geometric(p, k):
    p, k = _prob(p, "p"), _whole(k, "k", 1)
    return {"probability": p * math.exp((k - 1) * math.log1p(-p)), "cumulative": -math.expm1(k * math.log1p(-p))}


_cap("geometric_probability", level="intermediate", name="Geometric distribution: first success on trial k", topic=_PD,
     purpose="The probability that the first success occurs on trial k, and that it occurs on or before trial k.",
     inputs=[N("p", "probability of success on each trial"), I("k", "the trial number (1, 2, ...)", lo=1)],
     outputs=[O("probability", "P(X = k) = (1 - p)^(k-1) p"), O("cumulative", "P(X <= k) = 1 - (1 - p)^k")],
     primary="probability", conditions=["independent trials, constant p", "X counts trials up to and including the first success"],
     method="(1 - p)^(k - 1) p, and the complement of k failures", latex=r"P(X = k) = (1-p)^{k-1}p,\quad P(X \le k) = 1 - (1-p)^k",
     run=run_geometric,
     reference=lambda p, k: (lambda g: {"probability": float(g.pmf(int(k), p)), "cumulative": float(g.cdf(int(k), p))})(
         __import__("scipy.stats", fromlist=["geom"]).geom),
     wrong=lambda p, k: {"probability": (1 - p) ** k * p}, rnd=lambda rng: {"p": round(rng.uniform(0.01, 0.9), 3), "k": rng.randint(1, 40)},
     need=r"\bgeometric (?:distribution|probability|random)|first success", avoid=r"mean of|geometric mean|hypergeometric", aliases={"p": r"p|probability", "k": r"k|trial"},
     templates=["For a geometric distribution with p = 0.2, find the probability that the first success is on trial k = 4.",
                "What is the probability that the first success occurs on trial k = 3 when p = 0.5?"],
     examples=[X({"p": 0.2, "k": 4}), X({"p": 0.5, "k": 3})],
     questions=[Q("For a geometric distribution with p = 0.2, find the probability that the first success is on trial k = 4."),
                Q("What is the probability that the first success occurs on trial k = 3 when p = 0.5?"),
                Q("What is the probability that the first success occurs on trial k = 0 when p = 0.5?", outcome="INVALID_INPUT")])


def run_hypergeom(N, K, n, k):
    N, K, n, k = _whole(N, "N", 1), _whole(K, "K", 0), _whole(n, "n", 1), _whole(k, "k", 0)
    if K > N or n > N:
        raise ValueError("the successes in the population (K) and the draws (n) cannot exceed the population size (N)")
    if k > n:
        raise ValueError("k cannot exceed the number of draws n")
    tot = math.comb(N, n)
    pmf = lambda j: Fraction(math.comb(K, j) * math.comb(N - K, n - j), tot) if 0 <= j <= K and 0 <= n - j <= N - K else Fraction(0)
    return {"probability": float(pmf(k)), "cumulative": float(sum(pmf(j) for j in range(0, k + 1)))}


def _rnd_hyper(rng):
    N = rng.randint(5, 120)
    K, n = rng.randint(0, N), rng.randint(1, N)
    lo, hi = max(0, n - (N - K)), min(n, K)
    return {"N": N, "K": K, "n": n, "k": rng.randint(lo, hi)}


_cap("hypergeometric_probability", level="intermediate", name="Hypergeometric probability (sampling without replacement)",
     topic=_PD,
     purpose="The probability of exactly k successes, and of at most k, in n draws without replacement from a "
             "population of N items of which K are successes.",
     inputs=[I("N", "population size", lo=1), I("K", "number of successes in the population"),
             I("n", "number of draws", lo=1), I("k", "number of successes drawn")],
     outputs=[O("probability", "P(X = k)"), O("cumulative", "P(X <= k)")], primary="probability",
     conditions=["draws without replacement", "K <= N, n <= N, k <= n"],
     method="exact rational arithmetic: C(K, k) C(N - K, n - k) / C(N, n)",
     latex=r"P(X = k) = \frac{\binom{K}{k}\binom{N-K}{n-k}}{\binom{N}{n}}", run=run_hypergeom,
     reference=lambda N, K, n, k: (lambda h: {"probability": float(h.pmf(int(k), int(N), int(K), int(n))),
                                              "cumulative": float(h.cdf(int(k), int(N), int(K), int(n)))})(
         __import__("scipy.stats", fromlist=["hypergeom"]).hypergeom),
     wrong=lambda N, K, n, k: {"probability": _binom_pmf(int(k), int(n), K / N) if 0 < K < N else 0.5},
     rnd=_rnd_hyper, need=r"hypergeometric|without replacement",
     aliases={"N": r"population size|population", "K": r"successes in the population|population successes|successes in population",
              "n": r"draws|sample size|drawn", "k": r"k|successes drawn"},
     templates=["Find the hypergeometric probability with population size = 50, population successes = 5, draws = 10 and k = 2.",
                "Drawing without replacement: population size = 20, population successes = 8, draws = 5; what is P(X = k) for k = 3?"],
     examples=[X({"N": 50, "K": 5, "n": 10, "k": 2}), X({"N": 20, "K": 8, "n": 5, "k": 3}), X({"N": 52, "K": 4, "n": 5, "k": 0})],
     questions=[Q("Find the hypergeometric probability with population size = 50, population successes = 5, draws = 10 and k = 2."),
                Q("Drawing without replacement: population size = 20, population successes = 8, draws = 5; what is P(X = k) for k = 3?"),
                Q("Find the hypergeometric probability with population size = 10, population successes = 12, draws = 3 and k = 1.", outcome="INVALID_INPUT"),
                Q("Find the hypergeometric probability with population size = 50, draws = 10 and k = 2.", outcome="NEEDS_INPUT")])


def run_negbinom(r, p, n):
    r, n, p = _whole(r, "r", 1), _whole(n, "n", 1), _prob(p, "p")
    if n < r:
        raise ValueError("the r-th success cannot occur before trial r")
    return {"probability": math.exp(math.log(math.comb(n - 1, r - 1)) + r * math.log(p) + (n - r) * math.log1p(-p))}


_cap("negative_binomial_probability", level="advanced", name="Negative binomial: r-th success on trial n", topic=_PD,
     purpose="The probability that the r-th success occurs on trial n (independent trials with success probability p).",
     inputs=[I("r", "number of successes required", lo=1), N("p", "probability of success"), I("n", "the trial number", lo=1)],
     outputs=[O("probability", "C(n-1, r-1) p^r (1-p)^(n-r)")], primary="probability",
     conditions=["independent trials, constant p", "n >= r"], method="C(n - 1, r - 1) p^r (1 - p)^(n - r)",
     latex=r"P(N = n) = \binom{n-1}{r-1} p^r (1-p)^{n-r}", run=run_negbinom,
     reference=lambda r, p, n: {"probability": float(__import__("scipy.stats", fromlist=["nbinom"]).nbinom.pmf(int(n) - int(r), int(r), p))},
     wrong=lambda r, p, n: {"probability": math.comb(int(n), int(r)) * p ** r * (1 - p) ** (n - r)},
     rnd=lambda rng: (lambda r: {"r": r, "p": round(rng.uniform(0.05, 0.95), 2), "n": rng.randint(r, r + 40)})(rng.randint(1, 10)),
     need=r"negative binomial", aliases={"r": r"r|successes", "n": r"n|trial", "p": r"p|probability"},
     templates=["Negative binomial: find the probability that the r = 3 success occurs on trial n = 7 with p = 0.4.",
                "Using the negative binomial distribution with p = 0.5, what is the probability that success number r = 2 comes on trial n = 5?"],
     examples=[X({"r": 3, "p": 0.4, "n": 7}), X({"r": 2, "p": 0.5, "n": 5})],
     questions=[Q("Negative binomial: find the probability that the r = 3 success occurs on trial n = 7 with p = 0.4."),
                Q("Using the negative binomial distribution with p = 0.5, what is the probability that success number r = 2 comes on trial n = 5?"),
                Q("Negative binomial: find the probability that the r = 6 success occurs on trial n = 4 with p = 0.4.", outcome="INVALID_INPUT")])


# --- normal, exponential and sampling-distribution probabilities -----------------------------------------------
_BELOW = r"p\(\s*[xz]\s*<|less than|below|at most|\bunder\b|fewer than|lower than|shorter than"
_ABOVE = r"p\(\s*[xz]\s*>|greater than|more than|above|at least|exceeds?|\bover\b|higher than|longer than"
_BETWEEN = r"between|p\(\s*[-+]?[\d.]+\s*<"
_NORM_ALIAS = {"mu": r"mean|mu", "sigma": r"standard deviation|sd|sigma", "x": r"x|value"}


def _x_point(q, d):
    m = re.search(r"p\(\s*[xz]\s*[<>]=?\s*(" + _NUM + r")\s*\)", q) or \
        re.search(r"(?:less than|below|at most|under|fewer than|lower than|shorter than|greater than|more than|above|"
                  r"at least|exceeds?|over|higher than|longer than)\s+(" + _NUM + r")(?!\.?\d)", q)
    if m and "x" not in d:
        d["x"] = float(m.group(1))
    if re.search(r"standard normal|p\(\s*z", q):
        d.setdefault("mu", 0.0)
        d.setdefault("sigma", 1.0)


def _x_interval(q, d):
    m = re.search(r"p\(\s*(" + _NUM + r")\s*<=?\s*[xz]\s*<=?\s*(" + _NUM + r")\s*\)", q) or \
        re.search(r"between\s+(" + _NUM + r")\s+and\s+(" + _NUM + r")(?!\.?\d)", q)
    if m:
        d["a"], d["b"] = float(m.group(1)), float(m.group(2))
    if re.search(r"standard normal|p\(\s*[-+]?[\d.]+\s*<=?\s*z", q):
        d.setdefault("mu", 0.0)
        d.setdefault("sigma", 1.0)


def run_normal_below(x, mu, sigma):
    sigma = _pos(sigma, "the standard deviation")
    z = (x - mu) / sigma
    return {"probability": _phi(z), "z": z}


def run_normal_above(x, mu, sigma):
    sigma = _pos(sigma, "the standard deviation")
    z = (x - mu) / sigma
    return {"probability": _phic(z), "z": z}


def run_normal_between(a, b, mu, sigma):
    sigma = _pos(sigma, "the standard deviation")
    if not a < b:
        raise ValueError("the lower bound a must be below the upper bound b")
    za, zb = (a - mu) / sigma, (b - mu) / sigma
    p = (_phic(za) - _phic(zb)) if za > 0 else (_phi(zb) - _phi(za))
    return {"probability": p, "z_a": za, "z_b": zb}


def _rnd_norm(rng):
    mu, s = round(rng.uniform(-50, 200), 1), round(rng.uniform(0.5, 40), 2)
    return {"mu": mu, "sigma": s, "x": round(mu + s * rng.uniform(-5, 5), 2)}


def _rnd_norm_ab(rng):
    mu, s = round(rng.uniform(-50, 200), 1), round(rng.uniform(0.5, 40), 2)
    a, b = sorted(round(mu + s * rng.uniform(-5, 5), 2) for _ in range(2))
    return {"mu": mu, "sigma": s, "a": a, "b": b + (0.5 if a == b else 0.0)}


_NORM_AVOID = r"\[|inverse|percentile|quantile|binomial|sample mean|mean of (?:a |the )?(?:random )?sample|x-?bar|approximat|exponential|poisson|critical"
_NORM_IN = [N("x", "the value"), N("mu", "the mean"), N("sigma", "the standard deviation (> 0)")]
_cap("normal_probability_below", level="intermediate", name="Normal probability below a value", topic=_PD,
     purpose="P(X < x) for a normal distribution with mean mu and standard deviation sigma (the area to the left).",
     inputs=_NORM_IN, outputs=[O("probability", "P(X < x)"), O("z", "the z-score of x")], primary="probability",
     conditions=["X normally distributed", "sigma > 0"], method="z = (x - mu)/sigma; Phi(z) from the complementary error function",
     latex=r"P(X < x) = \Phi\left(\frac{x - \mu}{\sigma}\right)", run=run_normal_below,
     reference=lambda x, mu, sigma: {"probability": float(__import__("scipy.stats", fromlist=["norm"]).norm.cdf(x, mu, sigma))},
     wrong=lambda x, mu, sigma: {"probability": _phic((x - mu) / sigma)}, rnd=_rnd_norm,
     need=[r"normal", _BELOW], avoid=[_NORM_AVOID, _BETWEEN], aliases=_NORM_ALIAS, extract=_x_point,
     templates=["For a normal distribution with mean = 100 and standard deviation = 15, find P(X < 120).",
                "Heights are normal with mean = 170 and sd = 8; what is the probability that a height is less than 160?"],
     examples=[X({"x": 120, "mu": 100, "sigma": 15}), X({"x": 160, "mu": 170, "sigma": 8}), X({"x": 1.96, "mu": 0, "sigma": 1})],
     questions=[Q("For a normal distribution with mean = 100 and standard deviation = 15, find P(X < 120)."),
                Q("Heights are normal with mean = 170 and sd = 8; what is the probability that a height is less than 160?"),
                Q("For the standard normal distribution, find P(Z < 1.96)."),
                Q("For a normal distribution with mean = 100 and standard deviation = -15, find P(X < 120).", outcome="INVALID_INPUT"),
                Q("For a normal distribution with mean = 100, find P(X < 120).", outcome="NEEDS_INPUT")])

_cap("normal_probability_above", level="intermediate", name="Normal probability above a value", topic=_PD,
     purpose="P(X > x) for a normal distribution with mean mu and standard deviation sigma (the area to the right).",
     inputs=_NORM_IN, outputs=[O("probability", "P(X > x)"), O("z", "the z-score of x")], primary="probability",
     conditions=["X normally distributed", "sigma > 0"], method="z = (x - mu)/sigma; 1 - Phi(z) computed as erfc(z/sqrt 2)/2",
     latex=r"P(X > x) = 1 - \Phi\left(\frac{x - \mu}{\sigma}\right)", run=run_normal_above,
     reference=lambda x, mu, sigma: {"probability": float(__import__("scipy.stats", fromlist=["norm"]).norm.sf(x, mu, sigma))},
     wrong=lambda x, mu, sigma: {"probability": _phi((x - mu) / sigma)}, rnd=_rnd_norm,
     need=[r"normal", _ABOVE], avoid=[_NORM_AVOID, _BETWEEN], aliases=_NORM_ALIAS, extract=_x_point,
     templates=["For a normal distribution with mean = 100 and standard deviation = 15, find P(X > 130).",
                "Scores are normal with mean = 500 and sd = 100; what is the probability that a score is greater than 650?"],
     examples=[X({"x": 130, "mu": 100, "sigma": 15}), X({"x": 650, "mu": 500, "sigma": 100})],
     questions=[Q("For a normal distribution with mean = 100 and standard deviation = 15, find P(X > 130)."),
                Q("Scores are normal with mean = 500 and sd = 100; what is the probability that a score is greater than 650?"),
                Q("For the standard normal distribution, find P(Z > 2.5)."),
                Q("For a normal distribution with sd = 15, find P(X > 130).", outcome="NEEDS_INPUT")])

_cap("normal_probability_between", level="intermediate", name="Normal probability between two values", topic=_PD,
     purpose="P(a < X < b) for a normal distribution with mean mu and standard deviation sigma.",
     inputs=[N("a", "lower bound"), N("b", "upper bound"), N("mu", "the mean"), N("sigma", "the standard deviation (> 0)")],
     outputs=[O("probability", "P(a < X < b)"), O("z_a", "z-score of a"), O("z_b", "z-score of b")], primary="probability",
     conditions=["X normally distributed", "sigma > 0", "a < b"],
     method="Phi(z_b) - Phi(z_a), using upper tails when both z-scores are positive (no cancellation)",
     latex=r"P(a < X < b) = \Phi\left(\frac{b-\mu}{\sigma}\right) - \Phi\left(\frac{a-\mu}{\sigma}\right)", run=run_normal_between,
     reference=lambda a, b, mu, sigma: (lambda n: {"probability": float(n.cdf(b, mu, sigma) - n.cdf(a, mu, sigma))
                                                   if a < mu else float(n.sf(a, mu, sigma) - n.sf(b, mu, sigma))})(
         __import__("scipy.stats", fromlist=["norm"]).norm),
     wrong=lambda a, b, mu, sigma: {"probability": _phi((b - mu) / sigma ** 2) - _phi((a - mu) / sigma ** 2)}, rnd=_rnd_norm_ab,
     need=[r"normal", _BETWEEN], avoid=_NORM_AVOID, aliases=_NORM_ALIAS, extract=_x_interval,
     templates=["For a normal distribution with mean = 100 and standard deviation = 15, find P(85 < X < 115).",
                "IQ is normal with mean = 100 and sd = 15; what is the probability that IQ is between 90 and 130?"],
     examples=[X({"a": 85, "b": 115, "mu": 100, "sigma": 15}), X({"a": 90, "b": 130, "mu": 100, "sigma": 15}),
               X({"a": -1.96, "b": 1.96, "mu": 0, "sigma": 1})],
     questions=[Q("For a normal distribution with mean = 100 and standard deviation = 15, find P(85 < X < 115)."),
                Q("IQ is normal with mean = 100 and sd = 15; what is the probability that IQ is between 90 and 130?"),
                Q("For the standard normal distribution, find P(-1 < Z < 2)."),
                Q("For a normal distribution with mean = 100 and standard deviation = 15, find the probability between 130 and 90.", outcome="INVALID_INPUT")])


def run_inverse_normal(p, mu, sigma):
    sigma = _pos(sigma, "the standard deviation")
    p = _prob(p, "the cumulative probability")
    z = _z(p)
    return {"x": mu + z * sigma, "z": z}


def _x_inv(q, d):
    m = re.search(r"(\d+(?:\.\d+)?)\s*(?:st|nd|rd|th)\s+percentile", q)
    if m:
        d["p"] = float(m.group(1)) / 100.0
    m = re.search(r"p\(\s*x\s*<=?\s*x\s*\)\s*=\s*(" + _NUM + r")", q)
    if m:
        d["p"] = float(m.group(1))
    m = re.search(r"\b(top|highest|upper|bottom|lowest)\s+(\d+(?:\.\d+)?)\s*%", q)
    if m:
        v = float(m.group(2)) / 100.0
        d["p"] = 1.0 - v if m.group(1) in ("top", "highest", "upper") else v
    if re.search(r"standard normal", q):
        d.setdefault("mu", 0.0)
        d.setdefault("sigma", 1.0)


_cap("inverse_normal", level="intermediate", name="Inverse normal (normal percentile)", topic=_PD,
     purpose="The value x with P(X < x) = p for a normal distribution (percentiles, cut-offs, top and bottom percentages).",
     inputs=[N("p", "cumulative probability to the left of x (0 < p < 1)"), N("mu", "the mean"),
             N("sigma", "the standard deviation (> 0)")],
     outputs=[O("x", "the value with P(X < x) = p"), O("z", "the standard normal quantile")], primary="x",
     conditions=["0 < p < 1", "sigma > 0"], method="standard normal quantile (rational start, Halley refinement on erfc), x = mu + z sigma",
     latex=r"x = \mu + \sigma\,\Phi^{-1}(p)", run=run_inverse_normal,
     reference=lambda p, mu, sigma: {"x": float(__import__("scipy.stats", fromlist=["norm"]).norm.ppf(p, mu, sigma))},
     wrong=lambda p, mu, sigma: {"x": mu - _z(p) * sigma},
     rnd=lambda rng: {"p": rng.choice([round(rng.uniform(0.001, 0.999), 4), rng.choice([0.9, 0.95, 0.975, 0.99, 0.05, 0.1])]),
                      "mu": round(rng.uniform(-50, 200), 1), "sigma": round(rng.uniform(0.5, 40), 2)},
     need=[r"normal", r"percentile|quantile|inverse|cut-?off|p\(\s*x\s*<=?\s*x\s*\)|\b(?:top|bottom|highest|lowest|upper)\s+\d"],
     avoid=r"\[|critical|binomial", prio=6, aliases={"mu": r"mean|mu", "sigma": r"standard deviation|sd|sigma", "p": r"p|probability|area"},
     extract=_x_inv,
     templates=["Find the 90th percentile of a normal distribution with mean = 100 and standard deviation = 15.",
                "For a normal distribution with mean = 50 and sd = 10, find x such that P(X < x) = 0.95."],
     examples=[X({"p": 0.9, "mu": 100, "sigma": 15}), X({"p": 0.95, "mu": 50, "sigma": 10}), X({"p": 0.975, "mu": 0, "sigma": 1})],
     questions=[Q("Find the 90th percentile of a normal distribution with mean = 100 and standard deviation = 15."),
                Q("For a normal distribution with mean = 50 and sd = 10, find x such that P(X < x) = 0.95."),
                Q("Scores are normal with mean = 500 and sd = 100; what score marks the top 5%?"),
                Q("Find the 100th percentile of a normal distribution with mean = 100 and standard deviation = 15.", outcome="INVALID_INPUT")])


# --- critical values ---------------------------------------------------------------------------------------
_TAILS = C("tails", "two-tailed, right-tailed or left-tailed", ["two-tailed", "right-tailed", "left-tailed"], required=True)


def _x_tails(q, d):
    c = _read_confidence(q)
    if c is not None:
        d["alpha"] = 1.0 - c
        d.setdefault("tails", "two-tailed")
    else:
        a = _read_alpha(q)
        if a is not None:
            d["alpha"] = a
    if re.search(r"two[- ](?:tailed|sided)", q):
        d["tails"] = "two-tailed"
    elif re.search(r"left[- ]tailed|lower[- ]tailed", q):
        d["tails"] = "left-tailed"
    elif re.search(r"right[- ]tailed|upper[- ]tailed|one[- ](?:tailed|sided)", q):
        d["tails"] = "right-tailed"


def _crit(q_of, alpha, tails):
    alpha = _prob(alpha, "alpha")
    if tails == "two-tailed":
        return q_of(1.0 - alpha / 2.0)
    return q_of(1.0 - alpha) if tails == "right-tailed" else -q_of(1.0 - alpha)


def run_z_critical(alpha, tails):
    return {"critical_value": _crit(_z, alpha, tails)}


def run_t_critical(df, alpha, tails):
    df = _pos(df, "the degrees of freedom")
    return {"critical_value": _crit(lambda p: _t_ppf(p, df), alpha, tails)}


def _ref_crit(dist, alpha, tails):
    if tails == "two-tailed":
        return float(dist.ppf(1 - alpha / 2))
    return float(dist.isf(alpha)) if tails == "right-tailed" else float(dist.ppf(alpha))


def _wrong_crit(q_of, alpha, tails):
    if tails == "two-tailed":
        return q_of(1.0 - alpha)
    return q_of(1.0 - alpha / 2.0) * (1 if tails == "right-tailed" else -1)


_rnd_alpha = lambda rng: rng.choice([0.1, 0.05, 0.025, 0.01, 0.005, 0.001, round(rng.uniform(0.001, 0.4), 4)])
_rnd_tails = lambda rng: rng.choice(["two-tailed", "right-tailed", "left-tailed"])
_CRIT_ALIAS = {"alpha": r"alpha|significance level", "df": r"df|degrees of freedom", "df1": r"df1|numerator df|numerator degrees of freedom",
               "df2": r"df2|denominator df|denominator degrees of freedom"}

_cap("z_critical_value", level="intermediate", name="Critical value of z", topic="confidence intervals and tests",
     purpose="The standard normal critical value for a significance level (or confidence level), two-tailed or one-tailed.",
     inputs=[N("alpha", "significance level (1 - confidence)"), _TAILS],
     outputs=[O("critical_value", "z critical value (negative for a left-tailed test)")], primary="critical_value",
     conditions=["0 < alpha < 1", "a confidence level is read as two-tailed"],
     method="standard normal quantile at 1 - alpha/2 (two-tailed) or 1 - alpha (one-tailed)",
     latex=r"z_{\alpha/2} = \Phi^{-1}(1 - \alpha/2)", run=run_z_critical,
     reference=lambda alpha, tails: {"critical_value": _ref_crit(__import__("scipy.stats", fromlist=["norm"]).norm, alpha, tails)},
     wrong=lambda alpha, tails: {"critical_value": _wrong_crit(_z, alpha, tails)},
     rnd=lambda rng: {"alpha": _rnd_alpha(rng), "tails": _rnd_tails(rng)},
     need=[r"critical", r"\bz\b|z-value|z value|z-score|normal"], avoid=r"\bt\b|chi|\bf\b|student|degrees", prio=6,
     aliases=_CRIT_ALIAS, extract=_x_tails,
     templates=["Find the critical z value for a 95% confidence interval.", "What is the critical z value for alpha = 0.05, right-tailed?"],
     examples=[X({"alpha": 0.05, "tails": "two-tailed"}), X({"alpha": 0.05, "tails": "right-tailed"}),
               X({"alpha": 0.01, "tails": "left-tailed"})],
     questions=[Q("Find the critical z value for a 95% confidence interval."),
                Q("What is the critical z value for alpha = 0.05, right-tailed?"),
                Q("What is the critical z value for alpha = 0.01, two-tailed?"),
                Q("What is the critical z value for alpha = 0.05?", outcome="NEEDS_INPUT")])

_cap("t_critical_value", level="intermediate", name="Critical value of Student's t", topic="confidence intervals and tests",
     purpose="The Student t critical value for given degrees of freedom and significance (or confidence) level.",
     inputs=[N("df", "degrees of freedom (> 0)"), N("alpha", "significance level (1 - confidence)"), _TAILS],
     outputs=[O("critical_value", "t critical value (negative for a left-tailed test)")], primary="critical_value",
     conditions=["df > 0", "0 < alpha < 1", "a confidence level is read as two-tailed"],
     method="t quantile by bisection on the t distribution (regularized incomplete beta function)",
     latex=r"t_{\alpha/2,\,\nu}:\ P(T_\nu > t) = \alpha/2", run=run_t_critical,
     reference=lambda df, alpha, tails: {"critical_value": _ref_crit(__import__("scipy.stats", fromlist=["t"]).t(df), alpha, tails)},
     wrong=lambda df, alpha, tails: {"critical_value": _crit(_z, alpha, tails)},
     rnd=lambda rng: {"df": float(rng.choice([rng.randint(1, 40), rng.randint(1, 200)])), "alpha": _rnd_alpha(rng), "tails": _rnd_tails(rng)},
     need=[r"critical", r"\bt\b|t-value|t value|student"], avoid=r"\bz\b|chi|\bf\b|normal", prio=6, aliases=_CRIT_ALIAS,
     extract=_x_tails,
     templates=["Find the critical t value for a 95% confidence interval with df = 10.",
                "What is the critical t value with 20 degrees of freedom for alpha = 0.05, right-tailed?"],
     examples=[X({"df": 10, "alpha": 0.05, "tails": "two-tailed"}), X({"df": 20, "alpha": 0.05, "tails": "right-tailed"}),
               X({"df": 1, "alpha": 0.01, "tails": "two-tailed"})],
     questions=[Q("Find the critical t value for a 95% confidence interval with df = 10."),
                Q("What is the critical t value with df = 20 for alpha = 0.05, right-tailed?"),
                Q("Find the critical t value for alpha = 0.01, two-tailed, with df = 1."),
                Q("Find the critical t value for a 95% confidence interval.", outcome="NEEDS_INPUT")])


def run_chi2_critical(df, alpha):
    df, alpha = _pos(df, "the degrees of freedom"), _prob(alpha, "alpha")
    return {"critical_value": _chi2_ppf(1.0 - alpha, df), "left_critical_value": _chi2_ppf(alpha, df)}


_cap("chi_square_critical_value", level="intermediate", name="Critical value of chi-square", topic="confidence intervals and tests",
     purpose="The right-tail chi-square critical value (P(X > c) = alpha) for given degrees of freedom, and the "
             "left-tail value (P(X < c) = alpha).",
     inputs=[N("df", "degrees of freedom (> 0)"), N("alpha", "tail area")],
     outputs=[O("critical_value", "right-tail critical value"), O("left_critical_value", "left-tail critical value")],
     primary="critical_value", conditions=["df > 0", "0 < alpha < 1"],
     method="chi-square quantiles by bisection on the regularized incomplete gamma function",
     latex=r"\chi^2_{\alpha,\nu}:\ P(\chi^2_\nu > c) = \alpha", run=run_chi2_critical,
     reference=lambda df, alpha: (lambda c: {"critical_value": float(c.isf(alpha, df)), "left_critical_value": float(c.ppf(alpha, df))})(
         __import__("scipy.stats", fromlist=["chi2"]).chi2),
     wrong=lambda df, alpha: {"critical_value": _chi2_ppf(1.0 - alpha, df + 1)},
     rnd=lambda rng: {"df": float(rng.randint(1, 60)), "alpha": _rnd_alpha(rng)},
     need=[r"critical", r"chi"], avoid=r"\bt\b|\bz\b|\bf\b", prio=6, aliases=_CRIT_ALIAS, extract=_x_alpha,
     templates=["Find the chi-square critical value with df = 4 and alpha = 0.05.",
                "What is the critical value of chi-square for 10 degrees of freedom at alpha = 0.01?"],
     examples=[X({"df": 4, "alpha": 0.05}), X({"df": 10, "alpha": 0.01}), X({"df": 1, "alpha": 0.05})],
     questions=[Q("Find the chi-square critical value with df = 4 and alpha = 0.05."),
                Q("What is the critical value of chi-square for degrees of freedom = 10 at alpha = 0.01?"),
                Q("Find the chi-square critical value with df = 4.", outcome="NEEDS_INPUT")])


def run_f_critical(df1, df2, alpha):
    df1, df2, alpha = _pos(df1, "df1"), _pos(df2, "df2"), _prob(alpha, "alpha")
    return {"critical_value": _f_ppf(1.0 - alpha, df1, df2)}


_cap("f_critical_value", level="advanced", name="Critical value of F", topic="confidence intervals and tests",
     purpose="The right-tail F critical value (P(F > c) = alpha) for numerator and denominator degrees of freedom.",
     inputs=[N("df1", "numerator degrees of freedom"), N("df2", "denominator degrees of freedom"), N("alpha", "tail area")],
     outputs=[O("critical_value", "right-tail F critical value")], primary="critical_value",
     conditions=["df1, df2 > 0", "0 < alpha < 1"], method="F quantile by bisection on the regularized incomplete beta function",
     latex=r"F_{\alpha;\,\nu_1,\nu_2}:\ P(F_{\nu_1,\nu_2} > c) = \alpha", run=run_f_critical,
     reference=lambda df1, df2, alpha: {"critical_value": float(__import__("scipy.stats", fromlist=["f"]).f.isf(alpha, df1, df2))},
     wrong=lambda df1, df2, alpha: {"critical_value": _f_ppf(1.0 - alpha, df2, df1)},
     rnd=lambda rng: (lambda a, b: {"df1": float(a), "df2": float(b if b != a else b + 3), "alpha": _rnd_alpha(rng)})(
         rng.randint(1, 30), rng.randint(1, 60)),
     need=[r"critical", r"\bf\b|f-value|f value|f distribution|f-distribution"], avoid=r"\bt\b|\bz\b|chi", prio=6,
     aliases=_CRIT_ALIAS, extract=_x_alpha,
     templates=["Find the F critical value with df1 = 3, df2 = 20 and alpha = 0.05.",
                "What is the critical F value for numerator df = 5 and denominator df = 10 at alpha = 0.01?"],
     examples=[X({"df1": 3, "df2": 20, "alpha": 0.05}), X({"df1": 5, "df2": 10, "alpha": 0.01})],
     questions=[Q("Find the F critical value with df1 = 3, df2 = 20 and alpha = 0.05."),
                Q("What is the critical F value for numerator df = 5 and denominator df = 10 at alpha = 0.01?"),
                Q("Find the F critical value with df1 = 3 and alpha = 0.05.", outcome="NEEDS_INPUT")])


def _exp_rate(rate, mean):
    if (rate is None) == (mean is None):
        raise ValueError("give either the rate or the mean (not both)")
    return _pos(rate, "the rate") if rate is not None else 1.0 / _pos(mean, "the mean")


def run_exp_below(x, rate=None, mean=None):
    lam = _exp_rate(rate, mean)
    if x < 0:
        raise ValueError("x must be >= 0")
    return {"probability": -math.expm1(-lam * x)}


def run_exp_above(x, rate=None, mean=None):
    lam = _exp_rate(rate, mean)
    if x < 0:
        raise ValueError("x must be >= 0")
    return {"probability": math.exp(-lam * x)}


_EXP_IN = [N("x", "the value (>= 0)"), N("rate", "rate lambda (events per unit)", required=False),
           N("mean", "mean 1/lambda (instead of the rate)", required=False)]
_EXP_ALIAS = {"rate": r"rate|lambda", "mean": r"mean|average|mu", "x": r"x"}


def _rnd_exp(rng):
    d = {"x": round(rng.uniform(0, 30), 2)}
    if rng.random() < 0.5:
        d["rate"] = round(rng.uniform(0.02, 3), 3)
    else:
        d["mean"] = round(rng.uniform(0.3, 40), 2)
    return d


def _ref_exp(which):
    def ref(x, rate=None, mean=None):
        from scipy.stats import expon
        scale = mean if mean is not None else 1.0 / rate
        return {"probability": float(expon.cdf(x, scale=scale) if which == "cdf" else expon.sf(x, scale=scale))}
    return ref


for _key, _dir, _f, _which, _needd, _lx in (("exponential_probability_below", "below", run_exp_below, "cdf", _BELOW,
                                             r"P(X < x) = 1 - e^{-\lambda x}"),
                                            ("exponential_probability_above", "above", run_exp_above, "sf", _ABOVE,
                                             r"P(X > x) = e^{-\lambda x}")):
    _cap(_key, level="intermediate", name=f"Exponential probability {_dir} a value", topic=_PD,
         purpose=f"P(X {'<' if _dir == 'below' else '>'} x) for an exponential distribution given its rate lambda or its mean.",
         inputs=_EXP_IN, outputs=[O("probability", f"P(X {'<' if _dir == 'below' else '>'} x)")], primary="probability",
         conditions=["exponential (memoryless) waiting time", "x >= 0", "exactly one of rate or mean"],
         method="1 - exp(-lambda x) or exp(-lambda x)", latex=_lx, run=_f, reference=_ref_exp(_which),
         wrong=(lambda f: (lambda x, rate=None, mean=None: f(x, rate=mean, mean=None) if mean is not None else f(x, rate=None, mean=rate)))(_f),
         rnd=_rnd_exp, need=[r"exponential", _needd], avoid=[r"reliab|weibull|mtbf|failure rate|smoothing|growth|\[", _BETWEEN] +
         ([r"p\(\s*x\s*>|greater than|more than|above|at least|exceeds?|longer than"] if _dir == "below" else
          [r"p\(\s*x\s*<|less than|below|at most|\bunder\b|shorter than|fewer than"]),
         aliases=_EXP_ALIAS, extract=_x_point,
         templates=[f"For an exponential distribution with rate = 0.5, find P(X {'<' if _dir == 'below' else '>'} 3).",
                    f"Waiting times are exponential with mean = 10 minutes; what is the probability of waiting {'less' if _dir == 'below' else 'more'} than 5?"],
         examples=[X({"x": 3, "rate": 0.5}), X({"x": 5, "mean": 10})],
         questions=[Q(f"For an exponential distribution with rate = 0.5, find P(X {'<' if _dir == 'below' else '>'} 3)."),
                    Q(f"Waiting times are exponential with mean = 10 minutes; what is the probability of waiting {'less' if _dir == 'below' else 'more'} than 5?"),
                    Q(f"For an exponential distribution with rate = -2, find P(X {'<' if _dir == 'below' else '>'} 3).", outcome="INVALID_INPUT")])


def _sm_args(x, mu, sigma, n):
    return (x - mu) / (_pos(sigma, "sigma") / math.sqrt(_whole(n, "n", 1)))


_SM_IN = [N("x", "the value for the sample mean"), N("mu", "population mean"), N("sigma", "population standard deviation"),
          I("n", "sample size", lo=1)]
_SM_ALIAS = {"mu": r"mean|mu", "sigma": r"standard deviation|sd|sigma", "n": r"n|sample size|sample of size"}
for _key, _dir, _f, _which, _needd in (("sample_mean_probability_below", "below", _phi, "cdf", _BELOW),
                                      ("sample_mean_probability_above", "above", _phic, "sf", _ABOVE)):
    _cap(_key, level="intermediate", name=f"Probability that a sample mean is {_dir} a value", topic="sampling distributions",
         purpose=f"P(sample mean {'<' if _dir == 'below' else '>'} x) for samples of size n from a population with mean mu "
                 "and standard deviation sigma (central limit theorem).",
         inputs=_SM_IN, outputs=[O("probability", "the probability"), O("z", "z = (x - mu)/(sigma/sqrt(n))"),
                                 O("standard_error", "sigma/sqrt(n)")], primary="probability",
         conditions=["population normal, or n large enough for the central limit theorem", "independent observations"],
         method="standard error sigma/sqrt(n), then the normal probability of the z-score",
         latex=r"P(\bar{X} < x) = \Phi\left(\frac{x - \mu}{\sigma/\sqrt{n}}\right)",
         run=(lambda f: (lambda x, mu, sigma, n: (lambda z: {"probability": f(z), "z": z, "standard_error": sigma / math.sqrt(n)})(
             _sm_args(x, mu, sigma, n))))(_f),
         reference=(lambda w: (lambda x, mu, sigma, n: (lambda d: {"probability": float(d.cdf(x) if w == "cdf" else d.sf(x))})(
             __import__("scipy.stats", fromlist=["norm"]).norm(mu, sigma / math.sqrt(n)))))(_which),
         wrong=(lambda f: (lambda x, mu, sigma, n: {"probability": f((x - mu) / sigma)}))(_f),
         rnd=lambda rng: (lambda mu, s, n: {"mu": mu, "sigma": s, "n": n, "x": round(mu + s / math.sqrt(n) * rng.uniform(-4, 4), 2)})(
             round(rng.uniform(0, 200), 1), round(rng.uniform(1, 40), 2), rng.randint(2, 200)),
         need=[r"sample mean|mean of (?:a |the )?(?:random )?sample|sampling distribution|x-?bar|average of (?:a |the )?(?:random )?sample", _needd],
         avoid=[r"\[|confidence|test", _BETWEEN] + ([_ABOVE] if _dir == "below" else [_BELOW]), prio=6, aliases=_SM_ALIAS,
         extract=_x_point,
         templates=[f"A population has mean = 50 and standard deviation = 10; for a sample of size n = 25, find the probability that the sample mean is {'less' if _dir == 'below' else 'greater'} than 52.",
                    f"With mu = 100, sigma = 15 and n = 36, what is the probability that the sample mean is {'below' if _dir == 'below' else 'above'} 97?"],
         examples=[X({"x": 52, "mu": 50, "sigma": 10, "n": 25}), X({"x": 97, "mu": 100, "sigma": 15, "n": 36})],
         questions=[Q(f"A population has mean = 50 and standard deviation = 10; for a sample of size n = 25, find the probability that the sample mean is {'less' if _dir == 'below' else 'greater'} than 52."),
                    Q(f"With mu = 100, sigma = 15 and n = 36, what is the probability that the sample mean is {'below' if _dir == 'below' else 'above'} 97?"),
                    Q(f"With mu = 100 and sigma = 15, what is the probability that the sample mean is {'below' if _dir == 'below' else 'above'} 97?", outcome="NEEDS_INPUT")])

# ======================================================================================== confidence intervals ====
_CI = "confidence intervals"
_CIW = r"confidence interval|\bci\b|interval estimate"
_TWO_LISTS = r"\[[^\]]*\][^\[]*\["


def _rnd_conf(rng):
    return rng.choice([0.9, 0.95, 0.99, 0.8, round(rng.uniform(0.5, 0.999), 3)])


def _zc(conf):
    return _z(1.0 - (1.0 - _conf(conf)) / 2.0)


def _tc(conf, df):
    return _t_ppf(1.0 - (1.0 - _conf(conf)) / 2.0, df)


def run_ci_mean_z(xbar, sigma, n, confidence):
    sigma, n = _pos(sigma, "sigma"), _whole(n, "n", 1)
    m = _zc(confidence) * sigma / math.sqrt(n)
    return {"lower": xbar - m, "upper": xbar + m, "margin_of_error": m, "critical_value": _zc(confidence)}


_cap("ci_mean_known_sigma", level="intermediate", name="Confidence interval for a mean (sigma known, z-interval)", topic=_CI,
     purpose="The z confidence interval for a population mean when the population standard deviation sigma is known.",
     inputs=[N("xbar", "sample mean"), N("sigma", "population standard deviation (known)"), I("n", "sample size", lo=1), CONF],
     outputs=[O("lower", "lower limit"), O("upper", "upper limit"), O("margin_of_error", "z sigma / sqrt(n)"),
              O("critical_value", "z critical value")], primary="lower",
     conditions=["sigma known", "normal population or large n", "random sample"],
     method="xbar +/- z_(alpha/2) sigma / sqrt(n)", latex=r"\bar{x} \pm z_{\alpha/2}\frac{\sigma}{\sqrt{n}}", run=run_ci_mean_z,
     reference=lambda xbar, sigma, n, confidence: (lambda iv: {"lower": float(iv[0]), "upper": float(iv[1])})(
         __import__("scipy.stats", fromlist=["norm"]).norm.interval(confidence, loc=xbar, scale=sigma / math.sqrt(n))),
     wrong=lambda xbar, sigma, n, confidence: {"lower": xbar - _zc(confidence) * sigma / n, "upper": xbar + _zc(confidence) * sigma / n},
     rnd=lambda rng: {"xbar": round(rng.uniform(-50, 200), 2), "sigma": round(rng.uniform(0.5, 30), 2), "n": rng.randint(2, 400),
                      "confidence": _rnd_conf(rng)},
     need=[_CIW, r"standard deviation|sigma"], avoid=[r"\[|proportion|difference|variance|sample standard deviation|\bs\s*="],
     aliases={"xbar": r"sample mean|mean|x-?bar", "sigma": r"population standard deviation|standard deviation|sigma",
              "n": r"n|sample size"}, extract=_x_conf,
     templates=["Find the 95% confidence interval for the mean with sample mean = 52, population standard deviation = 8 and n = 36.",
                "Construct a 99% confidence interval for mu with x-bar = 120, sigma = 15, n = 50."],
     examples=[X({"xbar": 52, "sigma": 8, "n": 36, "confidence": 0.95}), X({"xbar": 120, "sigma": 15, "n": 50, "confidence": 0.99})],
     questions=[Q("Find the 95% confidence interval for the mean with sample mean = 52, population standard deviation = 8 and n = 36."),
                Q("Construct a 99% confidence interval for mu with x-bar = 120, sigma = 15, n = 50."),
                Q("Find the 95% confidence interval for the mean with sample mean = 52 and sigma = 8.", outcome="NEEDS_INPUT"),
                Q("Find the 150% confidence interval for the mean with sample mean = 52, sigma = 8, n = 36.", outcome="INVALID_INPUT")])


def _t_interval(m, s, n, confidence):
    t = _tc(confidence, n - 1)
    e = t * s / math.sqrt(n)
    return {"lower": m - e, "upper": m + e, "margin_of_error": e, "critical_value": t, "df": n - 1}


def run_ci_mean_t(x, confidence):
    x = _finite(x)
    _need(x, 2)
    out = _t_interval(_mean(x), _sd(x), len(x), confidence)
    out["mean"] = _mean(x)
    return out


def ref_ci_t(m, se, df, conf):
    from scipy.stats import t
    lo, hi = t.interval(conf, df, loc=m, scale=se)
    return {"lower": float(lo), "upper": float(hi)}


_cap("ci_mean_t", level="intermediate", name="Confidence interval for a mean from data (t-interval)", topic=_CI,
     purpose="The t confidence interval for a population mean from sample data (sigma unknown).",
     inputs=[_X2, CONF],
     outputs=[O("lower", "lower limit"), O("upper", "upper limit"), O("mean", "sample mean"),
              O("margin_of_error", "t s / sqrt(n)"), O("critical_value", "t critical value"), O("df", "n - 1")],
     primary="lower", conditions=["random sample from a roughly normal population (or large n)", "sigma unknown"],
     method="mean +/- t_(alpha/2, n-1) s / sqrt(n)", latex=r"\bar{x} \pm t_{\alpha/2,\,n-1}\frac{s}{\sqrt{n}}", run=run_ci_mean_t,
     reference=lambda x, confidence: ref_ci_t(float(__import__("numpy").mean(x)),
                                              float(__import__("scipy.stats", fromlist=["sem"]).sem(x)), len(x) - 1, confidence),
     wrong=lambda x, confidence: (lambda m, e: {"lower": m - e, "upper": m + e})(_mean(x), _zc(confidence) * _sd(x) / math.sqrt(len(x))),
     rnd=lambda rng: dict(_rnd_x(rng, 2, 40), confidence=_rnd_conf(rng)),
     need=[_CIW, r"\["], avoid=[_TWO_LISTS, r"proportion|variance|for (?:the )?(?:population )?standard deviation|difference|median"],
     aliases=_CONF_ALIAS, extract=_x_conf,
     templates=["Find the 95% confidence interval for the mean of [12, 15, 11, 18, 14, 16].",
                "Construct a 90% confidence interval for the population mean from the sample [4.8, 5.1, 5.3, 4.9, 5.0]."],
     examples=[X({"x": "12, 15, 11, 18, 14, 16", "confidence": 0.95}), X({"x": [4.8, 5.1, 5.3, 4.9, 5.0], "confidence": 0.9})],
     questions=[Q("Find the 95% confidence interval for the mean of [12, 15, 11, 18, 14, 16]."),
                Q("Construct a 90% confidence interval for the population mean from the sample [4.8, 5.1, 5.3, 4.9, 5.0]."),
                Q("Find the 95% confidence interval for the mean of [12].", outcome="INVALID_INPUT"),
                Q("Find the confidence interval for the mean of [12, 15, 11, 18].", outcome="NEEDS_INPUT")])


def run_ci_mean_t_summary(xbar, s, n, confidence):
    s, n = _pos(s, "s"), _whole(n, "n", 2)
    return _t_interval(xbar, s, n, confidence)


_cap("ci_mean_t_summary", level="intermediate", name="Confidence interval for a mean from summary statistics (t-interval)",
     topic=_CI, purpose="The t confidence interval for a mean from the sample mean, sample standard deviation and n.",
     inputs=[N("xbar", "sample mean"), N("s", "sample standard deviation"), I("n", "sample size (>= 2)", lo=2), CONF],
     outputs=[O("lower", "lower limit"), O("upper", "upper limit"), O("margin_of_error", "t s / sqrt(n)"),
              O("critical_value", "t critical value"), O("df", "n - 1")], primary="lower",
     conditions=["random sample from a roughly normal population (or large n)", "sigma unknown"],
     method="xbar +/- t_(alpha/2, n-1) s / sqrt(n)", latex=r"\bar{x} \pm t_{\alpha/2,\,n-1}\frac{s}{\sqrt{n}}", run=run_ci_mean_t_summary,
     reference=lambda xbar, s, n, confidence: ref_ci_t(xbar, s / math.sqrt(n), n - 1, confidence),
     wrong=lambda xbar, s, n, confidence: (lambda e: {"lower": xbar - e, "upper": xbar + e})(_zc(confidence) * s / math.sqrt(n)),
     rnd=lambda rng: {"xbar": round(rng.uniform(-50, 200), 2), "s": round(rng.uniform(0.5, 30), 2), "n": rng.randint(2, 60),
                      "confidence": _rnd_conf(rng)},
     need=[_CIW, r"standard deviation|\bs\s*="], avoid=[r"\[|proportion|difference|variance|sigma|population standard deviation|known"],
     aliases={"xbar": r"sample mean|mean|x-?bar", "s": r"sample standard deviation|standard deviation|s", "n": r"n|sample size"},
     extract=_x_conf,
     templates=["Find the 95% confidence interval for the mean with sample mean = 25, sample standard deviation = 4 and n = 16.",
                "Construct a 99% confidence interval with x-bar = 3.2, s = 0.5, n = 10."],
     examples=[X({"xbar": 25, "s": 4, "n": 16, "confidence": 0.95}), X({"xbar": 3.2, "s": 0.5, "n": 10, "confidence": 0.99})],
     questions=[Q("Find the 95% confidence interval for the mean with sample mean = 25, sample standard deviation = 4 and n = 16."),
                Q("Construct a 99% confidence interval with x-bar = 3.2, s = 0.5, n = 10."),
                Q("Construct a 99% confidence interval with x-bar = 3.2, s = 0.5, n = 1.", outcome="INVALID_INPUT")])


def _x_out_of(q, d):
    m = re.search(r"(\d+)\s+(?:[a-z]+\s+){0,2}?out of\s+(\d+)", q)
    if m and "x" not in d and "n" not in d:
        d["x"], d["n"] = float(m.group(1)), float(m.group(2))


def _prop_args(x, n):
    x, n = _whole(x, "x", 0), _whole(n, "n", 1)
    if x > n:
        raise ValueError("the number of successes x cannot exceed n")
    return x, n


def run_ci_prop(x, n, confidence):
    x, n = _prop_args(x, n)
    if x == 0 or x == n:
        raise ValueError("the Wald interval needs 0 < x < n; use the Wilson interval")
    p = x / n
    e = _zc(confidence) * math.sqrt(p * (1 - p) / n)
    return {"proportion": p, "lower": max(0.0, p - e), "upper": min(1.0, p + e), "margin_of_error": e}


def ref_ci_prop(x, n, confidence, method):
    import numpy as np
    from statsmodels.stats.proportion import proportion_confint
    lo, hi = proportion_confint(int(x), int(n), alpha=1 - confidence, method=method)
    return {"lower": float(np.clip(lo, 0, 1)), "upper": float(np.clip(hi, 0, 1))}


def _rnd_prop(rng):
    n = rng.randint(5, 2000)
    return {"x": rng.randint(1, n - 1), "n": n, "confidence": _rnd_conf(rng)}


_PROP_ALIAS = {"x": r"x|successes|number of successes", "n": r"n|sample size|trials"}
_cap("ci_proportion", level="intermediate", name="Confidence interval for a proportion (Wald)", topic=_CI,
     purpose="The large-sample (Wald) confidence interval for a population proportion from x successes in n trials.",
     inputs=[I("x", "number of successes"), I("n", "sample size", lo=1), CONF],
     outputs=[O("proportion", "sample proportion x/n"), O("lower", "lower limit (clipped at 0)"),
              O("upper", "upper limit (clipped at 1)"), O("margin_of_error", "z sqrt(p(1-p)/n)")], primary="lower",
     conditions=["0 < x < n", "large sample: n p and n(1 - p) at least about 10", "limits clipped to [0, 1]"],
     method="p +/- z_(alpha/2) sqrt(p(1 - p)/n)", latex=r"\hat{p} \pm z_{\alpha/2}\sqrt{\frac{\hat{p}(1-\hat{p})}{n}}", run=run_ci_prop,
     reference=lambda x, n, confidence: ref_ci_prop(x, n, confidence, "normal"),
     wrong=lambda x, n, confidence: (lambda p: {"lower": p - _z(_conf(confidence)) * math.sqrt(p * (1 - p) / n),
                                                "upper": p + _z(_conf(confidence)) * math.sqrt(p * (1 - p) / n)})(x / n),
     rnd=_rnd_prop, need=[_CIW, r"proportion|successes|out of"], avoid=r"wilson|agresti|difference|two proportions|\[",
     aliases=_PROP_ALIAS, extract=_chain(_x_conf, _x_out_of),
     templates=["Find the 95% confidence interval for a proportion with x = 48 successes out of n = 120.",
                "In a survey, 312 out of 500 people agreed; construct a 99% confidence interval for the proportion."],
     examples=[X({"x": 48, "n": 120, "confidence": 0.95}), X({"x": 312, "n": 500, "confidence": 0.99})],
     questions=[Q("Find the 95% confidence interval for a proportion with x = 48 successes out of n = 120."),
                Q("In a survey, 312 out of 500 people agreed; construct a 99% confidence interval for the proportion."),
                Q("Find the 95% confidence interval for a proportion with x = 0 successes out of n = 120.", outcome="INVALID_INPUT")])


def run_wilson(x, n, confidence):
    x, n = _prop_args(x, n)
    z = _zc(confidence)
    p = x / n
    den = 1 + z * z / n
    c = (p + z * z / (2 * n)) / den
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / den
    return {"proportion": p, "lower": max(0.0, c - h), "upper": min(1.0, c + h), "center": c}


_cap("wilson_interval", level="advanced", name="Wilson score interval for a proportion", topic=_CI,
     purpose="The Wilson score confidence interval for a proportion (good coverage for small n and for p near 0 or 1).",
     inputs=[I("x", "number of successes"), I("n", "sample size", lo=1), CONF],
     outputs=[O("proportion", "x/n"), O("lower", "lower limit"), O("upper", "upper limit"), O("center", "interval centre")],
     primary="lower", conditions=["0 <= x <= n", "independent trials"],
     method="invert the score test: centre (p + z^2/2n)/(1 + z^2/n), half-width z sqrt(p(1-p)/n + z^2/4n^2)/(1 + z^2/n)",
     latex=r"\frac{\hat{p} + \frac{z^2}{2n} \pm z\sqrt{\frac{\hat{p}(1-\hat{p})}{n} + \frac{z^2}{4n^2}}}{1 + \frac{z^2}{n}}",
     run=run_wilson, reference=lambda x, n, confidence: ref_ci_prop(x, n, confidence, "wilson"),
     wrong=lambda x, n, confidence: (lambda p, e: {"lower": max(0.0, p - e), "upper": min(1.0, p + e)})(
         x / n, _zc(confidence) * math.sqrt((x / n) * (1 - x / n) / n)),
     rnd=lambda rng: (lambda n: {"x": rng.randint(1, n - 1) if rng.random() < 0.9 else rng.choice([0, n]), "n": n,
                                 "confidence": _rnd_conf(rng)})(rng.randint(2, 300)),
     need=[r"wilson"], avoid=r"\[", aliases=_PROP_ALIAS, extract=_chain(_x_conf, _x_out_of),
     templates=["Find the 95% Wilson interval for x = 3 successes out of n = 20.",
                "Construct a 90% Wilson score confidence interval for a proportion with 0 successes out of 15."],
     examples=[X({"x": 3, "n": 20, "confidence": 0.95}), X({"x": 0, "n": 15, "confidence": 0.9})],
     questions=[Q("Find the 95% Wilson interval for x = 3 successes out of n = 20."),
                Q("Construct a 90% Wilson score confidence interval for a proportion with 0 successes out of 15."),
                Q("Find the 95% Wilson interval for x = 30 successes out of n = 20.", outcome="INVALID_INPUT")])


_VAR_CHOICE = C("variance", "welch (unequal variances) or pooled (equal variances)", ["welch", "pooled"], required=True)


def _x_variance(q, d):
    if re.search(r"welch|unequal|unpooled|not assum", q):
        d["variance"] = "welch"
    elif re.search(r"pooled|equal variances|assum\w* equal", q):
        d["variance"] = "pooled"


def _two_sample(x, y, variance):
    x, y = _finite(x, "x"), _finite(y, "y")
    _need(x, 2, "the first sample")
    _need(y, 2, "the second sample")
    n1, n2, v1, v2 = len(x), len(y), _var(x), _var(y)
    d = _mean(x) - _mean(y)
    if variance == "pooled":
        df = n1 + n2 - 2
        sp2 = ((n1 - 1) * v1 + (n2 - 1) * v2) / df
        se = math.sqrt(sp2 * (1 / n1 + 1 / n2))
    else:
        a, b = v1 / n1, v2 / n2
        se = math.sqrt(a + b)
        df = (a + b) ** 2 / (a * a / (n1 - 1) + b * b / (n2 - 1))
    if se == 0:
        raise ValueError("both samples have zero variance")
    return d, se, df


def run_ci_two_means(x, y, confidence, variance):
    d, se, df = _two_sample(x, y, variance)
    t = _tc(confidence, df)
    return {"difference": d, "lower": d - t * se, "upper": d + t * se, "df": df, "standard_error": se}


def ref_ci_two_means(x, y, confidence, variance):
    from scipy.stats import ttest_ind
    ci = ttest_ind(x, y, equal_var=(variance == "pooled")).confidence_interval(confidence_level=confidence)
    return {"lower": float(ci.low), "upper": float(ci.high)}


def _rnd_two(rng, lo=2, hi=30):
    mu = rng.uniform(0, 100)
    return {"x": _sample(rng, rng.randint(lo, hi), mu, rng.uniform(1, 15), rng.choice([1, 2])),
            "y": _sample(rng, rng.randint(lo, hi), mu + rng.uniform(-10, 10), rng.uniform(1, 15), rng.choice([1, 2]))}


_cap("ci_two_means", level="advanced", name="Confidence interval for the difference of two means", topic=_CI,
     purpose="The t confidence interval for mu1 - mu2 from two independent samples (Welch or pooled variances).",
     inputs=[V("x", "first sample"), V("y", "second sample"), CONF, _VAR_CHOICE],
     outputs=[O("difference", "mean(x) - mean(y)"), O("lower", "lower limit"), O("upper", "upper limit"),
              O("df", "degrees of freedom (Welch-Satterthwaite or n1 + n2 - 2)"), O("standard_error", "standard error of the difference")],
     primary="lower", conditions=["independent random samples", "roughly normal populations or large samples"],
     method="difference +/- t_(alpha/2, df) SE, with Welch's SE and df or the pooled variance",
     latex=r"(\bar{x}_1 - \bar{x}_2) \pm t_{\alpha/2,\,\nu}\sqrt{\frac{s_1^2}{n_1} + \frac{s_2^2}{n_2}}", run=run_ci_two_means,
     reference=ref_ci_two_means,
     wrong=lambda x, y, confidence, variance: run_ci_two_means(x, y, confidence, "pooled" if variance == "welch" else "welch"),
     rnd=lambda rng: dict(_rnd_two(rng), confidence=_rnd_conf(rng), variance=rng.choice(["welch", "pooled"])),
     need=[_CIW, _TWO_LISTS], avoid=r"proportion|paired|median|variances? ratio", aliases=_CONF_ALIAS,
     extract=_chain(_x_conf, _x_variance),
     templates=["Find the 95% confidence interval for the difference in means of [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8] using Welch.",
                "Construct a 90% confidence interval for mu1 - mu2 with pooled variances for [5.1, 4.9, 5.6, 5.8] and [4.2, 4.8, 4.4, 4.0, 4.6]."],
     examples=[X({"x": "12, 15, 11, 18, 14", "y": "10, 9, 13, 11, 8", "confidence": 0.95, "variance": "welch"}),
               X({"x": [5.1, 4.9, 5.6, 5.8], "y": [4.2, 4.8, 4.4, 4.0, 4.6], "confidence": 0.9, "variance": "pooled"})],
     questions=[Q("Find the 95% confidence interval for the difference in means of [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8] using Welch."),
                Q("Construct a 90% confidence interval for mu1 - mu2 with pooled variances for [5.1, 4.9, 5.6, 5.8] and [4.2, 4.8, 4.4, 4.0, 4.6]."),
                Q("Find the 95% confidence interval for the difference in means of [12, 15, 11] and [10, 9, 13].", outcome="NEEDS_INPUT")])


def run_ci_variance(x, confidence):
    x = _finite(x)
    _need(x, 2)
    n, v = len(x), _var(x)
    a = 1.0 - _conf(confidence)
    lo, hi = (n - 1) * v / _chi2_ppf(1 - a / 2, n - 1), (n - 1) * v / _chi2_ppf(a / 2, n - 1)
    return {"variance": v, "lower": lo, "upper": hi, "sd_lower": math.sqrt(lo), "sd_upper": math.sqrt(hi)}


def ref_ci_variance(x, confidence):
    import numpy as np
    from scipy.stats import chi2
    n, v, a = len(x), float(np.var(x, ddof=1)), 1 - confidence
    lo, hi = (n - 1) * v / chi2.isf(a / 2, n - 1), (n - 1) * v / chi2.ppf(a / 2, n - 1)
    return {"lower": float(lo), "upper": float(hi), "sd_lower": float(np.sqrt(lo)), "sd_upper": float(np.sqrt(hi))}


_cap("ci_variance", level="advanced", name="Confidence interval for a variance and standard deviation", topic=_CI,
     purpose="The chi-square confidence interval for a population variance (and standard deviation) from sample data.",
     inputs=[_X2, CONF],
     outputs=[O("variance", "sample variance"), O("lower", "lower limit for the variance"), O("upper", "upper limit for the variance"),
              O("sd_lower", "lower limit for the standard deviation"), O("sd_upper", "upper limit for the standard deviation")],
     primary="lower", conditions=["normal population (the interval is sensitive to non-normality)"],
     method="[(n-1)s^2 / chi2_(1-alpha/2), (n-1)s^2 / chi2_(alpha/2)]",
     latex=r"\left[\frac{(n-1)s^2}{\chi^2_{1-\alpha/2,\,n-1}},\ \frac{(n-1)s^2}{\chi^2_{\alpha/2,\,n-1}}\right]", run=run_ci_variance,
     reference=ref_ci_variance,
     wrong=lambda x, confidence: (lambda n, v, a: {"lower": (n - 1) * v / _chi2_ppf(a / 2, n - 1), "upper": (n - 1) * v / _chi2_ppf(1 - a / 2, n - 1)})(
         len(x), _var(x), 1 - confidence),
     rnd=lambda rng: dict(_rnd_x(rng, 2, 40), confidence=_rnd_conf(rng)),
     need=[_CIW, r"variance|standard deviation", r"\["], avoid=[_TWO_LISTS, r"\bmean\b|proportion|difference"], prio=6,
     aliases=_CONF_ALIAS, extract=_x_conf,
     templates=["Find the 95% confidence interval for the population variance of [12, 15, 11, 18, 14, 16].",
                "Construct a 90% confidence interval for the standard deviation from [4.8, 5.1, 5.3, 4.9, 5.0]."],
     examples=[X({"x": "12, 15, 11, 18, 14, 16", "confidence": 0.95}), X({"x": [4.8, 5.1, 5.3, 4.9, 5.0], "confidence": 0.9})],
     questions=[Q("Find the 95% confidence interval for the population variance of [12, 15, 11, 18, 14, 16]."),
                Q("Construct a 90% confidence interval for the standard deviation from [4.8, 5.1, 5.3, 4.9, 5.0]."),
                Q("Find the 95% confidence interval for the population variance of [12].", outcome="INVALID_INPUT")])


# ============================================================================================ hypothesis tests ====
_HT = "hypothesis tests"
_TESTW = r"\btest\b|p-value|hypothes|significan"
_TTEST = r"t[- ]test|student"


def run_z_test(xbar, mu0, sigma, n, alternative="two-sided"):
    _check_alt(alternative)
    sigma, n = _pos(sigma, "sigma"), _whole(n, "n", 1)
    z = (xbar - mu0) / (sigma / math.sqrt(n))
    return {"z": z, "p_value": _alt_p(z, alternative, _phic, _phi)}


def ref_z_test(xbar, mu0, sigma, n, alternative="two-sided"):
    from scipy.stats import norm
    d = norm(mu0, sigma / math.sqrt(n))
    p = {"greater": d.sf(xbar), "less": d.cdf(xbar), "two-sided": min(1.0, 2 * min(d.sf(xbar), d.cdf(xbar)))}[alternative]
    return {"p_value": float(p)}


_MU0 = {"mu0": r"mu0|mu_0|hypothesi[sz]ed mean|null mean|h0:\s*mu|mu"}
_cap("z_test_mean", level="intermediate", name="One-sample z-test for a mean (sigma known)", topic=_HT,
     purpose="The z statistic and p-value for H0: mu = mu0 when the population standard deviation is known.",
     inputs=[N("xbar", "sample mean"), N("mu0", "hypothesised mean"), N("sigma", "population standard deviation (known)"),
             I("n", "sample size", lo=1), ALT],
     outputs=[O("z", "(xbar - mu0) / (sigma / sqrt(n))"), O("p_value", "p-value for the chosen alternative")], primary="p_value",
     conditions=["sigma known", "normal population or large n", "the alternative must be stated for a one-sided claim"],
     method="z statistic and its normal tail probability", latex=r"z = \frac{\bar{x} - \mu_0}{\sigma/\sqrt{n}}", run=run_z_test,
     reference=ref_z_test, wrong=lambda xbar, mu0, sigma, n, alternative="two-sided": run_z_test(xbar, mu0, sigma * math.sqrt(n), n, alternative),
     rnd=lambda rng: (lambda mu0, s, n: {"mu0": mu0, "sigma": s, "n": n, "xbar": round(mu0 + s / math.sqrt(n) * rng.uniform(-3.5, 3.5), 2),
                                         "alternative": rng.choice(["two-sided", "greater", "less"])})(
         round(rng.uniform(-20, 200), 1), round(rng.uniform(0.5, 30), 2), rng.randint(2, 300)),
     need=[_TESTW, r"\bz\b|z-test|sigma|population standard deviation|known"], avoid=r"\[|proportion|two samples|t[- ]test",
     aliases=dict(_MU0, xbar=r"sample mean|x-?bar", sigma=r"population standard deviation|sigma|standard deviation", n=r"n|sample size"),
     extract=_x_alt,
     templates=["Perform a z-test of H0: mu = 50 with sample mean = 52, sigma = 5 and n = 36, two-sided.",
                "Test whether the mean exceeds 100 using a right-tailed z-test with sample mean = 103, population standard deviation = 12, n = 49, mu0 = 100."],
     examples=[X({"xbar": 52, "mu0": 50, "sigma": 5, "n": 36}), X({"xbar": 103, "mu0": 100, "sigma": 12, "n": 49, "alternative": "greater"})],
     questions=[Q("Perform a z-test of H0: mu = 50 with sample mean = 52, sigma = 5 and n = 36, two-sided."),
                Q("Test whether the mean exceeds 100 using a right-tailed z-test with sample mean = 103, population standard deviation = 12, n = 49, mu0 = 100."),
                Q("Test whether the mean is greater than 100 with a z-test: sample mean = 103, sigma = 12, n = 49, mu0 = 100.", outcome="OUT_OF_SCOPE"),
                Q("Perform a z-test of H0: mu = 50 with sample mean = 52 and sigma = 5, two-sided.", outcome="NEEDS_INPUT")])


def run_t_one(x, mu0, alternative="two-sided"):
    _check_alt(alternative)
    x = _finite(x)
    _need(x, 2)
    s = _sd(x)
    if s == 0:
        raise ValueError("all values are equal: the t statistic is undefined")
    n = len(x)
    t = (_mean(x) - mu0) / (s / math.sqrt(n))
    return {"t": t, "df": n - 1, "p_value": _alt_p(t, alternative, lambda v: _t_sf(v, n - 1), lambda v: _t_cdf(v, n - 1)),
            "mean": _mean(x), "standard_deviation": s}


def ref_t_one(x, mu0, alternative="two-sided"):
    from scipy.stats import ttest_1samp
    r = ttest_1samp(x, mu0, alternative=alternative)
    return {"t": float(r.statistic), "p_value": float(r.pvalue), "df": float(r.df)}


def _rnd_t_one(rng):
    d = _rnd_x(rng, 3, 40)
    return dict(d, mu0=round(_mean(d["x"]) + rng.uniform(-1, 1) * (_sd(d["x"]) or 1), 1), alternative=rng.choice(["two-sided", "greater", "less"]))


_cap("t_test_one_sample", level="intermediate", name="One-sample t-test", topic=_HT,
     purpose="The one-sample t-test of H0: mu = mu0 from sample data: t statistic, degrees of freedom and p-value.",
     inputs=[_X2, N("mu0", "hypothesised mean"), ALT],
     outputs=[O("t", "(mean - mu0)/(s/sqrt(n))"), O("df", "n - 1"), O("p_value", "p-value"), O("mean", "sample mean"),
              O("standard_deviation", "sample standard deviation")], primary="p_value",
     conditions=["random sample from a roughly normal population (or large n)", "the alternative must be stated for a one-sided claim"],
     method="t statistic with n - 1 degrees of freedom; tail probability from the incomplete beta function",
     latex=r"t = \frac{\bar{x} - \mu_0}{s/\sqrt{n}},\ \nu = n - 1", run=run_t_one, reference=ref_t_one,
     wrong=lambda x, mu0, alternative="two-sided": (lambda n, s: {"t": (_mean(x) - mu0) / (s / math.sqrt(n))})(len(x), _sd(x, 0) or 1.0),
     rnd=_rnd_t_one, need=[_TESTW, _TTEST, r"\["], avoid=[_TWO_LISTS, r"paired|proportion"],
     aliases=dict(_MU0), extract=_x_alt,
     templates=["Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided.",
                "Use a left-tailed t-test to check whether the mean of [98, 102, 95, 97, 99, 96] is below the hypothesized mean = 100."],
     examples=[X({"x": "5.1, 4.9, 5.3, 5.5, 5.0, 5.2", "mu0": 5}), X({"x": [98, 102, 95, 97, 99, 96], "mu0": 100, "alternative": "less"})],
     questions=[Q("Perform a one-sample t-test of [5.1, 4.9, 5.3, 5.5, 5.0, 5.2] against mu0 = 5, two-sided."),
                Q("Use a left-tailed t-test to check whether the mean of [98, 102, 95, 97, 99, 96] is below the hypothesized mean = 100."),
                Q("Use a t-test to check whether the mean of [98, 102, 95, 97] is below the hypothesized mean = 100.", outcome="OUT_OF_SCOPE"),
                Q("Perform a one-sample t-test of [5.1, 4.9, 5.3], two-sided.", outcome="NEEDS_INPUT")])


def _t_out(d, se, df, alternative):
    t = d / se
    return {"t": t, "df": df, "p_value": _alt_p(t, alternative, lambda v: _t_sf(v, df), lambda v: _t_cdf(v, df)), "difference": d}


def _run_t_two(variance):
    def run(x, y, alternative="two-sided"):
        _check_alt(alternative)
        d, se, df = _two_sample(x, y, variance)
        return _t_out(d, se, df, alternative)
    return run


def _ref_t_two(variance):
    def ref(x, y, alternative="two-sided"):
        from scipy.stats import ttest_ind
        r = ttest_ind(x, y, equal_var=(variance == "pooled"), alternative=alternative)
        return {"t": float(r.statistic), "p_value": float(r.pvalue), "df": float(r.df)}
    return ref


for _key, _var_kind, _nm, _need_v, _avoid_v, _lx in (
        ("t_test_two_sample_pooled", "pooled", "Two-sample t-test (pooled variances)", r"pooled|equal variances|assum\w* equal|t[- ]test",
         r"welch|unequal|unpooled|not assum", r"t = \frac{\bar{x}_1 - \bar{x}_2}{s_p\sqrt{1/n_1 + 1/n_2}},\ \nu = n_1 + n_2 - 2"),
        ("welch_t_test", "welch", "Welch's two-sample t-test (unequal variances)", r"welch|unequal|unpooled|not assum|t[- ]test",
         r"pooled|(?<!un)equal variances|assum\w* equal", r"t = \frac{\bar{x}_1 - \bar{x}_2}{\sqrt{s_1^2/n_1 + s_2^2/n_2}}")):
    _cap(_key, level="intermediate", name=_nm, topic=_HT,
         purpose=f"{_nm} comparing the means of two independent samples: t statistic, degrees of freedom, p-value.",
         inputs=[V("x", "first sample"), V("y", "second sample"), ALT],
         outputs=[O("t", "t statistic"), O("df", "degrees of freedom"), O("p_value", "p-value"), O("difference", "mean(x) - mean(y)")],
         primary="p_value",
         conditions=["independent random samples", "roughly normal populations or large samples"]
                    + (["equal population variances assumed"] if _var_kind == "pooled" else ["Welch-Satterthwaite degrees of freedom"]),
         method="pooled variance with n1 + n2 - 2 df" if _var_kind == "pooled" else "separate variances, Welch-Satterthwaite df",
         latex=_lx, run=_run_t_two(_var_kind), reference=_ref_t_two(_var_kind),
         wrong=(lambda other: (lambda x, y, alternative="two-sided": _run_t_two(other)(x, y, alternative)))("welch" if _var_kind == "pooled" else "pooled"),
         rnd=lambda rng: dict(_rnd_two(rng), alternative=rng.choice(["two-sided", "greater", "less"])),
         need=[_TESTW, _TTEST, _TWO_LISTS, _need_v], avoid=[_avoid_v, r"paired|matched|proportion|summary"], extract=_x_alt,
         templates=[f"Perform a two-sample t-test with {'pooled' if _var_kind == 'pooled' else 'unequal'} variances on [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8], two-sided.",
                    f"Is there a difference? Run a {'pooled' if _var_kind == 'pooled' else 'Welch'} t-test for [5.1, 4.9, 5.6, 5.8] versus [4.2, 4.8, 4.4, 4.0, 4.6], two-tailed."],
         examples=[X({"x": "12, 15, 11, 18, 14", "y": "10, 9, 13, 11, 8"}),
                   X({"x": [5.1, 4.9, 5.6, 5.8], "y": [4.2, 4.8, 4.4, 4.0, 4.6], "alternative": "greater"})],
         questions=[Q(f"Perform a two-sample t-test with {'pooled' if _var_kind == 'pooled' else 'unequal'} variances on [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8], two-sided."),
                    Q(f"Is there a difference? Run a {'pooled' if _var_kind == 'pooled' else 'Welch'} t-test for [5.1, 4.9, 5.6, 5.8] versus [4.2, 4.8, 4.4, 4.0, 4.6], two-tailed."),
                    Q(f"Run a {'pooled' if _var_kind == 'pooled' else 'Welch'} t-test for [5.1, 4.9, 5.6] versus [4.2, 4.8, 4.4], right-tailed."),
                    Q(f"Run a {'pooled' if _var_kind == 'pooled' else 'Welch'} t-test for [5.1] versus [4.2, 4.8, 4.4], two-sided.", outcome="INVALID_INPUT")])


def run_paired_t(x, y, alternative="two-sided"):
    _check_alt(alternative)
    x, y = _pair(x, y)
    d = [a - b for a, b in zip(x, y)]
    s = _sd(d)
    if s == 0:
        raise ValueError("all differences are equal: the t statistic is undefined")
    n = len(d)
    out = _t_out(_mean(d), s / math.sqrt(n), n - 1, alternative)
    out["mean_difference"] = out.pop("difference")
    return out


def ref_paired_t(x, y, alternative="two-sided"):
    from scipy.stats import ttest_rel
    r = ttest_rel(x, y, alternative=alternative)
    return {"t": float(r.statistic), "p_value": float(r.pvalue), "df": float(r.df)}


def _rnd_paired(rng):
    n = rng.randint(3, 30)
    x = _sample(rng, n, rng.uniform(20, 100), rng.uniform(2, 15), 1)
    return {"x": x, "y": [round(v + rng.gauss(rng.uniform(-3, 3), rng.uniform(0.5, 5)), 1) for v in x],
            "alternative": rng.choice(["two-sided", "greater", "less"])}


_cap("paired_t_test", level="intermediate", name="Paired t-test", topic=_HT,
     purpose="The paired (dependent samples) t-test on the differences x - y: t statistic, df and p-value.",
     inputs=[V("x", "first measurement of each pair"), V("y", "second measurement of each pair"), ALT],
     outputs=[O("t", "mean difference / (s_d / sqrt(n))"), O("df", "n - 1"), O("p_value", "p-value"),
              O("mean_difference", "mean of x - y")], primary="p_value",
     conditions=["pairs measured on the same units", "differences roughly normal (or many pairs)"],
     method="one-sample t-test on the differences", latex=r"t = \frac{\bar{d}}{s_d/\sqrt{n}},\ \nu = n - 1", run=run_paired_t,
     reference=ref_paired_t, wrong=lambda x, y, alternative="two-sided": _run_t_two("pooled")(x, y, alternative), rnd=_rnd_paired,
     need=[_TESTW, _TTEST, r"paired|matched|dependent|before|after"], avoid=r"proportion|wilcoxon|sign test", extract=_x_alt,
     templates=["Perform a paired t-test on before [200, 190, 210, 220, 205] and after [195, 185, 200, 214, 200], two-sided.",
                "Run a right-tailed paired t-test for [5.2, 6.1, 5.8, 6.4] versus [4.9, 5.7, 5.9, 5.8]."],
     examples=[X({"x": "200, 190, 210, 220, 205", "y": "195, 185, 200, 214, 200"}),
               X({"x": [5.2, 6.1, 5.8, 6.4], "y": [4.9, 5.7, 5.9, 5.8], "alternative": "greater"})],
     questions=[Q("Perform a paired t-test on before [200, 190, 210, 220, 205] and after [195, 185, 200, 214, 200], two-sided."),
                Q("Run a right-tailed paired t-test for [5.2, 6.1, 5.8, 6.4] versus [4.9, 5.7, 5.9, 5.8]."),
                Q("Run a paired t-test for [5.2, 6.1, 5.8] versus [4.9, 5.7], two-sided.", outcome="INVALID_INPUT")])


def run_t_summary(m1, s1, n1, m2, s2, n2, variance, alternative="two-sided"):
    _check_alt(alternative)
    s1, s2 = _pos(s1, "s1"), _pos(s2, "s2")
    n1, n2 = _whole(n1, "n1", 2), _whole(n2, "n2", 2)
    v1, v2 = s1 * s1, s2 * s2
    if variance == "pooled":
        df = n1 + n2 - 2
        se = math.sqrt(((n1 - 1) * v1 + (n2 - 1) * v2) / df * (1 / n1 + 1 / n2))
    else:
        a, b = v1 / n1, v2 / n2
        se = math.sqrt(a + b)
        df = (a + b) ** 2 / (a * a / (n1 - 1) + b * b / (n2 - 1))
    return _t_out(m1 - m2, se, df, alternative)


def ref_t_summary(m1, s1, n1, m2, s2, n2, variance, alternative="two-sided"):
    from scipy.stats import ttest_ind_from_stats
    r = ttest_ind_from_stats(m1, s1, n1, m2, s2, n2, equal_var=(variance == "pooled"), alternative=alternative)
    return {"t": float(r.statistic), "p_value": float(r.pvalue)}


_cap("t_test_summary_statistics", level="advanced", name="Two-sample t-test from summary statistics", topic=_HT,
     purpose="The two-sample t-test from the means, sample standard deviations and sizes of two groups (Welch or pooled).",
     inputs=[N("m1", "mean of group 1"), N("s1", "sample standard deviation of group 1"), I("n1", "size of group 1", lo=2),
             N("m2", "mean of group 2"), N("s2", "sample standard deviation of group 2"), I("n2", "size of group 2", lo=2),
             _VAR_CHOICE, ALT],
     outputs=[O("t", "t statistic"), O("df", "degrees of freedom"), O("p_value", "p-value"), O("difference", "m1 - m2")],
     primary="p_value", conditions=["independent random samples", "roughly normal populations or large samples"],
     method="pooled or Welch standard error and degrees of freedom from the summaries",
     latex=r"t = \frac{\bar{x}_1 - \bar{x}_2}{\sqrt{s_1^2/n_1 + s_2^2/n_2}}", run=run_t_summary, reference=ref_t_summary,
     wrong=lambda m1, s1, n1, m2, s2, n2, variance, alternative="two-sided": run_t_summary(m1, math.sqrt(s1), n1, m2, math.sqrt(s2), n2, variance, alternative),
     rnd=lambda rng: {"m1": round(rng.uniform(0, 100), 2), "s1": round(rng.uniform(1.5, 20), 2), "n1": rng.randint(2, 80),
                      "m2": round(rng.uniform(0, 100), 2), "s2": round(rng.uniform(1.5, 20), 2), "n2": rng.randint(2, 80),
                      "variance": rng.choice(["welch", "pooled"]), "alternative": rng.choice(["two-sided", "greater", "less"])},
     need=[_TESTW, _TTEST, r"summary|mean1|\bm1\b|n1"], avoid=r"\[|proportion",
     aliases={"m1": r"m1|mean1|mean 1", "m2": r"m2|mean2|mean 2", "s1": r"s1|sd1|sd 1", "s2": r"s2|sd2|sd 2", "n1": r"n1|n 1", "n2": r"n2|n 2"},
     extract=_chain(_x_alt, _x_variance),
     templates=["Two-sample t-test from summary statistics with pooled variances: mean1 = 20, sd1 = 4, n1 = 15, mean2 = 17, sd2 = 5, n2 = 12, two-sided.",
                "Welch t-test from summary statistics: m1 = 3.4, s1 = 0.8, n1 = 30, m2 = 3.0, s2 = 1.1, n2 = 25, right-tailed."],
     examples=[X({"m1": 20, "s1": 4, "n1": 15, "m2": 17, "s2": 5, "n2": 12, "variance": "pooled"}),
               X({"m1": 3.4, "s1": 0.8, "n1": 30, "m2": 3.0, "s2": 1.1, "n2": 25, "variance": "welch", "alternative": "greater"})],
     questions=[Q("Two-sample t-test from summary statistics with pooled variances: mean1 = 20, sd1 = 4, n1 = 15, mean2 = 17, sd2 = 5, n2 = 12, two-sided."),
                Q("Welch t-test from summary statistics: m1 = 3.4, s1 = 0.8, n1 = 30, m2 = 3.0, s2 = 1.1, n2 = 25, right-tailed."),
                Q("Two-sample t-test from summary statistics: mean1 = 20, sd1 = 4, n1 = 15, mean2 = 17, sd2 = 5, n2 = 12, two-sided.", outcome="NEEDS_INPUT")])


def run_prop_z(x, n, p0, alternative="two-sided"):
    _check_alt(alternative)
    x, n = _prop_args(x, n)
    p0 = _prob(p0, "p0")
    z = (x / n - p0) / math.sqrt(p0 * (1 - p0) / n)
    return {"z": z, "p_value": _alt_p(z, alternative, _phic, _phi), "proportion": x / n}


def ref_prop_z(x, n, p0, alternative="two-sided"):
    from statsmodels.stats.proportion import proportions_ztest
    z, p = proportions_ztest(int(x), int(n), value=p0, alternative={"greater": "larger", "less": "smaller"}.get(alternative, alternative),
                             prop_var=p0)
    return {"z": float(z), "p_value": float(p)}


_cap("proportion_z_test", level="intermediate", name="One-proportion z-test", topic=_HT,
     purpose="The z-test of H0: p = p0 for a population proportion from x successes in n trials.",
     inputs=[I("x", "number of successes"), I("n", "sample size", lo=1), N("p0", "hypothesised proportion"), ALT],
     outputs=[O("z", "(x/n - p0)/sqrt(p0(1 - p0)/n)"), O("p_value", "p-value"), O("proportion", "x/n")], primary="p_value",
     conditions=["n p0 and n(1 - p0) at least about 10", "independent trials"],
     method="z statistic with the null standard error sqrt(p0(1-p0)/n)", latex=r"z = \frac{\hat{p} - p_0}{\sqrt{p_0(1-p_0)/n}}",
     run=run_prop_z, reference=ref_prop_z,
     wrong=lambda x, n, p0, alternative="two-sided": (lambda p: {"z": (p - p0) / math.sqrt(max(p * (1 - p), 1e-12) / n)})(x / n),
     rnd=lambda rng: (lambda n: {"n": n, "x": rng.randint(1, n - 1), "p0": round(rng.uniform(0.05, 0.95), 2),
                                 "alternative": rng.choice(["two-sided", "greater", "less"])})(rng.randint(10, 2000)),
     need=[_TESTW, r"proportion"], avoid=r"two proportions|\bx1\b|\bn1\b|difference|two-proportion|\[",
     aliases=dict(_PROP_ALIAS, p0=r"p0|p_0|hypothesi[sz]ed proportion|null proportion|h0:\s*p"), extract=_chain(_x_alt, _x_out_of),
     templates=["Test H0: p = 0.5 for a proportion with x = 58 successes out of n = 100, p0 = 0.5, two-sided.",
                "A claim says the proportion is 0.3; in a sample 45 out of 120 agreed. Test it with p0 = 0.3, right-tailed."],
     examples=[X({"x": 58, "n": 100, "p0": 0.5}), X({"x": 45, "n": 120, "p0": 0.3, "alternative": "greater"})],
     questions=[Q("Test H0: p = 0.5 for a proportion with x = 58 successes out of n = 100, p0 = 0.5, two-sided."),
                Q("A claim says the proportion is 0.3; in a sample 45 out of 120 agreed. Test it with p0 = 0.3, right-tailed."),
                Q("Test the proportion with x = 58 successes out of n = 100, p0 = 1.2, two-sided.", outcome="INVALID_INPUT")])


def run_two_prop_z(x1, n1, x2, n2, alternative="two-sided"):
    _check_alt(alternative)
    x1, n1 = _prop_args(x1, n1)
    x2, n2 = _prop_args(x2, n2)
    p = (x1 + x2) / (n1 + n2)
    if p in (0.0, 1.0):
        raise ValueError("the pooled proportion is 0 or 1: the test is undefined")
    z = (x1 / n1 - x2 / n2) / math.sqrt(p * (1 - p) * (1 / n1 + 1 / n2))
    return {"z": z, "p_value": _alt_p(z, alternative, _phic, _phi), "pooled_proportion": p, "difference": x1 / n1 - x2 / n2}


def ref_two_prop_z(x1, n1, x2, n2, alternative="two-sided"):
    from statsmodels.stats.proportion import proportions_ztest
    z, p = proportions_ztest([int(x1), int(x2)], [int(n1), int(n2)],
                             alternative={"greater": "larger", "less": "smaller"}.get(alternative, alternative))
    return {"z": float(z), "p_value": float(p)}


_cap("two_proportion_z_test", level="intermediate", name="Two-proportion z-test", topic=_HT,
     purpose="The pooled z-test of H0: p1 = p2 from x1 of n1 and x2 of n2 successes.",
     inputs=[I("x1", "successes in sample 1"), I("n1", "size of sample 1", lo=1), I("x2", "successes in sample 2"),
             I("n2", "size of sample 2", lo=1), ALT],
     outputs=[O("z", "z statistic (pooled standard error)"), O("p_value", "p-value"), O("pooled_proportion", "(x1 + x2)/(n1 + n2)"),
              O("difference", "x1/n1 - x2/n2")], primary="p_value",
     conditions=["independent samples", "at least about 10 successes and failures in each sample"],
     method="pooled proportion, z = (p1 - p2)/sqrt(p(1-p)(1/n1 + 1/n2))",
     latex=r"z = \frac{\hat{p}_1 - \hat{p}_2}{\sqrt{\hat{p}(1-\hat{p})\left(\frac{1}{n_1} + \frac{1}{n_2}\right)}}", run=run_two_prop_z,
     reference=ref_two_prop_z,
     wrong=lambda x1, n1, x2, n2, alternative="two-sided": (lambda a, b: {"z": (a - b) / math.sqrt(max(a * (1 - a) / n1 + b * (1 - b) / n2, 1e-12))})(x1 / n1, x2 / n2),
     rnd=lambda rng: (lambda n1, n2: {"n1": n1, "x1": rng.randint(1, n1 - 1), "n2": n2, "x2": rng.randint(1, n2 - 1),
                                      "alternative": rng.choice(["two-sided", "greater", "less"])})(rng.randint(10, 1500), rng.randint(10, 1500)),
     need=[_TESTW, r"two proportions|\bx1\b|difference (?:in|between|of) (?:the )?proportions|two-proportion"], avoid=r"\[",
     aliases={"x1": r"x1", "n1": r"n1", "x2": r"x2", "n2": r"n2"}, extract=_x_alt,
     templates=["Two-proportion z-test with x1 = 45, n1 = 100, x2 = 30, n2 = 100, two-sided.",
                "Test the difference between two proportions: x1 = 120, n1 = 400, x2 = 90, n2 = 380, right-tailed."],
     examples=[X({"x1": 45, "n1": 100, "x2": 30, "n2": 100}), X({"x1": 120, "n1": 400, "x2": 90, "n2": 380, "alternative": "greater"})],
     questions=[Q("Two-proportion z-test with x1 = 45, n1 = 100, x2 = 30, n2 = 100, two-sided."),
                Q("Test the difference between two proportions: x1 = 120, n1 = 400, x2 = 90, n2 = 380, right-tailed."),
                Q("Two-proportion z-test with x1 = 145, n1 = 100, x2 = 30, n2 = 100, two-sided.", outcome="INVALID_INPUT"),
                Q("Two-proportion z-test with x1 = 45, n1 = 100, x2 = 30, two-sided.", outcome="NEEDS_INPUT")])


def run_gof(observed, proportions=None):
    o = _finite(observed, "observed counts")
    _need(o, 2, "the observed counts")
    if any(v < 0 or v != int(v) for v in o):
        raise ValueError("observed counts must be whole numbers >= 0")
    w = [1.0] * len(o) if proportions is None else _finite(proportions, "expected proportions")
    if len(w) != len(o):
        raise ValueError("give one expected proportion per category")
    if any(v <= 0 for v in w):
        raise ValueError("expected proportions must be positive")
    n, sw = math.fsum(o), math.fsum(w)
    if n <= 0:
        raise ValueError("the total count must be positive")
    e = [n * v / sw for v in w]
    chi2 = math.fsum((a - b) ** 2 / b for a, b in zip(o, e))
    k = len(o) - 1
    return {"chi_square": chi2, "df": k, "p_value": _chi2_sf(chi2, k), "expected": e}


def ref_gof(observed, proportions=None):
    import numpy as np
    from scipy.stats import chisquare
    o = np.asarray(observed, dtype=float)
    w = np.ones_like(o) if proportions is None else np.asarray(proportions, dtype=float)
    r = chisquare(o, f_exp=o.sum() * w / w.sum())
    return {"chi_square": float(r.statistic), "p_value": float(r.pvalue)}


def _rnd_gof(rng):
    k = rng.randint(2, 8)
    d = {"observed": [float(rng.randint(0, 60)) for _ in range(k)]}
    if sum(d["observed"]) == 0:
        d["observed"][0] = 5.0
    if rng.random() < 0.6:
        d["proportions"] = [float(rng.randint(1, 10)) for _ in range(k)]
    return d


_cap("chi_square_goodness_of_fit", level="intermediate", name="Chi-square goodness-of-fit test", topic=_HT,
     purpose="The chi-square goodness-of-fit test of observed counts against expected proportions (equal if omitted).",
     inputs=[V("observed", "observed counts per category"),
             V("proportions", "expected proportions or relative weights per category (equal if omitted)", required=False)],
     outputs=[O("chi_square", "sum (O - E)^2 / E"), O("df", "categories - 1"), O("p_value", "upper-tail p-value"),
              O("expected", "expected counts")], primary="p_value",
     conditions=["independent observations", "expected counts at least about 5", "proportions are rescaled to sum to 1"],
     method="expected counts n p_i; chi-square statistic with k - 1 df",
     latex=r"\chi^2 = \sum \frac{(O_i - E_i)^2}{E_i},\ \nu = k - 1", run=run_gof, reference=ref_gof,
     wrong=lambda observed, proportions=None: (lambda r: {"chi_square": math.fsum(abs(a - b) / b for a, b in zip(observed, r["expected"]))})(
         run_gof(observed, proportions)),
     rnd=_rnd_gof, need=r"goodness[- ]of[- ]fit|expected proportions|fair die|uniform(?:ly)? distributed", prio=6,
     templates=["Chi-square goodness-of-fit test for observed [18, 22, 20, 25, 15] with equal expected proportions.",
                "Goodness-of-fit: observed [50, 30, 20] against expected proportions [0.5, 0.3, 0.2]."],
     examples=[X({"observed": "18, 22, 20, 25, 15"}), X({"observed": [50, 30, 20], "proportions": [0.5, 0.3, 0.2]}),
               X({"observed": [30, 50, 20], "proportions": [0.25, 0.5, 0.25]})],
     questions=[Q("Chi-square goodness-of-fit test for observed [18, 22, 20, 25, 15] with equal expected proportions."),
                Q("Goodness-of-fit: observed [50, 30, 20] against expected proportions [0.5, 0.3, 0.2]."),
                Q("Goodness-of-fit: observed [50, 30, 20] against expected proportions [0.5, 0.5]."
                  , outcome="INVALID_INPUT")])


def run_independence(table):
    t = _matrix_ok(table, "the contingency table")
    if any(v < 0 for r in t for v in r):
        raise ValueError("counts must be >= 0")
    rs, cs = [math.fsum(r) for r in t], [math.fsum(c) for c in zip(*t)]
    n = math.fsum(rs)
    if any(v <= 0 for v in rs + cs):
        raise ValueError("every row and column needs a positive total")
    e = [[a * b / n for b in cs] for a in rs]
    chi2 = math.fsum((t[i][j] - e[i][j]) ** 2 / e[i][j] for i in range(len(t)) for j in range(len(t[0])))
    df = (len(t) - 1) * (len(t[0]) - 1)
    return {"chi_square": chi2, "df": df, "p_value": _chi2_sf(chi2, df), "expected": e}


def ref_independence(table):
    from scipy.stats import chi2_contingency
    r = chi2_contingency(table, correction=False)
    return {"chi_square": float(r.statistic), "df": int(r.dof), "p_value": float(r.pvalue),
            "expected": [[float(v) for v in row] for row in r.expected_freq]}


def _rnd_table(rng, lo=2, hi=5, top=60):
    rr, cc = rng.randint(lo, hi), rng.randint(lo, hi)
    while True:
        t = [[float(rng.randint(0, top)) for _ in range(cc)] for _ in range(rr)]
        if all(sum(r) > 0 for r in t) and all(sum(c) > 0 for c in zip(*t)):
            return t


_cap("chi_square_independence", level="intermediate", name="Chi-square test of independence", topic=_HT,
     purpose="The chi-square test of independence (or homogeneity) for an r x c contingency table of counts.",
     inputs=[M("table", "contingency table of counts (rows x columns)")],
     outputs=[O("chi_square", "Pearson chi-square statistic"), O("df", "(r - 1)(c - 1)"), O("p_value", "upper-tail p-value"),
              O("expected", "expected counts under independence")], primary="p_value",
     conditions=["independent observations", "expected counts at least about 5", "no continuity correction"],
     method="expected counts row total x column total / n; Pearson chi-square with (r-1)(c-1) df",
     latex=r"\chi^2 = \sum_{i,j}\frac{(O_{ij} - E_{ij})^2}{E_{ij}},\ E_{ij} = \frac{R_i C_j}{n}", run=run_independence,
     reference=ref_independence,
     wrong=lambda table: (lambda r: {"chi_square": r["chi_square"], "p_value": _chi2_sf(r["chi_square"], len(table) * len(table[0]) - 1)})(
         run_independence(table)),
     rnd=lambda rng: {"table": _rnd_table(rng)}, need=[r"chi", r"independen|association|contingency|homogeneity"],
     avoid=r"variance|goodness", prio=6,
     templates=["Chi-square test of independence for the table [[20, 30], [25, 25]].",
                "Is there an association? Use a chi-square test on the contingency table [[10, 20, 30], [20, 15, 25]]."],
     examples=[X({"table": "[[20, 30], [25, 25]]"}), X({"table": [[10, 20, 30], [20, 15, 25]]})],
     questions=[Q("Chi-square test of independence for the table [[20, 30], [25, 25]]."),
                Q("Is there an association? Use a chi-square test on the contingency table [[10, 20, 30], [20, 15, 25]]."),
                Q("Chi-square test of independence for the table [[20, 30], [25]].", outcome="INVALID_INPUT"),
                Q("Chi-square test of independence for the table [[0, 0], [25, 25]].", outcome="INVALID_INPUT")])


def run_chi2_var(x, sigma0, alternative="two-sided"):
    _check_alt(alternative)
    x = _finite(x)
    _need(x, 2)
    sigma0 = _pos(sigma0, "sigma0")
    n = len(x)
    stat = (n - 1) * _var(x) / sigma0 ** 2
    return {"chi_square": stat, "df": n - 1, "p_value": _alt_p(stat, alternative, lambda v: _chi2_sf(v, n - 1), lambda v: _chi2_cdf(v, n - 1)),
            "sample_variance": _var(x)}


def ref_chi2_var(x, sigma0, alternative="two-sided"):
    import numpy as np
    from scipy.stats import chi2
    n = len(x)
    s = (n - 1) * float(np.var(x, ddof=1)) / sigma0 ** 2
    p = {"greater": chi2.sf(s, n - 1), "less": chi2.cdf(s, n - 1),
         "two-sided": min(1.0, 2 * min(chi2.sf(s, n - 1), chi2.cdf(s, n - 1)))}[alternative]
    return {"chi_square": float(s), "p_value": float(p)}


_cap("chi_square_variance_test", level="advanced", name="Chi-square test for a variance", topic=_HT,
     purpose="The chi-square test of H0: sigma = sigma0 for a normal population from sample data.",
     inputs=[_X2, N("sigma0", "hypothesised standard deviation"), ALT],
     outputs=[O("chi_square", "(n - 1) s^2 / sigma0^2"), O("df", "n - 1"), O("p_value", "p-value"), O("sample_variance", "s^2")],
     primary="p_value", conditions=["normal population (the test is sensitive to non-normality)"],
     method="chi-square statistic with n - 1 df; two-sided p-value = twice the smaller tail",
     latex=r"\chi^2 = \frac{(n-1)s^2}{\sigma_0^2}", run=run_chi2_var, reference=ref_chi2_var,
     wrong=lambda x, sigma0, alternative="two-sided": {"chi_square": len(x) * _var(x) / sigma0 ** 2},
     rnd=lambda rng: (lambda d: dict(d, sigma0=round((_sd(d["x"]) or 1.0) * rng.uniform(0.6, 1.6), 2),
                                     alternative=rng.choice(["two-sided", "greater", "less"])))(_rnd_x(rng, 3, 40)),
     need=[_TESTW, r"chi", r"variance|standard deviation", r"\["], avoid=r"independen|goodness|contingency",
     aliases={"sigma0": r"sigma0|sigma_0|hypothesi[sz]ed standard deviation|sigma"}, extract=_x_alt,
     templates=["Chi-square test for a variance: data [10.2, 9.8, 10.5, 10.1, 9.6, 10.4], sigma0 = 0.3, two-sided.",
                "Test with a chi-square test whether the standard deviation of [52, 48, 55, 45, 50, 58, 41] exceeds sigma0 = 4, right-tailed."],
     examples=[X({"x": "10.2, 9.8, 10.5, 10.1, 9.6, 10.4", "sigma0": 0.3}),
               X({"x": [52, 48, 55, 45, 50, 58, 41], "sigma0": 4, "alternative": "greater"})],
     questions=[Q("Chi-square test for a variance: data [10.2, 9.8, 10.5, 10.1, 9.6, 10.4], sigma0 = 0.3, two-sided."),
                Q("Test with a chi-square test whether the standard deviation of [52, 48, 55, 45, 50, 58, 41] exceeds sigma0 = 4, right-tailed."),
                Q("Chi-square test for a variance: data [10.2, 9.8, 10.5], two-sided.", outcome="NEEDS_INPUT")])


def run_f_test(x, y, alternative="two-sided"):
    _check_alt(alternative)
    x, y = _finite(x, "x"), _finite(y, "y")
    _need(x, 2, "the first sample")
    _need(y, 2, "the second sample")
    v1, v2 = _var(x), _var(y)
    if v1 == 0 or v2 == 0:
        raise ValueError("a sample has zero variance")
    f, d1, d2 = v1 / v2, len(x) - 1, len(y) - 1
    return {"f": f, "df1": d1, "df2": d2,
            "p_value": _alt_p(f, alternative, lambda v: _f_tails(v, d1, d2)[1], lambda v: _f_tails(v, d1, d2)[0])}


def ref_f_test(x, y, alternative="two-sided"):
    import numpy as np
    from scipy.stats import f as F
    s = float(np.var(x, ddof=1) / np.var(y, ddof=1))
    d1, d2 = len(x) - 1, len(y) - 1
    p = {"greater": F.sf(s, d1, d2), "less": F.cdf(s, d1, d2), "two-sided": min(1.0, 2 * min(F.sf(s, d1, d2), F.cdf(s, d1, d2)))}[alternative]
    return {"f": s, "p_value": float(p)}


_cap("f_test_variances", level="advanced", name="F-test for equality of two variances", topic=_HT,
     purpose="The F-test of H0: sigma1^2 = sigma2^2 from two independent normal samples.",
     inputs=[V("x", "first sample"), V("y", "second sample"), ALT],
     outputs=[O("f", "s1^2 / s2^2"), O("df1", "n1 - 1"), O("df2", "n2 - 1"), O("p_value", "p-value")], primary="p_value",
     conditions=["independent samples from normal populations (sensitive to non-normality)"],
     method="variance ratio with (n1 - 1, n2 - 1) df; two-sided p-value = twice the smaller tail",
     latex=r"F = \frac{s_1^2}{s_2^2}", run=run_f_test, reference=ref_f_test,
     wrong=lambda x, y, alternative="two-sided": {"f": _sd(x) / _sd(y)},
     rnd=lambda rng: dict(_rnd_two(rng, 3, 30), alternative=rng.choice(["two-sided", "greater", "less"])),
     need=[r"f[- ]test|equal(?:ity of)? (?:two )?variances|variance ratio", _TWO_LISTS], avoid=r"levene|bartlett|anova|t[- ]test",
     extract=_x_alt,
     templates=["F-test for equality of variances of [21, 24, 19, 26, 23, 25] and [22, 23, 22, 24, 23], two-sided.",
                "Use an F-test to check whether [5.1, 6.3, 4.2, 7.0, 5.5] has a larger variance than [5.0, 5.2, 4.9, 5.1], right-tailed."],
     examples=[X({"x": "21, 24, 19, 26, 23, 25", "y": "22, 23, 22, 24, 23"}),
               X({"x": [5.1, 6.3, 4.2, 7.0, 5.5], "y": [5.0, 5.2, 4.9, 5.1], "alternative": "greater"})],
     questions=[Q("F-test for equality of variances of [21, 24, 19, 26, 23, 25] and [22, 23, 22, 24, 23], two-sided."),
                Q("Use an F-test to check whether [5.1, 6.3, 4.2, 7.0, 5.5] has a larger variance than [5.0, 5.2, 4.9, 5.1], right-tailed."),
                Q("F-test for equality of variances of [21, 24, 19] and [22, 23, 22], one-sided.", outcome="OUT_OF_SCOPE")])

# =================================================================================== correlation and regression ====
_RC = "regression and correlation"
_XY = [V("x", "values of the first variable (x)"), V("y", "paired values of the second variable (y)")]


def _rnd_xy(rng, lo=3, hi=40, ties=True):
    n = rng.randint(lo, hi)
    x = _sample(rng, n, rng.uniform(0, 50), rng.uniform(1, 10), rng.choice([0, 1]) if ties else 3)
    b = rng.uniform(-3, 3)
    y = [round(b * v + rng.gauss(0, rng.uniform(0.5, 20)), rng.choice([0, 1, 2]) if ties else 3) for v in x]
    if len(set(x)) < 2:
        x[0] += 1.0
    if len(set(y)) < 2:
        y[0] += 1.0
    return {"x": x, "y": y}


def run_pearson(x, y):
    x, y = _pair(x, y, 3)
    r, n = _pearson(x, y), len(x)
    t = r * math.sqrt((n - 2) / (1 - r * r)) if abs(r) < 1 else math.copysign(math.inf, r)
    return {"r": r, "r_squared": r * r, "p_value": _t_two_sided(t, n - 2) if abs(r) < 1 else 0.0, "n": n}


def ref_pearson(x, y):
    from scipy.stats import pearsonr
    r = pearsonr(x, y)
    return {"r": float(r.statistic), "p_value": float(r.pvalue)}


_cap("pearson_correlation", level="intermediate", name="Pearson correlation coefficient", topic=_RC,
     purpose="Pearson's correlation coefficient r between paired data, r squared, and the two-sided p-value for H0: rho = 0.",
     inputs=_XY, outputs=[O("r", "correlation coefficient"), O("r_squared", "coefficient of determination"),
                          O("p_value", "two-sided p-value (t with n - 2 df)"), O("n", "number of pairs")], primary="r",
     conditions=["paired observations", "linear relationship; the p-value assumes bivariate normality"],
     method="r = Sxy / sqrt(Sxx Syy); t = r sqrt((n - 2)/(1 - r^2))",
     latex=r"r = \frac{\sum (x_i - \bar{x})(y_i - \bar{y})}{\sqrt{\sum (x_i - \bar{x})^2 \sum (y_i - \bar{y})^2}}", run=run_pearson,
     reference=ref_pearson, wrong=lambda x, y: {"r": _pearson(x, y) * (len(x) - 1) / len(x)}, rnd=_rnd_xy,
     need=[r"correlation|\bpearson\b|\br\b"], avoid=r"spearman|kendall|rank|confidence|interval|partial|serial|auto", prio=4,
     templates=["Find the correlation coefficient of x = [1, 2, 3, 4, 5] and y = [2, 4, 5, 4, 5].",
                "What is the Pearson correlation between [43, 21, 25, 42, 57, 59] and [99, 65, 79, 75, 87, 81]?"],
     examples=[X({"x": "1, 2, 3, 4, 5", "y": "2, 4, 5, 4, 5"}), X({"x": [43, 21, 25, 42, 57, 59], "y": [99, 65, 79, 75, 87, 81]})],
     questions=[Q("Find the correlation coefficient of x = [1, 2, 3, 4, 5] and y = [2, 4, 5, 4, 5]."),
                Q("What is the Pearson correlation between [43, 21, 25, 42, 57, 59] and [99, 65, 79, 75, 87, 81]?"),
                Q("Find the correlation coefficient of x = [1, 2, 3] and y = [2, 4].", outcome="INVALID_INPUT"),
                Q("Find the correlation coefficient of x = [1, 1, 1] and y = [2, 4, 5].", outcome="INVALID_INPUT")])


def run_corr_test(r, n, alternative="two-sided"):
    _check_alt(alternative)
    n = _whole(n, "n", 3)
    if not -1 < r < 1:
        raise ValueError("r must be strictly between -1 and 1")
    t = r * math.sqrt((n - 2) / (1 - r * r))
    return {"t": t, "df": n - 2, "p_value": _alt_p(t, alternative, lambda v: _t_sf(v, n - 2), lambda v: _t_cdf(v, n - 2))}


def ref_corr_test(r, n, alternative="two-sided"):
    from scipy.stats import beta
    d = beta(n / 2 - 1, n / 2 - 1, loc=-1, scale=2)
    p = {"two-sided": 2 * d.cdf(-abs(r)), "greater": d.sf(r), "less": d.cdf(r)}[alternative]
    return {"p_value": float(min(1.0, p))}


_cap("correlation_t_test", level="advanced", name="Significance test for a correlation coefficient", topic=_RC,
     purpose="The t-test of H0: rho = 0 from a sample correlation r and the number of pairs n.",
     inputs=[N("r", "sample correlation coefficient"), I("n", "number of pairs (>= 3)", lo=3), ALT],
     outputs=[O("t", "r sqrt((n - 2)/(1 - r^2))"), O("df", "n - 2"), O("p_value", "p-value")], primary="p_value",
     conditions=["-1 < r < 1", "bivariate normal data"], method="t statistic with n - 2 degrees of freedom",
     latex=r"t = r\sqrt{\frac{n-2}{1-r^2}}", run=run_corr_test, reference=ref_corr_test,
     wrong=lambda r, n, alternative="two-sided": {"p_value": _alt_p(r * math.sqrt(n / (1 - r * r)), alternative, _phic, _phi)},
     rnd=lambda rng: {"r": round(rng.uniform(-0.95, 0.95), 3), "n": rng.randint(4, 200), "alternative": rng.choice(["two-sided", "greater", "less"])},
     need=[_TESTW, r"correlation|\brho\b|\br\s*="], avoid=r"\[|spearman|kendall", prio=5,
     aliases={"r": r"r|correlation coefficient|correlation", "n": r"n|pairs|sample size"}, extract=_x_alt,
     templates=["Test the significance of a correlation r = 0.45 with n = 30 pairs, two-sided.",
                "Is a correlation of r = -0.32 from n = 50 significant? Use a left-tailed test."],
     examples=[X({"r": 0.45, "n": 30}), X({"r": -0.32, "n": 50, "alternative": "less"})],
     questions=[Q("Test the significance of a correlation r = 0.45 with n = 30 pairs, two-sided."),
                Q("Is a correlation of r = -0.32 from n = 50 significant? Use a left-tailed test."),
                Q("Test the significance of a correlation r = 1.2 with n = 30 pairs, two-sided.", outcome="INVALID_INPUT")])


def run_corr_ci(x, y, confidence):
    x, y = _pair(x, y, 4)
    r = _pearson(x, y)
    if abs(r) >= 1:
        raise ValueError("|r| = 1: the Fisher interval is degenerate")
    z, se, c = math.atanh(r), 1 / math.sqrt(len(x) - 3), _zc(confidence)
    return {"r": r, "lower": math.tanh(z - c * se), "upper": math.tanh(z + c * se)}


def ref_corr_ci(x, y, confidence):
    from scipy.stats import pearsonr
    ci = pearsonr(x, y).confidence_interval(confidence_level=confidence)
    return {"lower": float(ci.low), "upper": float(ci.high)}


_cap("correlation_confidence_interval", level="expert", name="Confidence interval for a correlation (Fisher z)", topic=_RC,
     purpose="The Fisher z-transformation confidence interval for the population correlation from paired data.",
     inputs=_XY + [CONF], outputs=[O("r", "sample correlation"), O("lower", "lower limit"), O("upper", "upper limit")],
     primary="lower", conditions=["bivariate normal data", "at least 4 pairs", "|r| < 1"],
     method="tanh(atanh(r) +/- z_(alpha/2) / sqrt(n - 3))", latex=r"\tanh\left(\operatorname{atanh}(r) \pm \frac{z_{\alpha/2}}{\sqrt{n-3}}\right)",
     run=run_corr_ci, reference=ref_corr_ci,
     wrong=lambda x, y, confidence: (lambda r, h: {"lower": r - h, "upper": r + h})(_pearson(x, y), _zc(confidence) / math.sqrt(len(x) - 3)),
     rnd=lambda rng: dict(_rnd_xy(rng, 5, 40), confidence=_rnd_conf(rng)),
     need=[_CIW, r"correlation"], avoid=r"spearman|kendall", prio=6, aliases=_CONF_ALIAS, extract=_x_conf,
     templates=["Find the 95% confidence interval for the correlation of x = [1, 2, 3, 4, 5, 6] and y = [2, 3, 5, 4, 6, 7].",
                "Construct a 90% confidence interval for the correlation between [43, 21, 25, 42, 57, 59] and [99, 65, 79, 75, 87, 81]."],
     examples=[X({"x": "1, 2, 3, 4, 5, 6", "y": "2, 3, 5, 4, 6, 7", "confidence": 0.95}),
               X({"x": [43, 21, 25, 42, 57, 59], "y": [99, 65, 79, 75, 87, 81], "confidence": 0.9})],
     questions=[Q("Find the 95% confidence interval for the correlation of x = [1, 2, 3, 4, 5, 6] and y = [2, 3, 5, 4, 6, 7]."),
                Q("Construct a 90% confidence interval for the correlation between [43, 21, 25, 42, 57, 59] and [99, 65, 79, 75, 87, 81]."),
                Q("Find the 95% confidence interval for the correlation of x = [1, 2, 3] and y = [2, 3, 5].", outcome="INVALID_INPUT")])


def run_spearman(x, y):
    x, y = _pair(x, y, 3)
    rho, n = _pearson(_ranks(x), _ranks(y)), len(x)
    p = 0.0 if abs(rho) >= 1 else _t_two_sided(rho * math.sqrt((n - 2) / (1 - rho * rho)), n - 2)
    return {"rho": rho, "p_value": p, "n": n}


def ref_spearman(x, y):
    from scipy.stats import spearmanr
    r = spearmanr(x, y)
    return {"rho": float(r.statistic), "p_value": float(r.pvalue)}


_cap("spearman_correlation", level="advanced", name="Spearman rank correlation", topic=_RC,
     purpose="Spearman's rank correlation coefficient (ties given average ranks) with its two-sided p-value.",
     inputs=_XY, outputs=[O("rho", "Spearman's rho"), O("p_value", "two-sided p-value (t approximation, n - 2 df)"), O("n", "pairs")],
     primary="rho", conditions=["paired ordinal or continuous data", "p-value by the t approximation"],
     method="Pearson correlation of the average ranks", latex=r"\rho_s = r\left(\operatorname{rank}(x), \operatorname{rank}(y)\right)",
     run=run_spearman, reference=ref_spearman,
     wrong=lambda x, y: (lambda n, rx, ry: {"rho": 1 - 6 * math.fsum((a - b) ** 2 for a, b in zip(rx, ry)) / (n * (n * n - 1)) + 0.1})(
         len(x), _ranks(x), _ranks(y)),
     rnd=_rnd_xy, need=r"spearman|rank correlation", avoid=r"kendall|confidence", prio=5,
     templates=["Find the Spearman rank correlation of x = [10, 20, 30, 40, 50] and y = [12, 25, 22, 41, 48].",
                "What is Spearman's rho between [3, 1, 4, 1, 5, 9] and [2, 7, 1, 8, 2, 8]?"],
     examples=[X({"x": "10, 20, 30, 40, 50", "y": "12, 25, 22, 41, 48"}), X({"x": [3, 1, 4, 1, 5, 9], "y": [2, 7, 1, 8, 2, 8]})],
     questions=[Q("Find the Spearman rank correlation of x = [10, 20, 30, 40, 50] and y = [12, 25, 22, 41, 48]."),
                Q("What is Spearman's rho between [3, 1, 4, 1, 5, 9] and [2, 7, 1, 8, 2, 8]?"),
                Q("Find the Spearman rank correlation of x = [10, 20] and y = [12, 25].", outcome="INVALID_INPUT")])


def run_kendall(x, y):
    x, y = _pair(x, y, 3)
    n = len(x)
    con = dis = 0
    for i in range(n):
        for j in range(i + 1, n):
            s = (x[i] - x[j]) * (y[i] - y[j])
            con += s > 0
            dis += s < 0
    tx, ty = _tie_counts(x), _tie_counts(y)
    n0 = n * (n - 1) / 2
    n1, n2 = sum(t * (t - 1) / 2 for t in tx), sum(t * (t - 1) / 2 for t in ty)
    if n0 == n1 or n0 == n2:
        raise ValueError("a variable is constant: Kendall's tau is undefined")
    tau = (con - dis) / math.sqrt((n0 - n1) * (n0 - n2))
    m = n * (n - 1.0)
    x0, y0 = sum(t * (t - 1) * (t - 2) for t in tx), sum(t * (t - 1) * (t - 2) for t in ty)
    x1, y1 = sum(t * (t - 1) * (2 * t + 5) for t in tx), sum(t * (t - 1) * (2 * t + 5) for t in ty)
    var = (m * (2 * n + 5) - x1 - y1) / 18 + 2 * n1 * n2 / m + x0 * y0 / (9 * m * (n - 2))
    z = (con - dis) / math.sqrt(var)
    return {"tau": tau, "p_value": min(1.0, 2 * _phic(abs(z))), "concordant": con, "discordant": dis}


def ref_kendall(x, y):
    from scipy.stats import kendalltau
    r = kendalltau(x, y, method="asymptotic")
    return {"tau": float(r.statistic), "p_value": float(r.pvalue)}


_cap("kendall_tau", level="advanced", name="Kendall's tau-b rank correlation", topic=_RC,
     purpose="Kendall's tau-b (tie-adjusted) between paired data with its two-sided large-sample p-value.",
     inputs=_XY, outputs=[O("tau", "Kendall's tau-b"), O("p_value", "two-sided p-value (normal approximation with tie correction)"),
                          O("concordant", "concordant pairs"), O("discordant", "discordant pairs")], primary="tau",
     conditions=["paired ordinal or continuous data", "large-sample normal approximation for the p-value"],
     method="count concordant and discordant pairs; tau-b = (C - D)/sqrt((n0 - n1)(n0 - n2)); tie-corrected variance of C - D",
     latex=r"\tau_b = \frac{C - D}{\sqrt{(n_0 - n_1)(n_0 - n_2)}}", run=run_kendall, reference=ref_kendall,
     wrong=lambda x, y: (lambda r, n: {"tau": (r["concordant"] - r["discordant"]) / (n * (n - 1) / 2) + (0.05 if r["concordant"] == r["discordant"] else 0)})(
         run_kendall(x, y), len(x)),
     rnd=_rnd_xy, need=r"kendall", prio=5,
     templates=["Find Kendall's tau for x = [1, 2, 3, 4, 5, 6] and y = [2, 1, 4, 3, 6, 5].",
                "What is the Kendall rank correlation between [12, 2, 1, 12, 2] and [1, 4, 7, 1, 0]?"],
     examples=[X({"x": "1, 2, 3, 4, 5, 6", "y": "2, 1, 4, 3, 6, 5"}), X({"x": [12, 2, 1, 12, 2], "y": [1, 4, 7, 1, 0]})],
     questions=[Q("Find Kendall's tau for x = [1, 2, 3, 4, 5, 6] and y = [2, 1, 4, 3, 6, 5]."),
                Q("What is the Kendall rank correlation between [12, 2, 1, 12, 2] and [1, 4, 7, 1, 0]?"),
                Q("Find Kendall's tau for x = [1, 1, 1] and y = [2, 1, 4].", outcome="INVALID_INPUT")])


def run_cov(x, y):
    x, y = _pair(x, y, 2)
    mx, my = _mean(x), _mean(y)
    s = math.fsum((a - mx) * (b - my) for a, b in zip(x, y))
    return {"sample_covariance": s / (len(x) - 1), "population_covariance": s / len(x)}


_cap("covariance", level="intermediate", name="Covariance", topic=_RC,
     purpose="The sample (n - 1) and population (n) covariance of paired data.",
     inputs=_XY, outputs=[O("sample_covariance", "divisor n - 1"), O("population_covariance", "divisor n")], primary="sample_covariance",
     method="sum of products of deviations divided by n - 1 (sample) or n (population)",
     latex=r"s_{xy} = \frac{1}{n-1}\sum (x_i - \bar{x})(y_i - \bar{y})", run=run_cov,
     reference=lambda x, y: (lambda np: {"sample_covariance": float(np.cov(x, y, ddof=1)[0, 1]), "population_covariance": float(np.cov(x, y, ddof=0)[0, 1])})(
         __import__("numpy")),
     wrong=lambda x, y: {"sample_covariance": run_cov(x, y)["population_covariance"]}, rnd=lambda rng: _rnd_xy(rng, 2, 40),
     need=r"covariance", avoid=r"analysis of covariance|ancova|matrix", prio=5,
     templates=["Find the covariance of x = [2, 4, 6, 8] and y = [1, 3, 2, 5].", "What is the covariance between [1.5, 2.5, 3.0] and [10, 12, 15]?"],
     examples=[X({"x": "2, 4, 6, 8", "y": "1, 3, 2, 5"}), X({"x": [1.5, 2.5, 3.0], "y": [10, 12, 15]})],
     questions=[Q("Find the covariance of x = [2, 4, 6, 8] and y = [1, 3, 2, 5]."),
                Q("What is the covariance between [1.5, 2.5, 3.0] and [10, 12, 15]?"),
                Q("Find the covariance of x = [2, 4, 6] and y = [1, 3].", outcome="INVALID_INPUT")])


def _simple_fit(x, y):
    x, y = _pair(x, y, 3)
    mx, my = _mean(x), _mean(y)
    sxx = math.fsum((a - mx) ** 2 for a in x)
    if sxx == 0:
        raise ValueError("all x values are equal: the slope is undefined")
    sxy = math.fsum((a - mx) * (b - my) for a, b in zip(x, y))
    b1 = sxy / sxx
    return x, y, mx, my, sxx, b1, my - b1 * mx


def run_simple_regression(x, y):
    x, y, mx, my, sxx, b1, b0 = _simple_fit(x, y)
    n = len(x)
    sse = math.fsum((b - b0 - b1 * a) ** 2 for a, b in zip(x, y))
    syy = math.fsum((b - my) ** 2 for b in y)
    r2 = 1 - sse / syy if syy > 0 else 1.0
    s2 = sse / (n - 2)
    se1 = math.sqrt(s2 / sxx)
    se0 = math.sqrt(s2 * (1 / n + mx * mx / sxx))
    p = _t_two_sided(b1 / se1, n - 2) if se1 > 0 else 0.0
    r = math.copysign(math.sqrt(max(r2, 0.0)), b1) if b1 != 0 else 0.0
    return {"slope": b1, "intercept": b0, "r": r, "r_squared": r2, "p_value": p, "slope_se": se1, "intercept_se": se0,
            "residual_se": math.sqrt(s2)}


def ref_simple_regression(x, y):
    from scipy.stats import linregress
    r = linregress(x, y)
    return {"slope": float(r.slope), "intercept": float(r.intercept), "r": float(r.rvalue), "p_value": float(r.pvalue),
            "slope_se": float(r.stderr), "intercept_se": float(r.intercept_stderr)}


_cap("simple_linear_regression", level="intermediate", name="Simple linear regression (least squares line)", topic=_RC,
     purpose="The least-squares line y = intercept + slope x: coefficients, r, r squared, standard errors and the slope's p-value.",
     inputs=_XY, outputs=[O("slope", "b1 = Sxy/Sxx"), O("intercept", "b0 = ybar - b1 xbar"), O("r", "correlation"),
                          O("r_squared", "coefficient of determination"), O("p_value", "two-sided p-value for H0: slope = 0"),
                          O("slope_se", "standard error of the slope"), O("intercept_se", "standard error of the intercept"),
                          O("residual_se", "residual standard error")], primary="slope",
     conditions=["at least 3 pairs, x not all equal", "inference assumes independent normal errors with constant variance"],
     method="ordinary least squares; t test on the slope with n - 2 df",
     latex=r"\hat{y} = b_0 + b_1 x,\quad b_1 = \frac{S_{xy}}{S_{xx}},\quad b_0 = \bar{y} - b_1\bar{x}", run=run_simple_regression,
     reference=ref_simple_regression,
     wrong=lambda x, y: (lambda mx, my: {"slope": math.fsum((a - mx) * (b - my) for a, b in zip(x, y)) / math.fsum((b - my) ** 2 for b in y)})(
         _mean(x), _mean(y)),
     rnd=_rnd_xy, need=r"regression|least[- ]squares|line of best fit|best[- ]fit line|trend line|regression line",
     avoid=r"multiple|predict|estimate y|value of y|y when|logistic|durbin", prio=4,
     templates=["Find the least squares regression line for x = [1, 2, 3, 4, 5] and y = [2.2, 4.1, 6.1, 7.9, 10.2].",
                "Fit a simple linear regression of y = [65, 70, 75, 82, 88] on x = [2, 3, 4, 5, 6]."],
     examples=[X({"x": "1, 2, 3, 4, 5", "y": "2.2, 4.1, 6.1, 7.9, 10.2"}), X({"x": [2, 3, 4, 5, 6], "y": [65, 70, 75, 82, 88]})],
     questions=[Q("Find the least squares regression line for x = [1, 2, 3, 4, 5] and y = [2.2, 4.1, 6.1, 7.9, 10.2]."),
                Q("Find the regression line for x = [2, 3, 4, 5, 6] and y = [65, 70, 75, 82, 88]."),
                Q("Find the regression line for x = [2, 2, 2] and y = [65, 70, 75].", outcome="INVALID_INPUT")])


def run_predict(x, y, x0):
    x, y, mx, my, sxx, b1, b0 = _simple_fit(x, y)
    return {"y_hat": b0 + b1 * x0, "slope": b1, "intercept": b0}


def ref_predict(x, y, x0):
    import numpy as np
    c = np.polyfit(np.asarray(x, dtype=float), np.asarray(y, dtype=float), 1)
    return {"y_hat": float(np.polyval(c, x0)), "slope": float(c[0]), "intercept": float(c[1])}


_cap("regression_prediction", level="intermediate", name="Prediction from a least squares line", topic=_RC,
     purpose="The predicted y at a given x0 from the least-squares line fitted to paired data.",
     inputs=_XY + [N("x0", "the x value to predict at")],
     outputs=[O("y_hat", "b0 + b1 x0"), O("slope", "slope"), O("intercept", "intercept")], primary="y_hat",
     conditions=["x0 within the range of the data (extrapolation is unreliable)"], method="fit by least squares, evaluate at x0",
     latex=r"\hat{y}_0 = b_0 + b_1 x_0", run=run_predict, reference=ref_predict,
     wrong=lambda x, y, x0: (lambda r: {"y_hat": r["slope"] * x0})(run_predict(x, y, x0)),
     rnd=lambda rng: (lambda d: dict(d, x0=round(rng.uniform(min(d["x"]), max(d["x"])), 2)))(_rnd_xy(rng)),
     need=[r"regression|least[- ]squares|best[- ]fit|predict", r"predict|estimate|value of y|y when|at x"], avoid=r"multiple|logistic", prio=5,
     aliases={"x0": r"x0|x_0|x"},
     templates=["Using the regression line for x = [1, 2, 3, 4, 5] and y = [2.2, 4.1, 6.1, 7.9, 10.2], predict y at x0 = 3.5.",
                "Fit a least squares line to x = [2, 3, 4, 5, 6] and y = [65, 70, 75, 82, 88] and predict y when x0 = 4.5."],
     examples=[X({"x": "1, 2, 3, 4, 5", "y": "2.2, 4.1, 6.1, 7.9, 10.2", "x0": 3.5}),
               X({"x": [2, 3, 4, 5, 6], "y": [65, 70, 75, 82, 88], "x0": 4.5})],
     questions=[Q("Using the regression line for x = [1, 2, 3, 4, 5] and y = [2.2, 4.1, 6.1, 7.9, 10.2], predict y at x0 = 3.5."),
                Q("Fit a least squares line to x = [2, 3, 4, 5, 6] and y = [65, 70, 75, 82, 88] and predict y when x0 = 4.5."),
                Q("Using the regression line for x = [1, 2, 3, 4, 5] and y = [2.2, 4.1, 6.1, 7.9, 10.2], predict y.", outcome="NEEDS_INPUT")])


def _solve(A, b):
    """Gaussian elimination with partial pivoting (A square, b a vector); ValueError if singular."""
    n = len(A)
    M = [list(map(float, A[i])) + [float(b[i])] for i in range(n)]
    scale = max((abs(M[i][i]) for i in range(n)), default=1.0) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        if abs(M[p][c]) <= 1e-12 * scale:
            raise ValueError("the predictors are collinear (the design matrix is singular)")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (M[i][n] - math.fsum(M[i][k] * x[k] for k in range(i + 1, n))) / M[i][i]
    return x


def run_multiple_regression(X, y):
    X = _matrix_ok(X, "the predictor matrix", min_rows=3, min_cols=1)
    y = _finite(y, "y")
    n, k = len(X), len(X[0])
    if len(y) != n:
        raise ValueError("y needs one value per row of the predictor matrix")
    if n <= k + 1:
        raise ValueError("more observations than predictors + 1 are required")
    mx = [_mean([r[j] for r in X]) for j in range(k)]
    my = _mean(y)
    Xc = [[r[j] - mx[j] for j in range(k)] for r in X]
    yc = [v - my for v in y]
    S = [[math.fsum(row[a] * row[b] for row in Xc) for b in range(k)] for a in range(k)]
    Sy = [math.fsum(row[a] * v for row, v in zip(Xc, yc)) for a in range(k)]
    b = _solve(S, Sy)
    b0 = my - math.fsum(bj * m for bj, m in zip(b, mx))
    res = [v - b0 - math.fsum(bj * xj for bj, xj in zip(b, r)) for r, v in zip(X, y)]
    sse, sst = math.fsum(e * e for e in res), math.fsum(v * v for v in yc)
    if sst == 0:
        raise ValueError("y is constant")
    r2 = 1 - sse / sst
    dfe = n - k - 1
    adj = 1 - (1 - r2) * (n - 1) / dfe
    f = ((sst - sse) / k) / (sse / dfe) if sse > 0 else math.inf
    return {"coefficients": [b0] + b, "r_squared": r2, "adj_r_squared": adj, "f": f,
            "f_p_value": _f_tails(f, k, dfe)[1] if sse > 0 else 0.0, "residual_se": math.sqrt(sse / dfe)}


def ref_multiple_regression(X, y):
    import numpy as np
    import statsmodels.api as sm
    m = sm.OLS(np.asarray(y, dtype=float), sm.add_constant(np.asarray(X, dtype=float), has_constant="add")).fit()
    return {"coefficients": [float(v) for v in m.params], "r_squared": float(m.rsquared), "adj_r_squared": float(m.rsquared_adj),
            "f": float(m.fvalue), "f_p_value": float(m.f_pvalue)}


def _rnd_mreg(rng):
    n, k = rng.randint(6, 30), rng.randint(1, 4)
    X = [[round(rng.uniform(0, 20), 1) for _ in range(k)] for _ in range(n)]
    beta = [rng.uniform(-5, 5) for _ in range(k + 1)]
    y = [round(beta[0] + sum(b * v for b, v in zip(beta[1:], r)) + rng.gauss(0, rng.uniform(0.5, 8)), 2) for r in X]
    return {"X": X, "y": y}


_cap("multiple_linear_regression", level="advanced", name="Multiple linear regression", topic=_RC,
     purpose="Ordinary least squares with several predictors: coefficients (intercept first), R squared, adjusted R "
             "squared and the overall F-test.",
     inputs=[M("X", "predictor matrix: one row per observation, one column per predictor"), V("y", "response values", 3)],
     outputs=[O("coefficients", "[intercept, b1, ..., bk]"), O("r_squared", "R squared"), O("adj_r_squared", "adjusted R squared"),
              O("f", "overall F statistic"), O("f_p_value", "p-value of the F test"), O("residual_se", "residual standard error")],
     primary="r_squared", conditions=["n > k + 1", "predictors not collinear", "inference assumes independent normal errors"],
     method="centred normal equations solved by Gaussian elimination with partial pivoting; F = (SSR/k)/(SSE/(n-k-1))",
     latex=r"\hat{\beta} = (X^\top X)^{-1}X^\top y,\quad R^2 = 1 - \frac{SSE}{SST}", run=run_multiple_regression,
     reference=ref_multiple_regression,
     wrong=lambda X, y: (lambda r: dict(r, adj_r_squared=r["r_squared"], r_squared=r["adj_r_squared"]))(run_multiple_regression(X, y)),
     rnd=_rnd_mreg, need=r"multiple (?:linear )?regression|regress y on|predictors", prio=6,
     templates=["Fit a multiple regression of y = [10, 12, 15, 19, 22, 25] on X = [[1, 2], [2, 1], [3, 4], [4, 3], [5, 6], [6, 5]].",
                "Multiple linear regression with predictors X = [[2, 50], [3, 60], [5, 55], [7, 70], [8, 65], [9, 80], [11, 75]] and y = [20, 25, 28, 35, 36, 42, 44]."],
     examples=[X({"X": "[[1, 2], [2, 1], [3, 4], [4, 3], [5, 6], [6, 5]]", "y": "10, 12, 15, 19, 22, 25"}),
               X({"X": [[2, 50], [3, 60], [5, 55], [7, 70], [8, 65], [9, 80], [11, 75]], "y": [20, 25, 28, 35, 36, 42, 44]})],
     questions=[Q("Fit a multiple regression of y = [10, 12, 15, 19, 22, 25] on X = [[1, 2], [2, 1], [3, 4], [4, 3], [5, 6], [6, 5]]."),
                Q("Multiple linear regression with predictors X = [[2, 50], [3, 60], [5, 55], [7, 70], [8, 65], [9, 80], [11, 75]] and y = [20, 25, 28, 35, 36, 42, 44]."),
                Q("Fit a multiple regression of y = [1, 2, 3] on X = [[1, 2], [2, 4], [3, 6]].", outcome="INVALID_INPUT")])


def run_durbin_watson(residuals):
    e = _finite(residuals, "residuals")
    _need(e, 2, "the residuals")
    den = math.fsum(v * v for v in e)
    if den == 0:
        raise ValueError("all residuals are zero")
    return {"durbin_watson": math.fsum((e[i] - e[i - 1]) ** 2 for i in range(1, len(e))) / den}


_cap("durbin_watson", level="expert", name="Durbin-Watson statistic", topic=_RC,
     purpose="The Durbin-Watson statistic of regression residuals (about 2: no first-order autocorrelation; towards 0: "
             "positive; towards 4: negative).",
     inputs=[V("residuals", "regression residuals in time order")], outputs=[O("durbin_watson", "sum of squared successive differences / sum of squares")],
     primary="durbin_watson", conditions=["residuals in time order"], method="d = sum (e_t - e_(t-1))^2 / sum e_t^2",
     latex=r"d = \frac{\sum_{t=2}^{n} (e_t - e_{t-1})^2}{\sum_{t=1}^{n} e_t^2}", run=run_durbin_watson,
     reference=lambda residuals: {"durbin_watson": float(__import__("statsmodels.stats.stattools", fromlist=["durbin_watson"]).durbin_watson(residuals))},
     wrong=lambda residuals: {"durbin_watson": math.fsum((residuals[i] - residuals[i - 1]) ** 2 for i in range(1, len(residuals))) / max(1e-300, _ss(residuals))
                              + (1.0 if abs(_mean(residuals)) < 1e-12 else 0.0)},
     rnd=lambda rng: {"residuals": [round(rng.gauss(0, 3), 2) for _ in range(rng.randint(4, 40))]},
     need=r"durbin[- ]watson", prio=6,
     templates=["Compute the Durbin-Watson statistic for residuals [0.5, -0.2, 0.3, -0.4, 0.1, 0.6, -0.5].",
                "What is the Durbin-Watson statistic of the residuals [1.2, 0.8, 0.5, -0.1, -0.6, -0.9]?"],
     examples=[X({"residuals": "0.5, -0.2, 0.3, -0.4, 0.1, 0.6, -0.5"}), X({"residuals": [1.2, 0.8, 0.5, -0.1, -0.6, -0.9]})],
     questions=[Q("Compute the Durbin-Watson statistic for residuals [0.5, -0.2, 0.3, -0.4, 0.1, 0.6, -0.5]."),
                Q("What is the Durbin-Watson statistic of the residuals [1.2, 0.8, 0.5, -0.1, -0.6, -0.9]?"),
                Q("Compute the Durbin-Watson statistic for residuals [0, 0, 0].", outcome="INVALID_INPUT")])


# ============================================================================== analysis of variance (ANOVA) ====
_AN = "analysis of variance"
_G = [V("g1", "group 1"), V("g2", "group 2"), V("g3", "group 3", required=False), V("g4", "group 4", required=False),
      V("g5", "group 5", required=False)]


def _rnd_groups(rng, lo=2, hi=5, nlo=2, nhi=15, dec=None):
    k = rng.randint(lo, hi)
    base = rng.uniform(10, 100)
    sd = rng.uniform(1, 10)
    d = rng.choice([0, 1, 2]) if dec is None else dec
    return {f"g{i + 1}": _sample(rng, rng.randint(nlo, nhi), base + rng.uniform(-sd * 1.5, sd * 1.5), sd, d) for i in range(k)}


def run_anova(g1, g2, g3=None, g4=None, g5=None):
    gs = _groups(g1, g2, g3, g4, g5)
    for g in gs:
        _need(g, 1, "each group")
    allv = [v for g in gs for v in g]
    N, k = len(allv), len(gs)
    if N <= k:
        raise ValueError("more observations than groups are required")
    gm = _mean(allv)
    ssb = math.fsum(len(g) * (_mean(g) - gm) ** 2 for g in gs)
    ssw = math.fsum(_ss(g) for g in gs)
    if ssw == 0:
        raise ValueError("there is no variation within the groups")
    dfb, dfw = k - 1, N - k
    f = (ssb / dfb) / (ssw / dfw)
    return {"f": f, "p_value": _f_tails(f, dfb, dfw)[1], "df_between": dfb, "df_within": dfw, "ss_between": ssb,
            "ss_within": ssw, "ms_between": ssb / dfb, "ms_within": ssw / dfw, "eta_squared": ssb / (ssb + ssw)}


def ref_anova(g1, g2, g3=None, g4=None, g5=None):
    from scipy.stats import f_oneway
    gs = [g for g in (g1, g2, g3, g4, g5) if g is not None]
    r = f_oneway(*gs)
    N, k = sum(len(g) for g in gs), len(gs)
    F = float(r.statistic)
    return {"f": F, "p_value": float(r.pvalue), "eta_squared": F * (k - 1) / (F * (k - 1) + (N - k))}


_cap("one_way_anova", level="advanced", name="One-way ANOVA", topic=_AN,
     purpose="One-way analysis of variance for 2 to 5 independent groups: F statistic, p-value, sums of squares and eta squared.",
     inputs=_G, outputs=[O("f", "MS between / MS within"), O("p_value", "upper-tail p-value"), O("df_between", "k - 1"),
                         O("df_within", "N - k"), O("ss_between", "between-group sum of squares"), O("ss_within", "within-group sum of squares"),
                         O("ms_between", "mean square between"), O("ms_within", "mean square within"), O("eta_squared", "SSB / SST")],
     primary="p_value", conditions=["independent groups", "roughly normal populations with equal variances"],
     method="partition the total sum of squares into between- and within-group parts; F with (k - 1, N - k) df",
     latex=r"F = \frac{SS_B/(k-1)}{SS_W/(N-k)}", run=run_anova, reference=ref_anova,
     wrong=lambda g1, g2, g3=None, g4=None, g5=None: (lambda r: {"f": r["ss_between"] / r["ss_within"]})(run_anova(g1, g2, g3, g4, g5)),
     rnd=_rnd_groups, need=r"\banova\b|analysis of variance", avoid=r"two-way|block|repeated|kruskal|matrix|\[\s*\[", prio=5,
     templates=["Perform a one-way ANOVA on groups [23, 25, 21, 22], [30, 28, 31, 29] and [26, 27, 24, 25].",
                "One-way analysis of variance for [5.1, 4.8, 5.5], [6.2, 6.0, 6.8, 6.4] and [5.9, 5.6, 6.1]."],
     examples=[X({"g1": "23, 25, 21, 22", "g2": "30, 28, 31, 29", "g3": "26, 27, 24, 25"}),
               X({"g1": [5.1, 4.8, 5.5], "g2": [6.2, 6.0, 6.8, 6.4], "g3": [5.9, 5.6, 6.1]})],
     questions=[Q("Perform a one-way ANOVA on groups [23, 25, 21, 22], [30, 28, 31, 29] and [26, 27, 24, 25]."),
                Q("One-way analysis of variance for [5.1, 4.8, 5.5], [6.2, 6.0, 6.8, 6.4] and [5.9, 5.6, 6.1]."),
                Q("Perform a one-way ANOVA on groups [1, 2], [3, 4], [5, 6], [7, 8], [9, 10], [11, 12].", outcome="OUT_OF_SCOPE"),
                Q("Perform a one-way ANOVA on groups [23, 25, 21, 22].", outcome="NEEDS_INPUT")])


def run_block_anova(data):
    d = _matrix_ok(data, "the data table", 2, 2)
    b, t = len(d), len(d[0])
    gm = _mean([v for r in d for v in r])
    rm = [_mean(r) for r in d]
    cm = [_mean(c) for c in zip(*d)]
    sst = math.fsum((d[i][j] - cm[j] - rm[i] + gm) ** 2 for i in range(b) for j in range(t))
    ss_tr = b * math.fsum((c - gm) ** 2 for c in cm)
    ss_bl = t * math.fsum((r - gm) ** 2 for r in rm)
    if sst == 0:
        raise ValueError("there is no residual variation")
    dft, dfb, dfe = t - 1, b - 1, (t - 1) * (b - 1)
    ft, fb = (ss_tr / dft) / (sst / dfe), (ss_bl / dfb) / (sst / dfe)
    return {"f_treatment": ft, "p_treatment": _f_tails(ft, dft, dfe)[1], "f_block": fb, "p_block": _f_tails(fb, dfb, dfe)[1],
            "ss_treatment": ss_tr, "ss_block": ss_bl, "ss_error": sst, "df_treatment": dft, "df_block": dfb, "df_error": dfe}


def ref_block_anova(data):
    import pandas as pd
    import statsmodels.api as sm
    import statsmodels.formula.api as smf
    rows = [(float(v), f"b{i}", f"t{j}") for i, r in enumerate(data) for j, v in enumerate(r)]
    df = pd.DataFrame(rows, columns=["y", "blk", "trt"])
    a = sm.stats.anova_lm(smf.ols("y ~ trt + blk", data=df).fit(), typ=2)
    return {"f_treatment": float(a.loc["trt", "F"]), "p_treatment": float(a.loc["trt", "PR(>F)"]),
            "f_block": float(a.loc["blk", "F"]), "p_block": float(a.loc["blk", "PR(>F)"]),
            "ss_treatment": float(a.loc["trt", "sum_sq"]), "ss_block": float(a.loc["blk", "sum_sq"]),
            "ss_error": float(a.loc["Residual", "sum_sq"])}


def _rnd_block(rng):
    b, t = rng.randint(2, 8), rng.randint(2, 6)
    be = [rng.uniform(-5, 5) for _ in range(b)]
    te = [rng.uniform(-5, 5) for _ in range(t)]
    return {"data": [[round(50 + be[i] + te[j] + rng.gauss(0, 2), 1) for j in range(t)] for i in range(b)]}


_cap("randomized_block_anova", level="advanced", name="Randomized complete block design (two-way ANOVA without replication)",
     topic="experimental design",
     purpose="Two-way ANOVA for a randomized complete block design: rows are blocks, columns are treatments, one "
             "observation per cell; F-tests for treatments and blocks.",
     inputs=[M("data", "table with one row per block and one column per treatment")],
     outputs=[O("f_treatment", "F for treatments"), O("p_treatment", "p-value for treatments"), O("f_block", "F for blocks"),
              O("p_block", "p-value for blocks"), O("ss_treatment", "treatment sum of squares"), O("ss_block", "block sum of squares"),
              O("ss_error", "error sum of squares"), O("df_treatment", "t - 1"), O("df_block", "b - 1"), O("df_error", "(t - 1)(b - 1)")],
     primary="p_treatment", conditions=["one observation per block and treatment", "additive model (no block x treatment interaction)",
                                        "normal errors with constant variance"],
     method="sums of squares for treatments, blocks and the residual y_ij - row mean - column mean + grand mean",
     latex=r"F_{\text{trt}} = \frac{SS_{\text{trt}}/(t-1)}{SS_E/((t-1)(b-1))}", run=run_block_anova, reference=ref_block_anova,
     wrong=lambda data: (lambda r: {"f_treatment": r["f_block"], "f_block": r["f_treatment"]})(run_block_anova(data)),
     rnd=_rnd_block, need=r"randomi[sz]ed (?:complete )?block|two-way anova without replication|blocking", prio=6,
     templates=["Analyse the randomized block design with data [[12, 14, 15], [10, 13, 13], [11, 15, 16], [9, 12, 14]].",
                "Two-way ANOVA without replication for [[5.1, 5.6, 6.0, 5.8], [4.8, 5.2, 5.9, 5.5], [5.4, 5.9, 6.3, 6.1]]."],
     examples=[X({"data": "[[12, 14, 15], [10, 13, 13], [11, 15, 16], [9, 12, 14]]"}),
               X({"data": [[5.1, 5.6, 6.0, 5.8], [4.8, 5.2, 5.9, 5.5], [5.4, 5.9, 6.3, 6.1]]})],
     questions=[Q("Analyse the randomized block design with data [[12, 14, 15], [10, 13, 13], [11, 15, 16], [9, 12, 14]]."),
                Q("Two-way ANOVA without replication for [[5.1, 5.6, 6.0, 5.8], [4.8, 5.2, 5.9, 5.5], [5.4, 5.9, 6.3, 6.1]]."),
                Q("Analyse the randomized block design with data [[12, 14, 15]].", outcome="INVALID_INPUT")])


def run_levene(g1, g2, g3=None, g4=None, g5=None, center="median"):
    gs = _groups(g1, g2, g3, g4, g5)
    c = _median if center == "median" else _mean
    z = [[abs(v - c(g)) for v in g] for g in gs]
    for g in gs:
        _need(g, 2, "each group")
    r = run_anova(*z)
    return {"w": r["f"], "p_value": r["p_value"], "df1": r["df_between"], "df2": r["df_within"]}


def ref_levene(g1, g2, g3=None, g4=None, g5=None, center="median"):
    from scipy.stats import levene
    r = levene(*[g for g in (g1, g2, g3, g4, g5) if g is not None], center=center)
    return {"w": float(r.statistic), "p_value": float(r.pvalue)}


def _x_center(q, d):
    if re.search(r"brown|median", q):
        d["center"] = "median"
    elif re.search(r"\bmean\b|original", q):
        d["center"] = "mean"


_cap("levene_test", level="advanced", name="Levene's test for equal variances", topic=_HT,
     purpose="Levene's test of equal variances across 2 to 5 groups, centred at the median (Brown-Forsythe) or the mean.",
     inputs=_G + [C("center", "median (Brown-Forsythe) or mean (Levene's original)", ["median", "mean"], required=True)],
     outputs=[O("w", "Levene statistic"), O("p_value", "p-value"), O("df1", "k - 1"), O("df2", "N - k")], primary="p_value",
     conditions=["independent groups, at least 2 values each"],
     method="one-way ANOVA on the absolute deviations from each group's median (or mean)",
     latex=r"W = \frac{(N-k)\sum n_i(\bar{z}_{i\cdot} - \bar{z})^2}{(k-1)\sum\sum (z_{ij} - \bar{z}_{i\cdot})^2},\ z_{ij} = |x_{ij} - \tilde{x}_i|",
     run=run_levene, reference=ref_levene,
     wrong=lambda g1, g2, g3=None, g4=None, g5=None, center="median": run_levene(g1, g2, g3, g4, g5, "mean" if center == "median" else "median"),
     rnd=lambda rng: dict(_rnd_groups(rng, nlo=3), center=rng.choice(["median", "mean"])),
     need=r"levene|brown[- ]forsythe", prio=6, extract=_x_center,
     templates=["Levene's test (median centred) for [21, 24, 19, 26, 23], [22, 23, 22, 24, 23] and [18, 28, 20, 30, 24].",
                "Use Levene's test with the mean to compare variances of [5.1, 6.3, 4.2, 7.0] and [5.0, 5.2, 4.9, 5.1]."],
     examples=[X({"g1": "21, 24, 19, 26, 23", "g2": "22, 23, 22, 24, 23", "g3": "18, 28, 20, 30, 24", "center": "median"}),
               X({"g1": [5.1, 6.3, 4.2, 7.0], "g2": [5.0, 5.2, 4.9, 5.1], "center": "mean"})],
     questions=[Q("Levene's test (median centred) for [21, 24, 19, 26, 23], [22, 23, 22, 24, 23] and [18, 28, 20, 30, 24]."),
                Q("Use Levene's test with the mean to compare variances of [5.1, 6.3, 4.2, 7.0] and [5.0, 5.2, 4.9, 5.1]."),
                Q("Levene's test for [21, 24, 19] and [22, 23, 22].", outcome="NEEDS_INPUT")])


def run_bartlett(g1, g2, g3=None, g4=None, g5=None):
    gs = _groups(g1, g2, g3, g4, g5)
    for g in gs:
        _need(g, 2, "each group")
    vs = [_var(g) for g in gs]
    if any(v == 0 for v in vs):
        raise ValueError("a group has zero variance")
    k, N = len(gs), sum(len(g) for g in gs)
    sp = math.fsum((len(g) - 1) * v for g, v in zip(gs, vs)) / (N - k)
    num = (N - k) * math.log(sp) - math.fsum((len(g) - 1) * math.log(v) for g, v in zip(gs, vs))
    den = 1 + (math.fsum(1 / (len(g) - 1) for g in gs) - 1 / (N - k)) / (3 * (k - 1))
    t = num / den
    return {"statistic": t, "df": k - 1, "p_value": _chi2_sf(t, k - 1)}


_cap("bartlett_test", level="advanced", name="Bartlett's test for equal variances", topic=_HT,
     purpose="Bartlett's test of equal variances across 2 to 5 groups (normal populations).",
     inputs=_G, outputs=[O("statistic", "Bartlett's chi-square statistic"), O("df", "k - 1"), O("p_value", "p-value")],
     primary="p_value", conditions=["normal populations (Bartlett's test is sensitive to non-normality)", "at least 2 values per group"],
     method="log of the pooled variance against the group log variances, with Bartlett's correction",
     latex=r"T = \frac{(N-k)\ln s_p^2 - \sum (n_i - 1)\ln s_i^2}{1 + \frac{1}{3(k-1)}\left(\sum \frac{1}{n_i - 1} - \frac{1}{N-k}\right)}",
     run=run_bartlett,
     reference=lambda g1, g2, g3=None, g4=None, g5=None: (lambda r: {"statistic": float(r.statistic), "p_value": float(r.pvalue)})(
         __import__("scipy.stats", fromlist=["bartlett"]).bartlett(*[g for g in (g1, g2, g3, g4, g5) if g is not None])),
     wrong=lambda g1, g2, g3=None, g4=None, g5=None: (lambda gs: {"statistic": (sum(len(g) for g in gs) - len(gs)) * math.log(
         math.fsum((len(g) - 1) * _var(g) for g in gs) / (sum(len(g) for g in gs) - len(gs))) - math.fsum((len(g) - 1) * math.log(_var(g)) for g in gs)})(
         [g for g in (g1, g2, g3, g4, g5) if g is not None]),
     rnd=lambda rng: _rnd_groups(rng, nlo=3), need=r"bartlett", prio=6,
     templates=["Bartlett's test for [21, 24, 19, 26, 23], [22, 23, 22, 24, 23] and [18, 28, 20, 30, 24].",
                "Use Bartlett's test to compare the variances of [5.1, 6.3, 4.2, 7.0] and [5.0, 5.2, 4.9, 5.1]."],
     examples=[X({"g1": "21, 24, 19, 26, 23", "g2": "22, 23, 22, 24, 23", "g3": "18, 28, 20, 30, 24"}),
               X({"g1": [5.1, 6.3, 4.2, 7.0], "g2": [5.0, 5.2, 4.9, 5.1]})],
     questions=[Q("Bartlett's test for [21, 24, 19, 26, 23], [22, 23, 22, 24, 23] and [18, 28, 20, 30, 24]."),
                Q("Use Bartlett's test to compare the variances of [5.1, 6.3, 4.2, 7.0] and [5.0, 5.2, 4.9, 5.1]."),
                Q("Bartlett's test for [21, 21, 21] and [22, 23, 22].", outcome="INVALID_INPUT")])

# ========================================================================================== nonparametric tests ====
_NP = "nonparametric tests"
_ALTS = ["two-sided", "greater", "less"]


def _mw_core(x, y):
    x, y = _finite(x, "x"), _finite(y, "y")
    _need(x, 1, "the first sample")
    _need(y, 1, "the second sample")
    n1, n2 = len(x), len(y)
    r = _ranks(x + y)
    u1 = math.fsum(r[:n1]) - n1 * (n1 + 1) / 2.0
    return x, y, n1, n2, u1, n1 * n2 - u1


def run_mann_whitney(x, y, alternative="two-sided"):
    _check_alt(alternative)
    x, y, n1, n2, u1, u2 = _mw_core(x, y)
    n = n1 + n2
    tie = sum(t ** 3 - t for t in _tie_counts(x + y))
    s = math.sqrt(n1 * n2 / 12.0 * ((n + 1) - tie / (n * (n - 1))))
    if s == 0:
        raise ValueError("all values are tied")
    u, f = {"greater": (u1, 1), "less": (u2, 1)}.get(alternative, (max(u1, u2), 2))
    z = (u - n1 * n2 / 2.0 - 0.5) / s
    return {"u": u1, "p_value": min(1.0, f * _phic(z))}


def _ref_mw(method):
    def ref(x, y, alternative="two-sided"):
        from scipy.stats import mannwhitneyu
        r = mannwhitneyu(x, y, alternative=alternative, method=method, use_continuity=True)
        return {"u": float(r.statistic), "p_value": float(r.pvalue)}
    return ref


def _gauss_binom_counts(m, n):
    """Number of arrangements with each value of U (coefficients of the Gaussian binomial [m+n choose m]_q)."""
    deg = m * n
    c = [0] * (deg + m + n + 1)
    c[0] = 1
    for i in range(1, m + 1):
        s = n + i
        for k in range(len(c) - 1, s - 1, -1):
            c[k] -= c[k - s]
        for k in range(i, len(c)):
            c[k] += c[k - i]
    return c[:deg + 1]


def run_mann_whitney_exact(x, y, alternative="two-sided"):
    _check_alt(alternative)
    x, y, n1, n2, u1, u2 = _mw_core(x, y)
    if _tie_counts(x + y):
        raise ValueError("the exact test needs data without ties; use the large-sample Mann-Whitney test")
    if n1 > 30 or n2 > 30:
        raise ValueError("the exact test is limited to samples of at most 30; use the large-sample test")
    c = _gauss_binom_counts(n1, n2)
    tot = sum(c)
    u, f = {"greater": (u1, 1), "less": (u2, 1)}.get(alternative, (max(u1, u2), 2))
    return {"u": u1, "p_value": min(1.0, f * float(Fraction(sum(c[int(round(u)):]), tot)))}


def _rnd_mw(rng, distinct=False):
    if distinct:
        v = _distinct(rng, rng.randint(2, 24))
        k = rng.randint(1, len(v) - 1)
        return {"x": v[:k], "y": v[k:], "alternative": rng.choice(_ALTS)}
    return dict(_rnd_two(rng, 2, 30), alternative=rng.choice(_ALTS))


for _key, _lvl, _nm, _run, _meth, _need_w, _avoid_w, _rnd, _ex in (
        ("mann_whitney_u", "advanced", "Mann-Whitney U test (large-sample)", run_mann_whitney, "asymptotic",
         r"mann[- ]whitney|wilcoxon rank[- ]sum|rank[- ]sum", r"exact", lambda rng: _rnd_mw(rng), [3, 5, 4, 6, 7]),
        ("mann_whitney_exact", "expert", "Mann-Whitney U test (exact)", run_mann_whitney_exact, "exact",
         r"(?:mann[- ]whitney|rank[- ]sum).*exact|exact.*(?:mann[- ]whitney|rank[- ]sum)", r"(?!)", lambda rng: _rnd_mw(rng, True), [3.1, 5.2, 4.3, 6.4, 7.5])):
    _cap(_key, level=_lvl, name=_nm, topic=_NP,
         purpose=f"The {_nm}: compares two independent samples by ranks (statistic U of the first sample and the p-value).",
         inputs=[V("x", "first sample", 1), V("y", "second sample", 1), ALT],
         outputs=[O("u", "U statistic of the first sample (R1 - n1(n1+1)/2)"), O("p_value", "p-value")], primary="p_value",
         conditions=["independent samples, ordinal or continuous data"] + (
             ["normal approximation with tie correction and continuity correction"] if _meth == "asymptotic" else
             ["no ties", "samples of at most 30"]),
         method="rank the combined sample; U from the rank sum; " + ("normal approximation" if _meth == "asymptotic"
                                                                     else "exact null distribution of U (Gaussian binomial coefficients)"),
         latex=r"U_1 = R_1 - \frac{n_1(n_1+1)}{2}", run=_run, reference=_ref_mw(_meth),
         wrong=(lambda run: (lambda x, y, alternative="two-sided": {"u": run(x, y, alternative)["u"] + 1.0}))(_run),
         rnd=_rnd, need=[_need_w], avoid=[_avoid_w, r"signed|paired"], extract=_x_alt, prio=5 if _meth == "asymptotic" else 6,
         templates=[f"Perform a {'exact ' if _meth == 'exact' else ''}Mann-Whitney U test on [{', '.join(map(str, _ex))}] and [1.5, 2.6, 2.2, 3.9], two-sided.",
                    f"Use the {'exact ' if _meth == 'exact' else ''}Mann-Whitney test to check whether [12.1, 15.3, 14.2, 18.4] tends to be larger than [10.5, 9.6, 13.7, 11.8, 8.9], right-tailed."],
         examples=[X({"x": ", ".join(map(str, _ex)), "y": "1.5, 2.6, 2.2, 3.9"}),
                   X({"x": [12.1, 15.3, 14.2, 18.4], "y": [10.5, 9.6, 13.7, 11.8, 8.9], "alternative": "greater"})],
         questions=[Q(f"Perform a {'exact ' if _meth == 'exact' else ''}Mann-Whitney U test on [{', '.join(map(str, _ex))}] and [1.5, 2.6, 2.2, 3.9], two-sided."),
                    Q(f"Use the {'exact ' if _meth == 'exact' else ''}Mann-Whitney test to check whether [12.1, 15.3, 14.2, 18.4] tends to be larger than [10.5, 9.6, 13.7, 11.8, 8.9], right-tailed."),
                    Q(f"Perform a {'exact ' if _meth == 'exact' else ''}Mann-Whitney U test on [1, 2] and [3, 4], one-sided.", outcome="OUT_OF_SCOPE")])


def _signed(x, y):
    x = _finite(x, "x")
    if y is not None:
        y = _finite(y, "y")
        if len(x) != len(y):
            raise ValueError("x and y must have the same length (paired data)")
        d = [a - b for a, b in zip(x, y)]
    else:
        d = x
    d = [v for v in d if v != 0]
    if not d:
        raise ValueError("all differences are zero")
    r = _ranks([abs(v) for v in d])
    rp = math.fsum(ri for ri, v in zip(r, d) if v > 0)
    return d, r, rp, len(d) * (len(d) + 1) / 2.0 - rp


def run_wilcoxon(x, y=None, alternative="two-sided"):
    _check_alt(alternative)
    d, r, rp, rm = _signed(x, y)
    n = len(d)
    se2 = n * (n + 1) * (2 * n + 1) / 24.0 - sum(t ** 3 - t for t in _tie_counts([abs(v) for v in d])) / 48.0
    if se2 <= 0:
        raise ValueError("the variance of the statistic is zero")
    z = (rp - n * (n + 1) / 4.0) / math.sqrt(se2)
    p = {"greater": _phic(z), "less": _phi(z)}.get(alternative, min(1.0, 2 * _phic(abs(z))))
    return {"statistic": min(rp, rm) if alternative == "two-sided" else rp, "p_value": p, "r_plus": rp, "n": n}


def run_wilcoxon_exact(x, y=None, alternative="two-sided"):
    _check_alt(alternative)
    d, r, rp, rm = _signed(x, y)
    if _tie_counts([abs(v) for v in d]) or len(d) < len(x):
        raise ValueError("the exact test needs no zero differences and no tied absolute differences")
    n = len(d)
    if n > 60:
        raise ValueError("the exact test is limited to 60 differences")
    c = [1]
    for i in range(1, n + 1):
        c = [(c[k] if k < len(c) else 0) + (c[k - i] if 0 <= k - i < len(c) else 0) for k in range(len(c) + i)]
    tot, k = 2 ** n, int(round(rp))
    lo, hi = Fraction(sum(c[:k + 1]), tot), Fraction(sum(c[k:]), tot)
    p = {"greater": hi, "less": lo}.get(alternative, min(Fraction(1), 2 * min(lo, hi)))
    return {"statistic": min(rp, rm) if alternative == "two-sided" else rp, "p_value": float(p), "r_plus": rp, "n": n}


def _ref_wilcoxon(method):
    def ref(x, y=None, alternative="two-sided"):
        from scipy.stats import wilcoxon
        r = wilcoxon(x, y, zero_method="wilcox", correction=False, alternative=alternative, method=method)
        return {"statistic": float(r.statistic), "p_value": float(r.pvalue)}
    return ref


def _rnd_wil(rng, exact=False):
    n = rng.randint(2, 25)
    if exact:
        mags = rng.sample(range(1, 400), n)
        return {"x": [round(m / 10 * rng.choice([1, -1]) + (rng.uniform(0, 2) if rng.random() < 0.3 else 0) * 0, 1) for m in mags],
                "alternative": rng.choice(_ALTS)}
    x = _sample(rng, n, 50, 10, 0)
    d = {"x": x, "y": [round(v + rng.gauss(rng.uniform(-3, 3), 3), 0) for v in x], "alternative": rng.choice(_ALTS)}
    return d


for _key, _lvl, _nm, _run, _meth, _need_w, _avoid_w, _rnd in (
        ("wilcoxon_signed_rank", "advanced", "Wilcoxon signed-rank test (large-sample)", run_wilcoxon, "asymptotic",
         r"wilcoxon(?! rank[- ]sum)|signed[- ]rank", r"exact", lambda rng: _rnd_wil(rng)),
        ("wilcoxon_signed_rank_exact", "expert", "Wilcoxon signed-rank test (exact)", run_wilcoxon_exact, "exact",
         r"(?:wilcoxon|signed[- ]rank).*exact|exact.*(?:wilcoxon|signed[- ]rank)", r"rank[- ]sum", lambda rng: _rnd_wil(rng, True))):
    _cap(_key, level=_lvl, name=_nm, topic=_NP,
         purpose=f"The {_nm} for paired data (x - y) or for differences given directly (x alone); zero differences are dropped.",
         inputs=[V("x", "differences, or the first measurement of each pair", 1), V("y", "second measurement of each pair (optional)", 1, required=False), ALT],
         outputs=[O("statistic", "min(R+, R-) for a two-sided test, R+ otherwise"), O("p_value", "p-value"),
                  O("r_plus", "sum of ranks of positive differences"), O("n", "non-zero differences")], primary="p_value",
         conditions=["paired data, symmetric distribution of differences", "zero differences dropped"] + (
             ["normal approximation with tie correction, no continuity correction"] if _meth == "asymptotic" else
             ["no zero and no tied absolute differences", "at most 60 differences"]),
         method="rank |d|; R+ = sum of ranks of positive d; " + ("normal approximation" if _meth == "asymptotic" else "exact null distribution by counting subsets"),
         latex=r"R^+ = \sum_{d_i > 0} \operatorname{rank}(|d_i|)", run=_run, reference=_ref_wilcoxon(_meth),
         wrong=(lambda run: (lambda x, y=None, alternative="two-sided": {"statistic": run(x, y, alternative)["statistic"] + 0.5}))(_run),
         rnd=_rnd, need=[_need_w], avoid=[_avoid_w], extract=_x_alt, prio=5 if _meth == "asymptotic" else 6,
         templates=[f"Perform a {'exact ' if _meth == 'exact' else ''}Wilcoxon signed-rank test on before [125, 115, 130, 140, 141, 115, 140, 125] and after [110, 122, 125, 120, 140, 124, 123, 137], two-sided.",
                    f"Use the {'exact ' if _meth == 'exact' else ''}Wilcoxon signed-rank test on the differences [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9], right-tailed."],
         examples=[X({"x": "125, 115, 130, 140, 141, 115, 140, 125", "y": "110, 122, 125, 120, 140, 124, 123, 137"}),
                   X({"x": [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9], "alternative": "greater"})],
         questions=[Q(f"Perform a {'exact ' if _meth == 'exact' else ''}Wilcoxon signed-rank test on before [125, 115, 130, 140, 141, 115, 140, 125] and after [110, 122, 125, 120, 140, 124, 123, 137], two-sided."),
                    Q(f"Use the {'exact ' if _meth == 'exact' else ''}Wilcoxon signed-rank test on the differences [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9], right-tailed."),
                    Q(f"Use the {'exact ' if _meth == 'exact' else ''}Wilcoxon signed-rank test on the differences [0, 0, 0], two-sided.", outcome="INVALID_INPUT")])


def run_kruskal(g1, g2, g3=None, g4=None, g5=None):
    gs = _groups(g1, g2, g3, g4, g5)
    for g in gs:
        _need(g, 1, "each group")
    allv = [v for g in gs for v in g]
    N, k = len(allv), len(gs)
    r = _ranks(allv)
    h, i = 0.0, 0
    for g in gs:
        h += math.fsum(r[i:i + len(g)]) ** 2 / len(g)
        i += len(g)
    h = 12.0 / (N * (N + 1)) * h - 3 * (N + 1)
    c = 1 - sum(t ** 3 - t for t in _tie_counts(allv)) / (N ** 3 - N)
    if c == 0:
        raise ValueError("all values are tied")
    h /= c
    return {"h": h, "df": k - 1, "p_value": _chi2_sf(h, k - 1)}


_cap("kruskal_wallis", level="advanced", name="Kruskal-Wallis H test", topic=_NP,
     purpose="The Kruskal-Wallis rank test comparing 2 to 5 independent groups (nonparametric one-way ANOVA).",
     inputs=_G, outputs=[O("h", "H statistic (tie-corrected)"), O("df", "k - 1"), O("p_value", "chi-square p-value")], primary="p_value",
     conditions=["independent groups, ordinal or continuous data", "chi-square approximation (about 5 or more per group)"],
     method="rank all observations; H = 12/(N(N+1)) sum R_i^2/n_i - 3(N+1), divided by the tie correction",
     latex=r"H = \frac{12}{N(N+1)}\sum \frac{R_i^2}{n_i} - 3(N+1)", run=run_kruskal,
     reference=lambda g1, g2, g3=None, g4=None, g5=None: (lambda r: {"h": float(r.statistic), "p_value": float(r.pvalue)})(
         __import__("scipy.stats", fromlist=["kruskal"]).kruskal(*[g for g in (g1, g2, g3, g4, g5) if g is not None])),
     wrong=lambda g1, g2, g3=None, g4=None, g5=None: {"h": run_kruskal(g1, g2, g3, g4, g5)["h"] * 1.1},
     rnd=lambda rng: _rnd_groups(rng, dec=0), need=r"kruskal", prio=6,
     templates=["Perform a Kruskal-Wallis test on [27, 2, 4, 18, 7, 9], [20, 8, 14, 36, 21, 22] and [34, 31, 3, 23, 30, 6].",
                "Kruskal-Wallis H test for groups [5.1, 4.8, 5.5], [6.2, 6.0, 6.8, 6.4] and [5.9, 5.6, 6.1]."],
     examples=[X({"g1": "27, 2, 4, 18, 7, 9", "g2": "20, 8, 14, 36, 21, 22", "g3": "34, 31, 3, 23, 30, 6"}),
               X({"g1": [5.1, 4.8, 5.5], "g2": [6.2, 6.0, 6.8, 6.4], "g3": [5.9, 5.6, 6.1]})],
     questions=[Q("Perform a Kruskal-Wallis test on [27, 2, 4, 18, 7, 9], [20, 8, 14, 36, 21, 22] and [34, 31, 3, 23, 30, 6]."),
                Q("Kruskal-Wallis H test for groups [5.1, 4.8, 5.5], [6.2, 6.0, 6.8, 6.4] and [5.9, 5.6, 6.1]."),
                Q("Perform a Kruskal-Wallis test on [1, 1] and [1, 1].", outcome="INVALID_INPUT")])


def run_friedman(data):
    d = _matrix_ok(data, "the data table", 2, 3)
    n, k = len(d), len(d[0])
    ranks = [_ranks(r) for r in d]
    ties = sum(t * (t * t - 1) for r in d for t in _tie_counts(r))
    c = 1 - ties / (k * (k * k - 1) * n)
    if c == 0:
        raise ValueError("every block is fully tied")
    ssbn = math.fsum(math.fsum(col) ** 2 for col in zip(*ranks))
    q = (12.0 / (k * n * (k + 1)) * ssbn - 3 * n * (k + 1)) / c
    return {"chi_square": q, "df": k - 1, "p_value": _chi2_sf(q, k - 1)}


_cap("friedman_test", level="advanced", name="Friedman test", topic=_NP,
     purpose="The Friedman rank test for k >= 3 related treatments measured on the same blocks (subjects): rows are blocks, columns are treatments.",
     inputs=[M("data", "table with one row per block (subject) and one column per treatment")],
     outputs=[O("chi_square", "Friedman statistic (tie-corrected)"), O("df", "k - 1"), O("p_value", "chi-square p-value")], primary="p_value",
     conditions=["at least 3 treatments and 2 blocks", "chi-square approximation"],
     method="rank within each block; Q = 12/(nk(k+1)) sum R_j^2 - 3n(k+1), divided by the tie correction",
     latex=r"Q = \frac{12}{nk(k+1)}\sum_j R_j^2 - 3n(k+1)", run=run_friedman,
     reference=lambda data: (lambda r: {"chi_square": float(r.statistic), "p_value": float(r.pvalue)})(
         __import__("scipy.stats", fromlist=["friedmanchisquare"]).friedmanchisquare(*[list(c) for c in zip(*data)])),
     wrong=lambda data: {"chi_square": run_friedman(data)["chi_square"] * (len(data[0]) + 1) / len(data[0])},
     rnd=lambda rng: {"data": [[float(rng.randint(1, 9)) for _ in range(rng.randint(3, 6))] for _ in range(1)] and
                      (lambda k: [[float(rng.randint(1, 12)) for _ in range(k)] for _ in range(rng.randint(2, 12))])(rng.randint(3, 6))},
     need=r"friedman", prio=6,
     templates=["Perform a Friedman test on [[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]].",
                "Friedman test for 4 treatments on blocks [[1.2, 1.5, 1.9, 1.1], [2.1, 2.4, 2.8, 2.0], [1.7, 1.6, 2.2, 1.4]]."],
     examples=[X({"data": "[[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]"}),
               X({"data": [[1.2, 1.5, 1.9, 1.1], [2.1, 2.4, 2.8, 2.0], [1.7, 1.6, 2.2, 1.4]]})],
     questions=[Q("Perform a Friedman test on [[7, 9, 8], [6, 5, 7], [9, 7, 6], [8, 5, 6], [7, 8, 9]]."),
                Q("Friedman test for 4 treatments on blocks [[1.2, 1.5, 1.9, 1.1], [2.1, 2.4, 2.8, 2.0], [1.7, 1.6, 2.2, 1.4]]."),
                Q("Perform a Friedman test on [[7, 9], [6, 5]].", outcome="INVALID_INPUT")])


def run_sign_test(x, y=None, alternative="two-sided"):
    _check_alt(alternative)
    d, r, rp, rm = _signed(x, y)
    n, k = len(d), sum(1 for v in d if v > 0)
    pmf = lambda j: Fraction(math.comb(n, j), 2 ** n)
    lo, hi = sum(pmf(j) for j in range(0, k + 1)), sum(pmf(j) for j in range(k, n + 1))
    p = {"greater": hi, "less": lo}.get(alternative, min(Fraction(1), 2 * min(lo, hi)))
    return {"positives": k, "n": n, "p_value": float(p)}


_cap("sign_test", level="advanced", name="Sign test", topic=_NP,
     purpose="The sign test for the median difference of paired data (or of differences given directly): exact binomial p-value.",
     inputs=[V("x", "differences, or the first measurement of each pair", 1), V("y", "second measurement of each pair (optional)", 1, required=False), ALT],
     outputs=[O("positives", "number of positive differences"), O("n", "non-zero differences"), O("p_value", "exact binomial p-value")],
     primary="p_value", conditions=["paired data", "zero differences dropped"], method="count positive signs; exact Binomial(n, 1/2) tail probabilities",
     latex=r"P = 2\min\left(P(K \le k), P(K \ge k)\right),\ K \sim \mathrm{Bin}(n, \tfrac12)", run=run_sign_test,
     reference=lambda x, y=None, alternative="two-sided": (lambda d: {"p_value": float(__import__("scipy.stats", fromlist=["binomtest"]).binomtest(
         sum(1 for v in d if v > 0), len(d), 0.5, alternative=alternative).pvalue)})(
         [v for v in ([a - b for a, b in zip(x, y)] if y is not None else x) if v != 0]),
     wrong=lambda x, y=None, alternative="two-sided": {"p_value": run_sign_test(x, y, alternative)["p_value"] / 2 if alternative == "two-sided"
                                                       else min(1.0, 2 * run_sign_test(x, y, alternative)["p_value"])},
     rnd=lambda rng: _rnd_wil(rng), need=r"sign test", avoid=r"signed[- ]rank", prio=6, extract=_x_alt,
     templates=["Perform a sign test on before [125, 115, 130, 140, 141, 115, 140, 125] and after [110, 122, 125, 120, 140, 124, 123, 137], two-sided.",
                "Use a sign test on the differences [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9, 2.2], right-tailed."],
     examples=[X({"x": "125, 115, 130, 140, 141, 115, 140, 125", "y": "110, 122, 125, 120, 140, 124, 123, 137"}),
               X({"x": [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9, 2.2], "alternative": "greater"})],
     questions=[Q("Perform a sign test on before [125, 115, 130, 140, 141, 115, 140, 125] and after [110, 122, 125, 120, 140, 124, 123, 137], two-sided."),
                Q("Use a sign test on the differences [1.8, -0.7, 2.6, 3.1, -1.2, 4.4, 0.9, 2.2], right-tailed."),
                Q("Use a sign test on the differences [0, 0], two-sided.", outcome="INVALID_INPUT")])


# ============================================================================================ categorical data ====
def _two_by_two(table):
    t = _matrix_ok(table, "the 2 x 2 table")
    if len(t) != 2 or len(t[0]) != 2:
        raise ValueError("a 2 x 2 table is required")
    if any(v < 0 or v != int(v) for r in t for v in r):
        raise ValueError("counts must be whole numbers >= 0")
    return [[int(v) for v in r] for r in t]


def run_fisher(table, alternative="two-sided"):
    _check_alt(alternative)
    (a, b), (c, d) = _two_by_two(table)
    r1, r2, c1 = a + b, c + d, a + c
    N = r1 + r2
    if N == 0:
        raise ValueError("the table is empty")
    lo_, hi_ = max(0, c1 - r2), min(r1, c1)
    w = {x: math.comb(r1, x) * math.comb(r2, c1 - x) for x in range(lo_, hi_ + 1)}
    tot = math.comb(N, c1)
    if alternative == "greater":
        s = sum(v for x, v in w.items() if x >= a)
    elif alternative == "less":
        s = sum(v for x, v in w.items() if x <= a)
    else:
        s = sum(v for v in w.values() if v * 10 ** 7 <= w[a] * (10 ** 7 + 1))
    return {"p_value": min(1.0, float(Fraction(s, tot))), "odds_ratio": (a * d) / (b * c) if b * c else None}


def ref_fisher(table, alternative="two-sided"):
    from scipy.stats import fisher_exact
    r = fisher_exact([[int(v) for v in row] for row in table], alternative=alternative)
    o = float(r.statistic)
    return {"p_value": float(r.pvalue), "odds_ratio": o if math.isfinite(o) else None}


_cap("fisher_exact_test", level="advanced", name="Fisher's exact test (2 x 2)", topic="categorical data",
     purpose="Fisher's exact test of association in a 2 x 2 table, with the sample odds ratio.",
     inputs=[M("table", "2 x 2 table of counts [[a, b], [c, d]]"), ALT],
     outputs=[O("p_value", "exact p-value (two-sided: tables no more likely than the observed one)"),
              O("odds_ratio", "ad/(bc); empty when b c = 0")], primary="p_value",
     conditions=["fixed margins (conditional test)", "whole-number counts"],
     method="hypergeometric probabilities of all tables with the observed margins, in exact integer arithmetic",
     latex=r"P(a) = \frac{\binom{a+b}{a}\binom{c+d}{c}}{\binom{n}{a+c}}", run=run_fisher, reference=ref_fisher,
     wrong=lambda table, alternative="two-sided": {"p_value": min(1.0, run_fisher(table, alternative)["p_value"] * 0.5 + 1e-3)},
     rnd=lambda rng: {"table": [[float(rng.randint(0, 15)) for _ in range(2)] for _ in range(2)], "alternative": rng.choice(_ALTS)},
     need=r"fisher", avoid=r"fisher[- ]z|fisher transform", prio=6, extract=_x_alt,
     templates=["Fisher's exact test for the table [[8, 2], [1, 5]], two-sided.", "Use Fisher's exact test on [[3, 7], [9, 1]], left-tailed."],
     examples=[X({"table": "[[8, 2], [1, 5]]"}), X({"table": [[3, 7], [9, 1]], "alternative": "less"})],
     questions=[Q("Fisher's exact test for the table [[8, 2], [1, 5]], two-sided."), Q("Use Fisher's exact test on [[3, 7], [9, 1]], left-tailed."),
                Q("Fisher's exact test for the table [[8, 2, 1], [1, 5, 2]], two-sided.", outcome="INVALID_INPUT")])


def run_mcnemar(table):
    (a, b), (c, d) = _two_by_two(table)
    if b + c == 0:
        raise ValueError("there are no discordant pairs (b + c = 0)")
    stat = (abs(b - c) - 1) ** 2 / (b + c)
    m = min(b, c)
    exact = min(Fraction(1), 2 * Fraction(sum(math.comb(b + c, j) for j in range(0, m + 1)), 2 ** (b + c)))
    return {"chi_square": stat, "p_value": _chi2_sf(stat, 1), "exact_p_value": float(exact), "discordant": b + c}


def ref_mcnemar(table):
    from statsmodels.stats.contingency_tables import mcnemar
    t = [[int(v) for v in r] for r in table]
    a, e = mcnemar(t, exact=False, correction=True), mcnemar(t, exact=True)
    return {"chi_square": float(a.statistic), "p_value": float(a.pvalue), "exact_p_value": float(e.pvalue)}


_cap("mcnemar_test", level="advanced", name="McNemar's test for paired proportions", topic="categorical data",
     purpose="McNemar's test for a 2 x 2 table of paired outcomes: continuity-corrected chi-square and the exact binomial p-value.",
     inputs=[M("table", "2 x 2 table of paired counts [[both yes, yes/no], [no/yes, both no]]")],
     outputs=[O("chi_square", "(|b - c| - 1)^2 / (b + c)"), O("p_value", "chi-square p-value (1 df)"),
              O("exact_p_value", "exact binomial p-value"), O("discordant", "b + c")], primary="p_value",
     conditions=["paired (matched) binary outcomes", "b + c > 0"], method="only the discordant cells b and c carry information",
     latex=r"\chi^2 = \frac{(|b - c| - 1)^2}{b + c}", run=run_mcnemar, reference=ref_mcnemar,
     wrong=lambda table: {"chi_square": (table[0][1] - table[1][0]) ** 2 / max(1, table[0][1] + table[1][0]) + 0.01},
     rnd=lambda rng: {"table": [[float(rng.randint(0, 40)), float(rng.randint(1, 30))], [float(rng.randint(0, 30)), float(rng.randint(0, 40))]]},
     need=r"mcnemar", prio=6,
     templates=["McNemar's test for the paired table [[30, 12], [5, 53]].", "Use McNemar's test on [[45, 10], [20, 25]]."],
     examples=[X({"table": "[[30, 12], [5, 53]]"}), X({"table": [[45, 10], [20, 25]]})],
     questions=[Q("McNemar's test for the paired table [[30, 12], [5, 53]]."), Q("Use McNemar's test on [[45, 10], [20, 25]]."),
                Q("McNemar's test for the paired table [[30, 0], [0, 53]].", outcome="INVALID_INPUT")])


# ================================================================================================ effect sizes ====
def run_cohens_d(x, y):
    x, y = _finite(x, "x"), _finite(y, "y")
    _need(x, 2, "the first sample")
    _need(y, 2, "the second sample")
    n1, n2 = len(x), len(y)
    sp = math.sqrt(((n1 - 1) * _var(x) + (n2 - 1) * _var(y)) / (n1 + n2 - 2))
    if sp == 0:
        raise ValueError("both samples have zero variance")
    d = (_mean(x) - _mean(y)) / sp
    return {"d": d, "hedges_g": d * (1 - 3 / (4 * (n1 + n2) - 9)), "pooled_sd": sp}


def ref_cohens_d(x, y):
    import pingouin as pg
    return {"d": float(pg.compute_effsize(x, y, paired=False, eftype="cohen")),
            "hedges_g": float(pg.compute_effsize(x, y, paired=False, eftype="hedges"))}


_cap("cohens_d", level="advanced", name="Cohen's d and Hedges' g", topic="effect sizes",
     purpose="The standardized mean difference of two independent samples: Cohen's d (pooled SD) and Hedges' g (bias-corrected).",
     inputs=[V("x", "first sample"), V("y", "second sample")],
     outputs=[O("d", "(mean x - mean y) / pooled SD"), O("hedges_g", "d (1 - 3/(4(n1 + n2) - 9))"), O("pooled_sd", "pooled standard deviation")],
     primary="d", conditions=["independent samples"], method="difference of means over the pooled sample standard deviation",
     latex=r"d = \frac{\bar{x}_1 - \bar{x}_2}{s_p},\ s_p = \sqrt{\frac{(n_1-1)s_1^2 + (n_2-1)s_2^2}{n_1+n_2-2}}", run=run_cohens_d,
     reference=ref_cohens_d, wrong=lambda x, y: {"d": (_mean(x) - _mean(y)) / math.sqrt((_var(x) + _var(y)) / 2) * 1.05},
     rnd=lambda rng: _rnd_two(rng, 2, 30), need=r"cohen'?s d|effect size|hedges", avoid=r"power|sample size|cohen'?s kappa", prio=6,
     templates=["Find Cohen's d for [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8].", "What is the effect size (Cohen's d) between [5.1, 4.9, 5.6, 5.8] and [4.2, 4.8, 4.4, 4.0, 4.6]?"],
     examples=[X({"x": "12, 15, 11, 18, 14", "y": "10, 9, 13, 11, 8"}), X({"x": [5.1, 4.9, 5.6, 5.8], "y": [4.2, 4.8, 4.4, 4.0, 4.6]})],
     questions=[Q("Find Cohen's d for [12, 15, 11, 18, 14] and [10, 9, 13, 11, 8]."),
                Q("What is the effect size (Cohen's d) between [5.1, 4.9, 5.6, 5.8] and [4.2, 4.8, 4.4, 4.0, 4.6]?"),
                Q("Find Cohen's d for [12] and [10, 9, 13].", outcome="INVALID_INPUT")])


def run_cronbach(data):
    d = _matrix_ok(data, "the item table", 2, 2)
    k = len(d[0])
    iv = math.fsum(_var(list(c)) for c in zip(*d))
    tv = _var([math.fsum(r) for r in d])
    if tv == 0:
        raise ValueError("the total scores do not vary")
    return {"alpha": k / (k - 1) * (1 - iv / tv), "items": k, "respondents": len(d)}


def ref_cronbach(data):
    import pandas as pd
    import pingouin as pg
    return {"alpha": float(pg.cronbach_alpha(data=pd.DataFrame(data))[0])}


_cap("cronbach_alpha", level="expert", name="Cronbach's alpha (internal consistency)", topic="experimental design",
     purpose="Cronbach's alpha for a questionnaire or test: rows are respondents, columns are items.",
     inputs=[M("data", "scores: one row per respondent, one column per item")],
     outputs=[O("alpha", "k/(k-1) (1 - sum of item variances / variance of totals)"), O("items", "k"), O("respondents", "n")],
     primary="alpha", conditions=["at least 2 items and 2 respondents"], method="sample variances (n - 1) of each item and of the totals",
     latex=r"\alpha = \frac{k}{k-1}\left(1 - \frac{\sum s_i^2}{s_T^2}\right)", run=run_cronbach, reference=ref_cronbach,
     wrong=lambda data: {"alpha": 1 - math.fsum(_var(list(c)) for c in zip(*data)) / _var([math.fsum(r) for r in data])},
     rnd=lambda rng: (lambda k, n: (lambda base: {"data": [[float(min(5, max(1, round(b + rng.gauss(0, 1))))) for _ in range(k)] for b in base]})(
         [rng.uniform(1, 5) for _ in range(n)]))(rng.randint(2, 6), rng.randint(3, 25)),
     need=r"cronbach|internal consistency", prio=6,
     templates=["Find Cronbach's alpha for [[4, 5, 4], [3, 3, 4], [5, 5, 5], [2, 3, 2], [4, 4, 3]].",
                "Compute the internal consistency (Cronbach's alpha) of [[1, 2, 2, 1], [3, 3, 4, 3], [4, 5, 4, 5], [2, 2, 3, 2]]."],
     examples=[X({"data": "[[4, 5, 4], [3, 3, 4], [5, 5, 5], [2, 3, 2], [4, 4, 3]]"}), X({"data": [[1, 2, 2, 1], [3, 3, 4, 3], [4, 5, 4, 5], [2, 2, 3, 2]]})],
     questions=[Q("Find Cronbach's alpha for [[4, 5, 4], [3, 3, 4], [5, 5, 5], [2, 3, 2], [4, 4, 3]]."),
                Q("Compute the internal consistency (Cronbach's alpha) of [[1, 2, 2, 1], [3, 3, 4, 3], [4, 5, 4, 5], [2, 2, 3, 2]]."),
                Q("Find Cronbach's alpha for [[3, 3], [3, 3], [3, 3]].", outcome="INVALID_INPUT")])


# ======================================================================================== experimental design ====
_ED = "experimental design"


def _factorial(y, k):
    y = _finite(y, "responses")
    if len(y) != 2 ** k:
        raise ValueError(f"give {2 ** k} responses in standard (Yates) order")
    lv = [[1 if (i >> j) & 1 else -1 for j in range(k)] for i in range(2 ** k)]
    names, cols = [], []
    for mask in range(1, 2 ** k):
        names.append("".join("ABC"[j] for j in range(k) if (mask >> j) & 1))
        cols.append([math.prod(r[j] for j in range(k) if (mask >> j) & 1) for r in lv])
    order = sorted(range(len(names)), key=lambda i: (len(names[i]), names[i]))
    out = {"grand_mean": _mean(y)}
    scale = max(abs(v) for v in y) or 1.0
    for i in order:
        e = math.fsum(c * v for c, v in zip(cols[i], y)) / 2 ** (k - 1)
        out[names[i]] = 0.0 if abs(e) < 1e-12 * scale else e     # an exactly zero effect is not round-off noise
    return out


def _ref_factorial(y, k):
    import numpy as np
    lv = np.array([[1 if (i >> j) & 1 else -1 for j in range(k)] for i in range(2 ** k)], dtype=float)
    names, cols = [], [np.ones(2 ** k)]
    for mask in range(1, 2 ** k):
        names.append("".join("ABC"[j] for j in range(k) if (mask >> j) & 1))
        cols.append(np.prod(lv[:, [j for j in range(k) if (mask >> j) & 1]], axis=1))
    coef = np.linalg.lstsq(np.column_stack(cols), np.asarray(y, dtype=float), rcond=None)[0]
    out = {"grand_mean": float(coef[0])}
    scale = float(np.max(np.abs(y))) or 1.0
    out.update({n: (0.0 if abs(2 * c) < 1e-12 * scale else float(2 * c)) for n, c in zip(names, coef[1:])})
    return out


for _key, _k, _nm, _ex in (("factorial_2x2_effects", 2, "2^2 factorial design: main effects and interaction", [[28, 36, 18, 31], [20, 30, 25, 45]]),
                           ("factorial_2x2x2_effects", 3, "2^3 factorial design: main effects and interactions",
                            [[22, 32, 35, 55, 44, 40, 60, 39], [550, 669, 604, 650, 633, 642, 601, 635]])):
    _cap(_key, level="advanced", name=_nm, topic=_ED,
         purpose=f"Effects of a two-level {2 ** _k}-run factorial experiment from one response per run in standard (Yates) order "
                 f"({'(1), a, b, ab' if _k == 2 else '(1), a, b, ab, c, ac, bc, abc'}).",
         inputs=[V("y", f"{2 ** _k} responses in standard order", 2 ** _k, max_length=2 ** _k)],
         outputs=[O("grand_mean", "average response")] + [O(n, f"effect {n}") for n in (["A", "B", "AB"] if _k == 2 else
                                                                                       ["A", "B", "C", "AB", "AC", "BC", "ABC"])],
         primary="A", conditions=["responses in standard order", "one response (or cell mean) per run"],
         method="contrast of the +/- signs divided by 2^(k-1)", latex=r"\text{Effect} = \frac{\text{contrast}}{2^{k-1}}",
         run=(lambda k: (lambda y: _factorial(y, k)))(_k), reference=(lambda k: (lambda y: _ref_factorial(y, k)))(_k),
         wrong=(lambda k: (lambda y: {n: v / 2 if n != "grand_mean" else v for n, v in _factorial(y, k).items()}))(_k),
         rnd=(lambda k: (lambda rng: {"y": [round(rng.uniform(10, 100), 1) for _ in range(2 ** k)]}))(_k),
         need=[r"factorial|yates|main effects?", r"2\^2|2x2|two-factor|2\s*by\s*2" if _k == 2 else r"2\^3|2x2x2|three-factor"], prio=6,
         templates=[f"Find the effects of the 2^{_k} factorial design with responses in standard order [{', '.join(map(str, _ex[0]))}].",
                    f"Compute the main effects and interactions of a 2^{_k} factorial experiment with y = [{', '.join(map(str, _ex[1]))}]."],
         examples=[X({"y": ", ".join(map(str, _ex[0]))}), X({"y": _ex[1]})],
         questions=[Q(f"Find the effects of the 2^{_k} factorial design with responses in standard order [{', '.join(map(str, _ex[0]))}]."),
                    Q(f"Compute the main effects and interactions of a 2^{_k} factorial experiment with y = [{', '.join(map(str, _ex[1]))}]."),
                    Q(f"Find the effects of the 2^{_k} factorial design with responses in standard order [1, 2, 3].", outcome="INVALID_INPUT")])


# ===================================================================================== power and sample size ====
_PW = "power and sample size"


def run_n_prop(margin, confidence, p=0.5):
    e, p = _pos(margin, "the margin of error"), _prob(p, "p")
    if e >= 1:
        raise ValueError("the margin of error must be below 1")
    z = _zc(confidence)
    return {"n": math.ceil(z * z * p * (1 - p) / (e * e)), "n_exact": z * z * p * (1 - p) / (e * e)}


_cap("sample_size_proportion", level="expert", name="Sample size to estimate a proportion", topic=_PW,
     purpose="The smallest n giving a confidence interval for a proportion with the required margin of error (p = 0.5 if no estimate).",
     inputs=[N("margin", "margin of error (as a proportion, 0.03 for 3 points)"), CONF, N("p", "planning estimate of the proportion (default 0.5)", required=False)],
     outputs=[O("n", "required sample size (rounded up)"), O("n_exact", "z^2 p(1-p)/E^2 before rounding")], primary="n",
     conditions=["large-sample (Wald) interval", "simple random sample"], method="n = z^2 p(1 - p)/E^2, rounded up",
     latex=r"n = \left\lceil \frac{z_{\alpha/2}^2\,p(1-p)}{E^2} \right\rceil", run=run_n_prop,
     reference=lambda margin, confidence, p=0.5: {"n": math.ceil(float(__import__("statsmodels.stats.proportion", fromlist=["x"]).samplesize_confint_proportion(p, margin, alpha=1 - confidence)))},
     wrong=lambda margin, confidence, p=0.5: {"n": math.ceil(_z(confidence) ** 2 * p * (1 - p) / margin ** 2)},
     rnd=lambda rng: {"margin": round(rng.uniform(0.005, 0.2), 4), "confidence": _rnd_conf(rng), "p": round(rng.uniform(0.05, 0.95), 2)},
     need=[r"sample size|how many|how large", r"proportion|percentage|margin of error"], avoid=r"mean|sigma|standard deviation|power|\[", prio=6,
     aliases={"margin": r"margin of error|margin|e",
              "p": r"p|estimated proportion|proportion|planning (?:estimate|value)|preliminary (?:proportion )?estimate|"
                   r"prior (?:proportion )?estimate|anticipated proportion"}, extract=_x_conf,
     templates=["What sample size is needed to estimate a proportion within margin of error = 0.03 at 95% confidence?",
                "How large a sample is needed for a 99% confidence interval for a proportion with margin of error = 0.05 and p = 0.3?"],
     examples=[X({"margin": 0.03, "confidence": 0.95}), X({"margin": 0.05, "confidence": 0.99, "p": 0.3})],
     questions=[Q("What sample size is needed to estimate a proportion within margin of error = 0.03 at 95% confidence?"),
                Q("How large a sample is needed for a 99% confidence interval for a proportion with margin of error = 0.05 and p = 0.3?"),
                Q("What sample size is needed to estimate a proportion within margin of error = 0.03?", outcome="NEEDS_INPUT")])


def run_n_mean(sigma, margin, confidence):
    s, e = _pos(sigma, "sigma"), _pos(margin, "the margin of error")
    z = _zc(confidence)
    return {"n": math.ceil((z * s / e) ** 2), "n_exact": (z * s / e) ** 2}


def ref_n_mean(sigma, margin, confidence):
    from scipy.stats import norm
    lo, hi = 1, 1
    while norm.ppf(0.5 + confidence / 2) * sigma / math.sqrt(hi) > margin:
        hi *= 2
    while lo < hi:
        mid = (lo + hi) // 2
        if norm.ppf(0.5 + confidence / 2) * sigma / math.sqrt(mid) > margin:
            lo = mid + 1
        else:
            hi = mid
    return {"n": lo}


_cap("sample_size_mean", level="expert", name="Sample size to estimate a mean", topic=_PW,
     purpose="The smallest n giving a z confidence interval for a mean with the required margin of error, sigma known.",
     inputs=[N("sigma", "population standard deviation (planning value)"), N("margin", "margin of error"), CONF],
     outputs=[O("n", "required sample size (rounded up)"), O("n_exact", "(z sigma / E)^2")], primary="n",
     conditions=["sigma known or a planning estimate"], method="n = (z sigma / E)^2, rounded up",
     latex=r"n = \left\lceil \left(\frac{z_{\alpha/2}\,\sigma}{E}\right)^2 \right\rceil", run=run_n_mean, reference=ref_n_mean,
     wrong=lambda sigma, margin, confidence: {"n": math.ceil(_zc(confidence) * sigma / margin)},
     rnd=lambda rng: {"sigma": round(rng.uniform(1, 50), 2), "margin": round(rng.uniform(0.2, 10), 2), "confidence": _rnd_conf(rng)},
     need=[r"sample size|how many|how large", r"\bmean\b|average"], avoid=r"proportion|power|two|\[", prio=6,
     aliases={"sigma": r"sigma|standard deviation", "margin": r"margin of error|margin|within"}, extract=_x_conf,
     templates=["What sample size is needed to estimate the mean within margin of error = 2 with sigma = 15 at 95% confidence?",
                "How many observations are needed to estimate a mean with standard deviation = 8 and margin of error = 1.5 at 99% confidence?"],
     examples=[X({"sigma": 15, "margin": 2, "confidence": 0.95}), X({"sigma": 8, "margin": 1.5, "confidence": 0.99})],
     questions=[Q("What sample size is needed to estimate the mean within margin of error = 2 with sigma = 15 at 95% confidence?"),
                Q("How many observations are needed to estimate a mean with standard deviation = 8 and margin of error = 1.5 at 99% confidence?"),
                Q("What sample size is needed to estimate the mean within margin of error = 2 at 95% confidence?", outcome="NEEDS_INPUT")])


def _zpower(d, nobs, alpha, alternative):
    if alternative == "two-sided":
        c = _z(1 - alpha / 2)
        return _phic(c - d * math.sqrt(nobs)) + _phi(-c - d * math.sqrt(nobs))
    c = _z(1 - alpha)
    return _phic(c - d * math.sqrt(nobs)) if alternative == "greater" else _phi(-c - d * math.sqrt(nobs))


def _pw_args(d, alpha, alternative):
    _check_alt(alternative)
    if not math.isfinite(d) or d == 0:
        raise ValueError("the effect size must be a non-zero number")
    if alternative == "greater" and d < 0 or alternative == "less" and d > 0:
        raise ValueError("the effect size has the opposite sign of the one-sided alternative")
    return _prob(alpha, "alpha")


_D_ALIAS = {"d": r"d|effect size|cohen'?s d", "n": r"n|per group|sample size", "alpha": r"alpha|significance level", "power": r"power"}


def run_power_one(d, n, alpha, alternative="two-sided"):
    alpha = _pw_args(d, alpha, alternative)
    return {"power": _zpower(d, _pos(n, "n"), alpha, alternative)}


def run_power_two(d, n, alpha, alternative="two-sided"):
    alpha = _pw_args(d, alpha, alternative)
    return {"power": _zpower(d, _pos(n, "n") / 2.0, alpha, alternative)}


def _ref_power(ratio):
    def ref(d, n, alpha, alternative="two-sided"):
        from statsmodels.stats.power import NormalIndPower
        return {"power": float(NormalIndPower().power(d, n, alpha, ratio=ratio,
                                                       alternative={"greater": "larger", "less": "smaller"}.get(alternative, alternative)))}
    return ref


def _rnd_pw(rng, two=False):
    alt = rng.choice(_ALTS)
    d = round(rng.uniform(0.05, 1.5), 3) * (-1 if alt == "less" or (alt == "two-sided" and rng.random() < 0.3) else 1)
    n = max(2, round((rng.uniform(0.3, 2.5) / abs(d)) ** 2 * (2 if two else 1)))
    return {"d": d, "n": n, "alpha": rng.choice([0.05, 0.01, 0.1]), "alternative": alt}


for _key, _samples, _run, _ratio, _need_s in (("power_one_sample_z", "one", run_power_one, 0.0, r"one[- ]sample"),
                                             ("power_two_sample_z", "two", run_power_two, 1.0, r"two[- ]sample|two groups|per group")):
    _cap(_key, level="expert", name=f"Power of a {_samples}-sample z-test", topic=_PW,
         purpose=f"The power of a {_samples}-sample z-test for a standardized effect size d (Cohen's d), sample size n "
                 f"{'per group ' if _samples == 'two' else ''}and significance level alpha.",
         inputs=[N("d", "standardized effect size (difference / sigma)"), N("n", "sample size" + (" per group" if _samples == "two" else "")),
                 N("alpha", "significance level"), ALT],
         outputs=[O("power", "probability of rejecting H0 when the effect is d")], primary="power",
         conditions=["normal approximation (sigma known)", "two-sided power includes both rejection tails"],
         method="power = P(Z > z_crit - d sqrt(n_eff)) (+ the opposite tail when two-sided), n_eff = " + ("n" if _samples == "one" else "n/2"),
         latex=r"1 - \beta = \Phi\left(d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right) + \Phi\left(-d\sqrt{n_{\text{eff}}} - z_{1-\alpha/2}\right)",
         run=_run, reference=_ref_power(_ratio),
         wrong=(lambda run: (lambda d, n, alpha, alternative="two-sided": run(d, n * 2, alpha, alternative)))(_run),
         rnd=(lambda two: (lambda rng: _rnd_pw(rng, two)))(_samples == "two"),
         need=[r"\bpower\b", _need_s], avoid=r"sample size needed|how many|how large|\[", prio=6, aliases=_D_ALIAS,
         extract=_chain(_x_alt, _x_alpha),
         templates=[f"Find the power of a {_samples}-sample z-test with effect size d = 0.5, n = 30 and alpha = 0.05, two-sided.",
                    f"What is the power of a {_samples}-sample z-test for d = 0.3, n = 100, alpha = 0.01, right-tailed?"],
         examples=[X({"d": 0.5, "n": 30, "alpha": 0.05}), X({"d": 0.3, "n": 100, "alpha": 0.01, "alternative": "greater"})],
         questions=[Q(f"Find the power of a {_samples}-sample z-test with effect size d = 0.5, n = 30 and alpha = 0.05, two-sided."),
                    Q(f"What is the power of a {_samples}-sample z-test for d = 0.3, n = 100, alpha = 0.01, right-tailed?"),
                    Q(f"Find the power of a {_samples}-sample z-test with effect size d = 0.5 and alpha = 0.05, two-sided.", outcome="NEEDS_INPUT")])


def run_n_two_means(d, alpha, power, alternative="two-sided"):
    alpha = _pw_args(d, alpha, alternative)
    power = _prob(power, "power")
    if power <= alpha:
        raise ValueError("the power must exceed alpha")
    f = lambda n: _zpower(d, n / 2.0, alpha, alternative) - power
    lo, hi = 1e-9, 2.0
    while f(hi) < 0:
        lo, hi = hi, hi * 2
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if mid <= lo or mid >= hi:
            break
        lo, hi = (mid, hi) if f(mid) < 0 else (lo, mid)
    return {"n_per_group": math.ceil(hi), "n_exact": hi}


def ref_n_two_means(d, alpha, power, alternative="two-sided"):
    from statsmodels.stats.power import NormalIndPower
    n = NormalIndPower().solve_power(effect_size=d, nobs1=None, alpha=alpha, power=power, ratio=1.0,
                                     alternative={"greater": "larger", "less": "smaller"}.get(alternative, alternative))
    return {"n_per_group": math.ceil(float(n))}


_cap("sample_size_two_means", level="expert", name="Sample size per group to compare two means (z-test power)", topic=_PW,
     purpose="The sample size per group for a two-sample z-test to reach a target power for a standardized effect size d.",
     inputs=[N("d", "standardized effect size (difference / sigma)"), N("alpha", "significance level"), N("power", "target power (e.g. 0.8)"), ALT],
     outputs=[O("n_per_group", "required size per group (rounded up)"), O("n_exact", "solution before rounding")], primary="n_per_group",
     conditions=["equal group sizes", "normal approximation", "two-sided power includes both tails"],
     method="solve power(n) = target by bisection, round up",
     latex=r"n \approx 2\left(\frac{z_{1-\alpha/2} + z_{1-\beta}}{d}\right)^2", run=run_n_two_means, reference=ref_n_two_means,
     wrong=lambda d, alpha, power, alternative="two-sided": {"n_per_group": math.ceil(((_z(1 - alpha / 2) + _z(power)) / d) ** 2)},
     rnd=lambda rng: (lambda r: dict(r, power=rng.choice([0.8, 0.9, 0.85, 0.95])))(
         {k: v for k, v in _rnd_pw(rng).items() if k != "n"}),
     need=[r"sample size|how many|how large", r"\bpower\b", r"two|per group|groups"], avoid=r"\[|proportion", prio=7, aliases=_D_ALIAS,
     extract=_chain(_x_alt, _x_alpha),
     templates=["What sample size per group is needed for two groups with effect size d = 0.5, alpha = 0.05 and power = 0.8, two-sided?",
                "How many subjects per group for two groups to detect d = 0.3 with power = 0.9 at alpha = 0.05, right-tailed?"],
     examples=[X({"d": 0.5, "alpha": 0.05, "power": 0.8}), X({"d": 0.3, "alpha": 0.05, "power": 0.9, "alternative": "greater"})],
     questions=[Q("What sample size per group is needed for two groups with effect size d = 0.5, alpha = 0.05 and power = 0.8, two-sided?"),
                Q("How many subjects per group for two groups to detect d = 0.3 with power = 0.9 at alpha = 0.05, right-tailed?"),
                Q("What sample size per group is needed for two groups with effect size d = 0.5 and power = 0.8, two-sided?", outcome="NEEDS_INPUT")])


# ============================================================================================= quality control ====
_QC = "quality control"
_A2 = {2: 1.880, 3: 1.023, 4: 0.729, 5: 0.577, 6: 0.483, 7: 0.419, 8: 0.373, 9: 0.337, 10: 0.308}
_D3 = {2: 0.0, 3: 0.0, 4: 0.0, 5: 0.0, 6: 0.0, 7: 0.076, 8: 0.136, 9: 0.184, 10: 0.223}
_D4 = {2: 3.267, 3: 2.574, 4: 2.282, 5: 2.114, 6: 2.004, 7: 1.924, 8: 1.864, 9: 1.816, 10: 1.777}
_A3 = {2: 2.659, 3: 1.954, 4: 1.628, 5: 1.427, 6: 1.287, 7: 1.182, 8: 1.099, 9: 1.032, 10: 0.975}
_B3 = {2: 0.0, 3: 0.0, 4: 0.0, 5: 0.0, 6: 0.030, 7: 0.118, 8: 0.185, 9: 0.239, 10: 0.284}
_B4 = {2: 3.267, 3: 2.568, 4: 2.266, 5: 2.089, 6: 1.970, 7: 1.882, 8: 1.815, 9: 1.761, 10: 1.716}
_E2 = 2.660
_CONST_CACHE: dict = {}


def _exact_constants(n):
    """d2, d3 (moments of the range of n standard normals, by numerical integration) and c4 (gamma functions):
    the build-time check that the published 3-decimal table constants are right."""
    if n not in _CONST_CACHE:
        import numpy as np
        from scipy.special import gammaln
        from scipy.stats import norm
        x = np.linspace(-9.0, 9.0, 1801)[:, None]
        r = np.linspace(0.0, 14.0, 1401)[None, :]
        dens = n * (n - 1) * norm.pdf(x) * norm.pdf(x + r) * (norm.cdf(x + r) - norm.cdf(x)) ** (n - 2)
        m1 = float(np.trapezoid(np.trapezoid(r * dens, x[:, 0], axis=0), r[0]))
        m2 = float(np.trapezoid(np.trapezoid(r * r * dens, x[:, 0], axis=0), r[0]))
        c4 = math.sqrt(2 / (n - 1)) * math.exp(gammaln(n / 2) - gammaln((n - 1) / 2))
        _CONST_CACHE[n] = (m1, math.sqrt(m2 - m1 * m1), c4)
    return _CONST_CACHE[n]


def _check_table(n, kind):
    d2, d3, c4 = _exact_constants(n)
    want = {"A2": (_A2[n], 3 / (d2 * math.sqrt(n))), "D3": (_D3[n], max(0.0, 1 - 3 * d3 / d2)), "D4": (_D4[n], 1 + 3 * d3 / d2)} if kind == "R" else \
        {"A3": (_A3[n], 3 / (c4 * math.sqrt(n))), "B3": (_B3[n], max(0.0, 1 - 3 * math.sqrt(1 - c4 * c4) / c4)),
         "B4": (_B4[n], 1 + 3 * math.sqrt(1 - c4 * c4) / c4)}
    for name, (table, exact) in want.items():
        if abs(table - exact) > 0.001:
            raise AssertionError(f"table constant {name}(n={n}) = {table} but the exact value is {exact}")


def _subgroups(data):
    d = _matrix_ok(data, "the subgroup table", 2, 2)
    if len(d[0]) > 10:
        raise ValueError("subgroup sizes from 2 to 10 are supported")
    return d, len(d[0])


def run_xbar_r(data):
    d, n = _subgroups(data)
    xbb = _mean([_mean(r) for r in d])
    rb = _mean([max(r) - min(r) for r in d])
    return {"center_xbar": xbb, "mean_range": rb, "xbar_ucl": xbb + _A2[n] * rb, "xbar_lcl": xbb - _A2[n] * rb,
            "r_ucl": _D4[n] * rb, "r_lcl": _D3[n] * rb, "A2": _A2[n], "D3": _D3[n], "D4": _D4[n]}


def ref_xbar_r(data):
    import numpy as np
    a = np.asarray(data, dtype=float)
    n = a.shape[1]
    _check_table(n, "R")
    xbb, rb = float(a.mean(axis=1).mean()), float(np.ptp(a, axis=1).mean())
    return {"center_xbar": xbb, "mean_range": rb, "xbar_ucl": xbb + _A2[n] * rb, "xbar_lcl": xbb - _A2[n] * rb,
            "r_ucl": _D4[n] * rb, "r_lcl": _D3[n] * rb}


def _rnd_sub(rng):
    n, m = rng.randint(2, 10), rng.randint(2, 25)
    mu, s = rng.uniform(5, 100), rng.uniform(0.2, 5)
    return {"data": [_sample(rng, n, mu, s, 2) for _ in range(m)]}


_cap("xbar_r_chart", level="advanced", name="X-bar and R control chart limits", topic=_QC,
     purpose="Centre lines and 3-sigma control limits of X-bar and R charts from subgroups (rows) of size 2 to 10.",
     inputs=[M("data", "one row per subgroup, one column per observation (subgroup size 2-10)")],
     outputs=[O("center_xbar", "grand mean"), O("mean_range", "average range"), O("xbar_ucl", "X-bar UCL"), O("xbar_lcl", "X-bar LCL"),
              O("r_ucl", "R chart UCL"), O("r_lcl", "R chart LCL"), O("A2", "A2"), O("D3", "D3"), O("D4", "D4")], primary="xbar_ucl",
     conditions=["equal subgroup sizes 2 to 10", "standard 3-decimal control chart constants (A2, D3, D4)"],
     method="grand mean +/- A2 R-bar; R chart limits D3 R-bar and D4 R-bar",
     latex=r"UCL_{\bar{x}} = \bar{\bar{x}} + A_2\bar{R},\quad UCL_R = D_4\bar{R}", run=run_xbar_r, reference=ref_xbar_r,
     wrong=lambda data: (lambda r, n: dict(r, xbar_ucl=r["center_xbar"] + 3 * r["mean_range"] / math.sqrt(n)))(run_xbar_r(data), len(data[0])),
     rnd=_rnd_sub, need=r"x-?bar.{0,6}\br\b|range chart|\br chart", avoid=r"\bs chart|standard deviation chart|x-?bar.{0,6}\bs\b|moving range|individual", prio=6,
     templates=["Find the X-bar and R chart limits for subgroups [[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]].",
                "Compute X-bar R control limits for [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]."],
     examples=[X({"data": "[[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]]"}),
               X({"data": [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]})],
     questions=[Q("Find the X-bar and R chart limits for subgroups [[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]]."),
                Q("Compute X-bar R control limits for [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]."),
                Q("Compute X-bar R control limits for [[5, 6, 4, 1, 2, 3, 4, 5, 6, 7, 8], [6, 7, 5, 1, 2, 3, 4, 5, 6, 7, 8]].", outcome="INVALID_INPUT")])


def run_xbar_s(data):
    d, n = _subgroups(data)
    xbb = _mean([_mean(r) for r in d])
    sb = _mean([_sd(r) for r in d])
    return {"center_xbar": xbb, "mean_sd": sb, "xbar_ucl": xbb + _A3[n] * sb, "xbar_lcl": xbb - _A3[n] * sb,
            "s_ucl": _B4[n] * sb, "s_lcl": _B3[n] * sb, "A3": _A3[n], "B3": _B3[n], "B4": _B4[n]}


def ref_xbar_s(data):
    import numpy as np
    a = np.asarray(data, dtype=float)
    n = a.shape[1]
    _check_table(n, "S")
    xbb, sb = float(a.mean(axis=1).mean()), float(a.std(axis=1, ddof=1).mean())
    return {"center_xbar": xbb, "mean_sd": sb, "xbar_ucl": xbb + _A3[n] * sb, "xbar_lcl": xbb - _A3[n] * sb, "s_ucl": _B4[n] * sb, "s_lcl": _B3[n] * sb}


_cap("xbar_s_chart", level="advanced", name="X-bar and S control chart limits", topic=_QC,
     purpose="Centre lines and 3-sigma control limits of X-bar and S charts from subgroups (rows) of size 2 to 10.",
     inputs=[M("data", "one row per subgroup, one column per observation (subgroup size 2-10)")],
     outputs=[O("center_xbar", "grand mean"), O("mean_sd", "average subgroup standard deviation"), O("xbar_ucl", "X-bar UCL"),
              O("xbar_lcl", "X-bar LCL"), O("s_ucl", "S chart UCL"), O("s_lcl", "S chart LCL"), O("A3", "A3"), O("B3", "B3"), O("B4", "B4")],
     primary="xbar_ucl", conditions=["equal subgroup sizes 2 to 10", "standard 3-decimal constants (A3, B3, B4)"],
     method="grand mean +/- A3 s-bar; S chart limits B3 s-bar and B4 s-bar",
     latex=r"UCL_{\bar{x}} = \bar{\bar{x}} + A_3\bar{s},\quad UCL_s = B_4\bar{s}", run=run_xbar_s, reference=ref_xbar_s,
     wrong=lambda data: (lambda r: dict(r, xbar_ucl=r["center_xbar"] + 3 * r["mean_sd"]))(run_xbar_s(data)),
     rnd=_rnd_sub, need=r"x-?bar.{0,6}\bs\b|\bs chart|standard deviation chart", prio=6,
     templates=["Find the X-bar and S chart limits for subgroups [[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]].",
                "Compute X-bar S control limits for [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]."],
     examples=[X({"data": "[[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]]"}),
               X({"data": [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]})],
     questions=[Q("Find the X-bar and S chart limits for subgroups [[10.2, 9.9, 10.1, 10.0], [10.3, 10.1, 9.8, 10.2], [9.9, 10.0, 10.1, 9.7]]."),
                Q("Compute X-bar S control limits for [[5, 6, 4], [6, 7, 5], [5, 5, 6], [4, 6, 5]]."),
                Q("Compute X-bar S control limits for [[5, 6, 4]].", outcome="INVALID_INPUT")])


def run_imr(x):
    x = _finite(x)
    _need(x, 2)
    mrb = _mean([abs(x[i] - x[i - 1]) for i in range(1, len(x))])
    m = _mean(x)
    return {"center": m, "mean_moving_range": mrb, "x_ucl": m + _E2 * mrb, "x_lcl": m - _E2 * mrb, "mr_ucl": _D4[2] * mrb, "mr_lcl": 0.0}


def ref_imr(x):
    import numpy as np
    d2, d3, _c4 = _exact_constants(2)
    if abs(_E2 - 3 / d2) > 0.005 or abs(_D4[2] - (1 + 3 * d3 / d2)) > 0.001:
        raise AssertionError("I-MR constants disagree with the exact values")
    a = np.asarray(x, dtype=float)
    mr, m = float(np.abs(np.diff(a)).mean()), float(a.mean())
    return {"center": m, "mean_moving_range": mr, "x_ucl": m + 2.66 * mr, "x_lcl": m - 2.66 * mr, "mr_ucl": 3.267 * mr}


_cap("individuals_moving_range_chart", level="advanced", name="Individuals and moving range (I-MR) chart limits", topic=_QC,
     purpose="Control limits of the individuals (X) and moving range charts from individual measurements in time order.",
     inputs=[V("x", "individual measurements in time order")],
     outputs=[O("center", "mean"), O("mean_moving_range", "average moving range of span 2"), O("x_ucl", "mean + 2.66 MR-bar"),
              O("x_lcl", "mean - 2.66 MR-bar"), O("mr_ucl", "3.267 MR-bar"), O("mr_lcl", "0")], primary="x_ucl",
     conditions=["measurements in time order", "constants E2 = 2.66 (3/d2) and D4 = 3.267 for span 2"],
     method="moving ranges |x_i - x_(i-1)|; limits mean +/- 2.66 MR-bar", latex=r"UCL_x = \bar{x} + 2.66\,\overline{MR}", run=run_imr,
     reference=ref_imr, wrong=lambda x: (lambda r: dict(r, x_ucl=r["center"] + 3 * r["mean_moving_range"]))(run_imr(x)),
     rnd=lambda rng: {"x": _sample(rng, rng.randint(3, 40), rng.uniform(5, 100), rng.uniform(0.2, 5), 2)},
     need=r"individuals?(?: and moving range)? chart|i-mr|moving range chart|\bxmr\b", prio=6,
     templates=["Find the I-MR chart limits for [10.2, 9.8, 10.5, 10.1, 9.7, 10.3, 10.0].",
                "Compute individuals and moving range chart limits for [52, 49, 55, 51, 48, 53, 50, 54]."],
     examples=[X({"x": "10.2, 9.8, 10.5, 10.1, 9.7, 10.3, 10.0"}), X({"x": [52, 49, 55, 51, 48, 53, 50, 54]})],
     questions=[Q("Find the I-MR chart limits for [10.2, 9.8, 10.5, 10.1, 9.7, 10.3, 10.0]."),
                Q("Compute individuals and moving range chart limits for [52, 49, 55, 51, 48, 53, 50, 54]."),
                Q("Find the I-MR chart limits for [10.2].", outcome="INVALID_INPUT")])


def run_p_chart(defectives, n):
    d = _finite(defectives, "defectives")
    n = _whole(n, "n", 1)
    if any(v < 0 or v > n or v != int(v) for v in d):
        raise ValueError("each count of defectives must be a whole number between 0 and n")
    pb = math.fsum(d) / (len(d) * n)
    if pb in (0.0, 1.0):
        raise ValueError("the average fraction defective is 0 or 1")
    s = math.sqrt(pb * (1 - pb) / n)
    return {"center": pb, "ucl": min(1.0, pb + 3 * s), "lcl": max(0.0, pb - 3 * s), "sigma": s}


def ref_p_chart(defectives, n):
    import numpy as np
    from scipy.stats import binom
    pb = float(np.sum(defectives)) / (len(defectives) * n)
    s = float(binom(int(n), pb).std()) / n
    return {"center": pb, "ucl": min(1.0, pb + 3 * s), "lcl": max(0.0, pb - 3 * s), "sigma": s}


_cap("p_chart", level="advanced", name="p chart limits (fraction defective)", topic=_QC,
     purpose="Centre line and 3-sigma limits of a p chart from counts of defective items in samples of constant size n.",
     inputs=[V("defectives", "number of defective items in each sample", 1), I("n", "sample size (constant)", lo=1)],
     outputs=[O("center", "p-bar"), O("ucl", "upper control limit (at most 1)"), O("lcl", "lower control limit (at least 0)"),
              O("sigma", "sqrt(p-bar (1 - p-bar)/n)")], primary="ucl", conditions=["constant sample size", "LCL truncated at 0"],
     method="p-bar +/- 3 sqrt(p-bar(1 - p-bar)/n)", latex=r"\bar{p} \pm 3\sqrt{\frac{\bar{p}(1-\bar{p})}{n}}", run=run_p_chart,
     reference=ref_p_chart, wrong=lambda defectives, n: (lambda r: dict(r, ucl=r["center"] + 3 * r["sigma"] * math.sqrt(n) / 2))(run_p_chart(defectives, n)),
     rnd=lambda rng: (lambda n: {"n": n, "defectives": [float(rng.randint(0, max(1, n // 5))) for _ in range(rng.randint(2, 30))]})(rng.randint(10, 500)),
     need=r"\bp[- ]chart|fraction defective chart|proportion defective chart", prio=6, aliases={"n": r"n|sample size"},
     templates=["Find the p chart limits for defectives [4, 6, 3, 5, 7, 2, 5] with sample size n = 100.",
                "Compute p-chart control limits: n = 50 per sample, defectives [2, 3, 1, 4, 2, 5]."],
     examples=[X({"defectives": "4, 6, 3, 5, 7, 2, 5", "n": 100}), X({"defectives": [2, 3, 1, 4, 2, 5], "n": 50})],
     questions=[Q("Find the p chart limits for defectives [4, 6, 3, 5, 7, 2, 5] with sample size n = 100."),
                Q("Compute p-chart control limits: n = 50 per sample, defectives [2, 3, 1, 4, 2, 5]."),
                Q("Find the p chart limits for defectives [4, 6, 3] with sample size n = 5.", outcome="INVALID_INPUT"),
                Q("Find the p chart limits for defectives [4, 6, 3].", outcome="NEEDS_INPUT")])


def run_c_chart(counts):
    c = _finite(counts, "counts")
    if any(v < 0 for v in c):
        raise ValueError("counts must be >= 0")
    cb = _mean(c)
    if cb <= 0:
        raise ValueError("the average count must be positive")
    return {"center": cb, "ucl": cb + 3 * math.sqrt(cb), "lcl": max(0.0, cb - 3 * math.sqrt(cb))}


def ref_c_chart(counts):
    import numpy as np
    from scipy.stats import poisson
    cb = float(np.mean(counts))
    s = float(poisson(cb).std())
    return {"center": cb, "ucl": cb + 3 * s, "lcl": max(0.0, cb - 3 * s)}


_cap("c_chart", level="advanced", name="c chart limits (defects per unit)", topic=_QC,
     purpose="Centre line and 3-sigma limits of a c chart from counts of defects per inspection unit.",
     inputs=[V("counts", "defects counted in each unit", 1)],
     outputs=[O("center", "c-bar"), O("ucl", "c-bar + 3 sqrt(c-bar)"), O("lcl", "max(0, c-bar - 3 sqrt(c-bar))")], primary="ucl",
     conditions=["constant inspection unit", "Poisson-distributed counts"], method="c-bar +/- 3 sqrt(c-bar)",
     latex=r"\bar{c} \pm 3\sqrt{\bar{c}}", run=run_c_chart, reference=ref_c_chart,
     wrong=lambda counts: (lambda r: dict(r, ucl=r["center"] * 4))(run_c_chart(counts)),
     rnd=lambda rng: {"counts": [float(rng.randint(0, 25)) for _ in range(rng.randint(2, 30))] + [3.0]},
     need=r"\bc[- ]chart|defects per unit chart", prio=6,
     templates=["Find the c chart limits for defect counts [3, 5, 2, 6, 4, 7, 3, 4].", "Compute c-chart control limits for [12, 9, 15, 11, 8, 10]."],
     examples=[X({"counts": "3, 5, 2, 6, 4, 7, 3, 4"}), X({"counts": [12, 9, 15, 11, 8, 10]})],
     questions=[Q("Find the c chart limits for defect counts [3, 5, 2, 6, 4, 7, 3, 4]."), Q("Compute c-chart control limits for [12, 9, 15, 11, 8, 10]."),
                Q("Find the c chart limits for defect counts [0, 0, 0].", outcome="INVALID_INPUT")])


def run_capability(mu, sigma, lsl, usl):
    s = _pos(sigma, "sigma")
    if not usl > lsl:
        raise ValueError("the upper specification limit must exceed the lower one")
    cpu, cpl = (usl - mu) / (3 * s), (mu - lsl) / (3 * s)
    return {"cp": (usl - lsl) / (6 * s), "cpk": min(cpu, cpl), "cpu": cpu, "cpl": cpl,
            "ppm_out": 1e6 * (_phi((lsl - mu) / s) + _phic((usl - mu) / s))}


def ref_capability(mu, sigma, lsl, usl):
    from scipy.stats import norm
    z = lambda x: float(norm.isf(norm.sf(x, mu, sigma)) if x >= mu else norm.ppf(norm.cdf(x, mu, sigma)))
    zu, zl = z(usl), z(lsl)
    return {"cp": (zu - zl) / 6, "cpk": min(zu, -zl) / 3, "cpu": zu / 3, "cpl": -zl / 3,
            "ppm_out": 1e6 * float(norm.cdf(lsl, mu, sigma) + norm.sf(usl, mu, sigma))}


_cap("process_capability", level="advanced", name="Process capability (Cp, Cpk) and expected nonconforming", topic=_QC,
     purpose="Process capability indices Cp, Cpk (with Cpu, Cpl) and the expected parts per million outside the specification "
             "for a normal process with mean mu and standard deviation sigma.",
     inputs=[N("mu", "process mean"), N("sigma", "process standard deviation"), N("lsl", "lower specification limit"), N("usl", "upper specification limit")],
     outputs=[O("cp", "(USL - LSL)/(6 sigma)"), O("cpk", "min(Cpu, Cpl)"), O("cpu", "(USL - mu)/(3 sigma)"), O("cpl", "(mu - LSL)/(3 sigma)"),
              O("ppm_out", "expected nonconforming parts per million")], primary="cpk",
     conditions=["normal, stable (in-control) process", "USL > LSL"], method="capability ratios; normal tail areas for the ppm",
     latex=r"C_p = \frac{USL - LSL}{6\sigma},\quad C_{pk} = \min\left(\frac{USL - \mu}{3\sigma}, \frac{\mu - LSL}{3\sigma}\right)",
     run=run_capability, reference=ref_capability,
     wrong=lambda mu, sigma, lsl, usl: {"cp": (usl - lsl) / (3 * sigma), "cpk": min(usl - mu, mu - lsl) / (6 * sigma)},
     rnd=lambda rng: (lambda mu, s: {"mu": mu, "sigma": s, "lsl": round(mu - s * rng.uniform(-1, 6), 2), "usl": round(mu + s * rng.uniform(0.5, 6), 2)})(
         round(rng.uniform(5, 100), 2), round(rng.uniform(0.1, 3), 3)),
     need=r"\bcpk?\b|process capability|capability index", prio=6,
     aliases={"mu": r"mean|mu", "sigma": r"standard deviation|sd|sigma", "lsl": r"lsl|lower specification limit|lower spec",
              "usl": r"usl|upper specification limit|upper spec"},
     templates=["Find Cp and Cpk for a process with mean = 10.2, standard deviation = 0.1, LSL = 9.8 and USL = 10.5.",
                "Process capability: mu = 50, sigma = 2, lower specification limit = 44, upper specification limit = 58."],
     examples=[X({"mu": 10.2, "sigma": 0.1, "lsl": 9.8, "usl": 10.5}), X({"mu": 50, "sigma": 2, "lsl": 44, "usl": 58})],
     questions=[Q("Find Cp and Cpk for a process with mean = 10.2, standard deviation = 0.1, LSL = 9.8 and USL = 10.5."),
                Q("Process capability: mu = 50, sigma = 2, lower specification limit = 44, upper specification limit = 58."),
                Q("Find Cp and Cpk for a process with mean = 10.2, standard deviation = 0.1, LSL = 10.8 and USL = 10.5.", outcome="INVALID_INPUT"),
                Q("Find Cp and Cpk for a process with mean = 10.2, LSL = 9.8 and USL = 10.5.", outcome="NEEDS_INPUT")])


# ================================================================================================= reliability ====
_RL = "reliability and survival"


def run_weibull(beta, eta, t):
    b, e = _pos(beta, "the shape beta"), _pos(eta, "the scale eta")
    if t < 0:
        raise ValueError("t must be >= 0")
    u = (t / e) ** b
    return {"reliability": math.exp(-u), "unreliability": -math.expm1(-u), "hazard_rate": b / e * (t / e) ** (b - 1) if t > 0 or b >= 1 else math.inf,
            "mttf": e * math.gamma(1 + 1 / b)}


def ref_weibull(beta, eta, t):
    import numpy as np
    from scipy.stats import weibull_min
    w = weibull_min(beta, scale=eta)
    return {"reliability": float(w.sf(t)), "unreliability": float(w.cdf(t)), "hazard_rate": float(np.exp(w.logpdf(t) - w.logsf(t))), "mttf": float(w.mean())}


_cap("weibull_reliability", level="advanced", name="Weibull reliability, hazard rate and MTTF", topic=_RL,
     purpose="Reliability R(t), unreliability, hazard rate and mean time to failure of a two-parameter Weibull distribution.",
     inputs=[N("beta", "shape parameter beta"), N("eta", "scale parameter eta (characteristic life)"), N("t", "time")],
     outputs=[O("reliability", "exp(-(t/eta)^beta)"), O("unreliability", "1 - R(t)"), O("hazard_rate", "(beta/eta)(t/eta)^(beta-1)"),
              O("mttf", "eta Gamma(1 + 1/beta)")], primary="reliability", conditions=["two-parameter Weibull (no location shift)", "t >= 0"],
     method="closed forms of the Weibull survival, hazard and mean", latex=r"R(t) = e^{-(t/\eta)^\beta},\quad h(t) = \frac{\beta}{\eta}\left(\frac{t}{\eta}\right)^{\beta-1}",
     run=run_weibull, reference=ref_weibull, wrong=lambda beta, eta, t: {"reliability": math.exp(-(t / eta) ** (1 / beta)) if beta != 1 else 0.5, "mttf": eta},
     rnd=lambda rng: {"beta": round(rng.uniform(0.5, 4), 2), "eta": round(rng.uniform(50, 5000), 1), "t": round(rng.uniform(1, 6000), 1)},
     need=r"weibull", avoid=r"quartile|percentile|iqr|method", prio=6,
     aliases={"beta": r"beta|shape|shape parameter", "eta": r"eta|scale|scale parameter|characteristic life", "t": r"t|time"},
     templates=["For a Weibull distribution with beta = 2 and eta = 1000 hours, find the reliability at t = 500.",
                "Weibull with shape = 1.5 and scale = 2000: find R(t), the hazard rate and MTTF at time = 1200."],
     examples=[X({"beta": 2, "eta": 1000, "t": 500}), X({"beta": 1.5, "eta": 2000, "t": 1200})],
     questions=[Q("For a Weibull distribution with beta = 2 and eta = 1000 hours, find the reliability at t = 500."),
                Q("Weibull with shape = 1.5 and scale = 2000: find R(t), the hazard rate and MTTF at time = 1200."),
                Q("For a Weibull distribution with beta = -2 and eta = 1000 hours, find the reliability at t = 500.", outcome="INVALID_INPUT"),
                Q("For a Weibull distribution with beta = 2, find the reliability at t = 500.", outcome="NEEDS_INPUT")])


def run_exp_rel(t, rate=None, mtbf=None):
    if (rate is None) == (mtbf is None):
        raise ValueError("give either the failure rate or the MTBF (not both)")
    lam = _pos(rate, "the failure rate") if rate is not None else 1 / _pos(mtbf, "the MTBF")
    if t < 0:
        raise ValueError("t must be >= 0")
    return {"reliability": math.exp(-lam * t), "unreliability": -math.expm1(-lam * t), "mttf": 1 / lam, "failure_rate": lam}


def ref_exp_rel(t, rate=None, mtbf=None):
    from scipy.stats import expon
    e = expon(scale=mtbf if mtbf is not None else 1 / rate)
    return {"reliability": float(e.sf(t)), "unreliability": float(e.cdf(t)), "mttf": float(e.mean())}


_cap("exponential_reliability", level="advanced", name="Exponential reliability (constant failure rate)", topic=_RL,
     purpose="Reliability R(t) = exp(-lambda t), unreliability and MTTF for a constant failure rate (given the rate or the MTBF).",
     inputs=[N("t", "mission time"), N("rate", "failure rate lambda", required=False), N("mtbf", "mean time between failures (instead of the rate)", required=False)],
     outputs=[O("reliability", "exp(-lambda t)"), O("unreliability", "1 - R(t)"), O("mttf", "1/lambda"), O("failure_rate", "lambda")],
     primary="reliability", conditions=["constant failure rate (useful-life period)", "exactly one of rate or MTBF"],
     method="R(t) = exp(-lambda t)", latex=r"R(t) = e^{-\lambda t},\quad MTTF = 1/\lambda", run=run_exp_rel, reference=ref_exp_rel,
     wrong=lambda t, rate=None, mtbf=None: {"reliability": 1 - (rate if rate is not None else 1 / mtbf) * t},
     rnd=lambda rng: dict({"t": round(rng.uniform(1, 5000), 1)}, **({"rate": round(rng.uniform(1e-5, 2e-3), 6)} if rng.random() < 0.5 else {"mtbf": round(rng.uniform(100, 50000), 1)})),
     need=[r"reliab|mtbf|failure rate", r"exponential|constant failure rate|mtbf|failure rate"], avoid=r"weibull|series|parallel|out of|\[", prio=5,
     aliases={"rate": r"failure rate|rate|lambda", "mtbf": r"mtbf|mean time between failures", "t": r"t|time|mission time"},
     templates=["Find the reliability at t = 1000 hours for a constant failure rate = 0.0002 per hour.",
                "A component has MTBF = 5000 hours with exponential failures; what is the reliability for a mission time = 800?"],
     examples=[X({"t": 1000, "rate": 0.0002}), X({"t": 800, "mtbf": 5000})],
     questions=[Q("Find the reliability at t = 1000 hours for a constant failure rate = 0.0002 per hour."),
                Q("A component has MTBF = 5000 hours with exponential failures; what is the reliability for a mission time = 800?"),
                Q("Find the reliability at t = 1000 hours for a constant failure rate = -0.0002 per hour.", outcome="INVALID_INPUT")])


def _rels(r):
    r = _finite(r, "component reliabilities")
    if any(not 0 <= v <= 1 for v in r):
        raise ValueError("each reliability must be between 0 and 1")
    if len(r) > 20:
        raise ValueError("at most 20 components")
    return r


def _ref_rbd(r, need):
    p = 0.0
    for states in _iproduct((0, 1), repeat=len(r)):
        if need(states):
            p += math.prod(v if s else 1 - v for v, s in zip(r, states))
    return p


for _key, _nm, _f, _need_f, _lx in (("series_system_reliability", "series", lambda r: math.prod(r), all, r"R_s = \prod R_i"),
                                    ("parallel_system_reliability", "parallel", lambda r: 1 - math.prod(1 - v for v in r), any,
                                     r"R_p = 1 - \prod (1 - R_i)")):
    _cap(_key, level="advanced", name=f"Reliability of a {_nm} system", topic=_RL,
         purpose=f"The reliability of independent components in {_nm} ({'all must work' if _nm == 'series' else 'at least one must work'}).",
         inputs=[V("r", "component reliabilities (0 to 1)", 1, max_length=20)], outputs=[O("reliability", "system reliability")],
         primary="reliability", conditions=["independent components"], method="product rule" + (" on the unreliabilities" if _nm == "parallel" else ""),
         latex=_lx, run=(lambda f: (lambda r: {"reliability": f(_rels(r))}))(_f),
         reference=(lambda nf: (lambda r: {"reliability": _ref_rbd(r, nf)}))(_need_f),
         wrong=(lambda nm: (lambda r: {"reliability": min(r) if nm == "series" else max(r)}))(_nm),
         rnd=lambda rng: {"r": [round(rng.uniform(0.5, 0.999), 3) for _ in range(rng.randint(2, 10))]},
         need=[r"reliab", r"\b" + _nm + r"\b"], avoid=r"parallel" if _nm == "series" else r"\bseries\b", prio=6,
         templates=[f"Find the reliability of a {_nm} system with component reliabilities [0.95, 0.9, 0.98].",
                    f"Components with reliabilities [0.8, 0.85] are connected in {_nm}; what is the system reliability?"],
         examples=[X({"r": "0.95, 0.9, 0.98"}), X({"r": [0.8, 0.85]})],
         questions=[Q(f"Find the reliability of a {_nm} system with component reliabilities [0.95, 0.9, 0.98]."),
                    Q(f"Components with reliabilities [0.8, 0.85] are connected in {_nm}; what is the system reliability?"),
                    Q(f"Find the reliability of a {_nm} system with component reliabilities [0.95, 1.9].", outcome="INVALID_INPUT")])


def run_k_of_n(n, k, r):
    n, k, r = _whole(n, "n", 1), _whole(k, "k", 1), _prob(r, "r", open_=False)
    if k > n:
        raise ValueError("k cannot exceed n")
    return {"reliability": min(1.0, math.fsum(math.comb(n, j) * r ** j * (1 - r) ** (n - j) for j in range(k, n + 1)))}


_cap("k_out_of_n_reliability", level="advanced", name="k-out-of-n system reliability", topic=_RL,
     purpose="The reliability of a system that works when at least k of n identical independent components work.",
     inputs=[I("n", "number of components", lo=1), I("k", "number that must work", lo=1), N("r", "reliability of one component")],
     outputs=[O("reliability", "P(at least k of n work)")], primary="reliability", conditions=["identical, independent components"],
     method="binomial upper tail", latex=r"R = \sum_{j=k}^{n}\binom{n}{j} r^j (1-r)^{n-j}", run=run_k_of_n,
     reference=lambda n, k, r: {"reliability": float(__import__("scipy.stats", fromlist=["binom"]).binom.sf(int(k) - 1, int(n), r))},
     wrong=lambda n, k, r: {"reliability": math.fsum(math.comb(int(n), j) * r ** j * (1 - r) ** (n - j) for j in range(int(k) + 1, int(n) + 1))},
     rnd=lambda rng: (lambda n: {"n": n, "k": rng.randint(1, n), "r": round(rng.uniform(0.3, 0.999), 3)})(rng.randint(1, 30)),
     need=[r"out[- ]of", r"reliab|components?|system"], avoid=r"\[|successes|confidence|proportion", prio=6,
     aliases={"n": r"n|components", "k": r"k", "r": r"r|component reliability|reliability"},
     templates=["Find the reliability of a k-out-of-n system with k = 2, n = 3 and component reliability = 0.9.",
                "A system needs k = 3 out of n = 5 components working, each with r = 0.85; what is the system reliability?"],
     examples=[X({"n": 3, "k": 2, "r": 0.9}), X({"n": 5, "k": 3, "r": 0.85})],
     questions=[Q("Find the reliability of a k-out-of-n system with k = 2, n = 3 and component reliability = 0.9."),
                Q("A system needs k = 3 out of n = 5 components working, each with r = 0.85; what is the system reliability?"),
                Q("Find the reliability of a k-out-of-n system with k = 4, n = 3 and component reliability = 0.9.", outcome="INVALID_INPUT")])


# ===================================================================================================== survival ====
def _surv_data(times, events):
    t, e = _finite(times, "times"), _finite(events, "events")
    if len(t) != len(e):
        raise ValueError("give one event indicator per time")
    if any(v not in (0.0, 1.0) for v in e):
        raise ValueError("event indicators must be 1 (event) or 0 (censored)")
    if any(v < 0 for v in t):
        raise ValueError("times must be >= 0")
    return t, e


def run_km(times, events, t):
    T, E = _surv_data(times, events)
    s, curve_t, curve_s = 1.0, [], []
    for u in sorted(set(v for v, ev in zip(T, E) if ev == 1)):
        at_risk = sum(1 for v in T if v >= u)
        d = sum(1 for v, ev in zip(T, E) if v == u and ev == 1)
        s *= 1 - d / at_risk
        curve_t.append(u)
        curve_s.append(s)
    at_t = 1.0
    for u, v in zip(curve_t, curve_s):
        if u <= t:
            at_t = v
    return {"survival": at_t, "event_times": curve_t, "survival_curve": curve_s}


def ref_km(times, events, t):
    from lifelines import KaplanMeierFitter
    k = KaplanMeierFitter().fit(times, event_observed=events)
    ets = sorted(set(v for v, ev in zip(times, events) if ev == 1))
    return {"survival": float(k.survival_function_at_times(t).iloc[0]),
            "survival_curve": [float(v) for v in k.survival_function_at_times(ets).values] if ets else []}


def _rnd_surv(rng, n=None):
    n = n or rng.randint(3, 30)
    T = [float(rng.randint(1, 40)) for _ in range(n)]
    E = [float(rng.random() < 0.7) for _ in range(n)]
    return T, E


_cap("kaplan_meier", level="expert", name="Kaplan-Meier survival estimate", topic=_RL,
     purpose="The Kaplan-Meier (product-limit) estimate of the survival function from times with event/censoring indicators, "
             "evaluated at time t, and the whole step curve.",
     inputs=[V("times", "follow-up times", 1), V("events", "1 = event observed, 0 = censored", 1), N("t", "time at which to report S(t)")],
     outputs=[O("survival", "S(t)"), O("event_times", "distinct event times"), O("survival_curve", "S just after each event time")],
     primary="survival", conditions=["non-informative censoring", "a censoring at an event time counts as still at risk"],
     method="product over event times of (1 - deaths / number at risk)", latex=r"\hat{S}(t) = \prod_{t_i \le t}\left(1 - \frac{d_i}{n_i}\right)",
     run=run_km, reference=ref_km,
     wrong=lambda times, events, t: {"survival": sum(1 for v in times if v > t) / len(times) + (0.01 if all(e == 1 for e in events) else 0.0)},
     rnd=lambda rng: (lambda te: {"times": te[0], "events": te[1], "t": float(rng.randint(0, 45))})(_rnd_surv(rng)),
     need=r"kaplan|product[- ]limit|survival (?:function|curve|probability)", avoid=r"log[- ]rank", prio=6,
     aliases={"t": r"t|time"},
     templates=["Kaplan-Meier: times [6, 6, 6, 7, 10, 13, 16, 22, 23] with events [1, 1, 1, 0, 1, 1, 1, 1, 1]; find the survival at t = 12.",
                "Estimate the survival probability by Kaplan-Meier for times [2, 3, 3, 5, 8, 9, 12] and events [1, 0, 1, 1, 0, 1, 1] at time = 6."],
     examples=[X({"times": "6, 6, 6, 7, 10, 13, 16, 22, 23", "events": "1, 1, 1, 0, 1, 1, 1, 1, 1", "t": 12}),
               X({"times": [2, 3, 3, 5, 8, 9, 12], "events": [1, 0, 1, 1, 0, 1, 1], "t": 6})],
     questions=[Q("Kaplan-Meier: times [6, 6, 6, 7, 10, 13, 16, 22, 23] with events [1, 1, 1, 0, 1, 1, 1, 1, 1]; find the survival at t = 12."),
                Q("Estimate the survival probability by Kaplan-Meier for times [2, 3, 3, 5, 8, 9, 12] and events [1, 0, 1, 1, 0, 1, 1] at time = 6."),
                Q("Kaplan-Meier: times [6, 7, 10] with events [1, 2, 1]; find the survival at t = 8.", outcome="INVALID_INPUT")])


def run_logrank(times1, events1, times2, events2):
    T1, E1 = _surv_data(times1, events1)
    T2, E2 = _surv_data(times2, events2)
    o1 = e1 = v = 0.0
    for u in sorted(set(t for t, e in zip(T1 + T2, E1 + E2) if e == 1)):
        n1, n2 = sum(1 for t in T1 if t >= u), sum(1 for t in T2 if t >= u)
        d1 = sum(1 for t, e in zip(T1, E1) if t == u and e == 1)
        d = d1 + sum(1 for t, e in zip(T2, E2) if t == u and e == 1)
        n = n1 + n2
        o1 += d1
        e1 += d * n1 / n
        if n > 1:
            v += d * (n1 / n) * (n2 / n) * (n - d) / (n - 1)
    if v <= 0:
        raise ValueError("the log-rank variance is zero (no informative event times)")
    stat = (o1 - e1) ** 2 / v
    return {"chi_square": stat, "p_value": _chi2_sf(stat, 1), "observed_1": o1, "expected_1": e1}


def ref_logrank(times1, events1, times2, events2):
    from lifelines.statistics import logrank_test
    r = logrank_test(times1, times2, event_observed_A=events1, event_observed_B=events2)
    return {"chi_square": float(r.test_statistic), "p_value": float(r.p_value)}


_cap("log_rank_test", level="expert", name="Log-rank test comparing two survival curves", topic=_RL,
     purpose="The log-rank (Mantel-Cox) test of equal survival in two groups from times and event indicators.",
     inputs=[V("times1", "times, group 1", 1), V("events1", "events (1/0), group 1", 1), V("times2", "times, group 2", 1),
             V("events2", "events (1/0), group 2", 1)],
     outputs=[O("chi_square", "(O1 - E1)^2 / V"), O("p_value", "chi-square p-value (1 df)"), O("observed_1", "events in group 1"),
              O("expected_1", "expected events in group 1")], primary="p_value",
     conditions=["non-informative censoring", "proportional hazards gives the test its best power"],
     method="at each event time: expected events and hypergeometric variance; sum over times",
     latex=r"\chi^2 = \frac{\left(\sum (d_{1i} - e_{1i})\right)^2}{\sum v_i}", run=run_logrank, reference=ref_logrank,
     wrong=lambda times1, events1, times2, events2: (lambda r: {"chi_square": (r["observed_1"] - r["expected_1"]) ** 2 / max(r["expected_1"], 1e-9) + 0.01})(
         run_logrank(times1, events1, times2, events2)),
     rnd=lambda rng: (lambda a, b: {"times1": a[0], "events1": a[1], "times2": b[0], "events2": b[1]})(_rnd_surv(rng), _rnd_surv(rng)),
     need=r"log[- ]?rank|mantel[- ]cox", prio=6,
     templates=["Log-rank test: group 1 times [6, 6, 6, 7, 10, 13, 16] events [1, 1, 1, 0, 1, 1, 1], group 2 times [1, 1, 2, 2, 3, 4, 4] events [1, 1, 1, 1, 1, 1, 1].",
                "Compare survival with a log-rank test for times [5, 8, 12, 20] events [1, 1, 0, 1] versus times [3, 4, 9, 10] events [1, 1, 1, 0]."],
     examples=[X({"times1": "6, 6, 6, 7, 10, 13, 16", "events1": "1, 1, 1, 0, 1, 1, 1", "times2": "1, 1, 2, 2, 3, 4, 4", "events2": "1, 1, 1, 1, 1, 1, 1"}),
               X({"times1": [5, 8, 12, 20], "events1": [1, 1, 0, 1], "times2": [3, 4, 9, 10], "events2": [1, 1, 1, 0]})],
     questions=[Q("Log-rank test: group 1 times [6, 6, 6, 7, 10, 13, 16] events [1, 1, 1, 0, 1, 1, 1], group 2 times [1, 1, 2, 2, 3, 4, 4] events [1, 1, 1, 1, 1, 1, 1]."),
                Q("Compare survival with a log-rank test for times [5, 8, 12, 20] events [1, 1, 0, 1] versus times [3, 4, 9, 10] events [1, 1, 1, 0]."),
                Q("Log-rank test: group 1 times [6, 7] events [1, 1].", outcome="NEEDS_INPUT")])

# @@CAPABILITIES@@

# @@VALUES-BEGIN@@
# generated by a run of each example; every value is checked against the reference library on the
# compute box before release (examples gate: the procedure must reproduce them exactly)
_EXAMPLE_VALUES = {'bartlett_test': [{'df': 2, 'p_value': 0.013445540650031874, 'statistic': 8.618215556185413},
                   {'df': 1, 'p_value': 0.004316807338806683, 'statistic': 8.145503254971933}],
 'binomial_at_least': [{'probability': 0.8327102464000001}, {'probability': 0.5851584974698203},
                       {'probability': 0.015625000000000007}],
 'binomial_at_most': [{'probability': 0.3822806015999999}, {'probability': 0.617172654387105}, {'probability': 1.0}],
 'binomial_exactly': [{'probability': 0.21499084799999993}, {'probability': 0.20233115185692457},
                      {'probability': 0.015625000000000007}],
 'binomial_mean_sd': [{'mean': 15.0, 'standard_deviation': 3.24037034920393, 'variance': 10.5},
                      {'mean': 6.0, 'standard_deviation': 1.7320508075688772, 'variance': 3.0}],
 'c_chart': [{'center': 4.25, 'lcl': 0.0, 'ucl': 10.43465843842649},
             {'center': 10.833333333333334, 'lcl': 0.9591245042675833, 'ucl': 20.707542162399086}],
 'chi_square_critical_value': [{'critical_value': 9.487729036781154, 'left_critical_value': 0.7107230213973241},
                               {'critical_value': 23.20925115895436, 'left_critical_value': 2.558212160187205},
                               {'critical_value': 3.841458820694127, 'left_critical_value': 0.003932140000019525}],
 'chi_square_goodness_of_fit': [{'chi_square': 2.9,
                                 'df': 4,
                                 'expected': [20.0, 20.0, 20.0, 20.0, 20.0],
                                 'p_value': 0.5746972058298042},
                                {'chi_square': 0.0, 'df': 2, 'expected': [50.0, 30.0, 20.0], 'p_value': 1.0},
                                {'chi_square': 2.0,
                                 'df': 2,
                                 'expected': [25.0, 50.0, 25.0],
                                 'p_value': 0.3678794411714422}],
 'chi_square_independence': [{'chi_square': 1.0101010101010102,
                              'df': 1,
                              'expected': [[22.5, 27.5], [22.5, 27.5]],
                              'p_value': 0.31487864133641985},
                             {'chi_square': 4.5021645021645025,
                              'df': 2,
                              'expected': [[15.0, 17.5, 27.5], [15.0, 17.5, 27.5]],
                              'p_value': 0.10528521784009053}],
 'chi_square_variance_test': [{'chi_square': 6.666666666666667,
                               'df': 5,
                               'p_value': 0.49326830437210445,
                               'sample_variance': 0.12},
                              {'chi_square': 12.678571428571429,
                               'df': 6,
                               'p_value': 0.04843390567356849,
                               'sample_variance': 33.80952380952381}],
 'ci_mean_known_sigma': [{'critical_value': 1.959963984540054,
                          'lower': 49.38671468727993,
                          'margin_of_error': 2.613285312720072,
                          'upper': 54.61328531272007},
                         {'critical_value': 2.575829303548901,
                          'lower': 114.53584089684465,
                          'margin_of_error': 5.464159103155349,
                          'upper': 125.46415910315535}],
 'ci_mean_t': [{'critical_value': 2.570581835636313,
                'df': 5,
                'lower': 11.623702162510888,
                'margin_of_error': 2.709631170822446,
                'mean': 14.333333333333334,
                'upper': 17.04296450415578},
               {'critical_value': 2.1318467863266504,
                'df': 4,
                'lower': 4.836611605245187,
                'margin_of_error': 0.18338839475481358,
                'mean': 5.0200000000000005,
                'upper': 5.203388394754814}],
 'ci_mean_t_summary': [{'critical_value': 2.131449545559774,
                        'df': 15,
                        'lower': 22.868550454440225,
                        'margin_of_error': 2.131449545559774,
                        'upper': 27.131449545559775},
                       {'critical_value': 3.249835541592125,
                        'df': 9,
                        'lower': 2.686155883380101,
                        'margin_of_error': 0.5138441166198991,
                        'upper': 3.7138441166198994}],
 'ci_proportion': [{'lower': 0.3123477459423419,
                    'margin_of_error': 0.08765225405765815,
                    'proportion': 0.4,
                    'upper': 0.48765225405765816},
                   {'lower': 0.5682020434044758,
                    'margin_of_error': 0.0557979565955242,
                    'proportion': 0.624,
                    'upper': 0.6797979565955242}],
 'ci_two_means': [{'df': 7.174149270803547,
                   'difference': 3.8000000000000007,
                   'lower': 0.2782954860255704,
                   'standard_error': 1.4966629547095767,
                   'upper': 7.3217045139744314},
                  {'df': 7,
                   'difference': 0.9499999999999993,
                   'lower': 0.4867543607441512,
                   'standard_error': 0.24451117415307266,
                   'upper': 1.4132456392558475}],
 'ci_variance': [{'lower': 2.5975708672276663,
                  'sd_lower': 1.6116981315456274,
                  'sd_upper': 6.332622088215267,
                  'upper': 40.10210251215189,
                  'variance': 6.666666666666666},
                 {'lower': 0.015599096414563175,
                  'sd_lower': 0.12489634267889183,
                  'sd_upper': 0.45633172855158494,
                  'upper': 0.20823864648287738,
                  'variance': 0.03699999999999996}],
 'coefficient_of_variation': [{'cv': 0.22587697572631282,
                               'cv_percent': 22.58769757263128,
                               'mean': 14.0,
                               'standard_deviation': 3.1622776601683795},
                              {'cv': 0.07615773105863909,
                               'cv_percent': 7.6157731058639095,
                               'mean': 50.0,
                               'standard_deviation': 3.8078865529319543}],
 'cohens_d': [{'d': 1.6057930839841819, 'hedges_g': 1.4503937532760351, 'pooled_sd': 2.3664319132398464},
              {'d': 2.6063404905514878, 'hedges_g': 2.3167471027124336, 'pooled_sd': 0.36449573777637334}],
 'correlation_confidence_interval': [{'lower': 0.5591491951264971,
                                      'r': 0.9428571428571428,
                                      'upper': 0.9938998355880978},
                                     {'lower': -0.34501780003129634,
                                      'r': 0.5298089018901744,
                                      'upper': 0.9120423900566206}],
 'correlation_t_test': [{'df': 28, 'p_value': 0.012591071275196835, 't': 2.666405420850055},
                        {'df': 48, 'p_value': 0.011742763163910696, 't': -2.340071757206818}],
 'covariance': [{'population_covariance': 2.75, 'sample_covariance': 3.6666666666666665},
                {'population_covariance': 1.222222222222222, 'sample_covariance': 1.8333333333333333}],
 'cronbach_alpha': [{'alpha': 0.9130434782608696, 'items': 3, 'respondents': 5},
                    {'alpha': 0.9629629629629629, 'items': 4, 'respondents': 4}],
 'discrete_random_variable': [{'expected_value': 1.7000000000000002, 'standard_deviation': 0.9, 'variance': 0.81},
                              {'expected_value': 17.5, 'standard_deviation': 4.330127018922194, 'variance': 18.75}],
 'durbin_watson': [{'durbin_watson': 2.53448275862069}, {'durbin_watson': 0.2706552706552706}],
 'exponential_probability_above': [{'probability': 0.22313016014842982}, {'probability': 0.6065306597126334}],
 'exponential_probability_below': [{'probability': 0.7768698398515702}, {'probability': 0.39346934028736663}],
 'exponential_reliability': [{'failure_rate': 0.0002,
                              'mttf': 5000.0,
                              'reliability': 0.8187307530779818,
                              'unreliability': 0.18126924692201812},
                             {'failure_rate': 0.0002,
                              'mttf': 5000.0,
                              'reliability': 0.8521437889662113,
                              'unreliability': 0.14785621103378865}],
 'f_critical_value': [{'critical_value': 3.0983912121407844}, {'critical_value': 5.636326187669079}],
 'f_test_variances': [{'df1': 5, 'df2': 4, 'f': 9.714285714285714, 'p_value': 0.04685703619078499},
                      {'df1': 4, 'df2': 3, 'f': 70.02000000000012, 'p_value': 0.0027101153548101764}],
 'factorial_2x2_effects': [{'A': 10.5, 'AB': 2.5, 'B': -7.5, 'grand_mean': 28.25},
                           {'A': 15.0, 'AB': 5.0, 'B': 10.0, 'grand_mean': 30.0}],
 'factorial_2x2x2_effects': [{'A': 1.25,
                              'AB': -1.75,
                              'ABC': -6.75,
                              'AC': -13.75,
                              'B': 12.75,
                              'BC': -5.25,
                              'C': 9.75,
                              'grand_mean': 40.875},
                             {'A': 52.0,
                              'AB': -12.0,
                              'ABC': 24.5,
                              'AC': -30.5,
                              'B': -1.0,
                              'BC': -18.5,
                              'C': 9.5,
                              'grand_mean': 623.0}],
 'fisher_exact_test': [{'odds_ratio': 20.0, 'p_value': 0.03496503496503497},
                       {'odds_ratio': 0.047619047619047616, 'p_value': 0.009883305548940223}],
 'five_number_summary': [{'iqr': 7.0, 'maximum': 15.0, 'median': 8.0, 'minimum': 1.0, 'q1': 4.5, 'q3': 11.5},
                         {'iqr': 28.0, 'maximum': 49.0, 'median': 40.0, 'minimum': 6.0, 'q1': 15.0, 'q3': 43.0},
                         {'iqr': 17.0, 'maximum': 49.0, 'median': 40.0, 'minimum': 6.0, 'q1': 25.5, 'q3': 42.5}],
 'frequency_table_statistics': [{'mean': 2.65,
                                 'n': 20,
                                 'population_standard_deviation': 0.9630680142129111,
                                 'sample_standard_deviation': 0.9880869341680844},
                                {'mean': 21.0,
                                 'n': 10,
                                 'population_standard_deviation': 7.0,
                                 'sample_standard_deviation': 7.378647873726218}],
 'friedman_test': [{'chi_square': 0.4000000000000057, 'df': 2, 'p_value': 0.8187307530779795},
                   {'chi_square': 8.200000000000003, 'df': 3, 'p_value': 0.04205418289496617}],
 'geometric_mean': [{'geometric_mean': 4.0}, {'geometric_mean': 1.071202823901392},
                    {'geometric_mean': 0.49999999999999994}],
 'geometric_probability': [{'cumulative': 0.5904, 'probability': 0.1024},
                           {'cumulative': 0.875, 'probability': 0.125}],
 'harmonic_mean': [{'harmonic_mean': 47.99999999999999}, {'harmonic_mean': 1.7142857142857142}],
 'hypergeometric_probability': [{'cumulative': 0.9517396968037909, 'probability': 0.20983971757065453},
                                {'cumulative': 0.9422084623323014, 'probability': 0.23839009287925697},
                                {'cumulative': 0.6588419983377967, 'probability': 0.6588419983377967}],
 'individuals_moving_range_chart': [{'center': 10.085714285714285,
                                     'mean_moving_range': 0.4666666666666668,
                                     'mr_lcl': 0.0,
                                     'mr_ucl': 1.5246000000000004,
                                     'x_lcl': 8.844380952380952,
                                     'x_ucl': 11.327047619047619},
                                    {'center': 51.5,
                                     'mean_moving_range': 4.0,
                                     'mr_lcl': 0.0,
                                     'mr_ucl': 13.068,
                                     'x_lcl': 40.86,
                                     'x_ucl': 62.14}],
 'inverse_normal': [{'x': 119.22327348316901, 'z': 1.2815515655446006},
                    {'x': 66.44853626951473, 'z': 1.6448536269514724},
                    {'x': 1.959963984540054, 'z': 1.959963984540054}],
 'iqr_outliers': [{'iqr': 1.75,
                   'lower_fence': 9.375,
                   'outliers': [102.0],
                   'q1': 12.0,
                   'q3': 13.75,
                   'upper_fence': 16.375},
                  {'iqr': 2.5, 'lower_fence': 0.0, 'outliers': [30.0], 'q1': 3.75, 'q3': 6.25, 'upper_fence': 10.0}],
 'k_out_of_n_reliability': [{'reliability': 0.9720000000000001}, {'reliability': 0.9733881249999999}],
 'kaplan_meier': [{'event_times': [6.0, 10.0, 13.0, 16.0, 22.0, 23.0],
                   'survival': 0.5333333333333334,
                   'survival_curve': [0.6666666666666667, 0.5333333333333334, 0.4000000000000001, 0.2666666666666668,
                                      0.1333333333333334, 0.0]},
                  {'event_times': [2.0, 3.0, 5.0, 9.0, 12.0],
                   'survival': 0.5357142857142858,
                   'survival_curve': [0.8571428571428572, 0.7142857142857144, 0.5357142857142858, 0.2678571428571429,
                                      0.0]}],
 'kendall_tau': [{'concordant': 12, 'discordant': 3, 'p_value': 0.09087393998624908, 'tau': 0.6},
                 {'concordant': 2, 'discordant': 6, 'p_value': 0.28274545993277467, 'tau': -0.47140452079103173}],
 'kruskal_wallis': [{'df': 2, 'h': 2.853801169590639, 'p_value': 0.24005179059000692},
                    {'df': 2, 'h': 7.318181818181813, 'p_value': 0.02575591655624027}],
 'levene_test': [{'df1': 2, 'df2': 12, 'p_value': 0.0266649501869476, 'w': 4.977272727272727},
                 {'df1': 1, 'df2': 6, 'p_value': 0.008582732014852098, 'w': 14.72727272727274}],
 'log_rank_test': [{'chi_square': 13.81148342629696,
                    'expected_1': 10.422222222222222,
                    'observed_1': 6.0,
                    'p_value': 0.00020209722320530527},
                   {'chi_square': 0.5818837585002068,
                    'expected_1': 3.8380952380952382,
                    'observed_1': 3.0,
                    'p_value': 0.445574893590188}],
 'mann_whitney_exact': [{'p_value': 0.031746031746031744, 'u': 19.0}, {'p_value': 0.015873015873015872, 'u': 19.0}],
 'mann_whitney_u': [{'p_value': 0.037336415920662905, 'u': 19.0}, {'p_value': 0.018668207960331452, 'u': 19.0}],
 'mcnemar_test': [{'chi_square': 2.1176470588235294,
                   'discordant': 17,
                   'exact_p_value': 0.143463134765625,
                   'p_value': 0.1456100953968671},
                  {'chi_square': 2.7,
                   'discordant': 30,
                   'exact_p_value': 0.09873714670538902,
                   'p_value': 0.10034824646229112}],
 'mean': [{'mean': 5.0, 'n': 3, 'sum': 15.0}, {'mean': 14.0, 'n': 5, 'sum': 70.0},
          {'mean': 3.4375, 'n': 4, 'sum': 13.75}],
 'mean_absolute_deviation': [{'mean': 5.0, 'mean_absolute_deviation': 2.0},
                             {'mean': 25.0, 'mean_absolute_deviation': 12.0}],
 'median': [{'median': 5.0, 'n': 5}, {'median': 15.5, 'n': 6}, {'median': 3.2, 'n': 1}],
 'median_absolute_deviation': [{'median': 2.0, 'median_absolute_deviation': 1.0},
                               {'median': 13.0, 'median_absolute_deviation': 2.0}],
 'mode': [{'frequency': 3, 'has_mode': 'yes', 'modes': [7.0]},
          {'frequency': 2, 'has_mode': 'yes', 'modes': [2.0, 3.0]}, {'frequency': 1, 'has_mode': 'no', 'modes': []}],
 'multiple_linear_regression': [{'adj_r_squared': 0.989430894308943,
                                 'coefficients': [6.229166666666668, 3.0625, 0.06249999999999978],
                                 'f': 235.03846153846135,
                                 'f_p_value': 0.0005049916961929541,
                                 'r_squared': 0.9936585365853658,
                                 'residual_se': 0.6009252125773318},
                                {'adj_r_squared': 0.9979186837986387,
                                 'coefficients': [3.5407917383821115, 1.937349397590363, 0.25941480206540407],
                                 'f': 1439.395592864642,
                                 'f_p_value': 1.925278724466295e-06,
                                 'r_squared': 0.9986124558657591,
                                 'residual_se': 0.4050031342179138}],
 'negative_binomial_probability': [{'probability': 0.12441600000000003}, {'probability': 0.12500000000000003}],
 'normal_probability_above': [{'probability': 0.022750131948179216, 'z': 2.0},
                              {'probability': 0.06680720126885809, 'z': 1.5}],
 'normal_probability_below': [{'probability': 0.9087887802741321, 'z': 1.3333333333333333},
                              {'probability': 0.10564977366685527, 'z': -1.25},
                              {'probability': 0.9750021048517796, 'z': 1.96}],
 'normal_probability_between': [{'probability': 0.6826894921370857, 'z_a': -1.0, 'z_b': 1.0},
                                {'probability': 0.724757330504898, 'z_a': -0.6666666666666666, 'z_b': 2.0},
                                {'probability': 0.9500042097035591, 'z_a': -1.96, 'z_b': 1.96}],
 'one_way_anova': [{'df_between': 2,
                    'df_within': 9,
                    'eta_squared': 0.830954169797145,
                    'f': 22.12,
                    'ms_between': 46.083333333333336,
                    'ms_within': 2.0833333333333335,
                    'p_value': 0.00033575313917739546,
                    'ss_between': 92.16666666666667,
                    'ss_within': 18.75},
                   {'df_between': 2,
                    'df_within': 7,
                    'eta_squared': 0.7783905228758171,
                    'f': 12.29354838709678,
                    'ms_between': 1.2703333333333338,
                    'ms_within': 0.10333333333333332,
                    'p_value': 0.0051234104725530905,
                    'ss_between': 2.5406666666666675,
                    'ss_within': 0.7233333333333333}],
 'p_chart': [{'center': 0.045714285714285714, 'lcl': 0.0, 'sigma': 0.020886476437139503, 'ucl': 0.10837371502570423},
             {'center': 0.056666666666666664, 'lcl': 0.0, 'sigma': 0.032697264581477016, 'ucl': 0.15475846041109773}],
 'paired_t_test': [{'df': 4, 'mean_difference': 6.2, 'p_value': 0.0030701520818274476, 't': 6.3948117268045195},
                   {'df': 3,
                    'mean_difference': 0.2999999999999998,
                    'p_value': 0.06714609691852158,
                    't': 2.0380986614602685}],
 'parallel_system_reliability': [{'reliability': 0.9999}, {'reliability': 0.97}],
 'pearson_correlation': [{'n': 5,
                          'p_value': 0.12402706265755457,
                          'r': 0.7745966692414834,
                          'r_squared': 0.6000000000000001},
                         {'n': 6,
                          'p_value': 0.2796446570048715,
                          'r': 0.5298089018901744,
                          'r_squared': 0.28069747252207244}],
 'percentile': [{'percentile': 46.0}, {'percentile': 7.25}, {'percentile': 6.75}],
 'poisson_at_least': [{'probability': 0.9084218055563291}, {'probability': 0.14238644690422153},
                      {'probability': 1.0}],
 'poisson_at_most': [{'probability': 0.2381033055535444}, {'probability': 0.9347119029710464},
                     {'probability': 0.6065306597126334}],
 'poisson_exactly': [{'probability': 0.14652511110987348}, {'probability': 0.07709834987526794},
                     {'probability': 0.6065306597126334}],
 'population_standard_deviation': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.0, 'variance': 4.0},
                                   {'mean': 18.0, 'n': 8, 'standard_deviation': 4.898979485566356, 'variance': 24.0},
                                   {'mean': 2.0, 'n': 2, 'standard_deviation': 0.5, 'variance': 0.25}],
 'population_variance': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.0, 'variance': 4.0},
                         {'mean': 18.0, 'n': 8, 'standard_deviation': 4.898979485566356, 'variance': 24.0},
                         {'mean': 2.0, 'n': 2, 'standard_deviation': 0.5, 'variance': 0.25}],
 'power_one_sample_z': [{'power': 0.7819079987319799}, {'power': 0.7497337474667971}],
 'power_two_sample_z': [{'power': 0.49068556766996907}, {'power': 0.4187753078440144}],
 'probability_at_least_one': [{'none': 0.3276800000000001, 'probability': 0.67232},
                              {'none': 0.3486784401000001, 'probability': 0.6513215599000001},
                              {'none': 0.4822530864197532, 'probability': 0.5177469135802468}],
 'process_capability': [{'cp': 1.1666666666666654,
                         'cpk': 1.0000000000000022,
                         'cpl': 1.3333333333333284,
                         'cpu': 1.0000000000000022,
                         'ppm_out': 1381.569273463187},
                        {'cp': 1.1666666666666667,
                         'cpk': 1.0,
                         'cpl': 1.0,
                         'cpu': 1.3333333333333333,
                         'ppm_out': 1381.569273463216}],
 'proportion_z_test': [{'p_value': 0.10959858339911618, 'proportion': 0.58, 'z': 1.5999999999999992},
                       {'p_value': 0.03649902271505774, 'proportion': 0.375, 'z': 1.7928429140015907}],
 'randomized_block_anova': [{'df_block': 3,
                             'df_error': 6,
                             'df_treatment': 2,
                             'f_block': 9.250000000000002,
                             'f_treatment': 39.0,
                             'p_block': 0.011439899266339197,
                             'p_treatment': 0.0003644314868804664,
                             'ss_block': 12.333333333333336,
                             'ss_error': 2.6666666666666665,
                             'ss_treatment': 34.666666666666664},
                            {'df_block': 2,
                             'df_error': 6,
                             'df_treatment': 3,
                             'f_block': 62.68421052631568,
                             'f_treatment': 95.57894736842069,
                             'p_block': 9.52754066269349e-05,
                             'p_treatment': 1.868980779975043e-05,
                             'ss_block': 0.6616666666666671,
                             'ss_error': 0.03166666666666674,
                             'ss_treatment': 1.513333333333331}],
 'range': [{'maximum': 19.0, 'minimum': 4.0, 'range': 15.0}, {'maximum': 9.25, 'minimum': 1.0, 'range': 8.25}],
 'regression_prediction': [{'intercept': 0.16000000000000014,
                            'slope': 1.9799999999999998,
                            'y_hat': 7.089999999999999},
                           {'intercept': 52.8, 'slope': 5.8, 'y_hat': 78.89999999999999}],
 'sample_mean_probability_above': [{'probability': 0.15865525393145707, 'standard_error': 2.0, 'z': 1.0},
                                   {'probability': 0.8849303297782917, 'standard_error': 2.5, 'z': -1.2}],
 'sample_mean_probability_below': [{'probability': 0.8413447460685428, 'standard_error': 2.0, 'z': 1.0},
                                   {'probability': 0.1150696702217083, 'standard_error': 2.5, 'z': -1.2}],
 'sample_size_mean': [{'n': 217, 'n_exact': 216.08205866404455}, {'n': 189, 'n_exact': 188.72594776238125}],
 'sample_size_proportion': [{'n': 1068, 'n_exact': 1067.0718946372572}, {'n': 558, 'n_exact': 557.3313144857818}],
 'sample_size_two_means': [{'n_exact': 62.79088407460958, 'n_per_group': 63},
                           {'n_exact': 190.30771890373285, 'n_per_group': 191}],
 'sample_standard_deviation': [{'mean': 5.0,
                                'n': 8,
                                'standard_deviation': 2.138089935299395,
                                'variance': 4.571428571428571},
                               {'mean': 18.0,
                                'n': 8,
                                'standard_deviation': 5.237229365663817,
                                'variance': 27.428571428571427},
                               {'mean': 2.0, 'n': 2, 'standard_deviation': 0.7071067811865476, 'variance': 0.5}],
 'sample_variance': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.138089935299395, 'variance': 4.571428571428571},
                     {'mean': 18.0, 'n': 8, 'standard_deviation': 5.237229365663817, 'variance': 27.428571428571427},
                     {'mean': 2.0, 'n': 2, 'standard_deviation': 0.7071067811865476, 'variance': 0.5}],
 'series_system_reliability': [{'reliability': 0.8379}, {'reliability': 0.68}],
 'sign_test': [{'n': 8, 'p_value': 0.7265625, 'positives': 5}, {'n': 8, 'p_value': 0.14453125, 'positives': 6}],
 'simple_linear_regression': [{'intercept': 0.16000000000000014,
                               'intercept_se': 0.1432945684013642,
                               'p_value': 2.2873435857216702e-05,
                               'r': 0.9992865514048735,
                               'r_squared': 0.9985736118186449,
                               'residual_se': 0.13662601021279436,
                               'slope': 1.9799999999999998,
                               'slope_se': 0.04320493798938564},
                              {'intercept': 52.8,
                               'intercept_se': 0.9797958971132714,
                               'p_value': 0.0001384240864952827,
                               'r': 0.9976303284229832,
                               'r_squared': 0.9952662721893492,
                               'residual_se': 0.7302967433402217,
                               'slope': 5.8,
                               'slope_se': 0.23094010767585035}],
 'skewness_kurtosis': [{'excess_kurtosis': 1.372063741027569,
                        'excess_kurtosis_biased': -0.1783067745718463,
                        'skewness': 1.3998537567794695,
                        'skewness_biased': 1.0800106206404578},
                       {'excess_kurtosis': 5.6691358024691345,
                        'excess_kurtosis_biased': 2.0329218106995883,
                        'skewness': 2.240256043894075,
                        'skewness_biased': 1.7962008374788356}],
 'spearman_correlation': [{'n': 5, 'p_value': 0.037386073468498635, 'rho': 0.9},
                          {'n': 6, 'p_value': 0.7996973387806547, 'rho': -0.13434332265596974}],
 'standard_error_of_mean': [{'n': 5, 'standard_deviation': 2.7386127875258306, 'standard_error': 1.224744871391589},
                            {'n': 6,
                             'standard_deviation': 0.25819888974716104,
                             'standard_error': 0.10540925533894595}],
 't_critical_value': [{'critical_value': 2.228138851986274}, {'critical_value': 1.724718242920786},
                      {'critical_value': 63.65674116287147}],
 't_test_one_sample': [{'df': 5,
                        'mean': 5.166666666666667,
                        'p_value': 0.11738679514629244,
                        'standard_deviation': 0.2160246899469286,
                        't': 1.88982236504614},
                       {'df': 5,
                        'mean': 97.83333333333333,
                        'p_value': 0.04281080263985618,
                        'standard_deviation': 2.48327740429189,
                        't': -2.1371868349696492}],
 't_test_summary_statistics': [{'df': 25, 'difference': 3, 'p_value': 0.0952738392963551, 't': 1.733785460790376},
                               {'df': 42.919075023316495,
                                'difference': 0.3999999999999999,
                                'p_value': 0.06858367578390201,
                                't': 1.514745875360486}],
 't_test_two_sample_pooled': [{'df': 8,
                               'difference': 3.8000000000000007,
                               'p_value': 0.0347658418981868,
                               't': 2.5389817981680323},
                              {'df': 7,
                               'difference': 0.9499999999999993,
                               'p_value': 0.003005851509462015,
                               't': 3.885303006255516}],
 'trimmed_mean': [{'removed_each_end': 1, 'trimmed_mean': 7.625}, {'removed_each_end': 2, 'trimmed_mean': 5.75}],
 'two_proportion_z_test': [{'difference': 0.15000000000000002,
                            'p_value': 0.02845973691631057,
                            'pooled_proportion': 0.375,
                            'z': 2.1908902300206647},
                           {'difference': 0.06315789473684211,
                            'p_value': 0.023422736398744162,
                            'pooled_proportion': 0.2692307692307692,
                            'z': 1.9876946336070143}],
 'weibull_reliability': [{'hazard_rate': 0.001,
                          'mttf': 886.226925452758,
                          'reliability': 0.7788007830714049,
                          'unreliability': 0.22119921692859512},
                         {'hazard_rate': 0.0005809475019311126,
                          'mttf': 1805.4905859018672,
                          'reliability': 0.6282871313089982,
                          'unreliability': 0.37171286869100184}],
 'weighted_mean': [{'total_weight': 6.0, 'weighted_mean': 83.33333333333333},
                   {'total_weight': 9.0, 'weighted_mean': 3.533333333333333}],
 'welch_t_test': [{'df': 7.174149270803547,
                   'difference': 3.8000000000000007,
                   'p_value': 0.03794551168052193,
                   't': 2.538981798168032},
                  {'df': 5.4881209503239745,
                   'difference': 0.9499999999999993,
                   'p_value': 0.005600027598973452,
                   't': 3.7503246612706818}],
 'wilcoxon_signed_rank': [{'n': 8, 'p_value': 0.4008141693829345, 'r_plus': 24.0, 'statistic': 12.0},
                          {'n': 7, 'p_value': 0.04548447398767889, 'r_plus': 24.0, 'statistic': 24.0}],
 'wilcoxon_signed_rank_exact': [{'n': 8, 'p_value': 0.4609375, 'r_plus': 24.0, 'statistic': 12.0},
                                {'n': 7, 'p_value': 0.0546875, 'r_plus': 24.0, 'statistic': 24.0}],
 'wilson_interval': [{'center': 0.20639380531848678,
                      'lower': 0.052368745896216595,
                      'proportion': 0.15,
                      'upper': 0.36041886474075696},
                     {'center': 0.07640385230507012, 'lower': 0.0, 'proportion': 0.0, 'upper': 0.15280770461014023}],
 'xbar_r_chart': [{'A2': 0.729,
                   'D3': 0.0,
                   'D4': 2.282,
                   'center_xbar': 10.025,
                   'mean_range': 0.39999999999999974,
                   'r_lcl': 0.0,
                   'r_ucl': 0.9127999999999994,
                   'xbar_lcl': 9.733400000000001,
                   'xbar_ucl': 10.3166},
                  {'A2': 1.023,
                   'D3': 0.0,
                   'D4': 2.574,
                   'center_xbar': 5.333333333333333,
                   'mean_range': 1.75,
                   'r_lcl': 0.0,
                   'r_ucl': 4.5045,
                   'xbar_lcl': 3.543083333333333,
                   'xbar_ucl': 7.1235833333333325}],
 'xbar_s_chart': [{'A3': 1.628,
                   'B3': 0.0,
                   'B4': 2.266,
                   'center_xbar': 10.025,
                   'mean_sd': 0.17196888252883402,
                   's_lcl': 0.0,
                   's_ucl': 0.3896814878103379,
                   'xbar_lcl': 9.745034659243059,
                   'xbar_ucl': 10.304965340756942},
                  {'A3': 1.954,
                   'B3': 0.0,
                   'B4': 2.568,
                   'center_xbar': 5.333333333333333,
                   'mean_sd': 0.8943375672974064,
                   's_lcl': 0.0,
                   's_ucl': 2.29665887281974,
                   'xbar_lcl': 3.585797726834201,
                   'xbar_ucl': 7.080868939832465}],
 'z_critical_value': [{'critical_value': 1.959963984540054}, {'critical_value': 1.6448536269514724},
                      {'critical_value': -2.3263478740408408}],
 'z_score': [{'z': 1.5}, {'z': -2.0}],
 'z_test_mean': [{'p_value': 0.016395071849192273, 'z': 2.4}, {'p_value': 0.04005915686381709, 'z': 1.75}]}
_QUESTION_VALUES = {'bartlett_test': [{'df': 2, 'p_value': 0.013445540650031874, 'statistic': 8.618215556185413},
                   {'df': 1, 'p_value': 0.004316807338806683, 'statistic': 8.145503254971933}, None],
 'binomial_at_least': [{'probability': 0.8327102464000001}, {'probability': 0.5851584974698203}, None, None],
 'binomial_at_most': [{'probability': 0.3822806015999999}, {'probability': 0.617172654387105}, None, None],
 'binomial_exactly': [{'probability': 0.21499084799999993}, {'probability': 0.20233115185692457}, None, None],
 'binomial_mean_sd': [{'mean': 15.0, 'standard_deviation': 3.24037034920393, 'variance': 10.5},
                      {'mean': 6.0, 'standard_deviation': 1.7320508075688772, 'variance': 3.0}, None],
 'c_chart': [{'center': 4.25, 'lcl': 0.0, 'ucl': 10.43465843842649},
             {'center': 10.833333333333334, 'lcl': 0.9591245042675833, 'ucl': 20.707542162399086}, None],
 'chi_square_critical_value': [{'critical_value': 9.487729036781154, 'left_critical_value': 0.7107230213973241},
                               {'critical_value': 23.20925115895436, 'left_critical_value': 2.558212160187205},
                               None],
 'chi_square_goodness_of_fit': [{'chi_square': 2.9,
                                 'df': 4,
                                 'expected': [20.0, 20.0, 20.0, 20.0, 20.0],
                                 'p_value': 0.5746972058298042},
                                {'chi_square': 0.0, 'df': 2, 'expected': [50.0, 30.0, 20.0], 'p_value': 1.0}, None],
 'chi_square_independence': [{'chi_square': 1.0101010101010102,
                              'df': 1,
                              'expected': [[22.5, 27.5], [22.5, 27.5]],
                              'p_value': 0.31487864133641985},
                             {'chi_square': 4.5021645021645025,
                              'df': 2,
                              'expected': [[15.0, 17.5, 27.5], [15.0, 17.5, 27.5]],
                              'p_value': 0.10528521784009053},
                             None, None],
 'chi_square_variance_test': [{'chi_square': 6.666666666666667,
                               'df': 5,
                               'p_value': 0.49326830437210445,
                               'sample_variance': 0.12},
                              {'chi_square': 12.678571428571429,
                               'df': 6,
                               'p_value': 0.04843390567356849,
                               'sample_variance': 33.80952380952381},
                              None],
 'ci_mean_known_sigma': [{'critical_value': 1.959963984540054,
                          'lower': 49.38671468727993,
                          'margin_of_error': 2.613285312720072,
                          'upper': 54.61328531272007},
                         {'critical_value': 2.575829303548901,
                          'lower': 114.53584089684465,
                          'margin_of_error': 5.464159103155349,
                          'upper': 125.46415910315535},
                         None, None],
 'ci_mean_t': [{'critical_value': 2.570581835636313,
                'df': 5,
                'lower': 11.623702162510888,
                'margin_of_error': 2.709631170822446,
                'mean': 14.333333333333334,
                'upper': 17.04296450415578},
               {'critical_value': 2.1318467863266504,
                'df': 4,
                'lower': 4.836611605245187,
                'margin_of_error': 0.18338839475481358,
                'mean': 5.0200000000000005,
                'upper': 5.203388394754814},
               None, None],
 'ci_mean_t_summary': [{'critical_value': 2.131449545559774,
                        'df': 15,
                        'lower': 22.868550454440225,
                        'margin_of_error': 2.131449545559774,
                        'upper': 27.131449545559775},
                       {'critical_value': 3.249835541592125,
                        'df': 9,
                        'lower': 2.686155883380101,
                        'margin_of_error': 0.5138441166198991,
                        'upper': 3.7138441166198994},
                       None],
 'ci_proportion': [{'lower': 0.3123477459423419,
                    'margin_of_error': 0.08765225405765815,
                    'proportion': 0.4,
                    'upper': 0.48765225405765816},
                   {'lower': 0.5682020434044758,
                    'margin_of_error': 0.0557979565955242,
                    'proportion': 0.624,
                    'upper': 0.6797979565955242},
                   None],
 'ci_two_means': [{'df': 7.174149270803547,
                   'difference': 3.8000000000000007,
                   'lower': 0.2782954860255704,
                   'standard_error': 1.4966629547095767,
                   'upper': 7.3217045139744314},
                  {'df': 7,
                   'difference': 0.9499999999999993,
                   'lower': 0.4867543607441512,
                   'standard_error': 0.24451117415307266,
                   'upper': 1.4132456392558475},
                  None],
 'ci_variance': [{'lower': 2.5975708672276663,
                  'sd_lower': 1.6116981315456274,
                  'sd_upper': 6.332622088215267,
                  'upper': 40.10210251215189,
                  'variance': 6.666666666666666},
                 {'lower': 0.015599096414563175,
                  'sd_lower': 0.12489634267889183,
                  'sd_upper': 0.45633172855158494,
                  'upper': 0.20823864648287738,
                  'variance': 0.03699999999999996},
                 None],
 'coefficient_of_variation': [{'cv': 0.22587697572631282,
                               'cv_percent': 22.58769757263128,
                               'mean': 14.0,
                               'standard_deviation': 3.1622776601683795},
                              {'cv': 0.07615773105863909,
                               'cv_percent': 7.6157731058639095,
                               'mean': 50.0,
                               'standard_deviation': 3.8078865529319543},
                              None],
 'cohens_d': [{'d': 1.6057930839841819, 'hedges_g': 1.4503937532760351, 'pooled_sd': 2.3664319132398464},
              {'d': 2.6063404905514878, 'hedges_g': 2.3167471027124336, 'pooled_sd': 0.36449573777637334}, None],
 'correlation_confidence_interval': [{'lower': 0.5591491951264971,
                                      'r': 0.9428571428571428,
                                      'upper': 0.9938998355880978},
                                     {'lower': -0.34501780003129634,
                                      'r': 0.5298089018901744,
                                      'upper': 0.9120423900566206},
                                     None],
 'correlation_t_test': [{'df': 28, 'p_value': 0.012591071275196835, 't': 2.666405420850055},
                        {'df': 48, 'p_value': 0.011742763163910696, 't': -2.340071757206818}, None],
 'covariance': [{'population_covariance': 2.75, 'sample_covariance': 3.6666666666666665},
                {'population_covariance': 1.222222222222222, 'sample_covariance': 1.8333333333333333}, None],
 'cronbach_alpha': [{'alpha': 0.9130434782608696, 'items': 3, 'respondents': 5},
                    {'alpha': 0.9629629629629629, 'items': 4, 'respondents': 4}, None],
 'discrete_random_variable': [{'expected_value': 1.7000000000000002, 'standard_deviation': 0.9, 'variance': 0.81},
                              {'expected_value': 17.5, 'standard_deviation': 4.330127018922194, 'variance': 18.75},
                              None],
 'durbin_watson': [{'durbin_watson': 2.53448275862069}, {'durbin_watson': 0.2706552706552706}, None],
 'exponential_probability_above': [{'probability': 0.22313016014842982}, {'probability': 0.6065306597126334}, None],
 'exponential_probability_below': [{'probability': 0.7768698398515702}, {'probability': 0.39346934028736663}, None],
 'exponential_reliability': [{'failure_rate': 0.0002,
                              'mttf': 5000.0,
                              'reliability': 0.8187307530779818,
                              'unreliability': 0.18126924692201812},
                             {'failure_rate': 0.0002,
                              'mttf': 5000.0,
                              'reliability': 0.8521437889662113,
                              'unreliability': 0.14785621103378865},
                             None],
 'f_critical_value': [{'critical_value': 3.0983912121407844}, {'critical_value': 5.636326187669079}, None],
 'f_test_variances': [{'df1': 5, 'df2': 4, 'f': 9.714285714285714, 'p_value': 0.04685703619078499},
                      {'df1': 4, 'df2': 3, 'f': 70.02000000000012, 'p_value': 0.0027101153548101764}, None],
 'factorial_2x2_effects': [{'A': 10.5, 'AB': 2.5, 'B': -7.5, 'grand_mean': 28.25},
                           {'A': 15.0, 'AB': 5.0, 'B': 10.0, 'grand_mean': 30.0}, None],
 'factorial_2x2x2_effects': [{'A': 1.25,
                              'AB': -1.75,
                              'ABC': -6.75,
                              'AC': -13.75,
                              'B': 12.75,
                              'BC': -5.25,
                              'C': 9.75,
                              'grand_mean': 40.875},
                             {'A': 52.0,
                              'AB': -12.0,
                              'ABC': 24.5,
                              'AC': -30.5,
                              'B': -1.0,
                              'BC': -18.5,
                              'C': 9.5,
                              'grand_mean': 623.0},
                             None],
 'fisher_exact_test': [{'odds_ratio': 20.0, 'p_value': 0.03496503496503497},
                       {'odds_ratio': 0.047619047619047616, 'p_value': 0.009883305548940223}, None],
 'five_number_summary': [{'iqr': 7.0, 'maximum': 15.0, 'median': 8.0, 'minimum': 1.0, 'q1': 4.5, 'q3': 11.5},
                         {'iqr': 28.0, 'maximum': 49.0, 'median': 40.0, 'minimum': 6.0, 'q1': 15.0, 'q3': 43.0},
                         {'iqr': 2.5, 'maximum': 8.0, 'median': 5.0, 'minimum': 2.0, 'q1': 4.0, 'q3': 6.5}],
 'frequency_table_statistics': [{'mean': 2.65,
                                 'n': 20,
                                 'population_standard_deviation': 0.9630680142129111,
                                 'sample_standard_deviation': 0.9880869341680844},
                                {'mean': 21.0,
                                 'n': 10,
                                 'population_standard_deviation': 7.0,
                                 'sample_standard_deviation': 7.378647873726218},
                                None],
 'friedman_test': [{'chi_square': 0.4000000000000057, 'df': 2, 'p_value': 0.8187307530779795},
                   {'chi_square': 8.200000000000003, 'df': 3, 'p_value': 0.04205418289496617}, None],
 'geometric_mean': [{'geometric_mean': 4.0}, {'geometric_mean': 1.071202823901392}, None],
 'geometric_probability': [{'cumulative': 0.5904, 'probability': 0.1024}, {'cumulative': 0.875, 'probability': 0.125},
                           None],
 'harmonic_mean': [{'harmonic_mean': 47.99999999999999}, {'harmonic_mean': 1.7142857142857142}, None],
 'hypergeometric_probability': [{'cumulative': 0.9517396968037909, 'probability': 0.20983971757065453},
                                {'cumulative': 0.9422084623323014, 'probability': 0.23839009287925697}, None, None],
 'individuals_moving_range_chart': [{'center': 10.085714285714285,
                                     'mean_moving_range': 0.4666666666666668,
                                     'mr_lcl': 0.0,
                                     'mr_ucl': 1.5246000000000004,
                                     'x_lcl': 8.844380952380952,
                                     'x_ucl': 11.327047619047619},
                                    {'center': 51.5,
                                     'mean_moving_range': 4.0,
                                     'mr_lcl': 0.0,
                                     'mr_ucl': 13.068,
                                     'x_lcl': 40.86,
                                     'x_ucl': 62.14},
                                    None],
 'inverse_normal': [{'x': 119.22327348316901, 'z': 1.2815515655446006},
                    {'x': 66.44853626951473, 'z': 1.6448536269514724},
                    {'x': 664.4853626951473, 'z': 1.6448536269514724}, None],
 'iqr_outliers': [{'iqr': 1.75,
                   'lower_fence': 9.375,
                   'outliers': [102.0],
                   'q1': 12.0,
                   'q3': 13.75,
                   'upper_fence': 16.375},
                  {'iqr': 2.5, 'lower_fence': 0.0, 'outliers': [30.0], 'q1': 3.75, 'q3': 6.25, 'upper_fence': 10.0},
                  None],
 'k_out_of_n_reliability': [{'reliability': 0.9720000000000001}, {'reliability': 0.9733881249999999}, None],
 'kaplan_meier': [{'event_times': [6.0, 10.0, 13.0, 16.0, 22.0, 23.0],
                   'survival': 0.5333333333333334,
                   'survival_curve': [0.6666666666666667, 0.5333333333333334, 0.4000000000000001, 0.2666666666666668,
                                      0.1333333333333334, 0.0]},
                  {'event_times': [2.0, 3.0, 5.0, 9.0, 12.0],
                   'survival': 0.5357142857142858,
                   'survival_curve': [0.8571428571428572, 0.7142857142857144, 0.5357142857142858, 0.2678571428571429,
                                      0.0]},
                  None],
 'kendall_tau': [{'concordant': 12, 'discordant': 3, 'p_value': 0.09087393998624908, 'tau': 0.6},
                 {'concordant': 2, 'discordant': 6, 'p_value': 0.28274545993277467, 'tau': -0.47140452079103173},
                 None],
 'kruskal_wallis': [{'df': 2, 'h': 2.853801169590639, 'p_value': 0.24005179059000692},
                    {'df': 2, 'h': 7.318181818181813, 'p_value': 0.02575591655624027}, None],
 'levene_test': [{'df1': 2, 'df2': 12, 'p_value': 0.0266649501869476, 'w': 4.977272727272727},
                 {'df1': 1, 'df2': 6, 'p_value': 0.008582732014852098, 'w': 14.72727272727274}, None],
 'log_rank_test': [{'chi_square': 13.81148342629696,
                    'expected_1': 10.422222222222222,
                    'observed_1': 6.0,
                    'p_value': 0.00020209722320530527},
                   {'chi_square': 0.5818837585002068,
                    'expected_1': 3.8380952380952382,
                    'observed_1': 3.0,
                    'p_value': 0.445574893590188},
                   None],
 'mann_whitney_exact': [{'p_value': 0.031746031746031744, 'u': 19.0}, {'p_value': 0.015873015873015872, 'u': 19.0},
                        None],
 'mann_whitney_u': [{'p_value': 0.037336415920662905, 'u': 19.0}, {'p_value': 0.018668207960331452, 'u': 19.0}, None],
 'mcnemar_test': [{'chi_square': 2.1176470588235294,
                   'discordant': 17,
                   'exact_p_value': 0.143463134765625,
                   'p_value': 0.1456100953968671},
                  {'chi_square': 2.7,
                   'discordant': 30,
                   'exact_p_value': 0.09873714670538902,
                   'p_value': 0.10034824646229112},
                  None],
 'mean': [{'mean': 5.0, 'n': 3, 'sum': 15.0}, {'mean': 14.0, 'n': 5, 'sum': 70.0},
          {'mean': 4.375, 'n': 4, 'sum': 17.5}, None, None],
 'mean_absolute_deviation': [{'mean': 5.0, 'mean_absolute_deviation': 2.0},
                             {'mean': 25.0, 'mean_absolute_deviation': 12.0}],
 'median': [{'median': 5.0, 'n': 5}, {'median': 15.5, 'n': 6}, None],
 'median_absolute_deviation': [{'median': 2.0, 'median_absolute_deviation': 1.0},
                               {'median': 13.0, 'median_absolute_deviation': 2.0}],
 'mode': [{'frequency': 3, 'has_mode': 'yes', 'modes': [7.0]},
          {'frequency': 2, 'has_mode': 'yes', 'modes': [2.0, 3.0]}, {'frequency': 1, 'has_mode': 'no', 'modes': []}],
 'multiple_linear_regression': [{'adj_r_squared': 0.989430894308943,
                                 'coefficients': [6.229166666666668, 3.0625, 0.06249999999999978],
                                 'f': 235.03846153846135,
                                 'f_p_value': 0.0005049916961929541,
                                 'r_squared': 0.9936585365853658,
                                 'residual_se': 0.6009252125773318},
                                {'adj_r_squared': 0.9979186837986387,
                                 'coefficients': [3.5407917383821115, 1.937349397590363, 0.25941480206540407],
                                 'f': 1439.395592864642,
                                 'f_p_value': 1.925278724466295e-06,
                                 'r_squared': 0.9986124558657591,
                                 'residual_se': 0.4050031342179138},
                                None],
 'negative_binomial_probability': [{'probability': 0.12441600000000003}, {'probability': 0.12500000000000003}, None],
 'normal_probability_above': [{'probability': 0.022750131948179216, 'z': 2.0},
                              {'probability': 0.06680720126885809, 'z': 1.5},
                              {'probability': 0.006209665325776139, 'z': 2.5}, None],
 'normal_probability_below': [{'probability': 0.9087887802741321, 'z': 1.3333333333333333},
                              {'probability': 0.10564977366685527, 'z': -1.25},
                              {'probability': 0.9750021048517796, 'z': 1.96}, None, None],
 'normal_probability_between': [{'probability': 0.6826894921370857, 'z_a': -1.0, 'z_b': 1.0},
                                {'probability': 0.724757330504898, 'z_a': -0.6666666666666666, 'z_b': 2.0},
                                {'probability': 0.8185946141203638, 'z_a': -1.0, 'z_b': 2.0}, None],
 'one_way_anova': [{'df_between': 2,
                    'df_within': 9,
                    'eta_squared': 0.830954169797145,
                    'f': 22.12,
                    'ms_between': 46.083333333333336,
                    'ms_within': 2.0833333333333335,
                    'p_value': 0.00033575313917739546,
                    'ss_between': 92.16666666666667,
                    'ss_within': 18.75},
                   {'df_between': 2,
                    'df_within': 7,
                    'eta_squared': 0.7783905228758171,
                    'f': 12.29354838709678,
                    'ms_between': 1.2703333333333338,
                    'ms_within': 0.10333333333333332,
                    'p_value': 0.0051234104725530905,
                    'ss_between': 2.5406666666666675,
                    'ss_within': 0.7233333333333333},
                   None, None],
 'p_chart': [{'center': 0.045714285714285714, 'lcl': 0.0, 'sigma': 0.020886476437139503, 'ucl': 0.10837371502570423},
             {'center': 0.056666666666666664, 'lcl': 0.0, 'sigma': 0.032697264581477016, 'ucl': 0.15475846041109773},
             None, None],
 'paired_t_test': [{'df': 4, 'mean_difference': 6.2, 'p_value': 0.0030701520818274476, 't': 6.3948117268045195},
                   {'df': 3,
                    'mean_difference': 0.2999999999999998,
                    'p_value': 0.06714609691852158,
                    't': 2.0380986614602685},
                   None],
 'parallel_system_reliability': [{'reliability': 0.9999}, {'reliability': 0.97}, None],
 'pearson_correlation': [{'n': 5,
                          'p_value': 0.12402706265755457,
                          'r': 0.7745966692414834,
                          'r_squared': 0.6000000000000001},
                         {'n': 6,
                          'p_value': 0.2796446570048715,
                          'r': 0.5298089018901744,
                          'r_squared': 0.28069747252207244},
                         None, None],
 'percentile': [{'percentile': 46.0}, {'percentile': 7.25}, None],
 'poisson_at_least': [{'probability': 0.9084218055563291}, {'probability': 0.14238644690422153}, None, None],
 'poisson_at_most': [{'probability': 0.2381033055535444}, {'probability': 0.9347119029710464}, None, None],
 'poisson_exactly': [{'probability': 0.14652511110987348}, {'probability': 0.07709834987526794}, None, None],
 'population_standard_deviation': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.0, 'variance': 4.0},
                                   {'mean': 18.0, 'n': 8, 'standard_deviation': 4.898979485566356, 'variance': 24.0},
                                   {'mean': 394.0,
                                    'n': 5,
                                    'standard_deviation': 147.32277488562318,
                                    'variance': 21704.0},
                                   None],
 'population_variance': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.0, 'variance': 4.0},
                         {'mean': 18.0, 'n': 8, 'standard_deviation': 4.898979485566356, 'variance': 24.0},
                         {'mean': 394.0, 'n': 5, 'standard_deviation': 147.32277488562318, 'variance': 21704.0},
                         None],
 'power_one_sample_z': [{'power': 0.7819079987319799}, {'power': 0.7497337474667971}, None],
 'power_two_sample_z': [{'power': 0.49068556766996907}, {'power': 0.4187753078440144}, None],
 'probability_at_least_one': [{'none': 0.3276800000000001, 'probability': 0.67232},
                              {'none': 0.3486784401000001, 'probability': 0.6513215599000001}, None, None],
 'process_capability': [{'cp': 1.1666666666666654,
                         'cpk': 1.0000000000000022,
                         'cpl': 1.3333333333333284,
                         'cpu': 1.0000000000000022,
                         'ppm_out': 1381.569273463187},
                        {'cp': 1.1666666666666667,
                         'cpk': 1.0,
                         'cpl': 1.0,
                         'cpu': 1.3333333333333333,
                         'ppm_out': 1381.569273463216},
                        None, None],
 'proportion_z_test': [{'p_value': 0.10959858339911618, 'proportion': 0.58, 'z': 1.5999999999999992},
                       {'p_value': 0.03649902271505774, 'proportion': 0.375, 'z': 1.7928429140015907}, None],
 'randomized_block_anova': [{'df_block': 3,
                             'df_error': 6,
                             'df_treatment': 2,
                             'f_block': 9.250000000000002,
                             'f_treatment': 39.0,
                             'p_block': 0.011439899266339197,
                             'p_treatment': 0.0003644314868804664,
                             'ss_block': 12.333333333333336,
                             'ss_error': 2.6666666666666665,
                             'ss_treatment': 34.666666666666664},
                            {'df_block': 2,
                             'df_error': 6,
                             'df_treatment': 3,
                             'f_block': 62.68421052631568,
                             'f_treatment': 95.57894736842069,
                             'p_block': 9.52754066269349e-05,
                             'p_treatment': 1.868980779975043e-05,
                             'ss_block': 0.6616666666666671,
                             'ss_error': 0.03166666666666674,
                             'ss_treatment': 1.513333333333331},
                            None],
 'range': [{'maximum': 19.0, 'minimum': 4.0, 'range': 15.0}, {'maximum': 9.25, 'minimum': 1.0, 'range': 8.25}],
 'regression_prediction': [{'intercept': 0.16000000000000014,
                            'slope': 1.9799999999999998,
                            'y_hat': 7.089999999999999},
                           {'intercept': 52.8, 'slope': 5.8, 'y_hat': 78.89999999999999}, None],
 'sample_mean_probability_above': [{'probability': 0.15865525393145707, 'standard_error': 2.0, 'z': 1.0},
                                   {'probability': 0.8849303297782917, 'standard_error': 2.5, 'z': -1.2}, None],
 'sample_mean_probability_below': [{'probability': 0.8413447460685428, 'standard_error': 2.0, 'z': 1.0},
                                   {'probability': 0.1150696702217083, 'standard_error': 2.5, 'z': -1.2}, None],
 'sample_size_mean': [{'n': 217, 'n_exact': 216.08205866404455}, {'n': 189, 'n_exact': 188.72594776238125}, None],
 'sample_size_proportion': [{'n': 1068, 'n_exact': 1067.0718946372572}, {'n': 558, 'n_exact': 557.3313144857818},
                            None],
 'sample_size_two_means': [{'n_exact': 62.79088407460958, 'n_per_group': 63},
                           {'n_exact': 190.30771890373285, 'n_per_group': 191}, None],
 'sample_standard_deviation': [{'mean': 5.0,
                                'n': 8,
                                'standard_deviation': 2.138089935299395,
                                'variance': 4.571428571428571},
                               {'mean': 18.0,
                                'n': 8,
                                'standard_deviation': 5.237229365663817,
                                'variance': 27.428571428571427},
                               {'mean': 394.0, 'n': 5, 'standard_deviation': 164.7118696390761, 'variance': 27130.0},
                               None],
 'sample_variance': [{'mean': 5.0, 'n': 8, 'standard_deviation': 2.138089935299395, 'variance': 4.571428571428571},
                     {'mean': 18.0, 'n': 8, 'standard_deviation': 5.237229365663817, 'variance': 27.428571428571427},
                     {'mean': 394.0, 'n': 5, 'standard_deviation': 164.7118696390761, 'variance': 27130.0}, None],
 'series_system_reliability': [{'reliability': 0.8379}, {'reliability': 0.68}, None],
 'sign_test': [{'n': 8, 'p_value': 0.7265625, 'positives': 5}, {'n': 8, 'p_value': 0.14453125, 'positives': 6}, None],
 'simple_linear_regression': [{'intercept': 0.16000000000000014,
                               'intercept_se': 0.1432945684013642,
                               'p_value': 2.2873435857216702e-05,
                               'r': 0.9992865514048735,
                               'r_squared': 0.9985736118186449,
                               'residual_se': 0.13662601021279436,
                               'slope': 1.9799999999999998,
                               'slope_se': 0.04320493798938564},
                              {'intercept': 52.8,
                               'intercept_se': 0.9797958971132714,
                               'p_value': 0.0001384240864952827,
                               'r': 0.9976303284229832,
                               'r_squared': 0.9952662721893492,
                               'residual_se': 0.7302967433402217,
                               'slope': 5.8,
                               'slope_se': 0.23094010767585035},
                              None],
 'skewness_kurtosis': [{'excess_kurtosis': 1.372063741027569,
                        'excess_kurtosis_biased': -0.1783067745718463,
                        'skewness': 1.3998537567794695,
                        'skewness_biased': 1.0800106206404578},
                       {'excess_kurtosis': 5.6691358024691345,
                        'excess_kurtosis_biased': 2.0329218106995883,
                        'skewness': 2.240256043894075,
                        'skewness_biased': 1.7962008374788356},
                       None],
 'spearman_correlation': [{'n': 5, 'p_value': 0.037386073468498635, 'rho': 0.9},
                          {'n': 6, 'p_value': 0.7996973387806547, 'rho': -0.13434332265596974}, None],
 'standard_error_of_mean': [{'n': 5, 'standard_deviation': 2.7386127875258306, 'standard_error': 1.224744871391589},
                            {'n': 6,
                             'standard_deviation': 0.25819888974716104,
                             'standard_error': 0.10540925533894595},
                            None],
 't_critical_value': [{'critical_value': 2.228138851986274}, {'critical_value': 1.724718242920786},
                      {'critical_value': 63.65674116287147}, None],
 't_test_one_sample': [{'df': 5,
                        'mean': 5.166666666666667,
                        'p_value': 0.11738679514629244,
                        'standard_deviation': 0.2160246899469286,
                        't': 1.88982236504614},
                       {'df': 5,
                        'mean': 97.83333333333333,
                        'p_value': 0.04281080263985618,
                        'standard_deviation': 2.48327740429189,
                        't': -2.1371868349696492},
                       None, None],
 't_test_summary_statistics': [{'df': 25, 'difference': 3.0, 'p_value': 0.0952738392963551, 't': 1.733785460790376},
                               {'df': 42.919075023316495,
                                'difference': 0.3999999999999999,
                                'p_value': 0.06858367578390201,
                                't': 1.514745875360486},
                               None],
 't_test_two_sample_pooled': [{'df': 8,
                               'difference': 3.8000000000000007,
                               'p_value': 0.0347658418981868,
                               't': 2.5389817981680323},
                              {'df': 7,
                               'difference': 0.9499999999999993,
                               'p_value': 0.00601170301892403,
                               't': 3.885303006255516},
                              {'df': 4,
                               'difference': 0.7333333333333334,
                               'p_value': 0.02739354815874979,
                               't': 2.6877277758387175},
                              None],
 'trimmed_mean': [{'removed_each_end': 1, 'trimmed_mean': 7.625}, {'removed_each_end': 2, 'trimmed_mean': 5.75}, None,
                  None],
 'two_proportion_z_test': [{'difference': 0.15000000000000002,
                            'p_value': 0.02845973691631057,
                            'pooled_proportion': 0.375,
                            'z': 2.1908902300206647},
                           {'difference': 0.06315789473684211,
                            'p_value': 0.023422736398744162,
                            'pooled_proportion': 0.2692307692307692,
                            'z': 1.9876946336070143},
                           None, None],
 'weibull_reliability': [{'hazard_rate': 0.001,
                          'mttf': 886.226925452758,
                          'reliability': 0.7788007830714049,
                          'unreliability': 0.22119921692859512},
                         {'hazard_rate': 0.0005809475019311126,
                          'mttf': 1805.4905859018672,
                          'reliability': 0.6282871313089982,
                          'unreliability': 0.37171286869100184},
                         None, None],
 'weighted_mean': [{'total_weight': 6.0, 'weighted_mean': 83.33333333333333},
                   {'total_weight': 9.0, 'weighted_mean': 3.533333333333333}, None, None],
 'welch_t_test': [{'df': 7.174149270803547,
                   'difference': 3.8000000000000007,
                   'p_value': 0.03794551168052193,
                   't': 2.538981798168032},
                  {'df': 5.4881209503239745,
                   'difference': 0.9499999999999993,
                   'p_value': 0.011200055197946904,
                   't': 3.7503246612706818},
                  {'df': 3.8950108459869854,
                   'difference': 0.7333333333333334,
                   'p_value': 0.02817725246976119,
                   't': 2.6877277758387175},
                  None],
 'wilcoxon_signed_rank': [{'n': 8, 'p_value': 0.4008141693829345, 'r_plus': 24.0, 'statistic': 12.0},
                          {'n': 7, 'p_value': 0.04548447398767889, 'r_plus': 24.0, 'statistic': 24.0}, None],
 'wilcoxon_signed_rank_exact': [{'n': 8, 'p_value': 0.4609375, 'r_plus': 24.0, 'statistic': 12.0},
                                {'n': 7, 'p_value': 0.0546875, 'r_plus': 24.0, 'statistic': 24.0}, None],
 'wilson_interval': [{'center': 0.20639380531848678,
                      'lower': 0.052368745896216595,
                      'proportion': 0.15,
                      'upper': 0.36041886474075696},
                     {'center': 0.07640385230507012, 'lower': 0.0, 'proportion': 0.0, 'upper': 0.15280770461014023},
                     None],
 'xbar_r_chart': [{'A2': 0.729,
                   'D3': 0.0,
                   'D4': 2.282,
                   'center_xbar': 10.025,
                   'mean_range': 0.39999999999999974,
                   'r_lcl': 0.0,
                   'r_ucl': 0.9127999999999994,
                   'xbar_lcl': 9.733400000000001,
                   'xbar_ucl': 10.3166},
                  {'A2': 1.023,
                   'D3': 0.0,
                   'D4': 2.574,
                   'center_xbar': 5.333333333333333,
                   'mean_range': 1.75,
                   'r_lcl': 0.0,
                   'r_ucl': 4.5045,
                   'xbar_lcl': 3.543083333333333,
                   'xbar_ucl': 7.1235833333333325},
                  None],
 'xbar_s_chart': [{'A3': 1.628,
                   'B3': 0.0,
                   'B4': 2.266,
                   'center_xbar': 10.025,
                   'mean_sd': 0.17196888252883402,
                   's_lcl': 0.0,
                   's_ucl': 0.3896814878103379,
                   'xbar_lcl': 9.745034659243059,
                   'xbar_ucl': 10.304965340756942},
                  {'A3': 1.954,
                   'B3': 0.0,
                   'B4': 2.568,
                   'center_xbar': 5.333333333333333,
                   'mean_sd': 0.8943375672974064,
                   's_lcl': 0.0,
                   's_ucl': 2.29665887281974,
                   'xbar_lcl': 3.585797726834201,
                   'xbar_ucl': 7.080868939832465},
                  None],
 'z_critical_value': [{'critical_value': 1.959963984540054}, {'critical_value': 1.6448536269514724},
                      {'critical_value': 2.575829303548901}, None],
 'z_score': [{'z': 1.5}, {'z': -2.0}, None, None],
 'z_test_mean': [{'p_value': 0.016395071849192273, 'z': 2.4}, {'p_value': 0.04005915686381709, 'z': 1.75}, None,
                 None]}
for _k, _outs in _EXAMPLE_VALUES.items():
    if _k in PROCEDURES and len(_outs) == len(PROCEDURES[_k]['examples']):
        for _e, _o in zip(PROCEDURES[_k]['examples'], _outs):
            _e['outputs'] = _o
for _k, _outs in _QUESTION_VALUES.items():
    if _k in PROCEDURES and len(_outs) == len(PROCEDURES[_k]['questions']):
        for _q, _o in zip(PROCEDURES[_k]['questions'], _outs):
            if _o is not None and 'outputs' in _q:
                _q['outputs'] = _o
# @@VALUES-END@@


# ==================================================================================================================
# the reading guard: every parser above is wrapped by the same rules (ask_sweep 2026-10-23: 11 wrong answers, all of
# them a stated planning estimate of the proportion left unread, so that the default p = 0.5 was used)
#   1. every stated number is bound: a reading that leaves a number of the question unused is refused (an unread
#      planning estimate, an unread level, a value the capability has no slot for). Numbers that are part of a name or
#      a notation (H0, mu0, n1, x2, 2^3, 2x2, "3rd") are not stated values; a percentage is its fraction ("95%" = 0.95,
#      "a confidence level of 95" = 0.95, as the level readers take it);
#   2. a number the reading took into two different inputs where the question writes it once is refused;
#   3. an exception inside a parser is a question the parser cannot read: it is refused, never an error.
# ==================================================================================================================
_S_UNUM = r"(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"
_S_QNUM = re.compile(r"(?<![\w.^])(" + _S_UNUM + r")(\s*%|\s*percent\b|\s*per cent\b)?(?![a-z_\d^])")
_S_NOTATION = re.compile(r"\d+\s*\^\s*\(?\s*\d+|\b\d+\s*x\s*\d+(?:\s*x\s*\d+)*\b|\b\d+\s*by\s*\d+\s+(?:table|design|"
                         r"factorial|contingency)")
# numbers that are part of a capability's own name (a fixed constant of the method, never a free parameter)
_S_NAMES = {"iqr_outliers": r"\b1\.5\s*(?:x|\*|times|×)?\s*(?:the\s+)?(?:iqr|interquartile)"}


def _s_stated(q: str, key: str = "") -> list:
    q = _S_NOTATION.sub(lambda m: " " * len(m.group(0)), q)
    if key in _S_NAMES:
        q = re.sub(_S_NAMES[key], lambda m: " " * len(m.group(0)), q)
    out = []
    for m in _S_QNUM.finditer(q):
        v = float(m.group(1))
        out.append((v / 100.0 if m.group(2) else v, bool(m.group(2))))
    return out


def _s_bound(parsed) -> list:
    nums = []

    def walk(x):
        if isinstance(x, bool) or x is None:
            return
        if isinstance(x, (int, float)):
            nums.append(abs(float(x)))
        elif isinstance(x, str):
            nums.extend(abs(float(t)) for t in re.findall(_S_UNUM, x.lower()))
        elif isinstance(x, dict):
            for v in x.values():
                walk(v)
        elif isinstance(x, (list, tuple)):
            for v in x:
                walk(v)
    walk(parsed)
    return nums


def _s_same(a, b):
    return abs(a - b) <= 1e-12 * max(1.0, abs(a), abs(b))


def _s_guarded(key, parse):
    def guarded(question):
        try:
            p = parse(question)
        except (ValueError, TypeError, ZeroDivisionError, OverflowError, IndexError, KeyError, ArithmeticError):
            return None
        if not p or "_out_of_scope" in p:
            return p
        q = _norm(question)
        bound = _s_bound(p)
        levels = [abs(float(p[k])) for k in ("confidence", "alpha", "level") if isinstance(p.get(k), (int, float))]
        probs = [float(p[k]) for k in ("p", "confidence", "alpha", "level") if isinstance(p.get(k), (int, float))
                 and 0.0 < float(p[k]) < 1.0]
        sizes = _s_sizes(p)
        for v, pct in _s_stated(q, key):
            if any(_s_same(v, b) for b in bound):
                continue
            if not pct and v > 1 and any(_s_same(v / 100.0, b) for b in levels):   # "confidence level of 95"
                continue
            if 0.0 < v < 1.0 and any(_s_same(1.0 - v, b) for b in probs):         # a tail read as its complement
                continue                                                           # ("top 5%" is p = 0.95)
            if v == int(v) and int(v) in sizes:          # a count that describes the data given ("4 treatments")
                continue
            return None
        return p
    return guarded


def _s_sizes(parsed) -> set:
    """The sizes of the data a reading took: list lengths, and the rows and columns of a table."""
    out = set()
    for x in parsed.values():
        if isinstance(x, str):
            t = x.strip()
            if t.startswith("[["):
                rows = re.findall(r"\[([^\[\]]*)\]", t)
                out.add(len(rows))
                out.update(len(re.findall(_S_UNUM, r)) for r in rows)
            else:
                out.add(len(re.findall(_S_UNUM, t)))
        elif isinstance(x, (list, tuple)):
            out.add(len(x))
            out.update(len(r) for r in x if isinstance(r, (list, tuple)))
    return out


for _k, _spec in PROCEDURES.items():
    _spec["parse"] = _s_guarded(_k, _spec["parse"])
