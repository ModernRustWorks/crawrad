//! Reference documentation generation from the component catalog.

use crate::{ComponentDesc, PropKind, Registry};
use crawrad_ir::Value;
use crawrad_note::note;
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "test/reference/test.rs"]
mod tests;

impl Registry {
    #[note("registry-reference-markdown")]
    pub fn reference_markdown(&self) -> String {
        let mut by_palette: BTreeMap<&'static str, Vec<&ComponentDesc>> = BTreeMap::new();
        for d in self.all() {
            if !d.palette.is_empty() {
                by_palette.entry(d.palette).or_default().push(d);
            }
        }

        let mut out = String::new();
        out.push_str("# Components and properties\n\n");
        out.push_str(
            "This page is generated from the component catalogue; every component on the palette is listed here.\n",
        );

        for (palette, mut comps) in by_palette {
            out.push_str(&format!("\n## {palette}\n"));
            comps.sort_by_key(|d| d.display);
            for d in comps {
                out.push_str(&format!("\n### {}\n\n", d.display));
                out.push_str(&format!(
                    "Class `{}` · born as `{}1` · targets: {}\n\n",
                    d.class,
                    d.base_name,
                    targets_to_text(d.targets)
                ));

                out.push_str("#### Properties\n\n");
                out.push_str("| Property | Kind | Default | Category |\n");
                out.push_str("|---|---|---|---|\n");
                for p in &d.props {
                    let prop_name = if p.readonly {
                        format!("{} (read-only)", p.display)
                    } else {
                        p.display.to_string()
                    };
                    let kind_name = kind_to_text(&p.kind);
                    let default_text = value_to_text(&p.default);
                    let default_val = if default_text.is_empty() {
                        "—"
                    } else {
                        &default_text
                    };
                    let cat_name = format!("{:?}", p.category);

                    out.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        escape_cell(&prop_name),
                        escape_cell(&kind_name),
                        escape_cell(default_val),
                        escape_cell(&cat_name)
                    ));
                }

                if !d.events.is_empty() {
                    out.push_str("\n#### Events\n\n");
                    out.push_str("| Event | Handler signature |\n");
                    out.push_str("|---|---|\n");
                    for ev in &d.events {
                        out.push_str(&format!(
                            "| `{}` | `{}` |\n",
                            escape_cell(ev.display),
                            escape_cell(ev.signature)
                        ));
                    }
                }
            }
        }

        out
    }
}

/// Escape pipe characters in table cells.
fn escape_cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Helper to render a [`Value`] to text.
fn value_to_text(value: &Value) -> String {
    match value {
        Value::Str(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => format!("{f:.2}"),
        Value::Bool(b) => b.to_string(),
        Value::Enum(e) => e.clone(),
        Value::Ref(r) => r.clone(),
        Value::Cursor(c) => c.as_str().to_string(),
        Value::ModalResult(m) => m.as_str().to_string(),
        Value::FilePath(fp) => fp.path.clone(),
        Value::DateTime(dt) => dt.to_iso8601(),
        other => other.describe(),
    }
}

/// Targets line: `*` means every target.
fn targets_to_text(targets: &[&str]) -> String {
    if targets.is_empty() || targets.contains(&"*") {
        "all".to_string()
    } else {
        targets.join(", ")
    }
}

/// The kind of a property, as the reader of the manual would say it.
fn kind_to_text(kind: &PropKind) -> String {
    match kind {
        PropKind::String => "text".into(),
        PropKind::Multiline => "text (several lines)".into(),
        PropKind::Bool => "true / false".into(),
        PropKind::Int { min, max } => format!("whole number {min}..{max}"),
        PropKind::Float { min, max, .. } => format!("number {min}..{max}"),
        PropKind::Color => "color".into(),
        PropKind::Edges => "edges (left, top, right, bottom)".into(),
        PropKind::Set(items) => format!("any of: {}", items.join(", ")),
        PropKind::Enum(items) => format!("one of: {}", items.join(", ")),
        PropKind::Collection { item_class, .. } => match item_class {
            Some(class) => format!("collection of {class}"),
            None => "collection".into(),
        },
        PropKind::Reference(class) => format!("reference to a {class}"),
        PropKind::FieldOf(prop) => format!("field of {prop}"),
        PropKind::Rect => "position and size".into(),
        PropKind::DateTime => "date and time".into(),
        PropKind::FilePath => "file path".into(),
        PropKind::Cursor => "mouse cursor".into(),
        PropKind::ModalResult => "modal result".into(),
        PropKind::Shortcut => "keyboard shortcut".into(),
        PropKind::Font => "font".into(),
        PropKind::Picture => "picture".into(),
    }
}
