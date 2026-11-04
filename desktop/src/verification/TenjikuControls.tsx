import type { VerificationControlsProps } from "../product/ProductApp";
import { tenjikuProfiles } from "../product/native";

export function TenjikuControls({ snapshot, session, blocked }: VerificationControlsProps) {
  if (snapshot.game !== "tenjiku") return null;
  return <aside className="tenjiku-verification-banner" aria-label="Tenjiku contract verification">
    <strong>Tenjiku shared-contract verification</strong>
    <span>{tenjikuProfiles.find(profile => profile.id === snapshot.profile)?.label} · Engine play is unavailable. Final notation and artwork are pending.</span>
    {tenjikuProfiles.map(profile => <button key={profile.id} className="quiet" disabled={session.busy || blocked} onClick={() => { void session.newGame(snapshot.open_token, "tenjiku", { game_kind: "tenjiku", side: snapshot.human.id.side as "sente" | "gote" }, profile.id); }}>New {profile.label} game</button>)}
  </aside>;
}
