//! The Lines Editor **without screen** — the multi-line text state, window
//! title, and line count.
//!
//! # Why it is a separate file
//!
//! Lives in `crawrad-designer`: it is pure Rust, and the line count ratchet
//! of `crawrad-ide` (ADR 0043) asks that what is pure leaves the shell.
//!
//! The same reason as `crate::items_editor::model`,
//! `crate::explorer::model`, and `crate::package_manager::model`: the Makepad
//! shell is another `[[bin]]`, and a `mod` does not cross from one binary to the
//! other. Without this separation, the `String List Editor` on this side would have its
//! own idea of how many lines a text has — and **two line counts
//! disagreeing in the same IDE is the kind of thing nobody notices until the day
//! they notice**, which is the sentence that was already written in [`count_lines`]
//! when it lived in drawing.
//!
//! # What is NOT here
//!
//! The field size in rows (`filas_do_campo`), which is an `egui::Ui` calculation,
//! and the drawing. Each shell draws the way its framework does, and both
//! ask the same thing of this file.

use crawrad_ir::Value;

#[derive(Debug, Clone)]
pub struct LinesEditorState {
    /// The stable identity of the node (ADR 0008) — addresses, does not appear.
    pub node_id: String,
    /// The component class. It provides half the title — see
    /// [`editor_title`].
    pub node_class: String,
    /// The label that appears in the content header.
    pub node_name: String,
    /// The **key** of the property in the model (`text`, on `Memo`).
    ///
    /// Comes from outside, from the `PropDesc` that opened this window, and not from a literal
    /// inside here. It is what separates this window from `Memo`: whoever declares
    /// `PropDesc::multilinha` on an `sql` key is edited by it without a single
    /// line changing.
    pub prop_key: String,
    /// The property label (`Lines`) — the other half of the title.
    pub prop_display: String,
    /// The text being edited. It lives HERE, and is not rebuilt from the document each
    /// frame: the text field of both frameworks treats the `String` it receives as the source of
    /// truth, and a rebuild per frame erases what
    /// was typed. That was how `Name` and later `OnClick` became
    /// mute.
    pub text: String,
}

impl LinesEditorState {
    pub fn from_node(
        node_id: &str,
        node: &crawrad_ir::Node,
        p: &crawrad_registry::PropDesc,
    ) -> Self {
        // The FACTORY value when the node does not have the key: it is the same that the
        // inspector shows, and opening the window on an empty text when the screen says
        // `3 lines` would be the window and the panel disagreeing.
        let text = match node.props.get(p.key).unwrap_or(&p.default) {
            Value::Str(s) => s.clone(),
            other => other.kind().to_string(),
        };
        Self {
            node_id: node_id.to_string(),
            node_class: node.class.clone(),
            node_name: node.name().to_string(),
            prop_key: p.key.to_string(),
            prop_display: p.display.to_string(),
            text,
        }
    }
}

/// The window title: `Memo Lines Editor`.
///
/// **Both halves come from the catalog**, and neither is a literal: the class and the
/// property's `display`. It is the same rule as
/// [`crawrad_actions::items_editor::editor_title`], and it exists because of the defect that
/// that file describes at length — an `if` with two arms choosing between
/// two literals for three components, and `ListBox` receiving the name of
/// `TreeView`.
///
/// Here the property comes along because this window is not for a single
/// component: `Memo.Lines` today, and `Query.SQL` the day someone declares
/// `PropDesc::multilinha` on it — which would come out as `Query SQL Editor`, without
/// this file changing.
pub fn editor_title(class: &str, property: &str) -> String {
    format!("{class} {property} Editor")
}

/// How many lines the text has.
///
/// # Why `split` and not `str::lines`
///
/// `"Line1\n"` is ONE line for `str::lines` and TWO for someone looking at the
/// field: the cursor is on the second, empty line, because the person pressed Enter. The
/// project already decided this once, in the code editor
/// (`a_linha_vazia_do_fim_conta_como_linha`), and two line counts
/// disagreeing in the same IDE is the kind of thing nobody notices until the day
/// they notice.
///
/// Empty text has ZERO lines, and not one: it is the `(empty)` of the summary, and is the
/// only point where `split` alone would lie (`"".split('\n')` counts 1).
pub fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.split('\n').count()
    }
}

/// The summary that the Object Inspector shows in place of the text: `3 lines`.
///
/// **What it CANNOT do is show the text.** Showing the text was the
/// previous defect — `Line1□Line2□Line3` stuck on a single line —, and showing it
/// truncated would be the same lie shorter. What serves at a glance is *how many
/// lines there are*, which is what `3 items` of `Items` already does on the line above.
///
/// The format is the SAME as `Items` on purpose: `(empty)`, `1 line`, `N lines`.
/// Two collections summarized with two different grammars in the same grid is
/// noise without information.
pub fn summary(s: &str) -> String {
    match count_lines(s) {
        0 => "(empty)".into(),
        1 => "1 line".into(),
        n => format!("{n} lines"),
    }
}
