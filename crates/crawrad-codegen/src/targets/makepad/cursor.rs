//! **The pointer over a control, and over the form**, on Makepad.
//!
//! One block after `self.ui.handle_event`, and not an arm of the pointer
//! router: the widgets set their own pointer while they handle the event (a
//! button its hand, an edit its text beam), and something in that pass puts
//! the arrow back, so a pointer set before it does not last. Set after it,
//! ours is the last word of the event.
//!
//! On every move the topmost visible and enabled control under the pointer
//! answers with its `Cursor`; `Default` is the control's own pointer
//! ([`default_cursor`](super::components::codegen::MakepadComponentCodegen::default_cursor)),
//! as the classic `crDefault` is. Over no control, the form's `Cursor`
//! answers, the arrow when it has none.
//!
//! Makepad 1.0 shows `Wait` as the arrow on Windows (`win32_app.rs` maps
//! `MouseCursor::Wait` to `IDC_ARROW`): the app asks for it, the platform
//! does not draw it.

use super::components;
use super::layout::{has_widget, visibility_expression};
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-cursor-form-cursor-name")]
fn form_cursor_name(doc: &Document) -> Option<&'static str> {
    match doc.node(&doc.root)?.props.get("cursor") {
        Some(crawrad_ir::Value::Cursor(c)) if *c != crawrad_ir::Cursor::Default => Some(c.as_str()),
        _ => None,
    }
}

#[note("targets-makepad-cursor-after-widgets")]
pub fn after_widgets(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<String, Erro> {
    let mut checks = String::new();
    // the last drawn is the one on top
    for n in emitted.iter().rev() {
        if !has_widget(reg, &n.class)
            || !components::for_class(&n.class)
                .is_some_and(|c| c.struct_definition().contains("pub cursor:"))
        {
            continue;
        }
        let natural = components::for_class(&n.class).map_or("arrow", |c| c.default_cursor());
        let field = identificador(n.name())?;
        let enabled = if components::has_enabled_field(&n.class) {
            format!(" && self.form.{field}.enabled")
        } else {
            String::new()
        };
        let visible = visibility_expression(doc, reg, n)?;
        checks.push_str(&format!(
            "            if cursor.is_none() && {visible}{enabled} && self.ui.widget(id!({field})).area().rect(cx).contains(e.abs) {{\n\
             \x20               cursor = Some(match self.form.{field}.cursor {{ Cursor::Default => \"{natural}\", c => c.as_str() }});\n\
             \x20           }}\n"
        ));
    }
    let form = form_cursor_name(doc);
    if checks.is_empty() {
        return Ok(match form {
            Some(name) => format!(
                "        // the form's Cursor\n\
                 \x20       if let Event::MouseMove(_) = event {{\n\
                 \x20           crawrad_runtime_makepad::cursor::show(cx, \"{name}\");\n\
                 \x20       }}\n"
            ),
            None => String::new(),
        });
    }
    let over_none = match form {
        Some(name) => format!("\"{name}\""),
        None => "\"arrow\"".to_string(),
    };
    Ok(format!(
        "        // the Cursor of the control under the pointer, or the form's\n\
         \x20       if let Event::MouseMove(e) = event {{\n\
         \x20           let mut cursor: Option<&str> = None;\n\
         {checks}\
         \x20           crawrad_runtime_makepad::cursor::show(cx, cursor.unwrap_or({over_none}));\n\
         \x20       }}\n"
    ))
}

#[cfg(test)]
#[path = "../../test/makepad_cursor/test.rs"]
mod tests;
