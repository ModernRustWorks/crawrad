//! **Code Lens** — actionable links embedded in the code.
//!
//! A worker thread requests lenses (`textDocument/codeLens`) and resolves
//! every unresolved lens (`codeLens/resolve`) before handing back the answer.

use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::client::LspClient;
use crate::protocol::{Location, Range};
use crate::settler::{Settle, Settler};
use crawrad_note::note;

#[note("code-lens-settle")]
pub const SETTLE: Duration = Duration::from_millis(300);

#[note("code-lens-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(5);

#[note("code-lens-codelens")]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeLens {
    pub range: Range,
    pub command: Option<LensCommand>,
    data: Option<serde_json::Value>,
}

impl CodeLens {
    #[must_use]
    pub const fn new(range: Range, command: Option<LensCommand>) -> Self {
        Self {
            range,
            command,
            data: None,
        }
    }

    #[note("code-lens-codelens-to-json")]
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "range".to_string(),
            serde_json::to_value(self.range).unwrap_or_default(),
        );
        if let Some(data) = &self.data {
            map.insert("data".to_string(), data.clone());
        }
        serde_json::Value::Object(map)
    }
}

#[note("code-lens-lenscommand")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensCommand {
    pub title: String,
    pub kind: LensKind,
}

#[note("code-lens-lenskind")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LensKind {
    ShowLocations(Vec<Location>),
    Run,
    Other,
}

#[note("code-lens-parse-code-lens")]
pub fn parse_code_lens(val: &serde_json::Value) -> Option<CodeLens> {
    let range: Range = serde_json::from_value(val.get("range")?.clone()).ok()?;
    let command = val
        .get("command")
        .filter(|v| !v.is_null())
        .and_then(parse_command);
    let data = val.get("data").cloned().filter(|v| !v.is_null());
    Some(CodeLens {
        range,
        command,
        data,
    })
}

#[note("code-lens-code-lenses-from")]
pub fn code_lenses_from(val: &serde_json::Value) -> Vec<CodeLens> {
    val.as_array()
        .map(|items| items.iter().filter_map(parse_code_lens).collect())
        .unwrap_or_default()
}

fn parse_command(val: &serde_json::Value) -> Option<LensCommand> {
    if val.is_null() {
        return None;
    }
    let title = val.get("title").and_then(|t| t.as_str()).unwrap_or("");
    let title = plain_title(title);
    let cmd = val.get("command").and_then(|c| c.as_str()).unwrap_or("");
    let kind = match cmd {
        "rust-analyzer.showReferences" => {
            let locs = val
                .get("arguments")
                .and_then(|a| a.as_array())
                .and_then(|args| args.get(2))
                .and_then(|arg| serde_json::from_value::<Vec<Location>>(arg.clone()).ok());
            match locs {
                Some(locations) => LensKind::ShowLocations(locations),
                None => LensKind::Other,
            }
        }
        "rust-analyzer.runSingle" | "rust-analyzer.debugSingle" => LensKind::Run,
        _ => LensKind::Other,
    };
    Some(LensCommand { title, kind })
}

/// The title without the leading symbol the server draws in front of it
/// (a play or a gear sign before "Run" / "Debug"): the editor font has no
/// glyph for it and it showed as an empty box.
pub fn plain_title(title: &str) -> String {
    title
        .trim_start_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

#[note("code-lens-rows")]
pub fn rows(lenses: &[CodeLens]) -> Vec<(usize, String)> {
    lenses
        .iter()
        .filter_map(|l| {
            let cmd = l.command.as_ref()?;
            Some((l.range.start.line as usize, cmd.title.clone()))
        })
        .collect()
}

#[note("code-lens-at-line")]
pub fn at_line(lenses: &[CodeLens], line: usize) -> Option<&LensCommand> {
    let mut first = None;
    for l in lenses {
        if l.range.start.line as usize == line
            && let Some(cmd) = l.command.as_ref()
        {
            if matches!(cmd.kind, LensKind::ShowLocations(_)) {
                return Some(cmd);
            }
            if first.is_none() {
                first = Some(cmd);
            }
        }
    }
    first
}

#[note("code-lens-json")]
pub fn json(lenses: &[CodeLens], bands: &[(usize, [f64; 4])]) -> serde_json::Value {
    let items: Vec<serde_json::Value> = lenses
        .iter()
        .map(|l| {
            let (title, kind, locations) = match &l.command {
                Some(cmd) => {
                    let title = serde_json::Value::String(cmd.title.clone());
                    let (kind, locs) = match &cmd.kind {
                        LensKind::ShowLocations(locs) => ("show_locations", locs.len()),
                        LensKind::Run => ("run", 0),
                        LensKind::Other => ("other", 0),
                    };
                    (title, kind, locs)
                }
                None => (serde_json::Value::Null, "other", 0),
            };
            let line = l.range.start.line as usize;
            let band = bands.iter().find(|(b, _)| *b == line).map(|(_, r)| r);
            serde_json::json!({
                "line": line + 1,
                "title": title,
                "kind": kind,
                "locations": locations,
                "band": band,
            })
        })
        .collect();
    serde_json::Value::Array(items)
}

#[note("code-lens-codelensanswer")]
#[derive(Debug, Clone, PartialEq)]
pub struct CodeLensAnswer {
    pub file: PathBuf,
    pub version: i32,
    pub lenses: Vec<CodeLens>,
}

struct Job {
    client: Arc<LspClient>,
    file: PathBuf,
    version: i32,
}

#[note("code-lens-codelensworker")]
pub struct CodeLensWorker {
    inner: Settler<Job, CodeLensAnswer>,
    shown: Mutex<Vec<CodeLens>>,
}

impl Default for CodeLensWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeLensWorker {
    pub fn new() -> Self {
        let inner = Settler::new("lsp-code-lens", Settle::Quiet(SETTLE), |job, jobs| {
            let mut newer = None;
            let Some(lenses) = lenses_of(&job, jobs, &mut newer) else {
                return Err(newer);
            };
            Ok(CodeLensAnswer {
                file: job.file,
                version: job.version,
                lenses,
            })
        });
        Self {
            inner,
            shown: Mutex::new(Vec::new()),
        }
    }

    #[note("code-lens-codelensworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.inner.set_waker(wake);
    }

    #[note("code-lens-codelensworker-request")]
    pub fn request(&self, client: Arc<LspClient>, file: &Path, version: i32) {
        let _ = self.inner.send(Job {
            client,
            file: file.to_path_buf(),
            version,
        });
    }

    #[note("code-lens-codelensworker-poll")]
    pub fn poll(&self, client: Arc<LspClient>, file: &Path, version: i32) -> Option<Vec<CodeLens>> {
        if client.take_code_lens_refresh() {
            self.request(client, file, version);
        }
        let answer = self.take()?;
        if answer.file == file && answer.version == version {
            if let Ok(mut shown) = self.shown.lock() {
                *shown = answer.lenses.clone();
            }
            Some(answer.lenses)
        } else {
            None
        }
    }

    #[note("code-lens-codelensworker-shown")]
    pub fn shown(&self) -> Vec<CodeLens> {
        match self.shown.lock() {
            Ok(s) => s.clone(),
            Err(_) => Vec::new(),
        }
    }

    #[note("code-lens-codelensworker-take")]
    pub fn take(&self) -> Option<CodeLensAnswer> {
        self.inner.answers().try_iter().last()
    }
}

#[note("code-lens-lenses-of")]
fn lenses_of(job: &Job, jobs: &Receiver<Job>, newer: &mut Option<Job>) -> Option<Vec<CodeLens>> {
    let mut superseded = || {
        if let Some(job) = jobs.try_iter().last() {
            *newer = Some(job);
        }
        newer.is_some()
    };
    let mut lenses = job
        .client
        .request_code_lenses(&job.file, TIMEOUT, &mut superseded)
        .unwrap_or_default();
    if superseded() {
        return None;
    }
    for lens in &mut lenses {
        if lens.command.is_none() {
            if let Ok(resolved) = job.client.resolve_code_lens(lens, TIMEOUT, &mut superseded) {
                *lens = resolved;
            }
            if superseded() {
                return None;
            }
        }
    }
    lenses.sort_by_key(|l| (l.range.start, l.range.end));
    Some(lenses)
}

#[cfg(test)]
#[path = "test/code_lens/test.rs"]
mod tests;
