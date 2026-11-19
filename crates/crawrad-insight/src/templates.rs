//! **Code templates**: ready snippets the editor inserts on `Ctrl+J`.
//! The shipped set is fixed here; the user's own come from the per-user file
//! (`crawrad_project::templates`) as plain entries. Each becomes one row of the
//! completion list — trigger, description, origin — whose body is an LSP
//! snippet, so the editor's own tabstops walk it. Pure.

use crate::item::{ItemKind, ItemSugestao};

/// Where a template comes from: shipped (read-only) or the user's file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateOrigin {
    BuiltIn,
    Mine,
}

impl TemplateOrigin {
    /// The word in the list's right column.
    pub fn label(self) -> &'static str {
        match self {
            Self::BuiltIn => "Built-in",
            Self::Mine => "Mine",
        }
    }
}

/// One template: typed `trigger`, shown `description`, inserted `body`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeTemplate {
    pub trigger: String,
    pub description: String,
    /// LSP snippet syntax: `${1:name}` stops, `$0` where the caret ends.
    pub body: String,
    pub origin: TemplateOrigin,
}

/// A user entry as the per-user file holds it: (trigger, description, body).
pub type TemplateEntry = (String, String, String);

const BUILTIN: &[(&str, &str, &str)] = &[
    (
        "evt",
        "Event handler",
        "pub fn ${1:btn_ok}_${2:click}(&mut self) {\n    $0\n}",
    ),
    (
        "evtk",
        "Key event handler",
        "pub fn ${1:edt_name}_key_down(&mut self, key: &str) {\n    $0\n}",
    ),
    (
        "each",
        "Iterate a dataset",
        "for ${1:row} in ${2:dataset}.rows() {\n    $0\n}",
    ),
    (
        "test",
        "Unit test",
        "#[test]\nfn ${1:it_works}() {\n    $0\n}",
    ),
    ("note", "Note link", "#[note(\"${1:name}\")]\n$0"),
    ("impl", "Impl block", "impl ${1:Type} {\n    $0\n}"),
    ("match", "Match", "match ${1:value} {\n    ${2:_} => $0,\n}"),
    (
        "ok",
        "Result function",
        "fn ${1:run}() -> Result<${2:()}, ${3:String}> {\n    $0\n    Ok(())\n}",
    ),
];

/// The shipped set: Rust and CrawRAD idioms.
pub fn builtin() -> Vec<CodeTemplate> {
    BUILTIN
        .iter()
        .map(|(t, d, b)| CodeTemplate {
            trigger: (*t).to_string(),
            description: (*d).to_string(),
            body: (*b).to_string(),
            origin: TemplateOrigin::BuiltIn,
        })
        .collect()
}

/// The user's entries as templates.
pub fn mine(entries: &[TemplateEntry]) -> Vec<CodeTemplate> {
    entries
        .iter()
        .map(|(t, d, b)| CodeTemplate {
            trigger: t.clone(),
            description: d.clone(),
            body: b.clone(),
            origin: TemplateOrigin::Mine,
        })
        .collect()
}

/// Everything the picker lists: the shipped set first, then the user's.
pub fn all(user: &[TemplateEntry]) -> Vec<CodeTemplate> {
    let mut list = builtin();
    list.extend(mine(user));
    list
}

/// The body as it will be written, stops filled with their placeholders.
pub fn preview(body: &str) -> String {
    crate::snippet::expand(body, "").text
}

/// The completion rows: the trigger, the description beside it, the origin
/// on the right and the preview in the panel beside the list.
pub fn items(templates: &[CodeTemplate]) -> Vec<ItemSugestao> {
    templates
        .iter()
        .map(|t| {
            let mut item = ItemSugestao::novo(t.trigger.clone(), ItemKind::Snippet)
                .com_detalhe(t.description.clone())
                .com_insert_text(t.body.clone());
            item.origin = Some(t.origin.label().to_string());
            item.doc = Some(preview(&t.body));
            item.snippet = true;
            item.filter = Some(t.trigger.clone());
            item
        })
        .collect()
}

/// Selection target in the code templates manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selected {
    BuiltIn(usize),
    Mine(usize),
}

/// A row rendered in the templates list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateRow {
    pub trigger: String,
    pub description: String,
    pub origin: TemplateOrigin,
    pub selected: bool,
    pub target: Selected,
}

/// Items that populate the manager's left column list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateListItem {
    Section(&'static str),
    Item(TemplateRow),
}

/// The pure state of the code templates manage dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateManager {
    pub mine: Vec<TemplateEntry>,
    pub selected: Selected,
    pub problem: Option<String>,
}

impl TemplateManager {
    /// Creates a new manager with the user's custom templates.
    pub fn new(user: Vec<TemplateEntry>) -> Self {
        Self {
            mine: user,
            selected: Selected::BuiltIn(0),
            problem: None,
        }
    }

    /// All rows (built-ins then Mine) with the selected flag set.
    pub fn rows(&self) -> Vec<TemplateRow> {
        let mut result = Vec::new();
        for (i, (tr, desc, _)) in BUILTIN.iter().enumerate() {
            result.push(TemplateRow {
                trigger: (*tr).to_string(),
                description: (*desc).to_string(),
                origin: TemplateOrigin::BuiltIn,
                selected: self.selected == Selected::BuiltIn(i),
                target: Selected::BuiltIn(i),
            });
        }
        for (i, (tr, desc, _)) in self.mine.iter().enumerate() {
            result.push(TemplateRow {
                trigger: tr.clone(),
                description: desc.clone(),
                origin: TemplateOrigin::Mine,
                selected: self.selected == Selected::Mine(i),
                target: Selected::Mine(i),
            });
        }
        result
    }

    /// List items including section headers for the left column.
    pub fn list_items(&self) -> Vec<TemplateListItem> {
        let mut items = Vec::new();
        items.push(TemplateListItem::Section(
            "BUILT-IN (read-only \u{2014} Duplicate to change)",
        ));
        for row in self
            .rows()
            .into_iter()
            .filter(|r| r.origin == TemplateOrigin::BuiltIn)
        {
            items.push(TemplateListItem::Item(row));
        }
        items.push(TemplateListItem::Section("MINE"));
        for row in self
            .rows()
            .into_iter()
            .filter(|r| r.origin == TemplateOrigin::Mine)
        {
            items.push(TemplateListItem::Item(row));
        }
        items
    }

    /// Selects a built-in or user entry.
    pub fn select(&mut self, target: Selected) {
        match target {
            Selected::BuiltIn(idx) => {
                let max = BUILTIN.len().saturating_sub(1);
                self.selected = Selected::BuiltIn(idx.min(max));
            }
            Selected::Mine(idx) => {
                if !self.mine.is_empty() {
                    let max = self.mine.len().saturating_sub(1);
                    self.selected = Selected::Mine(idx.min(max));
                }
            }
        }
    }

    /// Fields of the currently selected template: (trigger, description, body, read_only).
    pub fn fields(&self) -> (&str, &str, &str, bool) {
        match self.selected {
            Selected::BuiltIn(idx) => {
                if let Some(&(tr, desc, body)) = BUILTIN.get(idx) {
                    (tr, desc, body, true)
                } else {
                    ("", "", "", true)
                }
            }
            Selected::Mine(idx) => {
                if let Some((tr, desc, body)) = self.mine.get(idx) {
                    (tr.as_str(), desc.as_str(), body.as_str(), false)
                } else {
                    ("", "", "", false)
                }
            }
        }
    }

    /// Sets trigger of the selected entry (no-op on built-in).
    pub fn set_trigger(&mut self, val: impl Into<String>) {
        if let Selected::Mine(idx) = self.selected
            && let Some(entry) = self.mine.get_mut(idx)
        {
            entry.0 = val.into();
        }
    }

    /// Sets description of the selected entry (no-op on built-in).
    pub fn set_description(&mut self, val: impl Into<String>) {
        if let Selected::Mine(idx) = self.selected
            && let Some(entry) = self.mine.get_mut(idx)
        {
            entry.1 = val.into();
        }
    }

    /// Sets body of the selected entry (no-op on built-in).
    pub fn set_body(&mut self, val: impl Into<String>) {
        if let Selected::Mine(idx) = self.selected
            && let Some(entry) = self.mine.get_mut(idx)
        {
            entry.2 = val.into();
        }
    }

    /// Adds a new Mine template entry ("new", "new2", ...) and selects it.
    pub fn add_new(&mut self) {
        let trigger = if !self.mine.iter().any(|(t, _, _)| t == "new") {
            "new".to_string()
        } else {
            let mut counter = 2;
            loop {
                let candidate = format!("new{counter}");
                if !self.mine.iter().any(|(t, _, _)| t == &candidate) {
                    break candidate;
                }
                counter += 1;
            }
        };
        self.mine.push((trigger, String::new(), "$0".to_string()));
        self.selected = Selected::Mine(self.mine.len() - 1);
        self.problem = None;
    }

    /// Duplicates the selected entry as a Mine entry ("<trigger>2", ...) and selects it.
    pub fn duplicate(&mut self) {
        let (base_trigger, desc, body, _) = self.fields();
        let base_trigger = base_trigger.to_string();
        let desc = desc.to_string();
        let body = body.to_string();

        let mut counter = 2;
        let trigger = loop {
            let candidate = format!("{base_trigger}{counter}");
            if !self.mine.iter().any(|(t, _, _)| t == &candidate) {
                break candidate;
            }
            counter += 1;
        };

        self.mine.push((trigger, desc, body));
        self.selected = Selected::Mine(self.mine.len() - 1);
        self.problem = None;
    }

    /// Deletes the selected Mine entry (no-op on built-in).
    pub fn delete(&mut self) {
        if let Selected::Mine(idx) = self.selected
            && idx < self.mine.len()
        {
            self.mine.remove(idx);
            self.selected = match self.mine.len() {
                0 => Selected::BuiltIn(0),
                n => Selected::Mine(idx.min(n - 1)),
            };
            self.problem = None;
        }
    }

    /// Imports entries into Mine (replaces entries with matching trigger).
    pub fn import(&mut self, entries: Vec<TemplateEntry>) {
        for entry in entries {
            if let Some(pos) = self.mine.iter().position(|m| m.0 == entry.0) {
                self.mine[pos] = entry;
            } else {
                self.mine.push(entry);
            }
        }
    }

    /// Validates Mine entries: rejects empty triggers and duplicate triggers.
    pub fn validate(&self) -> Result<(), String> {
        for (i, a) in self.mine.iter().enumerate() {
            if a.0.trim().is_empty() {
                return Err("Trigger cannot be empty".to_string());
            }
            for b in self.mine.iter().skip(i + 1) {
                if a.0 == b.0 {
                    return Err(format!("Duplicate trigger '{}'", a.0));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/templates/test.rs"]
mod tests;
