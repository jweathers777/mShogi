"""Offline independent geometry/one-ply mate controls; python-chess1.11.2."""
import json
from pathlib import Path
import chess

cases = [
    ('queen-supported', '7k/8/5KQ1/8/8/8/8/8 w - - 0 1'),
    ('queen-unsupported', '7k/8/6Q1/5K2/8/8/8/8 w - - 0 1'),
    ('rook-supported', '7k/R7/6K1/8/8/8/8/8 w - - 0 1'),
    ('rook-unsupported', '7k/R7/8/5K2/8/8/8/8 w - - 0 1'),
    ('rook-defence', 'r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1'),
]
rows = []
for name, fen in cases:
    board = chess.Board(fen)
    assert board.is_valid(), fen
    mates, checks, safe = [], {}, []
    for move in list(board.legal_moves):
        board.push(move)
        if board.is_checkmate():
            mates.append(move.uci())
        if board.is_check():
            checks[move.uci()] = [m.uci() for m in board.legal_moves]
        replies = []
        for reply in list(board.legal_moves):
            board.push(reply)
            if board.is_checkmate():
                replies.append(reply.uci())
            board.pop()
        if not replies:
            safe.append(move.uci())
        board.pop()
    rows.append(dict(id=name, fen=fen,
        legal_actions=sorted(m.uci() for m in board.legal_moves),
        mate_in_one=mates, checking_replies=checks,
        avoids_every_immediate_reply_mate=safe))
result = dict(oracle='python-chess1.11.2 exhaustive native-independent legal moves/checkmate', rows=rows)
assert chess.__version__ == '1.11.2'
assert result == json.loads(Path(__file__).with_name('independent-controls.json').read_text())
print('Five independent controls reproduce exactly; no engine or match work.')
