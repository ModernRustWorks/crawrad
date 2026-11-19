//! Makepad component implementation for `RadioButton`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_CHECKED, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct RadioButtonCodegen;

impl MakepadComponentCodegen for RadioButtonCodegen {
    fn class(&self) -> &'static str {
        "RadioButton"
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
    let desc = crawrad_registry::components::radio_button();
    let mut fields = util::common_control_fields();
    fields.push(FIELD_CHECKED.to_string());
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct RadioButton {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_checked(node));
    inits.push(util::init_text(node));

    let desc = crawrad_registry::components::radio_button();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = inits.join(", ");
    format!("RadioButton {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    format!(
        "        if let Some(mut inner) = self.ui.radio_button(id!({field})).borrow_mut() {{\n\
         \x20           let is_active = inner.animator_in_state(cx, id!(active.on));\n\
         \x20           if is_active != self.form.{field}.checked {{\n\
         \x20               let state = if self.form.{field}.checked {{ id!(active.on) }} else {{ id!(active.off) }};\n\
         \x20               inner.animator_play(cx, state);\n\
         \x20           }}\n\
         \x20       }}\n{}",
        util::disabled_mark_sync(field, field)
    )
}

pub fn emit_actions(
    doc: &Document,
    node: &Node,
    field: &str,
    emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let on_click_call = match layout::handler(node, "on_click") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    // a radio whose `checked` really moved calls its own OnChange —
    // the sibling that loses the mark, and the one the click marks
    let on_change_of = |n: &Node| -> Result<String, crate::Erro> {
        Ok(match layout::handler(n, "on_change") {
            Some(h) => format!("                self.form.{}();\n", identificador(&h)?),
            None => String::new(),
        })
    };
    let parent_id = doc.parent_of(&node.id);
    let mut uncheck_siblings = String::new();
    for sibling in emitted {
        if sibling.class == "RadioButton"
            && sibling.id != node.id
            && doc.parent_of(&sibling.id) == parent_id
        {
            let sibling_field = identificador(sibling.name())?;
            uncheck_siblings.push_str(&format!(
                "            if self.form.{sibling_field}.checked {{\n\
                 \x20               self.form.{sibling_field}.checked = false;\n\
                 {}\
                 \x20           }}\n",
                on_change_of(sibling)?
            ));
        }
    }
    // with no OnChange of its own the mark is just set: an `if` that only
    // guards an assignment is `clippy::collapsible_if` when nothing else sits
    // beside it
    let own_change = on_change_of(node)?;
    let mark = if own_change.is_empty() {
        format!("            self.form.{field}.checked = true;\n")
    } else {
        format!(
            "            if !self.form.{field}.checked {{\n\
             \x20               self.form.{field}.checked = true;\n\
             {own_change}\
             \x20           }}\n"
        )
    };
    code.push_str(&format!(
        "        if let RadioButtonAction::Clicked = actions.find_widget_action_cast(self.ui.radio_button(id!({field})).widget_uid()) {{\n\
         {uncheck_siblings}\
         {mark}\
         {on_click_call}\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = caption.replace('\\', "\\\\").replace('"', "\\\"");
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <RadioButton> {{\n\
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
