"""Paired-game statistics for the engine game test. No game rules live here.

Each paired opening gives one observation: the candidate's mean score over the
two games (0, .25, .5, .75 or 1). Pair scores keep the opening/color correlation
out of the variance (pentanomial model).
"""
import math
from dataclasses import dataclass

PAIR_SCORES = (0.0, 0.25, 0.5, 0.75, 1.0)


def expected_score(elo):
    """Logistic expected score for an Elo difference."""
    return 1.0 / (1.0 + 10.0 ** (-elo / 400.0))


def elo_from_score(score):
    """Inverse of expected_score; infinite at 0 or 1."""
    if score <= 0.0:
        return -math.inf
    if score >= 1.0:
        return math.inf
    return -400.0 * math.log10(1.0 / score - 1.0)


def pentanomial(pair_scores):
    """Counts of pair scores 0, .25, .5, .75, 1 (candidate perspective)."""
    counts = [0] * 5
    for s in pair_scores:
        index = round(s * 4)
        if abs(index / 4 - s) > 1e-9 or not 0 <= index <= 4:
            raise ValueError('pair score must be one of 0, .25, .5, .75, 1')
        counts[index] += 1
    return counts


def mean_variance(counts):
    n = sum(counts)
    if n == 0:
        return None, None
    mean = sum(c * s for c, s in zip(counts, PAIR_SCORES)) / n
    var = sum(c * (s - mean) ** 2 for c, s in zip(counts, PAIR_SCORES)) / n
    return mean, var


def regularized_variance(counts):
    """Pair-score variance with two symmetric prior pairs (scores .25 and .75).

    Draw-dominated small samples otherwise give a near-zero variance and an
    early, unjustified decision. The prior does not move the mean and its
    weight vanishes as pairs accumulate.
    """
    n = sum(counts)
    mean, var = mean_variance(counts)
    if mean is None:
        return None
    return (n * var + (0.25 - mean) ** 2 + (0.75 - mean) ** 2) / (n + 2)


def llr(counts, elo0, elo1):
    """Generalized SPRT log-likelihood ratio (normal approximation on pair scores).

    LLR = N (s1 - s0) (2 m - s0 - s1) / (2 v), with m the sample mean and v the
    regularized variance of pair scores. Returns 0 with no pairs.
    """
    n = sum(counts)
    mean, _ = mean_variance(counts)
    var = regularized_variance(counts)
    if n == 0 or var is None or var <= 0.0:
        return 0.0
    s0, s1 = expected_score(elo0), expected_score(elo1)
    return n * (s1 - s0) * (2.0 * mean - s0 - s1) / (2.0 * var)


def bounds(alpha, beta):
    """Wald stopping bounds (lower, upper)."""
    return math.log(beta / (1.0 - alpha)), math.log((1.0 - beta) / alpha)


@dataclass(frozen=True)
class Sprt:
    elo0: float = 0.0
    elo1: float = 15.0
    alpha: float = 0.05
    beta: float = 0.05
    min_pairs: int = 30

    def __post_init__(self):
        if not (self.elo1 > self.elo0 and 0 < self.alpha < .5 and 0 < self.beta < .5 and self.min_pairs >= 1):
            raise ValueError('SPRT needs elo1 > elo0, 0 < alpha, beta < 0.5 and min_pairs >= 1')

    def status(self, counts):
        value = llr(counts, self.elo0, self.elo1)
        lower, upper = bounds(self.alpha, self.beta)
        decision = None
        if sum(counts) >= self.min_pairs:
            decision = 'H1' if value >= upper else 'H0' if value <= lower else None
        return {'llr': value, 'lower': lower, 'upper': upper, 'decision': decision}


def estimate(counts, z=1.959964):
    """Elo estimate and approximate 95% interval from pair scores."""
    n = sum(counts)
    mean, _ = mean_variance(counts)
    if not n:
        return {'pairs': 0}
    var = regularized_variance(counts)
    half = z * math.sqrt(var / n) if n > 1 else math.inf
    lo, hi = max(mean - half, 0.0), min(mean + half, 1.0)
    finite = lambda x: x if math.isfinite(x) else None  # strict JSON output
    return {'pairs': n, 'score': mean, 'score_low': lo, 'score_high': hi,
            'elo': finite(elo_from_score(mean)), 'elo_low': finite(elo_from_score(lo)),
            'elo_high': finite(elo_from_score(hi))}
