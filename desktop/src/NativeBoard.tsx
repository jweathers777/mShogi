// Concrete presentation adapter. Native side labels, ordering, hands and complete
// choices arrive from Rust. No notation, promotion or legality is calculated here.
import { Board, type BoardChoice, type NativeHand, type NativePiece } from "./Board";
export interface NativeBoardSnapshot {
  token: { document: number; version: number; revision: number; selection: number };
  game: "chess" | "shogi" | "xiangqi";
  profile: string;
  board: { files: string[]; ranks: string[]; label: string; initial_focus: string;
    intersections?: { points: string[]; river_between_ranks: [number, number]; palaces: { side: { key: string; label: string }; points: string[]; diagonals: string[][] }[] } | null };
  orientations: { side: { key: string; label: string }; files: string[]; ranks: string[] }[];
  sides: { key: string; first_mover: boolean }[];
  pieces: NativePiece[];
  hands: NativeHand[];
  choices: BoardChoice[];
  capabilities: { board_action: boolean };
}
export function nativeSnapshotIsNewer(current: NativeBoardSnapshot | null, incoming: NativeBoardSnapshot): boolean {
  if (current === null) return true;
  return incoming.token.document > current.token.document || (incoming.token.document === current.token.document && incoming.token.revision > current.token.revision);
}
export function NativeBoard({ snapshot, bottomSide, onPlay }: {
  snapshot: NativeBoardSnapshot; bottomSide: string; onPlay: (action: string) => void;
}) {
  if (snapshot.board.intersections) return <p role="alert">This game requires its native intersection renderer.</p>;
  const orientation = snapshot.orientations.find((o) => o.side.key === bottomSide);
  if (!orientation) return <p role="alert">Unsupported native orientation.</p>;
  return <Board pieces={snapshot.pieces} choices={snapshot.choices} hands={snapshot.hands}
    geometry={{ ...snapshot.board, initialFocus: snapshot.board.initial_focus }}
    nativeOrientation={{ files: orientation.files, ranks: orientation.ranks, label: orientation.side.label, bottomOwner: orientation.side.key }}
    nativeFirstOwner={snapshot.sides.find((side) => side.first_mover)!.key}
    orientation="white" version={snapshot.token.selection}
    profile={`${snapshot.profile}:${snapshot.token.document}:${snapshot.token.version}`}
    disabled={!snapshot.capabilities.board_action} onPlay={onPlay} />;
}
