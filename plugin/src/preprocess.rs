use std::collections::HashMap;

use plantuml_core::tim::resolver::{BundledResolver, ResolvedFile, Resolver};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RenderConfig {
    pub theme: Option<String>,
    pub preamble: Option<String>,
    pub auto_wrap: bool,
    pub deterministic: bool,
    pub includes: HashMap<String, String>,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            theme: None,
            preamble: None,
            auto_wrap: true,
            deterministic: false,
            includes: HashMap::new(),
        }
    }
}

pub struct UnimolaResolver {
    includes: HashMap<String, String>,
    bundled: BundledResolver,
}

impl UnimolaResolver {
    pub fn new(includes: HashMap<String, String>) -> Self {
        Self {
            includes,
            bundled: BundledResolver,
        }
    }
}

impl Resolver for UnimolaResolver {
    fn read_file(&self, name: &str, current_dir: Option<&str>) -> Option<ResolvedFile> {
        self.includes
            .get(name)
            .map(|content| ResolvedFile {
                content: content.clone(),
                dir: None,
                identity: name.to_string(),
            })
            .or_else(|| self.bundled.read_file(name, current_dir))
    }

    fn read_stdlib(&self, name: &str) -> Option<String> {
        self.bundled.read_stdlib(name)
    }

    fn read_theme(
        &self,
        name: &str,
        from: Option<&str>,
        current_dir: Option<&str>,
    ) -> Option<String> {
        let key = format!("puml-theme-{name}.puml");
        self.includes
            .get(name)
            .or_else(|| self.includes.get(&key))
            .cloned()
            .or_else(|| self.bundled.read_theme(name, from, current_dir))
    }

    fn all_theme_names(&self) -> Vec<String> {
        self.bundled.all_theme_names()
    }

    fn all_stdlib_names(&self) -> Vec<String> {
        self.bundled.all_stdlib_names()
    }
}

pub fn prepare_source(raw: &str, cfg: &RenderConfig) -> String {
    let trimmed = raw.trim();
    let has_start = trimmed
        .lines()
        .any(|line| line.trim_start().starts_with("@start"));

    let mut source = if !has_start && cfg.auto_wrap {
        format!("@startuml\n{trimmed}\n@enduml\n")
    } else {
        raw.to_string()
    };

    if let Some(preamble) = cfg.preamble.as_deref().map(str::trim).filter(|p| !p.is_empty()) {
        source = inject_after_start(&source, preamble);
    }

    if let Some(theme) = cfg.theme.as_deref().map(str::trim).filter(|t| !t.is_empty()) {
        let has_theme = source
            .lines()
            .any(|line| line.trim_start().starts_with("!theme"));
        if !has_theme {
            source = inject_after_start(&source, &format!("!theme {theme}"));
        }
    }

    source
}

fn inject_after_start(source: &str, directive: &str) -> String {
    let mut out = String::with_capacity(source.len() + directive.len() + 2);
    let mut injected = false;
    for line in source.split_inclusive('\n') {
        out.push_str(line);
        if !injected && line.trim_start().starts_with("@start") {
            if !line.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(directive);
            out.push('\n');
            injected = true;
        }
    }
    if !injected {
        out.insert_str(0, &format!("{directive}\n"));
    }
    out
}
