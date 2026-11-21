"""Judged move quality rules shared by the blunder screens (MSH-193 Shogi,
MSH-198 Xiangqi, MSH-207 Tori), from docs/m5/strength-roadmap.md.

- Win probability from the side to move: p = 1/(1+exp(-cp/600)); a mate
  score is 1 (the side to move mates) or 0.
- Clear blunder: p_before >= 0.30 and p_after <= p_before - 0.30.
- Mistake: not a clear blunder, p_before >= 0.20 and loss >= 0.15.
- Held-out split: by game, sha256(game id)[0] % 4 == 0.
"""
import hashlib
import math


def win_probability(score):
    """Side-to-move win probability of a judge score ('cp' or 'mate', value)."""
    if score is None:
        return None
    kind, value = score
    if kind == 'mate':
        return 1.0 if value > 0 else 0.0
    return 1 / (1 + math.exp(-value / 600))


def judgement(p_before, p_after):
    """'clear_blunder', 'mistake' or None for the mover's probabilities."""
    if p_before >= 0.30 and p_after <= p_before - 0.30:
        return 'clear_blunder'
    if p_before >= 0.20 and p_before - p_after >= 0.15:
        return 'mistake'
    return None


def split_of(game_id):
    return 'held-out' if hashlib.sha256(game_id.encode()).digest()[0] % 4 == 0 else 'development'
