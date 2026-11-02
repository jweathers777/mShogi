# Tori piece artwork

The MSH-42 visual checkpoint uses original vector bird engravings on warm,
pentagonal wooden tiles. The tile silhouette, border and ownership rotation
relate deliberately to the existing Shogi board. The bird imagery belongs to
Tori rather than borrowing standard Shogi glyphs for different native pieces.

Every native face has an explicit artwork entry: Phoenix (Ph), Falcon (Fa),
Eagle (Ea), Crane (Cr), Pheasant (Pt), Left Quail (LQ), Right Quail (RQ),
Swallow (Sw), and Goose (Go). An engraved two-letter identity aid below each
bird uses the notation abbreviation, rendered as small capitals. The letters
are original vector strokes, so neither the imagery nor its identity aid
depends on an installed font, a missing Unicode glyph, or a raster resource.
The application supplies full native piece names through its accessible board
and hand labels; the decorative SVG is hidden from assistive technology.

The Phoenix has a crest and broad wings; the Falcon has a hooked beak and
folded wing; the Eagle has a separate wide-wing silhouette; the Crane has a
tall neck; the Pheasant has a long tail; the quails are compact mirrored birds;
the Swallow has pointed wings and a forked tail; the Goose has a rounded body
and long neck. Quail handedness is also spelled out by distinct LQ and RQ
identity aids. Rotating ownership rotates the entire tile, bird and label; it
does not exchange those native identities.

The native promotion flag changes engraving to dark red and adds two corner
chevrons, so promotion is not conveyed by color alone. Eagle and Goose have
their own face artwork. No frontend promotion pairing, movement geometry or
rules are inferred from the illustrations. Unknown face values never render
as a different bird or a raw diagnostic label.

The dark engraving (#29251e) and promoted red (#8b2d25) sit against a light
wood face (#f6deb0). Main line weights are 2.8–3 units in a 100-unit SVG;
the artwork is intended to remain clear at approximately 45px board-piece
size. Actual normal, narrow and minimum-window application captures are the
visual acceptance evidence; component tests do not establish final visual
approval. In particular, the owner may refine these bird and identity-aid
choices after reviewing the running application.

The bird, letter and promotion-mark paths in
`desktop/src/product/ToriPiece.tsx` were drawn specifically for this mShogi
implementation as original project source. The tile outline and bevel reuse
the project's existing `ShogiPiece` geometry to preserve visual continuity.
No separate asset license is introduced or assumed. No
third-party icon set, external artwork, downloaded image, font or generative
bitmap asset was used. There are no additional artwork attribution or
redistribution requirements.
