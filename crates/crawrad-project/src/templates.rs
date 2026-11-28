//! The **templates** of the welcome screen — [ADR 0021].
//!
//! A template is the starting point that `New Project…` writes: the answer to
//! *"now what?"* from whoever just opened the IDE. The classic RAD IDE's `File → New →
//! Application` is the direct ancestor, and what is copied from it is the
//! promise: two seconds later there is a form on screen.
//!
//! # The template is a recipe, not a file
//!
//! The obvious path would be storing a ready `.crawform` in the binary with
//! `include_str!`. It was rejected by rule 9 of the study: a `Node` that is
//! not born from [`Registry::instantiate`] does not carry catalogue properties.
//! A frozen file starts out right **once** — the day `Button` gains a new
//! property, the template stays without it, and a project created here begins
//! to differ from a project made by dragging the same button from the palette.
//! Nothing breaks and nothing warns; that is how `Panel.border_width` lived
//! for months under a green test suite.
//!
//! The recipe never ages: it says *`Button` called `Button1` at (x, y, w,
//! h)*, and whatever answers what a `Button` is is the catalogue, every time.
//!
//! # A single `Document`
//!
//! [`Template::document`] is the only source: it is what the welcome screen
//! preview paints, and it is what `New Project…` saves. There is no second
//! path that could disagree with the first.
//!

use crawrad_ir::{Document, Rect};
use crawrad_note::note;
use crawrad_registry::Registry;

#[note("templates-templatepiece")]
pub struct TemplatePiece {
    pub class: &'static str,
    pub name: &'static str,
    /// The caption/text, when it is not the name. `&` is the accelerator key.
    pub text: Option<&'static str>,
    pub rect: Rect,
}

#[note("templates-template")]
pub struct Template {
    /// The label, in English like the rest of the interface.
    pub name: &'static str,
    /// What comes ready, in the list row. Short: it is a column, not a
    /// paragraph.
    pub description: &'static str,
    /// Form size in design units.
    pub size: (f32, f32),
    /// Form caption — classic RAD IDE's `Caption`.
    pub caption: &'static str,
    pub pieces: &'static [TemplatePiece],
}

#[note("templates-form")]
const FORM: (f32, f32) = (640.0, 420.0);

#[note("templates-templates")]
pub static TEMPLATES: &[Template] = &[Template {
    name: "Blank form",
    description: "an empty form, ready to design",
    size: FORM,
    caption: "Form1",
    pieces: &[],
}];

impl Template {
    #[note("templates-template-document")]
    pub fn document(&self, registry: &Registry) -> Option<Document> {
        let mut doc = Document::new("form_main", "Form");
        // The ROOT starts from the catalogue, like any piece — and that is the point of the module.
        //
        // `Document::new` sets up the root node with a raw `Node::new`: class, id,
        // and name, and nothing else. The catalogue `Form` has `enabled`, `visible`,
        // `hint`, `anchors`, `align` and `tab_order`, and without this the saved
        // form was born without any of them — rule 9 violated right in the module
        // that exists to enforce it. It did not show on screen because the Inspector
        // falls back to the descriptor's default when a key is missing, which is exactly
        // the disguise under which `Panel.border_width` lived for months.
        //
        // It replaces the ENTIRE node, and that is safe here and only here: it happens
        // before any `add_child`, so there is no child to lose along with it.
        let mut root = registry.instantiate(
            "Form",
            doc.root.clone(),
            Rect::new(0.0, 0.0, self.size.0, self.size.1),
        )?;
        root.set_text(self.caption);
        doc.nodes.insert(doc.root.clone(), root);
        for p in self.pieces {
            // The node starts from the CATALOGUE, always — the caption is written
            // on top afterwards, and never in its place. Building the node by hand
            // here to already have the right text would lose the other catalogue
            // properties, which is the defect this entire module avoids.
            let mut node = registry.instantiate(p.class, p.name, p.rect)?;
            if let Some(t) = p.text {
                node.set_text(t);
            }
            doc.add_child("form_main", node).ok()?;
        }
        Some(doc)
    }

    #[note("templates-template-default-template")]
    pub fn default_template() -> &'static Template {
        &TEMPLATES[0]
    }
}

pub const NP_SEM_MODELO: &str = "the default template is not in the catalog";
pub const CRIANDO_JA: &str = "already creating a project — wait for it to finish";
pub const NP_SEM_PASTA: &str = "choose a folder for the project — the field is empty";

#[cfg(test)]
#[path = "test/templates/test.rs"]
mod tests;
