# Unimola — Project Guide

Unimola is a Typst package (`@preview/unimola`) that renders PlantUML diagrams natively as SVG using a pure-Rust WebAssembly plugin powered by `plantuml-core` (no Java or external Graphviz required).

## Repository Layout

- `typst.toml`: Package manifest (`name = "unimola"`, `compiler = "0.14.0"`, `exclude` rules).
- `lib.typ`: Package entrypoint. Only re-exports public symbols from `src/`.
- `src/plugin.typ`: Loads `../assets/unimola.wasm`.
- `src/render.typ`: Core Typst API (`unimola`, `unimola-config`, `unimola-result`, `unimola-svg`, `unimola-version`), `unimola-state` global config, and fixed framed error box rendering.
- `src/raw.typ`: `show-unimola-blocks` show-rule helper for `plantuml` / `puml` raw blocks.
- `assets/unimola.wasm`: Compiled WASM plugin binary (`wasm32-unknown-unknown`), git-ignored (`/assets/*.wasm`) and built locally or via CI.
- `plugin/`: Rust `cdylib` (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/preprocess.rs`).
  - Dependencies: `plantuml-core = "=0.5.4"`, `serde`, `serde_json`, `wasm-minimal-protocol = "0.2.1"`.
  - Keep `plantuml-core` pinned to `=0.5.4` (newer versions pull `getrandom`/`wasm-bindgen` host imports unsupported by Typst).
- `.github/workflows/release.yml`: CI workflow on `v*` tags that builds `assets/unimola.wasm`, verifies `example/demo.typ`, and attaches `unimola-<version>.tar.gz` + `assets/unimola.wasm` to GitHub Releases.
- `example/`: Runnable demo (`demo.typ`, `diagrams/architecture.puml`, `themes/corporate.puml`).
- `draft/docs/`: Local copy of official `typst/packages` guidelines (git-ignored).

## Build & Local Verification

Rebuild the WASM plugin after editing `plugin/`:

```sh
cd plugin
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/unimola.wasm ../assets/unimola.wasm
```

Link the workspace as a local `@preview` package and compile the demo:

```sh
mkdir -p "${XDG_DATA_HOME:-$HOME/.local/share}/typst/packages/preview/unimola"
ln -sfn "$(pwd)" "${XDG_DATA_HOME:-$HOME/.local/share}/typst/packages/preview/unimola/0.1.0"
typst compile example/demo.typ /tmp/unimola-demo.pdf
```

## Typst Package Guidelines (`draft/docs/`)

- **Code style** (`draft/docs/typst.md`): 2-space indentation, `kebab-case` identifiers, `lib.typ` only re-exports public functions.
- **Manifest & description** (`draft/docs/manifest.md`): Imperative `description` without `"Typst"` or `"package"`; valid SPDX `license` matching `LICENSE`; root-anchored `exclude` globs (`/draft/**`, `/plugin/**`, `/example/**`, `/tests/**`, `/.github/**`).
- **Documentation** (`draft/docs/documentation.md`): `README.md` and `example/demo.typ` must use `#import "@preview/unimola:<version>"`. Avoid GitHub-only Markdown (alert blocks, emoji shortcodes, task lists) as Typst Universe does not render them.
- **Resources & bundle size** (`draft/docs/resources.md`, `draft/docs/tips.md`): Never read user files by string path inside package code (accept `str`, `raw` content, or `read(...)` output). Exclude non-runtime files from the downloaded bundle.

## Releasing a New Version to Typst Universe

Typst Universe (`typst/packages`) does **not** sync automatically from GitHub releases. Each release requires a new PR:

1. Update `version` in `typst.toml` (SemVer).
2. Update `#import "@preview/unimola:<version>"` in `README.md` and `example/demo.typ`.
3. If `plugin/` changed, rebuild `assets/unimola.wasm`.
4. Verify `typst compile example/demo.typ /tmp/unimola-demo.pdf` succeeds.
5. Tag and push to GitHub (`RepolloDev/unimola`). GitHub Actions (`.github/workflows/release.yml`) will build `assets/unimola.wasm` and publish `unimola-<version>.tar.gz` in the Release.
6. In your fork of `typst/packages` (via sparse checkout), create `packages/preview/unimola/<version>/` (never overwrite previous versions) and copy the contents of `unimola-<version>.tar.gz` (or manually copy):
   - `typst.toml`, `lib.typ`, `src/`, `assets/unimola.wasm`, `README.md`, `LICENSE`
   - `example/demo.typ`, `example/diagrams/`, `example/themes/` (linked from `README.md`, excluded from bundle by `typst.toml`)
   - Do **not** copy `.git/`, `draft/`, `.github/`, `plugin/target/`, or compiled PDFs.
7. Open a PR to `typst/packages` (`main`). Mention that `assets/unimola.wasm` bundles `plantuml-core` + `graphviz-pure` for pure-Rust rendering without external binaries.
