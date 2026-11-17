# Unimola

Render PlantUML diagrams natively as SVG using a pure-Rust WebAssembly plugin powered by `plantuml-core` — no Java, no external Graphviz required.

## Usage

Import the package from Typst Universe (see [example/demo.typ](example/demo.typ) for a complete runnable document):

```typst
#import "@preview/unimola:0.1.0": (
  unimola,
  unimola-config,
  show-unimola-blocks,
)
```

### Render a diagram inline or as a block

```typst
#unimola(
  width: 80%,
  ```plantuml
  @startuml
  Alice -> Bob: Hello
  Bob --> Alice: Hi!
  @enduml
  ```,
)
```

### Render inside a `figure`

```typst
#figure(
  caption: [Order processing sequence],
  unimola(
    width: 85%,
    ```plantuml
    Alice -> API: POST /orders
    API --> Alice: 201 Created
    ```,
  ),
) <fig-sequence>
```

### Automatic rendering for fenced code blocks

```typst
#show raw.where(lang: "plantuml"): show-unimola-blocks(width: 90%)
#show raw.where(lang: "puml"): show-unimola-blocks(width: 90%)
```

### Global configuration and custom themes

Use `unimola-config(...)` to set document-wide defaults, or pass `preamble` / `includes` to load custom `.puml` themes and `!include` files:

```typst
#unimola-config(preamble: read("themes/corporate.puml"))

// Override per diagram
#unimola(theme: "amiga", preamble: none, "Alice -> Bob: Hi")

// Resolve `!theme corporate` from a virtual includes map
#unimola(
  preamble: none,
  includes: (corporate: read("themes/corporate.puml")),
  "@startuml\n!theme corporate\nAlice -> Bob: Custom\n@enduml",
)
```

### Load from an external `.puml` file

Pass `read("diagram.puml")` directly from your document:

```typst
#unimola(read("diagram.puml"), width: 90%)
```

## API Reference

- `unimola(source, theme: auto, preamble: auto, auto-wrap: auto, deterministic: auto, includes: auto, width: auto, height: auto, fit: "contain", scale: none, alt: none) -> content`: Renders PlantUML markup (string or raw block) as an SVG `image`, or an inline error box on invalid syntax. Arguments set to `auto` inherit from `unimola-config(...)`.
- `unimola-config(theme: auto, preamble: auto, auto-wrap: auto, deterministic: auto, includes: auto) -> content`: Updates document-wide defaults for subsequent `unimola` and `show-unimola-blocks` calls.
- `show-unimola-blocks(..) -> function`: Returns a show-rule handler for `raw` blocks.
- `unimola-svg(source, theme: none, preamble: none, auto-wrap: true, deterministic: false, includes: (:)) -> str`: Returns the raw SVG string or panics on error.
- `unimola-result(source, theme: none, preamble: none, auto-wrap: true, deterministic: false, includes: (:)) -> dictionary`: Returns the raw plugin envelope `(ok, kind, svg, error)`.
- `unimola-version() -> str`: Returns the underlying `plantuml-core` specification version.

## How It Works

1. Typst passes the PlantUML source and JSON-encoded options (`theme`, `preamble`, `auto_wrap`, `deterministic`, `includes`) to `assets/unimola.wasm` via `wasm-minimal-protocol`.
2. The Rust plugin normalizes the source, resolves custom `!include` / `!theme` entries in memory, and renders SVG via `plantuml-core`.
3. Typst decodes the JSON response and embeds the resulting SVG via `image(bytes(svg), format: "svg")`.

## License

MIT License. See [LICENSE](LICENSE) for details.
