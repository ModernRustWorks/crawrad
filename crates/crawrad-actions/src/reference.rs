//! The manual page "Menus and commands", generated from the action catalogue.

use crate::{KeymapScheme, catalogue_with_scheme};
use crawrad_note::note;

#[cfg(test)]
#[path = "test/reference/test.rs"]
mod tests;

#[note("commands-reference-markdown")]
pub fn commands_reference_markdown() -> String {
    let vscode = catalogue_with_scheme(KeymapScheme::VsCode);
    let classic = catalogue_with_scheme(KeymapScheme::Classic);
    let mut groups: Vec<&str> = Vec::new();
    for a in &vscode {
        if !groups.contains(&a.group) {
            groups.push(a.group);
        }
    }
    let mut out = String::from("# Menus and commands\n\n");
    out.push_str(
        "This page is generated from the command catalogue: every command the IDE \
         offers, grouped by where you find it. The id is the name an agent uses to \
         run the command over MCP. Shortcuts are shown for both keymap \
         schemes, Modern (the default) and Classic, chosen in Tools > Options.\n",
    );
    for group in groups {
        out.push_str(&format!("\n## {group}\n\n"));
        out.push_str("| Command | Shortcut | Classic keymap | What it does | Id |\n");
        out.push_str("|---|---|---|---|---|\n");
        for (a, c) in vscode
            .iter()
            .zip(&classic)
            .filter(|(a, _)| a.group == group)
        {
            out.push_str(&format!(
                "| {} | {} | {} | {} | `{}` |\n",
                cell(a.label),
                key(a.shortcut),
                key(c.shortcut),
                cell(a.hint),
                a.id
            ));
        }
    }
    out
}

/// Escape pipe characters so a table cell stays one cell.
fn cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// A shortcut cell: the keys in backticks, or a dash when there is none.
fn key(shortcut: Option<&str>) -> String {
    match shortcut {
        Some(k) if !k.is_empty() => format!("`{}`", k.replace('|', "\\|")),
        _ => "—".to_string(),
    }
}
