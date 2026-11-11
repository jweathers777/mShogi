#!/usr/bin/env python3
"""Mechanical diagnostic phase tags from complete canonical histories.
This is not rules/outcome authority or evaluation; python-chess validates replay.
Usage: python phase_tags.py NATIVE_REPORT OUTPUT_JSON
Position ply p is the board BEFORE decision p; the last row is the final board.
"""
import argparse
from pathlib import Path
import chess
from analysis import START_FEN, decode, digest, extract_decisions, load_report, save

POINTS = {chess.QUEEN: 9, chess.ROOK: 5, chess.BISHOP: 3, chess.KNIGHT: 3}
HOME = [(chess.WHITE, s) for s in (chess.B1,chess.C1,chess.F1,chess.G1)] + [(chess.BLACK, s) for s in (chess.B8,chess.C8,chess.F8,chess.G8)]


def tag_history(initial_fen, history, history_origin='standard_start'):
    board = chess.Board(initial_fen)
    if not board.is_valid(): raise ValueError('Invalid initial diagnostic board')
    known = initial_fen == START_FEN and history_origin == 'standard_start'
    original = {f'{"white" if color else "black"}_{chess.square_name(square)}':
                {'color':color, 'home':square, 'square':square} for color,square in HOME} if known else {}
    castled = {chess.WHITE:False, chess.BLACK:False}
    positions = []
    def snapshot(ply):
        queens_gone = not board.pieces(chess.QUEEN,chess.WHITE) and not board.pieces(chess.QUEEN,chess.BLACK)
        minors = {name:{'home':chess.square_name(p['home']),
                         'current':chess.square_name(p['square']) if p['square'] is not None else None,
                         'captured':p['square'] is None, 'on_original_home':p['square']==p['home']}
                  for name,p in original.items()}
        sides = {}
        for color,name in [(chess.WHITE,'white'),(chess.BLACK,'black')]:
            home = sum(p['color']==color and p['square']==p['home'] for p in original.values()) if known else None
            rights = board.has_castling_rights(color)
            sides[name] = {'original_minors_home':home, 'castled_in_known_history':castled[color],
                           'castling_rights_remaining':bool(rights),
                           'king_disposed':bool(castled[color] or not rights or queens_gone)}
        qualifies = known and all(s['original_minors_home']<=1 and s['king_disposed'] for s in sides.values())
        material = sum(value*len(board.pieces(piece,color)) for piece,value in POINTS.items() for color in [chess.WHITE,chess.BLACK])
        positions.append({'ply':ply, 'fen':board.fen(en_passant='fen'),
                          'fullmove_boundary':board.turn==chess.WHITE,
                          'fullmove_number':board.fullmove_number,
                          'indicators':{'original_minor_identities_known':known,'original_minors':minors,
                                        'sides':sides,'both_queens_gone':bool(queens_gone),
                                        'development_king_condition':bool(qualifies),
                                        'nonpawn_nonking_points':material,'endgame_material':material<=26}})
    snapshot(0)
    for ply, uci in enumerate(history,1):
        move=chess.Move.from_uci(uci)
        if move not in board.legal_moves: raise ValueError(f'Illegal diagnostic history at ply{ply}: {uci}')
        moving = next((p for p in original.values() if p['square']==move.from_square),None)
        captured = next((p for p in original.values() if p['square']==move.to_square),None)
        if captured is not None: captured['square']=None
        if moving is not None: moving['square']=move.to_square
        if board.is_castling(move):castled[board.turn]=True
        board.push(move);snapshot(ply)
    boundaries=[p for p in positions if p['fullmove_boundary']]
    boundary=None
    for i in range(len(boundaries)-2):
        if all(p['indicators']['development_king_condition'] for p in boundaries[i:i+3]):
            boundary=boundaries[i]['ply'];break
    for p in positions:
        p['phase']=('endgame' if p['indicators']['endgame_material'] else
                    'middlegame' if boundary is not None and p['ply']>=boundary else
                    'opening' if known else 'opening_or_middlegame_unresolved')
    return {'schema':1,'method':'development-king-three-boundaries-material26-v1',
            'initial_fen':initial_fen,'history_origin':history_origin,'history_plies':len(history),
            'opening_boundary_ply':boundary,'opening_boundary_unresolved':boundary is None,
            'opening_boundary_reason':('first_of_three_qualifying_fullmove_boundaries' if boundary is not None else
                                        'original_identity_unknown' if not known else 'no_three_consecutive_qualifying_boundaries'),
            'unconfirmed_tail_candidates':[p['ply'] for p in boundaries[-2:] if p['indicators']['development_king_condition']] if boundary is None else [],
            'positions':positions,
            'limitations':['Mechanical phase convention, not position quality or king safety.',
                           'Boundary confirmation uses subsequent history; final short tails can remain unresolved.',
                           'Endgame material tag takes precedence and is evaluated per position, including promotions.',
                           'Imported or unknown-origin histories do not reconstruct original minor identities.']}


def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('report');parser.add_argument('output');args=parser.parse_args()
    destination=Path(args.output)
    if destination.exists():raise FileExistsError('Preserve phase output; choose a new path')
    report=load_report(args.report);_,games=extract_decisions(report);results=[]
    for game in games:
        gi=game['game']
        if game.get('missing_match_record'):
            results.append({'game':gi,'phase_unavailable':'missing_match_record'});continue
        record=decode(report['games'][gi]['match_record_json']);initial=decode(record['initial_record_json'])
        results.append({'game':gi,'opening_id':game['opening_id'],
                        **tag_history(game['initial_fen'],game['complete_history'],initial.get('history_origin'))})
    save(destination,{'schema':1,'source_report_sha256':digest(args.report),'helper_sha256':digest(__file__),
                      'python_chess_version':chess.__version__,'games':results})

if __name__=='__main__':main()
