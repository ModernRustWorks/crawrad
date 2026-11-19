//! Makepad component implementation for `Panel`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct PanelCodegen;

impl MakepadComponentCodegen for PanelCodegen {
    fn class(&self) -> &'static str {
        "Panel"
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
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_dsl(doc, reg, node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::panel();
    let mut fields = util::common_control_fields();
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Panel {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));

    let desc = crawrad_registry::components::panel();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("Panel {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, _field: &str) -> String {
    String::new()
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    if let Some(h) = layout::handler(node, "on_click") {
        let h_name = identificador(&h)?;
        code.push_str(&format!(
            "        if self.ui.view(id!({field})).finger_up(actions).is_some() {{\n\
             \x20           self.form.{h_name}();\n\
             \x20       }}\n"
        ));
    }
    Ok(code)
}

pub fn emit_dsl(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               flow: Overlay\n\
         \x20               show_bg: true\n\
         \x20               draw_bg: {{ color: #252538 }}\n"
    ));
    // a View tests the pointer only with a `cursor` (makepad 1.0.0
    // `view.rs:701`); without it the `finger_up` below never comes
    if layout::handler(node, "on_click").is_some() {
        dsl.push_str("                cursor: Default\n");
    }
    // always written, even empty: the caption and its font are
    // applied to it at run time
    {
        let caption_esc = caption.replace('\\', "\\\\").replace('"', "\\\"");
        dsl.push_str(&format!(
            "                label = <Label> {{\n\
             \x20                   align: {{x: 0.5, y: 0.5}}\n\
             \x20                   text: \"{caption_esc}\"\n\
             \x20               }}\n"
        ));
    }

    dsl.push_str(&super::emit_children(doc, reg, node, "    ")?);

    dsl.push_str("            }\n");
    Ok(dsl)
}

pub fn emitir_dsl(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    emit_dsl(doc, reg, node, x, y, w, h)
}
