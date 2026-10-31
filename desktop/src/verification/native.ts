import type { NativeChoice, BoardGeometry } from "../boardPresentation";
import type { BoardPiece } from "../native";
import type { CompletedActionGeometry } from "../boardPresentation";

export interface SyntheticSnapshot {
  profile: "mshogi.synthetic.strider16.v1";
  version: number;
  geometry: Omit<BoardGeometry, "initialFocus"> & { initial_focus: string };
  pieces: BoardPiece[];
  choices: NativeChoice[];
  record: {
    schema_version: 1;
    rules_profile: "mshogi.synthetic.strider16.v1";
    initial_fixture: string;
    actions: string[];
  };
  complete: boolean;
  last_action?: CompletedActionGeometry | null;
}
export type SyntheticCommand =
  | { kind: "play"; version: number; action: string }
  | { kind: "reset"; version: number };
