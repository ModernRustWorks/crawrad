//! Makepad component implementation for `CheckBox`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_CHECKED, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct CheckBoxCodegen;

impl MakepadComponentCodegen for CheckBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckBox"
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
    let desc = crawrad_registry::components::check_box();
    let mut fields = util::common_control_fields();
    fields.push(FIELD_CHECKED.to_string());
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct CheckBox {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_checked(node));
    inits.push(util::init_text(node));

    let desc = crawrad_registry::components::check_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = inits.join(", ");
    format!("CheckBox {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    format!(
        "        // `checked` is the truth for on/off; `state` follows it,\n\
         \x20       // except for cbGrayed, which only `state` can say\n\
         \x20       if self.form.{field}.state != \"cbGrayed\" {{\n\
         \x20           let want = if self.form.{field}.checked {{ \"cbChecked\" }} else {{ \"cbUnchecked\" }};\n\
         \x20           if self.form.{field}.state != want {{\n\
         \x20               self.form.{field}.state = want.to_string();\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       if let Some(mut inner) = self.ui.check_box(id!({field}_box)).borrow_mut() {{\n\
         \x20           let is_active = inner.animator_in_state(cx, id!(active.on));\n\
         \x20           if is_active != self.form.{field}.checked {{\n\
         \x20               let state = if self.form.{field}.checked {{ id!(active.on) }} else {{ id!(active.off) }};\n\
         \x20               inner.animator_play(cx, state);\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       // the caption, at the side `Alignment` says\n\
         \x20       crawrad_runtime_makepad::check::align(cx, &self.ui.label(id!({field}_lead)), &self.ui.check_box(id!({field}_box)), &self.form.{field}.text, &self.form.{field}.alignment);\n\
         \x20       crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({field}_box), self.form.{field}.enabled);\n\
         \x20       crawrad_runtime_makepad::check::grayed(cx, &self.ui.widget(id!({field}_box)), self.form.{field}.state == \"cbGrayed\");\n{}",
        util::disabled_mark_sync(field, &format!("{field}_box"))
    )
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let mut calls = String::new();
    // the click first, then the change, as the classic RAD IDE orders them
    for event in ["on_click", "on_change"] {
        if let Some(h) = layout::handler(node, event) {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("            self.form.{h_name}();\n"));
        }
    }
    // a click moves `state` one step (Unchecked -> Checked -> Grayed
    // when `allow_grayed`, a toggle otherwise), and `checked` follows it
    let wb = crate::targets::data_binding::makepad_writeback(_doc, node)?;
    code.push_str(&format!(
        "        if let Some(_val) = self.ui.check_box(id!({field}_box)).changed(actions) {{\n\
         \x20           let next = if self.form.{field}.allow_grayed {{\n\
         \x20               match self.form.{field}.state.as_str() {{\n\
         \x20                   \"cbUnchecked\" => \"cbChecked\",\n\
         \x20                   \"cbChecked\" => \"cbGrayed\",\n\
         \x20                   _ => \"cbUnchecked\",\n\
         \x20               }}\n\
         \x20           }} else if _val {{\n\
         \x20               \"cbChecked\"\n\
         \x20           }} else {{\n\
         \x20               \"cbUnchecked\"\n\
         \x20           }};\n\
         \x20           self.form.{field}.state = next.to_string();\n\
         \x20           self.form.{field}.checked = next == \"cbChecked\";\n\
         {wb}\
         {calls}\
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
        "            {name} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               flow: Right, align: {{ y: 0.5 }}\n\
         \x20               {name}_lead = <Label> {{ width: 0.0, text: \"\" }}\n\
         \x20               {name}_box = <CheckBox> {{\n\
         \x20                   width: Fill\n\
         \x20                   text: \"{caption_esc}\"\n\
         \x20               }}\n\
         \x20           }}\n"
    ));
    Ok(dsl)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
