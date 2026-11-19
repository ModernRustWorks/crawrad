//! Makepad component implementation for `StatusBar`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct StatusBarCodegen;

impl MakepadComponentCodegen for StatusBarCodegen {
    fn class(&self) -> &'static str {
        "StatusBar"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("StatusBar").expect("StatusBar desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct StatusBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("StatusBar").expect("StatusBar desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("StatusBar {{ {body} }}")
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("StatusBar").expect("StatusBar desc");
        let fields = state::fields_of(desc);
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        // SimplePanel shows the simple text, off it shows the panels.
        // A makepad Label does not hide with `set_visible`, so the simple
        // text is emptied instead
        let panels = panels_of(doc, node);
        let shown = if panels.is_empty() {
            format!("self.form.{field}.simple_text.as_str()")
        } else {
            format!(
                "if self.form.{field}.simple_panel {{ self.form.{field}.simple_text.as_str() }} else {{ \"\" }}"
            )
        };
        let mut s = format!(
            "        let _ = ({reads});\n\
             \x20       let lbl = self.ui.label(id!({field}.lbl));\n\
             \x20       let shown = {shown};\n\
             \x20       if lbl.text() != shown {{\n\
             \x20           lbl.set_text(cx, shown);\n\
             \x20       }}\n"
        );
        if !panels.is_empty() {
            s.push_str(&format!(
                "        self.ui.widget(id!({field}.panels)).set_visible(cx, !self.form.{field}.simple_panel);\n"
            ));
            for (i, p) in panels.iter().enumerate() {
                s.push_str(&format!(
                    "        crawrad_runtime_makepad::status::panel(cx, &self.ui.widget(id!({field}.panels.p_{i})), &self.ui.label(id!({field}.panels.p_{i}.t)), &self.form.{p}.text, self.form.{p}.width, &self.form.{p}.alignment);\n"
                ));
            }
        }
        s
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_dsl(
        &self,
        doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let txt = match node.props.get("simple_text") {
            Some(crawrad_ir::Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let txt_esc = txt.replace('\\', "\\\\").replace('"', "\\\"");
        // The panels, side by side, over the simple text (the sync
        // shows one or the other by SimplePanel)
        let panels = panels_of(doc, node);
        let mut row = String::new();
        if !panels.is_empty() {
            row.push_str(
                "                panels = <View> {\n\
                 \x20                   width: Fill, height: Fill, flow: Right, visible: false,\n",
            );
            for i in 0..panels.len() {
                // a View that is the panel's box and aligns; the Label inside
                // fits its text (a fixed-width Label pushed right-aligned text
                // past its own edge on makepad 1.0.0)
                row.push_str(&format!(
                    "                    p_{i} = <View> {{\n\
                     \x20                       width: 50.0, height: Fill,\n\
                     \x20                       align: {{ x: 0.0, y: 0.5 }},\n\
                     \x20                       padding: {{ left: 4.0, right: 4.0 }},\n\
                     \x20                       t = <Label> {{\n\
                     \x20                           width: Fit, height: Fit,\n\
                     \x20                           draw_text: {{ color: #cccccc, text_style: {{ font_size: 9.0 }} }},\n\
                     \x20                           text: \"\"\n\
                     \x20                       }}\n\
                     \x20                   }}\n"
                ));
            }
            row.push_str("                }\n");
        }
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               flow: Overlay,\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{ color: #252526 }},\n\
             \x20               lbl = <Label> {{\n\
             \x20                   width: Fill, height: Fill,\n\
             \x20                   draw_text: {{ color: #cccccc, text_style: {{ font_size: 9.0 }} }},\n\
             \x20                   text: \"{txt_esc}\"\n\
             \x20               }}\n\
             {row}\
             \x20           }}\n"
        ))
    }
}

#[crawrad_note::note("targets-makepad-components-status-bar-panels-of")]
fn panels_of(doc: &Document, node: &Node) -> Vec<String> {
    node.children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "StatusPanel")
        .filter_map(|c| identificador(c.name()).ok())
        .collect()
}

#[crawrad_note::note("targets-makepad-components-status-bar-statuspanelcodegen")]
pub struct StatusPanelCodegen;

impl MakepadComponentCodegen for StatusPanelCodegen {
    fn class(&self) -> &'static str {
        "StatusPanel"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("StatusPanel").expect("StatusPanel desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct StatusPanel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("StatusPanel").expect("StatusPanel desc");
        let body = state::inits_of(node, desc)
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("StatusPanel {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }
}
