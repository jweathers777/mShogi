import { Dialog } from "./Dialog";
import type { GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import { version } from "../../package.json";
import licenseUrl from "../assets/fonts/OFL.txt?url";
import fontLicense from "../assets/fonts/OFL.txt?raw";
import fontNotice from "../assets/fonts/NOTICE.txt?raw";

export function SupportDialog({ snapshot, session, close }: { snapshot: GameSnapshot; session: GameSession; close: () => void }) {
  const diagnostics = { token: snapshot.token, profile: snapshot.profile, document: snapshot.document,
    context: snapshot.context, moves: snapshot.moves, capacity: snapshot.capacity, diagnostic: snapshot.diagnostic,
    io: snapshot.io, error: session.error, analysis: snapshot.game === "chess" ? snapshot.chess.analysis : snapshot.analysis,
    observations: snapshot.observations, ...(snapshot.game === "xiangqi" ? { xiangqi: snapshot.xiangqi } : snapshot.game === "chess" ? { chess: snapshot.chess, chess_match: snapshot.chess_match } : {}) };
  return <Dialog title="About mShogi" className="support-dialog" cancel={close}>
    <p>mShogi {version} · Chess, Shogi and Xiangqi</p>
    <details><summary>Diagnostics</summary><p>Game profile, exact history, engine requests, work limits and resource details.</p><pre data-native-diagnostics>{JSON.stringify(diagnostics, null, 2)}</pre></details>
    <details><summary>Font license</summary><p>{fontNotice}</p><a href={licenseUrl} download="mshogi-pieces-OFL.txt">Save the font license</a><pre>{fontLicense}</pre></details>
    <div className="dialog-actions"><button className="quiet" onClick={() => { void session.requestClose(); }}>Quit application</button><button data-cancel className="primary" onClick={close}>Close</button></div>
  </Dialog>;
}
