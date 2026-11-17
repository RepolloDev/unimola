# Unimola WASM Plugin

Pure-Rust WebAssembly bridge (`cdylib`) that compiles PlantUML diagrams to SVG inside Typst without Java or external Graphviz.

## Dependencies

- `plantuml-core = "=0.5.4"` (pinned to avoid `getrandom`/`wasm-bindgen` host imports unsupported by Typst)
- `wasm-minimal-protocol = "0.2.1"`
- `serde` + `serde_json`

## Build

Requires the `wasm32-unknown-unknown` Rust target:

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
mkdir -p ../assets
cp target/wasm32-unknown-unknown/release/unimola.wasm ../assets/unimola.wasm
```

## How It Works

1. **Protocol (`src/lib.rs`)**: Initializes `wasm-minimal-protocol` and exports two functions to Typst:
   - `render_svg_json(source: &[u8], options: &[u8]) -> Vec<u8>`
   - `plantuml_version() -> Vec<u8>`
2. **Preprocessing (`src/preprocess.rs`)**:
   - Deserializes `RenderConfig` (`theme`, `preamble`, `auto_wrap`, `deterministic`, `includes`).
   - Wraps markup in `@startuml ... @enduml` when `auto_wrap` is enabled and no `@start` directive is present.
   - Injects `!theme` and `preamble` (`skinparam`/`<style>`) right after `@start`.
   - Resolves `!include` and custom `!theme` entries from the `includes` map via `UnimolaResolver`, falling back to `BundledResolver`.
3. **Rendering**: Calls `plantuml_core::render_with_metrics` and returns a JSON envelope `{ ok, kind, svg, error }` so Typst can render errors gracefully without panicking.
