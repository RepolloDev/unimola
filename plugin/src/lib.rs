mod preprocess;

use std::rc::Rc;

use plantuml_core::{render_with_metrics, Error};
use preprocess::{prepare_source, RenderConfig, UnimolaResolver};
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[derive(Serialize)]
struct RenderEnvelope {
    ok: bool,
    kind: &'static str,
    svg: Option<String>,
    error: Option<String>,
}

fn error_envelope(kind: &'static str, msg: String) -> RenderEnvelope {
    RenderEnvelope {
        ok: false,
        kind,
        svg: None,
        error: Some(msg),
    }
}

fn error_kind(err: &Error) -> &'static str {
    match err {
        Error::NoDiagram => "no-diagram",
        Error::Unsupported(_) => "unsupported",
        Error::Syntax(_) => "syntax",
    }
}

fn render_impl(source_bytes: &[u8], options_bytes: &[u8]) -> RenderEnvelope {
    let Ok(source) = std::str::from_utf8(source_bytes) else {
        return error_envelope("encoding", "invalid UTF-8 source".to_string());
    };

    let cfg: RenderConfig = if options_bytes.is_empty() {
        RenderConfig::default()
    } else {
        match serde_json::from_slice(options_bytes) {
            Ok(c) => c,
            Err(e) => return error_envelope("options", format!("invalid options JSON: {e}")),
        }
    };

    let prepared = prepare_source(source, &cfg);
    let resolver = Rc::new(UnimolaResolver::new(cfg.includes));

    match render_with_metrics(&prepared, resolver, cfg.deterministic) {
        Ok(svg) => RenderEnvelope {
            ok: true,
            kind: "ok",
            svg: Some(svg),
            error: None,
        },
        Err(err) => error_envelope(error_kind(&err), err.to_string()),
    }
}

/// Renders a PlantUML diagram and returns a JSON envelope `{ ok, kind, svg, error }`.
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn render_svg_json(source: &[u8], options: &[u8]) -> Vec<u8> {
    let envelope = render_impl(source, options);
    serde_json::to_vec(&envelope).unwrap_or_else(|e| {
        format!(
            "{{\"ok\":false,\"kind\":\"internal\",\"svg\":null,\"error\":\"serialization failed: {e}\"}}"
        )
        .into_bytes()
    })
}

/// Returns the underlying PlantUML core specification version string.
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn plantuml_version() -> Vec<u8> {
    plantuml_core::VERSION.as_bytes().to_vec()
}
