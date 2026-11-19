//! Makepad component implementation for `Label`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct LabelCodegen;

impl MakepadComponentCodegen for LabelCodegen {
    fn class(&self) -> &'static str {
        "Label"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        emit_sync(doc, node, field)
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(doc, node, field, emitted)
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_dsl(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::label();
    let mut fields = util::common_control_fields();
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Label {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));

    let desc = crawrad_registry::components::label();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("Label {{ {body} }}")
}

#[crawrad_note::note("targets-makepad-components-label-emit-sync")]
pub fn emit_sync(_doc: &Document, node: &Node, field: &str) -> String {
    let (w, h) = node
        .rect()
        .map(|r| (r.w as f64, r.h as f64))
        .unwrap_or((100.0, 20.0));
    format!(
        "        let lbl = self.ui.label(id!({field}.{field}_text));\n\
         \x20       if lbl.text() != self.form.{field}.text {{\n\
         \x20           lbl.set_text(cx, &self.form.{field}.text);\n\
         \x20       }}\n\
         \x20       // where the text sits, whether it breaks, whether the box fits it\n\
         \x20       crawrad_runtime_makepad::label::apply(\n\
         \x20           cx,\n\
         \x20           &self.ui.widget(id!({field})),\n\
         \x20           &lbl,\n\
         \x20           &crawrad_runtime_makepad::label::LabelLook {{\n\
         \x20               alignment: &self.form.{field}.alignment,\n\
         \x20               layout: &self.form.{field}.layout,\n\
         \x20               word_wrap: self.form.{field}.word_wrap,\n\
         \x20               auto_size: self.form.{field}.auto_size,\n\
         \x20               size: ({w:?}, {h:?}),\n\
         \x20           }},\n\
         \x20       );\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    _node: &Node,
    _field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    Ok(String::new())
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = caption.replace('\\', "\\\\").replace('"', "\\\"");
    let mut dsl = String::new();
    // **`Label` COMES WRAPPED, and it is the only class that does.**
    //
    // # The framework bug, measured
    //
    // Makepad's `Label::draw_walk` spends the walk TWICE
    // (`makepad-widgets-1.0.0/src/label.rs:228-238`): it opens the turtle with
    // the walk — already offset by the margin — and hands **the same walk** to
    // `draw_text` inside. Because the margin is relative, the offset is applied
    // twice and the text lands outside its own turtle, which
    // `end_turtle_with_area` clips out. The label exists, has size, and does not
    // paint a single pixel.
    //
    // `Button` gets it right (`button.rs:721-725`) and so does `View`. So the
    // fix is giving the margin to what knows how to spend it: a `View` positions, and the
    // `Label` inside it starts with ZERO margin — doubling zero is zero.
    //
    // The instance name stays on the `View`, not on the `Label`: that is how
    // `sync_ui` reaches the component, and `.label()` steps down one level.
    dsl.push_str(&format!(
        "            {name} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               {name}_text = <Label> {{\n\
         \x20                   width: Fill, height: Fill\n\
         \x20                   text: \"{caption_esc}\"\n\
         \x20               }}\n\
         \x20           }}\n"
    ));
    Ok(dsl)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
