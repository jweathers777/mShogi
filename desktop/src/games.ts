const white = { side: "white", label: "White" } as const;
const black = { side: "black", label: "Black" } as const;
const red = { side: "red", label: "Red" } as const;
const shogiSides = [{ side: "sente", label: "Sente" }, { side: "gote", label: "Gote" }] as const;

export const tenjikuProfiles = [
  { id: "mshogi.tenjiku.adams1999-modern.v1", label: "Adams 1999 Modern" },
  { id: "mshogi.tenjiku.muller-completed.v1", label: "Muller Completed" },
] as const;

export const games = {
  chess: { name: "Chess", sides: [white, black], profile: "mshogi.chess.match.v1" },
  shogi: { name: "Shogi", sides: shogiSides, profile: "mshogi.shogi.csa36.v1" },
  xiangqi: { name: "Xiangqi", sides: [red, black], profile: "mshogi.xiangqi.wxf2018.v1" },
  tori: { name: "Tori Shogi", sides: shogiSides, profile: "mshogi.tori.modern.v1" },
  chu: { name: "Chu Shogi", header: "CHU", sides: shogiSides, profile: "mshogi.chu.jcsa2019.v1" },
  tenjiku: { name: "Tenjiku Shogi", header: "TENJIKU", sides: shogiSides, profile: tenjikuProfiles[0].id },
} as const;

export type GameKind = keyof typeof games;
export type GameSide<K extends GameKind> = (typeof games)[K]["sides"][number]["side"];
export type TenjikuProfile = (typeof tenjikuProfiles)[number]["id"];
export type RulesProfile = (typeof games)[GameKind]["profile"] | TenjikuProfile;

export function gameHeader(kind: GameKind): string {
  const game = games[kind];
  return "header" in game ? game.header : game.name.toUpperCase();
}

export function sideLabel<K extends GameKind>(game: K, side: GameSide<K>): string {
  return games[game].sides.find(entry => entry.side === side)!.label;
}
