//! **The Form check panel's state**: the findings of the open form,
//! which kinds are shown, what the canvas marks, and what a click does —
//! everything but the drawing.

use crate::form_check::{Finding, Fix, Theme, check};
use crawrad_doc::Transaction;
use crawrad_ir::{Document, NodeId, Value};
use crawrad_registry::Registry;
use crawrad_runtime::{FindingPin, PinKind};

/// What the user did in the panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckInput {
    /// The Errors (0) or Warnings (1) chip.
    Chip(usize),
    /// A row, by its place in the shown list.
    Row(usize),
    /// A row's fix button.
    Fix(usize),
    FixAll,
}

/// What the shell does next.
#[derive(Debug)]
pub enum CheckEffect {
    Paint,
    /// Select this control on the form.
    Select(NodeId),
    /// Apply this change (one undo step).
    Apply(Transaction),
    /// Create and wire a handler, through the `event.bind` action.
    Bind {
        node: NodeId,
        event: String,
        handler: String,
    },
}

/// One row as the view draws it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckRow {
    pub number: usize,
    pub error: bool,
    pub rule: &'static str,
    pub text: String,
    pub fix: Option<String>,
    pub selected: bool,
}

/// The head (chips, note, Fix all) and the rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckView {
    pub chips: [(String, bool); 2],
    pub note: String,
    pub fix_all: Option<String>,
    pub rows: Vec<CheckRow>,
}

#[derive(Debug, Default)]
pub struct CheckPanel {
    pub findings: Vec<Finding>,
    hidden: [bool; 2],
    selected: Option<usize>,
    /// The document revision the findings were read at.
    rev: Option<(String, u64)>,
}

impl CheckPanel {
    /// Checks `doc` again if it changed since (`form` and `rev` say which
    /// document, at which revision). Returns whether the findings changed.
    pub fn refresh(
        &mut self,
        doc: &Document,
        reg: &Registry,
        theme: Theme,
        form: &str,
        rev: u64,
    ) -> bool {
        let at = Some((form.to_string(), rev));
        if self.rev == at {
            return false;
        }
        self.rev = at;
        let found = check(doc, reg, theme);
        if found == self.findings {
            return false;
        }
        self.findings = found;
        self.selected = None;
        true
    }

    /// No form open: nothing to say.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Errors and warnings.
    pub fn counts(&self) -> (usize, usize) {
        let errors = self.findings.iter().filter(|f| f.rule.is_error()).count();
        (errors, self.findings.len() - errors)
    }

    /// `2 errors · 5 warnings`, or `No problems`.
    pub fn summary(&self) -> String {
        match self.counts() {
            (0, 0) => "No problems".into(),
            (e, w) => format!("{} \u{00b7} {}", plural(e, "error"), plural(w, "warning")),
        }
    }

    fn shown(&self) -> Vec<usize> {
        (0..self.findings.len())
            .filter(|i| !self.hidden[usize::from(!self.findings[*i].rule.is_error())])
            .collect()
    }

    fn fixable(&self) -> Vec<&Fix> {
        self.findings
            .iter()
            .filter_map(|f| f.fix.as_ref())
            .filter(|f| !matches!(f, Fix::AddHandler { .. }))
            .collect()
    }

    pub fn view(&self) -> CheckView {
        let (errors, warnings) = self.counts();
        let fixable = self.fixable().len();
        let rows = self
            .shown()
            .into_iter()
            .map(|i| {
                let f = &self.findings[i];
                CheckRow {
                    number: i + 1,
                    error: f.rule.is_error(),
                    rule: f.rule.label(),
                    text: f.text.clone(),
                    fix: f.fix.as_ref().map(Fix::label),
                    selected: self.selected == Some(i),
                }
            })
            .collect();
        CheckView {
            chips: [
                (format!("Errors {errors}"), !self.hidden[0]),
                (format!("Warnings {warnings}"), !self.hidden[1]),
            ],
            note: format!("Checked on every change \u{00b7} {fixable} fixable"),
            fix_all: (fixable > 0).then(|| format!("Fix all {fixable}")),
            rows,
        }
    }

    pub fn input(&mut self, input: CheckInput, doc: &Document) -> CheckEffect {
        let row = |i: usize| self.shown().get(i).copied();
        match input {
            CheckInput::Chip(c) => {
                let c = c.min(1);
                self.hidden[c] = !self.hidden[c];
            }
            CheckInput::Row(i) => {
                let Some(at) = row(i) else {
                    return CheckEffect::Paint;
                };
                self.selected = Some(at);
                if let Some(node) = self.findings[at].nodes.first() {
                    return CheckEffect::Select(node.clone());
                }
            }
            CheckInput::Fix(i) => {
                let Some(fix) = row(i).and_then(|at| self.findings[at].fix.clone()) else {
                    return CheckEffect::Paint;
                };
                if let Fix::AddHandler {
                    node,
                    event,
                    handler,
                } = fix
                {
                    return CheckEffect::Bind {
                        node,
                        event,
                        handler,
                    };
                }
                if let Some(tx) = fix.transaction(doc) {
                    return CheckEffect::Apply(tx);
                }
            }
            CheckInput::FixAll => {
                let mut all = Transaction::user("Fix all form-check findings");
                for fix in self.fixable() {
                    all.commands
                        .extend(fix.transaction(doc).into_iter().flat_map(|t| t.commands));
                }
                if !all.is_empty() {
                    return CheckEffect::Apply(all);
                }
            }
        }
        CheckEffect::Paint
    }

    /// The canvas marks: each finding's number on its first control, and
    /// every tied Tab stop.
    pub fn pins(&self) -> Vec<FindingPin> {
        let mut out = Vec::new();
        for (i, f) in self.findings.iter().enumerate() {
            let kind = if f.rule.is_error() {
                PinKind::Error
            } else {
                PinKind::Warning
            };
            if let Some(node) = f.nodes.first() {
                out.push(FindingPin {
                    node: node.clone(),
                    number: i + 1,
                    kind,
                });
            }
            if f.rule == crate::form_check::Rule::TabOrder {
                out.extend(f.nodes.iter().map(|node| FindingPin {
                    node: node.clone(),
                    number: i + 1,
                    kind: PinKind::TabClash,
                }));
            }
        }
        out
    }

    /// Hover tooltip data for finding `number` (1-based): (text, error, fix label).
    pub fn tip(&self, number: usize) -> Option<(String, bool, Option<String>)> {
        if number == 0 {
            return None;
        }
        let f = self.findings.get(number - 1)?;
        Some((
            f.text.clone(),
            f.rule.is_error(),
            f.fix.as_ref().map(Fix::label),
        ))
    }

    /// The properties behind the findings on `node`, for the Inspector.
    pub fn props_of(&self, node: &str) -> Vec<(&'static str, bool)> {
        self.findings
            .iter()
            .filter(|f| f.nodes.first().is_some_and(|n| n == node))
            .filter_map(|f| f.prop.map(|p| (p, f.rule.is_error())))
            .collect()
    }

    /// What `form.check` answers over MCP.
    pub fn to_json(&self, doc: &Document) -> serde_json::Value {
        let name = |id: &NodeId| doc.node(id).map_or(id.clone(), |n| n.name().to_string());
        let findings: Vec<serde_json::Value> = self
            .findings
            .iter()
            .enumerate()
            .map(|(i, f)| {
                serde_json::json!({
                    "number": i + 1,
                    "severity": if f.rule.is_error() { "error" } else { "warning" },
                    "rule": f.rule.label(),
                    "controls": f.nodes.iter().map(name).collect::<Vec<_>>(),
                    "property": f.prop,
                    "text": f.text,
                    "fix": f.fix.as_ref().map(Fix::label),
                })
            })
            .collect();
        serde_json::json!({ "summary": self.summary(), "findings": findings })
    }
}

/// The Tab-order badge of every control that has a Tab stop.
pub fn tab_badges(doc: &Document) -> Vec<(NodeId, String)> {
    doc.in_paint_order()
        .into_iter()
        .filter_map(|id| {
            let t = doc.node(&id)?.props.get("tab_order")?;
            let t = match t {
                Value::Int(i) => *i,
                _ => 0,
            };
            Some((id, t.to_string()))
        })
        .collect()
}

fn plural(n: usize, word: &str) -> String {
    match n {
        1 => format!("1 {word}"),
        n => format!("{n} {word}s"),
    }
}

#[cfg(test)]
#[path = "test/form_check_panel/test.rs"]
mod tests;
