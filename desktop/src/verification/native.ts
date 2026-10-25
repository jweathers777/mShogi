import type { BoardChoice, BoardGeometry } from "../Board";
import type { BoardPiece } from "../native";

export interface SyntheticSnapshot {
  profile: "mshogi.synthetic.strider16.v1";
  version: number;
  geometry: Omit<BoardGeometry, "initialFocus"> & { initial_focus: string };
  pieces: BoardPiece[];
  choices: BoardChoice[];
  record: {
    schema_version: 1;
    rules_profile: "mshogi.synthetic.strider16.v1";
    initial_fixture: string;
    actions: string[];
  };
  complete: boolean;
}
export type SyntheticCommand =
  | { kind: "play"; version: number; action: string }
  | { kind: "reset"; version: number };
