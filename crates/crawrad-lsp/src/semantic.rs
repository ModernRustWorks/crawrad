//! **Semantic tokens** off the window's thread.
//!
//! The server classifies the names of a file — a macro, a trait, a `mut`
//! binding, a parameter, a field — and the editor colours them over its own
//! tokenizer (`textDocument/semanticTokens/full`, LSP 3.17). After the first
//! answer, each edit asks only what changed (`full/delta` with the previous
//! `resultId`), and the delta is applied to the kept data.
//!
//! Measured on rust-analyzer (2026-10-04): the legend has 58 types and 23
//! modifiers; `full: { delta: true }`; with `utf-8` negotiated, a token's
//! start and length are bytes (`ção` is 5), so positions follow the position
//! encoding as `Position` does.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::Value;

use crate::client::LspClient;
use crawrad_note::note;

#[note("semantic-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(5);

#[note("semantic-class")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Class {
    Macro,
    Trait,
    Mutable,
    Parameter,
    Field,
    EnumMember,
    /// The `unsafe` keyword (also in `#[unsafe(no_mangle)]`) and what is
    /// unsafe to use, as the server marks them.
    Unsafe,
}

#[note("semantic-legend")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Legend {
    pub types: Vec<String>,
    pub modifiers: Vec<String>,
}

impl Legend {
    #[note("semantic-legend-from-capabilities")]
    pub fn from_capabilities(capabilities: Option<&Value>) -> Self {
        let list = |k: &str| -> Vec<String> {
            capabilities
                .and_then(|c| c.pointer(&format!("/semanticTokensProvider/legend/{k}")))
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|t| t.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };
        Self {
            types: list("tokenTypes"),
            modifiers: list("tokenModifiers"),
        }
    }

    #[note("semantic-legend-has")]
    fn has(&self, mods: u32, name: &str) -> bool {
        self.modifiers
            .iter()
            .position(|x| x == name)
            .is_some_and(|bit| bit < 32 && mods >> bit & 1 == 1)
    }

    #[note("semantic-legend-deprecated")]
    pub fn deprecated(&self, mods: u32) -> bool {
        self.has(mods, "deprecated")
    }

    #[note("semantic-legend-class")]
    pub fn class(&self, ty: u32, mods: u32) -> Option<Class> {
        let ty = self.types.get(ty as usize)?.as_str();
        let has = |m: &str| self.has(mods, m);
        if has("unsafe") {
            return Some(Class::Unsafe);
        }
        match ty {
            "macro" | "macroBang" => Some(Class::Macro),
            "interface" => Some(Class::Trait),
            "variable" | "parameter" if has("mutable") => Some(Class::Mutable),
            "parameter" => Some(Class::Parameter),
            "property" => Some(Class::Field),
            "enumMember" => Some(Class::EnumMember),
            _ => None,
        }
    }
}

#[note("semantic-span")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub class: Option<Class>,
    pub deprecated: bool,
}

#[note("semantic-decode")]
pub fn decode(
    data: &[u32],
    legend: &Legend,
    text: &str,
    enc: crate::encoding::PositionEncoding,
) -> Vec<Span> {
    let lines: Vec<&str> = text
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .collect();
    let (mut line, mut start) = (0usize, 0u32);
    let mut out = Vec::new();
    for t in data.chunks_exact(5) {
        let (dl, ds, len, ty, mods) = (t[0], t[1], t[2], t[3], t[4]);
        line += dl as usize;
        start = if dl == 0 { start + ds } else { ds };
        let (class, deprecated) = (legend.class(ty, mods), legend.deprecated(mods));
        let Some(l) = lines.get(line).filter(|_| class.is_some() || deprecated) else {
            continue;
        };
        let (s, e) = (enc.byte(l, start), enc.byte(l, start + len));
        if s < e {
            out.push(Span {
                line,
                start: s,
                end: e,
                class,
                deprecated,
            });
        }
    }
    out
}

#[note("semantic-apply-delta")]
pub fn apply_delta(prev: &[u32], edits: &[Value]) -> Option<Vec<u32>> {
    let mut data = prev.to_vec();
    let mut parsed: Vec<(usize, usize, Vec<u32>)> = edits
        .iter()
        .map(|e| {
            let n = |k: &str| e.get(k).and_then(Value::as_u64).map(|v| v as usize);
            let ints = e
                .get("data")
                .and_then(Value::as_array)
                .map(|a| ints(a))
                .unwrap_or_default();
            Some((n("start")?, n("deleteCount")?, ints))
        })
        .collect::<Option<_>>()?;
    parsed.sort_by_key(|e| std::cmp::Reverse(e.0));
    for (start, delete, insert) in parsed {
        if start + delete > data.len() {
            return None;
        }
        data.splice(start..start + delete, insert);
    }
    Some(data)
}

fn ints(a: &[Value]) -> Vec<u32> {
    a.iter()
        .filter_map(|v| v.as_u64().map(|v| v as u32))
        .collect()
}

#[note("semantic-next-data")]
pub fn next_data(answer: &Value, prev: Option<&[u32]>) -> Option<(Option<String>, Vec<u32>)> {
    let id = answer
        .get("resultId")
        .and_then(Value::as_str)
        .map(str::to_string);
    if let Some(data) = answer.get("data").and_then(Value::as_array) {
        return Some((id, ints(data)));
    }
    let edits = answer.get("edits")?.as_array()?;
    Some((id, apply_delta(prev?, edits)?))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAnswer {
    pub ticket: u64,
    pub file: PathBuf,
    pub version: i32,
    /// The whole data for that version; `None` when the server did not answer.
    pub data: Option<Vec<u32>>,
}

type Kept = Arc<Mutex<HashMap<PathBuf, (String, Vec<u32>)>>>;

#[note("semantic-semanticworker")]
pub struct SemanticWorker {
    asker: crate::asker::Asker<SemanticAnswer>,
    kept: Kept,
}

impl Default for SemanticWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticWorker {
    pub fn new() -> Self {
        Self {
            asker: crate::asker::Asker::new("lsp-semantic"),
            kept: Kept::default(),
        }
    }

    #[note("semantic-semanticworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.asker.set_waker(wake);
    }

    #[note("semantic-semanticworker-request")]
    pub fn request(&self, client: Arc<LspClient>, ticket: u64, file: &Path, version: i32) -> bool {
        let (file, kept) = (file.to_path_buf(), Arc::clone(&self.kept));
        self.asker.ask(ticket, move || {
            let prev = kept.lock().ok().and_then(|k| k.get(&file).cloned());
            let doc = serde_json::json!({ "uri": crate::path_to_uri(&file) });
            let answer = match &prev {
                Some((id, _)) => client.send_request(
                    "textDocument/semanticTokens/full/delta",
                    serde_json::json!({ "textDocument": doc, "previousResultId": id }),
                    TIMEOUT,
                ),
                None => client.send_request(
                    "textDocument/semanticTokens/full",
                    serde_json::json!({ "textDocument": doc }),
                    TIMEOUT,
                ),
            };
            let next = answer
                .ok()
                .and_then(|v| next_data(&v, prev.as_ref().map(|p| p.1.as_slice())));
            if let Ok(mut k) = kept.lock() {
                match &next {
                    Some((Some(id), data)) => {
                        k.insert(file.clone(), (id.clone(), data.clone()));
                    }
                    _ => {
                        k.remove(&file);
                    }
                }
            }
            SemanticAnswer {
                ticket,
                file,
                version,
                data: next.map(|n| n.1),
            }
        })
    }

    #[note("semantic-semanticworker-take")]
    pub fn take(&self, ticket: u64) -> Option<SemanticAnswer> {
        self.asker.take(ticket)
    }
}

#[cfg(test)]
#[path = "test/semantic/test.rs"]
mod tests;
