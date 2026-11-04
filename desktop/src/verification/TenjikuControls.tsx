import "./tenjiku-verification.css";
import { TenjikuLayout } from "./TenjikuLayout";
import type { VerificationControlsProps } from "../product/ProductApp";
import { tenjikuProfiles } from "../product/native";

export function TenjikuControls({ snapshot, session, blocked }: VerificationControlsProps) {
  if (snapshot.game !== "tenjiku") return null;
  return <aside style={{ position: "relative" }} className="tenjiku-verification-banner" aria-label="Tenjiku contract verification">
    <TenjikuLayout />
    <strong>Tenjiku visual checkpoint</strong>
    <span>{tenjikuProfiles.find(profile => profile.id === snapshot.profile)?.label} · approval pending</span>
    {tenjikuProfiles.map(profile => <button key={profile.id} className="quiet" disabled={session.busy || blocked} onClick={() => { void session.newGame(snapshot.open_token, "tenjiku", { game_kind: "tenjiku", side: snapshot.human.id.side as "sente" | "gote" }, profile.id); }}>New {profile.label} game</button>)}
  </aside>;
}
