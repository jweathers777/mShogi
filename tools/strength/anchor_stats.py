"""Joint fixed-anchor Elo estimates for MSH-185 (Chess Strength Anchor).

Input is one row per scored game: game_id, pair_id, opponent_id, family, anchor,
score (0, .5, 1). Never omit a scored game because its partner is unscored.

Let a=ln(10)/400, p_c=logistic(a*(R-anchor_c)), m_c be the number of
scored games in opening cluster c (one or two), and s_c their total score.
The point estimate maximizes sum_c[s_c log(p_c)+(m_c-s_c)log(1-p_c)]
minus (R-prior_mean)^2/(2*prior_sd^2). Defaults are Normal(2500,500^2).
Draws enter as half-points, not independent Bernoulli trials.

H=a^2 sum_c[m_c*p_c*(1-p_c)]+1/prior_sd^2. Cluster scores are
u_c=a*(s_c-m_c*p_c). For C>1 the sandwich variance is
C/(C-1)*sum_c(u_c^2)/H^2. Its interval uses t_(C-1,.975); table lookup
rounds degrees of freedom DOWN, so interpolation cannot narrow the interval.
This is an approximate regularized, pair-cluster interval, not a posterior
credible interval or an anytime-valid confidence sequence.

Small samples need a conservative routing bound. Maximize the same objective
with each cluster contributing ONE fractional trial (s_c/m_c), plus the same
prior. Its profile endpoints are a log-objective drop of 1.920729410347062.
For fewer than 30 clusters, or zero observed cluster variation, the reported
interval is the envelope of that profile interval and the sandwich interval.
This fractional-score profile is a quasi-likelihood routing approximation,
not an exact sampling distribution. Precision stopping is disallowed before
30 clusters and when cluster variation is zero. Always report systematic
anchor/time-control/hardware/family uncertainty separately.
"""
import math

A = math.log(10) / 400
PROFILE_DROP = 1.920729410347062
T975 = ((1, 12.706205), (2, 4.302653), (3, 3.182447), (4, 2.776446),
        (5, 2.570582), (6, 2.446912), (7, 2.364625), (8, 2.306005),
        (9, 2.262158), (10, 2.228139), (12, 2.178813), (15, 2.131450),
        (20, 2.085964), (25, 2.059539), (30, 2.042273), (40, 2.021076),
        (60, 2.000298), (120, 1.979931))


def expected_score(rating, anchor):
    x = A * (rating - anchor)
    if x >= 0:
        return 1 / (1 + math.exp(-x))
    exp_x = math.exp(x)
    return exp_x / (1 + exp_x)


def _clusters(games):
    ids, opponents, pairs = set(), {}, {}
    for game in games:
        for key in ('game_id', 'pair_id', 'opponent_id', 'family'):
            if not isinstance(game.get(key), str) or not game[key]:
                raise ValueError('missing nonempty ' + key)
        if game['game_id'] in ids:
            raise ValueError('duplicate game_id')
        ids.add(game['game_id'])
        anchor, score = game.get('anchor'), game.get('score')
        if isinstance(anchor, bool) or not isinstance(anchor, (int, float)) or not math.isfinite(anchor):
            raise ValueError('anchor must be finite')
        if isinstance(score, bool) or score not in (0, .5, 1):
            raise ValueError('score must be 0, .5 or 1')
        identity = (anchor, game['family'])
        old = opponents.setdefault(game['opponent_id'], identity)
        if old != identity:
            raise ValueError('opponent anchor/family changed')
        pair = pairs.setdefault(game['pair_id'], {
            'opponent_id': game['opponent_id'], 'anchor': anchor,
            'family': game['family'], 'n': 0, 's': 0})
        if pair['opponent_id'] != game['opponent_id']:
            raise ValueError('pair spans opponents')
        pair['n'] += 1
        pair['s'] += score
        if pair['n'] > 2:
            raise ValueError('more than two games in pair')
    return [pairs[key] for key in sorted(pairs)]


def _bisect(function, lo, hi):
    """Root of a continuous decreasing function with opposite endpoint signs."""
    if function(lo) < 0 or function(hi) > 0:
        raise ValueError('root not bracketed')
    for _ in range(100):
        mid = (lo + hi) / 2
        if function(mid) > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def _mode(clusters, mean, sd, per_pair=False):
    def derivative(r):
        return A * sum((c['s'] - c['n'] * expected_score(r, c['anchor'])) /
                       (c['n'] if per_pair else 1) for c in clusters) - (r - mean) / sd**2
    lo, hi = mean - sd, mean + sd
    while derivative(lo) < 0:
        lo = mean + 2 * (lo - mean)
    while derivative(hi) > 0:
        hi = mean + 2 * (hi - mean)
    return _bisect(derivative, lo, hi)


def _routing_interval(clusters, mean, sd):
    mode = _mode(clusters, mean, sd, per_pair=True)

    def objective(r):
        value = -(r - mean)**2 / (2 * sd**2)
        for c in clusters:
            x = A * (r - c['anchor'])
            softplus = max(x, 0) + math.log1p(math.exp(-abs(x)))
            value += (c['s'] / c['n']) * x - softplus
        return value

    target = objective(mode) - PROFILE_DROP
    lo, hi = mode - sd, mode + sd
    while objective(lo) > target:
        lo = mode + 2 * (lo - mode)
    while objective(hi) > target:
        hi = mode + 2 * (hi - mode)
    lower = _bisect(lambda r: target - objective(r), lo, mode)
    upper = _bisect(lambda r: objective(r) - target, mode, hi)
    return [lower, upper]


def fit(games, prior_mean=2500.0, prior_sd=500.0):
    """Fit every supplied scored game; accept one-game incomplete clusters."""
    if (not math.isfinite(prior_mean) or not math.isfinite(prior_sd) or prior_sd <= 0):
        raise ValueError('finite mean and positive finite prior SD required')
    clusters = _clusters(games)
    count = len(clusters)
    rating = _mode(clusters, prior_mean, prior_sd)
    routing = _routing_interval(clusters, prior_mean, prior_sd)
    scores = [A * (c['s'] - c['n'] * expected_score(rating, c['anchor'])) for c in clusters]
    hessian = 1 / prior_sd**2 + A**2 * sum(
        c['n'] * expected_score(rating, c['anchor']) *
        (1 - expected_score(rating, c['anchor'])) for c in clusters)
    sandwich, se, critical = None, None, None
    observed_scores = [c['s'] / c['n'] for c in clusters]
    variation = count > 1 and max(observed_scores) - min(observed_scores) > 1e-12
    if count > 1:
        se = math.sqrt(count / (count - 1) * sum(s * s for s in scores)) / hessian
        critical = [value for df, value in T975 if df <= count - 1][-1]
        sandwich = [rating - critical * se, rating + critical * se]
    floor_used = count < 30 or not variation
    interval = list(sandwich) if sandwich is not None else [rating, rating]
    if floor_used:
        interval = [min(interval[0], routing[0], rating), max(interval[1], routing[1], rating)]
    return {'estimate': rating, 'interval95': interval,
            'half_width': max(rating - interval[0], interval[1] - rating),
            'scored_games': sum(c['n'] for c in clusters), 'clusters': count,
            'complete_pairs': sum(c['n'] == 2 for c in clusters),
            'singleton_clusters': sum(c['n'] == 1 for c in clusters),
            'prior': {'mean': prior_mean, 'sd': prior_sd},
            'sandwich_se': se, 't_critical': critical, 'sandwich_interval95': sandwich,
            'routing_interval95': routing, 'routing_floor_used': floor_used,
            'precision_stop_eligible': count >= 30 and variation,
            'interval_method': 'approximate pair-cluster t; conservative routing envelope for small/degenerate samples'}


def summarize(games, prior_mean=2500.0, prior_sd=500.0):
    """Joint fit, family agreement and prespecified prior-width sensitivity."""
    games = list(games)
    joint = fit(games, prior_mean, prior_sd)
    families = {family: fit([g for g in games if g['family'] == family], prior_mean, prior_sd)
                for family in sorted({g['family'] for g in games})}
    estimates = [result['estimate'] for result in families.values()]
    spread = max(estimates) - min(estimates) if estimates else None
    return {'joint': joint, 'families': families, 'family_spread': spread,
            'family_disagreement_over_150': spread is not None and spread > 150,
            'prior_sensitivity': {str(sd): fit(games, prior_mean, sd)
                                  for sd in (250.0, 500.0, 1000.0)}}
