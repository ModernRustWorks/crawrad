//! **The editor's own chords** — what a key the code editor's framework leaves
//! unwired asks the editor to do, in each keymap scheme (ADR 0011).
//!
//! Plain data over chord names (`Ctrl+Shift+K`), so every rule is measured with
//! no window: the shell only says whether a name is the key just pressed
//! (`matches_shortcut`), the same way the catalogue's shortcuts are read.
//!
//! # `Ctrl+Y` was here for one round, and the owner took it out
//!
//! It redid the text, on the chord the catalogue gives `edit.redo`. He tried it
//! and ruled:
//!
//! > *"Ctrl+Y does not do it, but the [modern scheme] standard doesn't either, that's only in [the classic one] which
//! > deletes the line. So Ctrl+Y should only work when we have the switch
//! > keymap scheme option and select [the classic one]."*
//!
//! So in the Modern scheme the key is **deliberately silent**; in the Classic
//! scheme it is *delete line*, not redo. Silent is not dangerous: the chord is in
//! the catalogue's `Edit` group, so with the cursor in a file
//! `crawrad_actions::key_routing` keeps it away from the FORM's history. Redo in
//! the text is `Ctrl+Shift+Z`.

pub use crate::word_motion::Motion;
use crawrad_note::note;

#[note("editor-chords-navigation")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Navigation {
    pub motion: Motion,
    /// `Shift` held: drag the anchor along instead of collapsing the selection.
    pub extend: bool,
}

#[note("editor-chords-linedirection")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineDirection {
    Up,
    Down,
}

#[note("editor-chords-command")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Move(Navigation),
    ToggleComment,
    MoveLines(LineDirection),
    DuplicateLines(LineDirection),
    DeleteLine,
    NextOccurrence,
    /// `true`: with the replace field.
    OpenFind(bool),
    FindNext,
    FindPrevious,
    OpenGoto,
    /// A catalogue action by id, run as the menu and MCP run it.
    RunAction(&'static str),
    Fold,
    Unfold,
    Complete,
    /// Modern F2 with the code editor focused; elsewhere F2 is
    /// the Explorer's rename.
    RenameSymbol,
    /// Numbered bookmark: `set` toggles it on the caret's line, else go to it.
    Bookmark {
        number: u8,
        set: bool,
    },
}

#[note("editor-chords-chord")]
type Chord = (Option<bool>, &'static str, Command);

#[rustfmt::skip]
const CHORDS: &[Chord] = {
    use Command::*;
    use LineDirection::{Down, Up};
    const V: Option<bool> = Some(false);
    const C: Option<bool> = Some(true);
    &[
        (None, "Ctrl+/", ToggleComment),
        (None, "Ctrl+;", ToggleComment),
        (C, "Ctrl+Shift+C", ToggleComment),
        // The classic Ctrl+D is the vendor's Format Source.
        (V, "Ctrl+D", NextOccurrence),
        (C, "Ctrl+Shift+D", NextOccurrence),
        (None, "Ctrl+F", OpenFind(false)),
        (V, "Ctrl+H", OpenFind(true)),
        (C, "Ctrl+R", OpenFind(true)),
        (V, "Ctrl+G", OpenGoto),
        (C, "Alt+G", OpenGoto),
        (V, "Ctrl+P", RunAction("file.quick_open")),
        (C, "Ctrl+O", RunAction("file.quick_open")),
        (None, "Ctrl+Shift+P", RunAction("view.command_palette")),
        (None, "F3", FindNext),
        (None, "Shift+F3", FindPrevious),
        // The classic Alt+Up is the vendor's Find Declaration.
        (V, "Alt+Up", MoveLines(Up)),
        (V, "Alt+Down", MoveLines(Down)),
        (C, "Ctrl+Alt+Up", MoveLines(Up)),
        (C, "Ctrl+Alt+Down", MoveLines(Down)),
        (None, "Shift+Alt+Up", DuplicateLines(Up)),
        (None, "Shift+Alt+Down", DuplicateLines(Down)),
        (V, "Ctrl+Shift+K", DeleteLine),
        (C, "Ctrl+Y", DeleteLine),
        (None, "Ctrl+Shift+N", RunAction("tools.sidecar_note")),
        (None, "Ctrl+Alt+D", RunAction("editor.add_documentation")),
        (None, "Ctrl+Alt+N", RunAction("editor.add_note")),
        (None, "Ctrl+Alt+M", RunAction("editor.add_comment")),
        (None, "Ctrl+Shift+[", Fold),
        (None, "Ctrl+Shift+]", Unfold),
        (V, "F2", RenameSymbol),
        (None, "Ctrl+Space", Complete),
        (None, "Ctrl+J", RunAction("editor.templates")),
    ]
};

#[note("editor-chords-navigation-2")]
pub fn navigation(is: &dyn Fn(&str) -> bool) -> Option<Navigation> {
    const KEYS: [(&str, Motion); 4] = [
        ("Left", Motion::WordLeft),
        ("Right", Motion::WordRight),
        ("Home", Motion::FileStart),
        ("End", Motion::FileEnd),
    ];
    KEYS.iter().find_map(|&(key, motion)| {
        let extend = is(&format!("Ctrl+Shift+{key}"));
        (extend || is(&format!("Ctrl+{key}"))).then_some(Navigation { motion, extend })
    })
}

#[note("editor-chords-command-2")]
pub fn command(is: &dyn Fn(&str) -> bool, classic: bool) -> Option<Command> {
    if let Some(n) = navigation(is) {
        return Some(Command::Move(n));
    }
    let chord = CHORDS
        .iter()
        .find(|(scheme, name, _)| scheme.is_none_or(|c| c == classic) && is(name));
    if let Some((.., command)) = chord {
        return Some(*command);
    }
    // Ctrl+Shift+1..9 sets a numbered bookmark, Ctrl+1..9 goes to it.
    (1..=9u8).find_map(|number| {
        let set = is(&format!("Ctrl+Shift+{number}"));
        (set || is(&format!("Ctrl+{number}"))).then_some(Command::Bookmark { number, set })
    })
}

/// What a chord does, in the words of the manual.
fn describe(command: Command) -> String {
    match command {
        Command::Move(_) | Command::Bookmark { .. } => String::new(),
        Command::ToggleComment => "Comment or uncomment the lines".into(),
        Command::MoveLines(LineDirection::Up) => "Move the lines up".into(),
        Command::MoveLines(LineDirection::Down) => "Move the lines down".into(),
        Command::DuplicateLines(LineDirection::Up) => "Copy the lines up".into(),
        Command::DuplicateLines(LineDirection::Down) => "Copy the lines down".into(),
        Command::DeleteLine => "Delete the line".into(),
        Command::NextOccurrence => "Select the next occurrence".into(),
        Command::OpenFind(false) => "Find".into(),
        Command::OpenFind(true) => "Find and replace".into(),
        Command::FindNext => "Find next".into(),
        Command::FindPrevious => "Find previous".into(),
        Command::OpenGoto => "Go to line".into(),
        Command::RunAction(id) => {
            let label = match id {
                "file.quick_open" => "Quick Open",
                "view.command_palette" => "Command Palette",
                "tools.sidecar_note" => "Notes",
                "editor.add_documentation" => "Add documentation",
                "editor.add_note" => "Add note",
                "editor.add_comment" => "Add comment",
                "editor.templates" => "Code Templates",
                _ => "Run the command",
            };
            format!("{label} (`{id}`)")
        }
        Command::Fold => "Fold the block".into(),
        Command::Unfold => "Unfold the block".into(),
        Command::Complete => "Show completions".into(),
        Command::RenameSymbol => "Rename the symbol".into(),
    }
}

#[note("editor-chords-reference-markdown")]
pub fn shortcuts_reference_markdown() -> String {
    let mut rows: Vec<(String, Vec<&str>, Vec<&str>)> = Vec::new();
    for &(scheme, name, command) in CHORDS {
        let what = describe(command);
        let at = match rows.iter().position(|r| r.0 == what) {
            Some(i) => i,
            None => {
                rows.push((what, Vec::new(), Vec::new()));
                rows.len() - 1
            }
        };
        if scheme != Some(true) {
            rows[at].1.push(name);
        }
        if scheme != Some(false) {
            rows[at].2.push(name);
        }
    }
    let keys = |k: &[&str]| -> String {
        if k.is_empty() {
            "—".into()
        } else {
            k.iter()
                .map(|k| format!("`{k}`"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    };
    let mut out = String::from("# Keyboard shortcuts\n\n");
    out.push_str(
        "This page is generated from the code editor's chord table. The shortcuts \
         of the IDE commands are listed with each command in Menus and commands. \
         The keymap scheme, Modern (the default) or Classic, is chosen in \
         Tools > Options.\n\n## Code editor\n\n",
    );
    out.push_str("| Action | Modern | Classic |\n|---|---|---|\n");
    for (what, modern, classic) in &rows {
        out.push_str(&format!(
            "| {what} | {} | {} |\n",
            keys(modern),
            keys(classic)
        ));
    }
    out.push_str(
        "\n## Moving the cursor\n\n\
         | Action | Keys |\n|---|---|\n\
         | One word left / right | `Ctrl+Left`, `Ctrl+Right` |\n\
         | Start / end of the file | `Ctrl+Home`, `Ctrl+End` |\n\
         | The same, extending the selection | add `Shift` |\n\
         \n## Bookmarks\n\n\
         | Action | Keys |\n|---|---|\n\
         | Set bookmark 1 to 9 | `Ctrl+Shift+1` … `Ctrl+Shift+9` |\n\
         | Go to bookmark 1 to 9 | `Ctrl+1` … `Ctrl+9` |\n",
    );
    out
}

#[cfg(test)]
#[path = "test/editor_chords/test.rs"]
mod tests;
