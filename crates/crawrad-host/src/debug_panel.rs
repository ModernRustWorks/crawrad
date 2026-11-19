//! **The Run and Debug panel as rows**.
//!
//! The shell draws a list; this module decides what is in it, with no window,
//! so every sentence and every order below is tested instead of eyeballed.
//!
//! The sections are a modern editor's, top to bottom — Variables, Call Stack,
//! Breakpoints — because that is the presentation the owner chose for the
//! debugger (classic RAD behaviour, a modern editor's presentation). Watch is not here
//! yet: a section with no way to add a watch would be a promise, not a tool.

use std::path::Path;

use crawrad_dap::ShownVariable;

use crate::breakpoints::Breakpoints;
use crate::debug::{Debug, Phase};
use crawrad_note::note;

#[note("debug-panel-max-depth")]
pub const MAX_DEPTH: usize = 8;

#[note("debug-panel-row")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// A section title: `VARIABLES`, `CALL STACK`, `BREAKPOINTS`.
    Section(&'static str),
    /// A local of the frame the panel shows, or a child of an open one, in the
    /// presenter's text. `reference` is what a click opens (`0`: nothing to
    /// open); `depth` is how far it is indented.
    Variable {
        name: String,
        value: String,
        depth: usize,
        reference: i64,
        open: bool,
    },
    /// A watched expression: `index` is its place in the list (what its ✕
    /// removes), `value` what it evaluates to where the panel stands.
    Watch {
        index: usize,
        expression: String,
        value: String,
        reference: i64,
        open: bool,
    },
    /// A frame of the stack: `index` is what a click picks, `selected` is the
    /// one whose locals the panel shows.
    Frame {
        index: usize,
        function: String,
        place: String,
        selected: bool,
        /// Code of the project, not of a dependency, the standard library or
        /// the system — a modern editor dims the others, and so does this panel.
        own: bool,
    },
    /// A breakpoint: the file's name and the line.
    Breakpoint { file: String, line: u32 },
    /// What a section says when it has nothing to list, or is still waiting.
    Note(String),
}

impl Row {
    #[note("debug-panel-row-template")]
    pub fn template(&self) -> &'static str {
        match self {
            Row::Section(_) => "Section",
            Row::Variable { .. } => "Variable",
            Row::Watch { .. } => "Watch",
            Row::Frame { selected: true, .. } => "FrameCurrent",
            Row::Frame { own: false, .. } => "FrameForeign",
            Row::Frame { .. } => "Frame",
            Row::Breakpoint { .. } => "Breakpoint",
            Row::Note(_) => "Note",
        }
    }
}

#[note("debug-panel-rows")]
pub fn rows(debug: &Debug, breakpoints: &Breakpoints) -> Vec<Row> {
    let mut out = vec![Row::Section("VARIABLES")];
    let stop = debug.current_stop();
    match (stop, debug.shown_locals()) {
        (Some(_), Some([])) => out.push(Row::Note("no locals in this frame".into())),
        (Some(_), Some(locals)) => variables(&mut out, debug, locals, 0),
        (Some(_), None) => out.push(Row::Note("reading the frame…".into())),
        (None, _) => out.push(Row::Note(idle_sentence(debug.phase()).into())),
    }

    out.push(Row::Section("WATCH"));
    watches(&mut out, debug);

    out.push(Row::Section("CALL STACK"));
    match stop {
        Some(stop) => out.extend(stop.frames.iter().enumerate().map(|(i, f)| {
            Row::Frame {
                index: i,
                function: tail(&pretty_function(&f.name), FUNCTION_CHARS),
                place: f
                    .path
                    .as_deref()
                    .map(|p| format!("{}:{}", file_name(p), f.line))
                    .unwrap_or_default(),
                selected: i == debug.selected_frame(),
                own: f.path.as_deref().is_some_and(is_own_source),
            }
        })),
        None => out.push(Row::Note(idle_sentence(debug.phase()).into())),
    }

    out.push(Row::Section("BREAKPOINTS"));
    let before = out.len();
    for (file, lines) in breakpoints.files() {
        for line in lines {
            out.push(Row::Breakpoint {
                file: file_name(file),
                line,
            });
        }
    }
    if out.len() == before {
        out.push(Row::Note(
            "no breakpoints — click the gutter, or press the toggle key of your scheme".into(),
        ));
    }
    out
}

#[note("debug-panel-watches")]
fn watches(out: &mut Vec<Row>, debug: &Debug) {
    if debug.watches().is_empty() {
        out.push(Row::Note(
            "no watches — Add Watch watches the selection or the word under the caret".into(),
        ));
        return;
    }
    let values = debug.watch_values();
    for (index, expression) in debug.watches().iter().enumerate() {
        let shown = values.and_then(|v| v.get(index));
        let (value, reference) = match shown {
            Some(v) if !v.value.starts_with("<error") => (preview(&v.value), v.reference),
            Some(v) => (preview(&v.value), 0),
            None if debug.is_stopped() => ("reading…".into(), 0),
            None => ("not available — nothing is stopped".into(), 0),
        };
        let children = (reference != 0)
            .then(|| debug.children(reference))
            .flatten();
        out.push(Row::Watch {
            index,
            expression: expression.clone(),
            value,
            reference,
            open: children.is_some(),
        });
        match children {
            Some(Some(kids)) => variables(out, debug, kids, 1),
            Some(None) => out.push(Row::Note("reading…".into())),
            None => {}
        }
    }
}

#[note("debug-panel-variables")]
fn variables(out: &mut Vec<Row>, debug: &Debug, vars: &[ShownVariable], depth: usize) {
    for v in vars {
        // A local the adapter cannot read (`<error: variable not available>`,
        // measured on 30/09/2026 for an out-of-scope `_val`) has nothing to open.
        let opens = v.reference != 0 && depth < MAX_DEPTH && !v.value.starts_with("<error");
        let children = if opens {
            debug.children(v.reference)
        } else {
            None
        };
        out.push(Row::Variable {
            name: v.name.clone(),
            value: preview(&v.value),
            depth,
            reference: if opens { v.reference } else { 0 },
            open: children.is_some(),
        });
        match children {
            Some(Some(kids)) => variables(out, debug, kids, depth + 1),
            Some(None) => out.push(Row::Note("reading…".into())),
            None => {}
        }
    }
}

#[note("debug-panel-idle-sentence")]
fn idle_sentence(phase: &Phase) -> &'static str {
    match phase {
        Phase::Idle | Phase::Ended(_) => {
            "not debugging — Run starts the program under the debugger"
        }
        Phase::Starting => "starting the debugger…",
        Phase::Running => "running — it shows here when the program stops",
        Phase::Stopped => "",
    }
}

#[note("debug-panel-preview-chars")]
pub const PREVIEW_CHARS: usize = 48;

#[note("debug-panel-function-chars")]
pub const FUNCTION_CHARS: usize = 34;

#[note("debug-panel-tail")]
pub fn tail(text: &str, max: usize) -> String {
    let n = text.chars().count();
    if n <= max {
        return text.to_string();
    }
    let kept: String = text.chars().skip(n - (max - 1)).collect();
    format!("…{kept}")
}

#[note("debug-panel-preview")]
pub fn preview(value: &str) -> String {
    let one_line = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() <= PREVIEW_CHARS {
        return one_line;
    }
    let cut: String = one_line.chars().take(PREVIEW_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

#[note("debug-panel-is-own-source")]
pub fn is_own_source(path: &Path) -> bool {
    let p = path
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let rust_file = p.ends_with(".rs") && !p.contains('`');
    rust_file
        && ![
            "/.cargo/registry/",
            "/.cargo/git/",
            "/.rustup/",
            "/rustlib/",
            "/rustc/",
        ]
        .iter()
        .any(|foreign| p.contains(foreign))
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

#[note("debug-panel-pretty-function")]
pub fn pretty_function(name: &str) -> String {
    let mut s = name.to_string();
    // The hash the linker appends: `::h` and sixteen hex digits, at the end.
    if let Some(at) = s.rfind("::h") {
        let hash = &s[at + 3..];
        if hash.len() == 16 && hash.bytes().all(|b| b.is_ascii_hexdigit()) {
            s.truncate(at);
        }
    }
    if !s.contains('$') {
        return s;
    }
    for (from, to) in [
        ("$LT$", "<"),
        ("$GT$", ">"),
        ("$u20$", " "),
        ("$C$", ","),
        ("$RF$", "&"),
        ("$BP$", "*"),
        ("$u7b$", "{"),
        ("$u7d$", "}"),
        ("$u5b$", "["),
        ("$u5d$", "]"),
        ("$u27$", "'"),
        ("..", "::"),
    ] {
        s = s.replace(from, to);
    }
    s.strip_prefix('_').map(str::to_string).unwrap_or(s)
}

#[cfg(test)]
#[path = "test/debug_panel/test.rs"]
mod tests;
