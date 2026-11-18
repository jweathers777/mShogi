"""Write a frozen Shogi Texel fit into crates/mshogi-search/src/shogi/eval.rs DEFAULT.

Usage: python3 -m tools.strength.shogi_weights FIT.json
Checks kind and names against the source before replacing the array.
"""
import json
import re
import sys
from pathlib import Path

SOURCE = Path('crates/mshogi-search/src/shogi/eval.rs')


def main():
    fit = json.loads(Path(sys.argv[1]).read_text())
    text = SOURCE.read_text()
    names = re.findall(r'"([a-z0-9_]+)"', text[text.index('pub const NAMES'):text.index('pub const INITIAL')])
    if fit.get('kind') != 'mshogi-shogi-texel-fit' or fit['names'] != names:
        raise SystemExit('fit kind or parameter names differ from the source')
    body = ', '.join(str(v) for v in fit['weights'])
    replacement = f'pub const DEFAULT: [i32; COUNT] = [{body}];'
    text, n = re.subn(r'pub const DEFAULT: \[i32; COUNT\] = [^;]+;', replacement, text, count=1, flags=re.S)
    if n != 1:
        raise SystemExit('DEFAULT not found')
    SOURCE.write_text(text)
    print('DEFAULT replaced from', sys.argv[1])


if __name__ == '__main__':
    main()
