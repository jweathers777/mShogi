# Desktop development and packages

The desktop uses Tauri 2.12.5, React 19.3.1, strict TypeScript 7.0.2 and
Vite 8.3.5. npm and Cargo lockfiles are committed. Node 24.19.0 is pinned in
`.node-version`. The desktop host has its own Rust 1.90.0 toolchain and lockfile;
the existing native engine workspace keeps Rust 1.89.0.

Install the [native prerequisites](https://v2.tauri.app/start/prerequisites/):
Xcode command line tools on macOS, Visual Studio C++ build tools and WebView2 on
Windows, or WebKitGTK 4.1 development dependencies on Linux. Tauri 2.12 requires
Rust 1.90 or newer. From the repository root:

```sh
cd desktop
npm ci
npm run desktop:dev
```

`desktop:dev` first builds the actual engine using the root workspace's pinned
toolchain. The host resolves only a sibling `mshogi` executable (`mshogi.exe` on
Windows). The frontend has no shell or filesystem plugin. Vite listens on
127.0.0.1:1420 during development; production web assets are embedded and do not
require a web server.

Build a normal distributable with the platform's supported bundle format:

```sh
# macOS
npm run desktop:build -- --bundles app
open src-tauri/target/release/bundle/macos/mShogi.app

# Linux (Debian package)
npm run desktop:build -- --bundles deb
sudo apt install ./src-tauri/target/release/bundle/deb/*.deb
mshogi-desktop

# Windows (PowerShell): run the resulting per-user installer, then use Start menu.
npm run desktop:build -- --bundles nsis
Get-ChildItem src-tauri/target/release/bundle/nsis/*-setup.exe
```

These are development packages, without a promised release signing or
notarization arrangement. macOS local builds use the normal Tauri application
bundle; production distribution signing and minimum OS release policy remain
part of the later packaged verification work.

`npm run check` checks strict TypeScript. `npm run build` builds embedded web
assets. In `src-tauri`, `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked`
verify the native host. Native CI builds the normal packages on Windows,
macOS and Linux, extracts/installs them and runs their actual bundled engine
through UCI handshake and a bounded initial-position search. Its artifact
includes the package and `package-smoke.json` with package inventory, executable
SHA-256 and actual search observations.

`desktop/scripts/package-smoke.py` is an external verifier. Tests and fixtures
are not resources or sidecars; only the `mshogi` engine is listed as
`externalBin`. No instrumentation or test driver is compiled into the normal
host. The later MSH-38 ticket verifies installed application interactions on all
three systems after the remaining M3 features exist; a package build and engine
smoke do not establish that broader result.
