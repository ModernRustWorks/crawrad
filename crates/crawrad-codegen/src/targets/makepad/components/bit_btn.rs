//! Makepad component implementation for `BitBtn`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct BitBtnCodegen;

impl MakepadComponentCodegen for BitBtnCodegen {
    fn class(&self) -> &'static str {
        "BitBtn"
    }

    fn default_cursor(&self) -> &'static str {
        "hand"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("BitBtn").expect("BitBtn desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct BitBtn {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("BitBtn").expect("BitBtn desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("BitBtn {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("BitBtn").expect("BitBtn desc");
        let fields = state::fields_of(desc);
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "        let _ = ({reads});\n\
             \x20       let btn = self.ui.button(id!({field}));\n\
             \x20       if btn.text() != self.form.{field}.text {{\n\
             \x20           btn.set_text(cx, &self.form.{field}.text);\n\
             \x20       }}\n{}",
            crate::targets::makepad::glyph::sync(field)
        )
    }

    fn emit_actions(
        &self,
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

    fn app_state(
        &self,
        _node: &Node,
        field: &str,
    ) -> Result<Option<super::codegen::AppState>, crate::Erro> {
        Ok(Some(crate::targets::makepad::glyph::app_state(field)))
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
        let pos = layout::posicao(x, y);
        let field = identificador(node.name())?;
        let (caption, _modal_result) = bit_btn_caption_and_modal_result(node);
        let lit_cap = crate::literal(&caption);
        Ok(format!(
            "            {field} = <Button> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               draw_bg: {{ color: #2b2d30 }},\n\
             \x20               draw_text: {{ color: #d4d4d4, text_style: {{ font_size: 9.0 }} }},\n\
             \x20               text: {lit_cap}\n\
             \x20           }}\n{}",
            crate::targets::makepad::glyph::dsl(&field, x, y, h)
        ))
    }
}

pub fn bit_btn_caption_and_modal_result(node: &Node) -> (String, &'static str) {
    let kind = match node.props.get("kind") {
        Some(crawrad_ir::Value::Enum(k)) => k.as_str(),
        _ => "bkCustom",
    };
    let (std_caption, default_mr) = match kind {
        "bkOK" => (Some("OK"), "mrOk"),
        "bkCancel" => (Some("Cancel"), "mrCancel"),
        "bkHelp" => (Some("Help"), "mrNone"),
        "bkClose" => (Some("Close"), "mrClose"),
        "bkYes" => (Some("Yes"), "mrYes"),
        "bkNo" => (Some("No"), "mrNo"),
        _ => (None, "mrNone"),
    };
    let text = if let Some(cap) = std_caption {
        let raw = node.text();
        if raw.is_none_or(|t| t.is_empty() || t == "BitBtn" || t == node.name()) {
            cap.to_string()
        } else {
            layout::text(node)
        }
    } else {
        layout::text(node)
    };
    let mr = match node.props.get("modal_result") {
        Some(crawrad_ir::Value::Enum(s)) => match s.as_str() {
            "mrOk" => "mrOk",
            "mrCancel" => "mrCancel",
            "mrYes" => "mrYes",
            "mrNo" => "mrNo",
            "mrClose" => "mrClose",
            "mrNone" => "mrNone",
            _ => default_mr,
        },
        _ => default_mr,
    };
    (text, mr)
}
