//! Makepad component implementation for `SpeedButton`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct SpeedButtonCodegen;

impl MakepadComponentCodegen for SpeedButtonCodegen {
    fn class(&self) -> &'static str {
        "SpeedButton"
    }

    fn default_cursor(&self) -> &'static str {
        "hand"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SpeedButton").expect("SpeedButton desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct SpeedButton {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SpeedButton").expect("SpeedButton desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("SpeedButton {{ {body} }}")
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SpeedButton").expect("SpeedButton desc");
        // On a ToolBar, the bar's Flat and ShowCaptions count too
        let bar = crate::targets::makepad::glyph::tool_bar_of(doc, node)
            .ok()
            .flatten();
        let (caption, flat) = match &bar {
            Some(b) => (
                format!(
                    "if self.form.{b}.show_captions {{ self.form.{field}.text.as_str() }} else {{ \"\" }}"
                ),
                format!("self.form.{field}.flat || self.form.{b}.flat"),
            ),
            None => (
                format!("self.form.{field}.text.as_str()"),
                format!("self.form.{field}.flat"),
            ),
        };
        let fields = state::fields_of(desc);
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "        let _ = ({reads});\n\
             \x20       let btn = self.ui.button(id!({field}));\n\
             \x20       let caption = {caption};\n\
             \x20       if btn.text() != caption {{\n\
             \x20           btn.set_text(cx, caption);\n\
             \x20       }}\n\
             \x20       crawrad_runtime_makepad::speed::look(cx, &self.ui.widget(id!({field})), self.form.{field}.down, {flat});\n{}",
            crate::targets::makepad::glyph::sync(field)
        )
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // A click puts it down in its group (or lifts it), then OnClick
        let call = match layout::handler(node, "on_click") {
            Some(h) => format!("            self.form.{}();\n", identificador(&h)?),
            None => String::new(),
        };
        Ok(crate::targets::speed_groups::click_block(
            doc,
            node,
            field,
            "self.form.",
            &format!("self.ui.button(id!({field})).clicked(actions)"),
            &call,
        ))
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
        let caption = layout::text(node);
        let lit_cap = crate::literal(&caption);
        Ok(format!(
            "            {field} = <Button> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               draw_bg: {{ color: #1e1e1e, color_hover: #2c2c34, color_down: #2e477a }},\n\
             \x20               draw_text: {{ color: #cccccc, text_style: {{ font_size: 9.0 }} }},\n\
             \x20               text: {lit_cap}\n\
             \x20           }}\n{}",
            crate::targets::makepad::glyph::dsl(&field, x, y, h)
        ))
    }
}
