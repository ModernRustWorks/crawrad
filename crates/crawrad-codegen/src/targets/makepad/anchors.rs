//! **The form's controls follow the window by their `Anchors`** in the
//! Makepad target: the call that hands each anchored control to
//! `crawrad_runtime_makepad::anchors::place` when the window's geometry changes.
//!
//! `Align` is spent here as the anchors it implies, as in the egui target
//! (`targets::egui::layout::ancoras`): the designer already laid the rect
//! out, and what the alignment adds over it is which edges follow the
//! window. Only the form's own controls take part: a control inside a
//! container is nested in the container's DSL, and goes where it goes.

use super::layout::is_inside_container;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-anchors-still")]
const STILL: &str = "Anchors { left: true, top: true, right: false, bottom: false }";

#[note("targets-makepad-anchors-anchors-call")]
pub fn anchors_call(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<String, Erro> {
    let Some(form) = doc.node(&doc.root).and_then(|r| r.rect()) else {
        return Ok(String::new());
    };
    let mut places = String::new();
    for n in emitted {
        if !reg.get(&n.class).is_some_and(|d| d.visual) || is_inside_container(doc, reg, &n.id) {
            continue;
        }
        let anchors = crate::targets::egui::layout::ancoras(n);
        if anchors == STILL {
            continue;
        }
        let Some(r) = doc.rect_in_form(&n.id) else {
            continue;
        };
        let f = identificador(n.name())?;
        places.push_str(&format!(
            "            crawrad_runtime_makepad::anchors::place(cx, &self.ui.widget(id!({f})), ({:.1}, {:.1}, {:.1}, {:.1}), crawrad_runtime_makepad::anchors::{anchors}, grown);\n",
            r.x, r.y, r.w, r.h
        ));
    }
    if places.is_empty() {
        return Ok(String::new());
    }
    Ok(format!(
        "        // The form's controls follow the window by their Anchors.\n\
         \x20       if let Event::WindowGeomChange(e) = event {{\n\
         \x20           let caption = self.ui.view(id!(caption_bar)).visible();\n\
         \x20           let grown = crawrad_runtime_makepad::anchors::grown(e.new_geom.inner_size, caption, dvec2({:.1}, {:.1}));\n\
         {places}\
         \x20           self.ui.redraw(cx);\n\
         \x20       }}\n",
        form.w, form.h
    ))
}

#[cfg(test)]
#[path = "../../test/makepad_anchors/test.rs"]
mod tests;
