//! Makepad component implementation for `Button`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct ButtonCodegen;

impl MakepadComponentCodegen for ButtonCodegen {
    fn class(&self) -> &'static str {
        "Button"
    }

    fn default_cursor(&self) -> &'static str {
        "hand"
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
    let desc = crawrad_registry::components::button();
    let mut fields = util::common_control_fields();
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Button {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));

    let desc = crawrad_registry::components::button();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("Button {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    format!(
        "        let btn = self.ui.button(id!({field}));\n\
         \x20       if btn.text() != self.form.{field}.text {{\n\
         \x20           btn.set_text(cx, &self.form.{field}.text);\n\
         \x20       }}\n"
    )
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
            "        if self.ui.button(id!({field})).clicked(actions) {{\n\
             \x20           self.form.{h_name}();\n\
             \x20       }}\n"
        ));
    }
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = caption.replace('\\', "\\\\").replace('"', "\\\"");
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <Button> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               text: \"{caption_esc}\"\n\
         \x20           }}\n"
    ));
    Ok(dsl)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
