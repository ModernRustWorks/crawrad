//! Makepad component implementation for `Edit`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_TEXT};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct EditCodegen;

impl MakepadComponentCodegen for EditCodegen {
    fn class(&self) -> &'static str {
        "Edit"
    }

    fn default_cursor(&self) -> &'static str {
        "text"
    }

    fn enter_key(&self) -> super::EnterKey {
        super::EnterKey::Swallowed
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
    struct_of(&crawrad_registry::components::edit())
}

#[crawrad_note::note("targets-makepad-components-edit-struct-of")]
pub fn struct_of(desc: &crawrad_registry::ComponentDesc) -> String {
    let mut fields = util::common_control_fields();
    fields.push(FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!(
        "#[derive(Debug, Clone, Default)]\npub struct {} {{\n{body}\n}}\n",
        desc.class
    )
}

pub fn emit_default(node: &Node) -> String {
    let text = format!("{}.to_string()", cased_text_literal(node));
    default_of(node, &crawrad_registry::components::edit(), &text)
}

#[crawrad_note::note("targets-makepad-components-edit-cased-text-literal")]
pub fn cased_text_literal(node: &Node) -> String {
    crate::literal(&crawrad_ir::char_case::cased_for(node, &layout::text(node)))
}

#[crawrad_note::note("targets-makepad-components-edit-default-of")]
pub fn default_of(node: &Node, desc: &crawrad_registry::ComponentDesc, text: &str) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(format!("text: {text}"));
    inits.extend(util::catalogue_inits(
        node,
        desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("{} {{ {body} }}", desc.class)
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    let mut code = format!(
        "        let ti = self.ui.text_input(id!({field}));\n\
         \x20       if ti.text() != self.form.{field}.text {{\n\
         \x20           ti.set_text(cx, &self.form.{field}.text);\n\
         \x20       }}\n"
    );
    code.push_str(&util::text_input_flags_sync(field, true));
    code.push_str(&format!(
        "        if self.form.{field}.numbers_only {{\n\
         \x20           crawrad_runtime_makepad::edit::set_numbers_only(cx, &ti, true);\n\
         \x20       }}\n\
         \x20       crawrad_runtime_makepad::edit::set_text_hint(cx, &ti, &self.form.{field}.text_hint);\n\
         \x20       crawrad_runtime_makepad::edit::set_alignment(cx, &ti, &self.form.{field}.alignment);\n"
    ));
    code
}

pub fn emit_actions(
    document: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    actions_of(document, node, field, "")
}

#[crawrad_note::note("targets-makepad-components-edit-actions-of")]
pub fn actions_of(
    document: &Document,
    node: &Node,
    field: &str,
    hold: &str,
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let wb = crate::targets::data_binding::makepad_writeback(document, node)?;
    code.push_str(&format!(
        "        if let Some(_val) = self.ui.text_input(id!({field})).changed(actions) {{\n\
         \x20           let _val = crawrad_runtime_makepad::memo::hold_to(cx, &self.ui.text_input(id!({field})), &_val, self.form.{field}.max_length).unwrap_or(_val);\n\
         \x20           let _val = crawrad_runtime_makepad::edit::hold_case(cx, &self.ui.text_input(id!({field})), &_val, &self.form.{field}.char_case).unwrap_or(_val);\n\
         {hold}\
         \x20           self.form.{field}.text = _val;\n\
         {wb}\
         {on_change_call}\
         \x20       }}\n"
    ));
    if let Some(h) = layout::handler(node, "on_return") {
        let h_name = identificador(&h)?;
        code.push_str(&format!(
            "        if self.ui.text_input(id!({field})).returned(actions).is_some() {{\n\
             \x20           self.form.{h_name}();\n\
             \x20       }}\n"
        ));
    }
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let text = crawrad_ir::char_case::cased_for(node, layout::str_val(node, "text").unwrap_or(""));
    let text_esc = text.replace('\\', "\\\\").replace('"', "\\\"");
    let text_hint = layout::str_val(node, "text_hint").unwrap_or("");
    let hint_esc = text_hint.replace('\\', "\\\\").replace('"', "\\\"");
    let flags = util::text_input_flags(node);
    let numeric = if matches!(
        node.props.get("numbers_only"),
        Some(crawrad_ir::Value::Bool(true))
    ) {
        "                is_numeric_only: true\n"
    } else {
        ""
    };
    let alignment = match node.props.get("alignment") {
        Some(crawrad_ir::Value::Enum(s)) | Some(crawrad_ir::Value::Str(s)) => s.as_str(),
        _ => "taLeftJustify",
    };
    let ax = match alignment {
        "taCenter" => 0.5,
        "taRightJustify" => 1.0,
        _ => 0.0,
    };
    let align_dsl = if ax > 0.0 {
        format!("                label_align: {{ x: {ax:.1}, y: 0.0 }}\n")
    } else {
        String::new()
    };
    // the line sits with its CAPITALS in the middle of the box, by the
    // one rule the runtime's own controls use; the theme's top padding put it
    // against the top of a fixed-height Edit.
    let top = text_top(node, h);
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <TextInput> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               padding: {{ top: {top:.1}, bottom: 0.0, left: 6.0, right: 6.0 }}\n\
         \x20               empty_text: \"{hint_esc}\"\n\
         \x20               text: \"{text_esc}\"\n\
         {flags}\
         {numeric}\
         {align_dsl}\
         \x20           }}\n"
    ));
    Ok(dsl)
}

#[crawrad_note::note("targets-makepad-components-edit-text-top")]
pub fn text_top(node: &Node, h: f32) -> f32 {
    use crawrad_component::text_line::{cap_centred_top, font_px};
    cap_centred_top(h, font_px(font_points(node))).max(0.0)
}

#[crawrad_note::note("targets-makepad-components-edit-font-points")]
fn font_points(node: &Node) -> f32 {
    let parent = !matches!(
        node.props.get("parent_font"),
        Some(crawrad_ir::Value::Bool(false))
    );
    match node.props.get("font") {
        Some(crawrad_ir::Value::Font(f)) if !parent && f.size > 0.0 => f.size,
        _ => 10.0,
    }
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
