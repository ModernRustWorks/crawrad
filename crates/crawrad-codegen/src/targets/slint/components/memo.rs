//! Slint component implementation for `Memo`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct MemoCodegen;

impl SlintComponentCodegen for MemoCodegen {
    fn class(&self) -> &'static str {
        "Memo"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(node, field)
    }

    fn emit_slint(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(node, x, y, w, h)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_edited(string);")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::memo();
    let mut fields = vec![
        util::FIELD_VISIBLE.to_string(),
        util::FIELD_ENABLED.to_string(),
        util::FIELD_LINES.to_string(),
        util::FIELD_TEXT.to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text", "scroll_bars"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Memo {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let txt = layout::text(node);
    let lines_lit = crate::literal(&txt);
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("lines: Strings::from_text({lines_lit})"),
        format!("text: {lines_lit}.to_string()"),
    ];
    let desc = crawrad_registry::components::memo();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text", "scroll_bars"],
    ));
    let body = inits.join(", ");
    format!("Memo {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let on_change_call = match layout::event_handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("                    f.form.{h_name}();\n")
        }
        None => String::new(),
    };
    code.push_str(&format!(
        "        {{\n\
         \x20           let form = app.clone();\n\
         \x20           window.on_{field}_edited(move |val| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.text = val.to_string();\n\
         {on_change_call}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let txt = layout::text(node);
    let text_esc = util::escape_slint_string(&txt);
    let en = layout::is_enabled(node);
    let read_only = matches!(
        node.props.get("read_only"),
        Some(crawrad_ir::Value::Bool(true))
    );
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := TextEdit {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            text: \"{text_esc}\";\n"));
    if !en {
        out.push_str("            enabled: false;\n");
    }
    if read_only {
        out.push_str("            read-only: true;\n");
    }
    out.push_str(&format!(
        "            edited(val) => {{ root.{name}_edited(val); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
