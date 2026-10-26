# Shogi product visual review

MSH-74 is In Review for owner visual approval. These are actual normal packaged
application captures, not the development harness. Board/font/layout source is
unchanged between `60796ec`, `5de7441` and `a9111ae`; the final revision adds close protection.
See [evidence and limitations](README.md), [independent visual review](visual-review.md),
and [exact capture revisions and hashes](screenshots.json).

## Initial Sente

macOS normal package; source `a9111ae`.

![Initial Sente](normal-sente-macos.jpg)

## Gote, after the real opening engine move

macOS normal package; source `60796ec`.

![Gote, after the real opening engine move](normal-gote-macos.jpg)

## Both hands and native promoted pieces

macOS normal package; source `60796ec`.

![Both hands and native promoted pieces](normal-hands-macos.jpg)

## Drop selection

macOS normal package; source `60796ec`.

![Drop selection](normal-drop-macos.jpg)

## Optional promotion

macOS normal package; source `60796ec`.

![Optional promotion](normal-promotion-macos.jpg)

## Historical viewing

macOS normal package; source `60796ec`.

![Historical viewing](normal-history-macos.jpg)

## Actual active bounded analysis

macOS normal package; source `60796ec`.

![Actual active bounded analysis](normal-analysis-macos.jpg)

## Concluded game

macOS normal package; source `60796ec`.

![Concluded game](normal-concluded-macos.jpg)

## Linux normal size

Ubuntu 24.04 normal installed package, CI 40408728525; source `5de7441`.

![Linux normal size](normal-sente-linux.png)

## Linux supported minimum 760 × 650

Ubuntu 24.04 normal installed package, CI 40408728525; source `5de7441`.

![Linux supported minimum 760 × 650](normal-minimum-linux.png)

## Windows normal size

Windows Server 2022 normal installed package, CI 40408728525; source `5de7441`.

![Windows normal size](normal-sente-windows.png)

## Windows supported minimum 760 × 650

Windows Server 2022 normal installed package, CI 40408728525; source `5de7441`.

![Windows supported minimum 760 × 650](normal-minimum-windows.png)


## Corrected fresh-process reopening

macOS normal package; source `a9111ae`. The exact Windows regression archive
is reopened with work stopped.

![Corrected Shogi reopening](normal-reopened-macos.jpg)
