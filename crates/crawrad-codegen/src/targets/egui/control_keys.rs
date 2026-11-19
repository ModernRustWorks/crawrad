//! **A control's own `OnKeyDown` / `OnKeyUp`** in the egui target: the keys
//! pressed and let go while the control has the focus.
//!
//! The form's keyboard is `KeyPreview` (`form_events`): every key of the
//! frame. A control's is narrower — only while it has the focus — and needs
//! the control's `Response`, so it is emitted inside the control's block,
//! right after the widget is added.
//!
//! The classes that deliver it are listed in [`CLASSES`]: a class enters the
//! list when its emitter calls [`dispatch`]. A class that declares the event
//! but is not listed would get a `crawrad_key` nobody calls (`dead_code` in the
//! user's project), so the list is what the conversions follow, never the
//! catalogue alone.

use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::Node;
use crawrad_note::note;

#[note("targets-egui-control-keys-classes")]
pub const CLASSES: [&str; 2] = ["Edit", "MaskEdit"];

#[note("targets-egui-control-keys-wants")]
pub fn wants(n: &Node) -> bool {
    CLASSES.contains(&n.class.as_str())
        && (handler(n, "on_key_down").is_some() || handler(n, "on_key_up").is_some())
}

#[note("targets-egui-control-keys-dispatch")]
pub fn dispatch(n: &Node) -> Result<String, Erro> {
    let down = handler(n, "on_key_down")
        .map(|h| identificador(&h))
        .transpose()?;
    let up = handler(n, "on_key_up")
        .map(|h| identificador(&h))
        .transpose()?;
    let call = match (&down, &up) {
        (Some(d), Some(u)) => format!(
            "                    if pressed {{\n\
             \x20                       self.{d}(e);\n\
             \x20                   }} else {{\n\
             \x20                       self.{u}(e);\n\
             \x20                   }}\n"
        ),
        (Some(d), None) => format!(
            "                    if pressed {{\n\
             \x20                       self.{d}(e);\n\
             \x20                   }}\n"
        ),
        (None, Some(u)) => format!(
            "                    if !pressed {{\n\
             \x20                       self.{u}(e);\n\
             \x20                   }}\n"
        ),
        (None, None) => return Ok(String::new()),
    };
    Ok(format!(
        "            if res.has_focus() || res.lost_focus() {{\n\
         \x20               let keys: Vec<(KeyEvent, bool)> = ui.input(|i| {{\n\
         \x20                   i.events\n\
         \x20                       .iter()\n\
         \x20                       .filter_map(|e| match e {{\n\
         \x20                           egui::Event::Key {{ key, pressed, modifiers, .. }} => Some((\n\
         \x20                               KeyEvent {{ key: crawrad_key(*key), shift: crawrad_shift(modifiers) }},\n\
         \x20                               *pressed,\n\
         \x20                           )),\n\
         \x20                           _ => None,\n\
         \x20                       }})\n\
         \x20                       .collect()\n\
         \x20               }});\n\
         \x20               for (e, pressed) in keys {{\n\
         {call}\
         \x20               }}\n\
         \x20           }}\n"
    ))
}
