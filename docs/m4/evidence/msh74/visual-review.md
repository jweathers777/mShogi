# Independent visual review — MSH-74

Fresh delegated reviewer `msh74_visual_review`, 2026-10-26. Method: inspect all nine
normal packaged macOS screenshots with `view_image`, compare owner specification
and product design, inspect targeted CSS and calculate sRGB text contrast.
Separate from implementation review, rule conformance, native GUI operation,
screen-reader testing and owner approval.

Initial assessment at source `3803c1d`: coherent board-centered surface, complete
9x9 visible at normal/minimum sizes, readable kanji/pentagons, opposite ownership
orientation, distinct promoted glyphs, integrated counted hands and drop marks,
contextual promotion, explicit history/Return to live, separate analysis and
collapsed diagnostics. Material finding: several small secondary labels used
approximately3.0–3.9:1 contrast. These were darkened rather than excused as styling.

Source `60796ec` replaces those labels with `#53634f`: independently calculated
5.63–6.25:1 against the actual ivory/analysis/chooser backgrounds. Four residual
small-text colors (perspective, dirty badge, declaration advice, dialog text) were
also darkened before packaging. Singular event wording was corrected. No material
layout change; the normal screenshot set was recaptured for the owner.

Independent final screenshot assessment and platform screenshot differences are
recorded below. This review never substitutes for explicit owner
visual approval; MSH-74 must remain In Review at the supplied checkpoint.

Final static review: reviewer inspected the eight recaptured normal macOS images
at `60796ec` and the unaltered normal Linux1180×820 and760×650 screenshots from
that same product revision. Complete nine-state coverage is therefore eight
Mac states plus current-source Linux minimum, not nine newly captured Mac
states. All81 squares, both player/hand areas and keyboard hints are visible at
minimum without board scrolling/zoom/pan. Kanji and ownership orientation render
correctly; WebKitGTK number spinners differ from macOS native controls. Affected
text now measures5.30–6.25:1. No outstanding material visual finding. Concluded
side-panel scroll framing was subsequently returned to the top for the owner.

The60796ec Linux phase failed later on stale verifier labels; those earlier
screenshots established actual states, not a passing whole journey. The current
repository platform images have since been replaced by unaltered copies from
passing Desktop CI40408728525 at5de7441. The reviewer independently inspected all
four replacements (normal/minimum Linux/Windows) and confirmed the same visual
verdict. Product visuals are unchanged from60796ec. The CI report establishes
journey completion separately from this static assessment. Owner approval is
still absent and required.

Windows follow-up: reviewer independently inspected the unaltered60796ec normal
1180×820 and760×650 captures. Complete board/player/hand areas remain visible,
with no clipped or missing kanji. General Windows UI text appears thinner and
somewhat smaller than Linux/macOS; Linux shows native number spinners whereas
Windows static captures do not. No new material visual finding. Normal-size
appearance is independently reviewed on all three systems and current minimum
appearance on Windows/Linux. These observations remain distinct from interaction
completion and owner approval.


Reviewer independently viewed the replacement normal Sente image at21bfe3e:
complete board/player areas, readable kanji/controls and coherent hierarchy; no
new material finding. Final a9111ae changes native poll dispatch only; Sente is
recaptured from its normal package with the same visual source. Capture revisions
are explicit in screenshots.json; owner approval remains absent.
