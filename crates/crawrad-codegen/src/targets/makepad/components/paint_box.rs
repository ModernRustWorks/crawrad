//! Makepad component implementation for `PaintBox`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct PaintBoxCodegen;

impl MakepadComponentCodegen for PaintBoxCodegen {
    fn class(&self) -> &'static str {
        "PaintBox"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PaintBox").expect("PaintBox desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct PaintBox {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PaintBox").expect("PaintBox desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("PaintBox {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("PaintBox").expect("PaintBox desc");
        let reads = state::fields_of(desc)
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut out = format!("        let _ = ({reads});\n");
        // OnPaint draws on a Canvas of the box's size; its commands go
        // to the CrawradPaintBox, which draws them (and redraws on a change)
        if let Some(h) = layout::handler(node, "on_paint") {
            let Ok(h) = identificador(&h) else {
                return out;
            };
            let rect = node
                .rect()
                .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 0.0, 0.0));
            out.push_str(&canvas_to_box(
                &format!("self.form.{h}(&mut canvas);"),
                rect.w,
                rect.h,
                field,
            ));
        }
        out
    }

    #[crawrad_note::note("targets-makepad-components-paint-box-paintboxcodegen-app-state")]
    fn app_state(&self, node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        if layout::handler(node, "on_paint").is_none() {
            return Ok(None);
        }
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::paint::CrawradPaintBoxWidgetRefExt;\n".to_string(),
            ..AppState::default()
        }))
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
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        // The box that draws what OnPaint drew
        // (`crawrad_runtime_makepad::paint::CrawradPaintBox`)
        Ok(format!(
            "            {name} = <CrawradPaintBox> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20           }}\n"
        ))
    }
}

#[crawrad_note::note("targets-makepad-components-paint-box-canvas-to-box")]
pub fn canvas_to_box(call: &str, w: f32, h: f32, path: &str) -> String {
    format!(
        "        {{\n\
         \x20           let mut canvas = Canvas::new({w:.1}, {h:.1});\n\
         \x20           {call}\n\
         \x20           use crawrad_runtime_makepad::paint::Command;\n\
         \x20           let commands = canvas.commands().iter().map(|c| match c {{\n\
         \x20               PaintCommand::Rect {{ x, y, w, h, radius, fill, stroke }} => Command::Rect {{ x: *x, y: *y, w: *w, h: *h, radius: *radius, fill: *fill, stroke: *stroke }},\n\
         \x20               PaintCommand::Circle {{ x, y, r, fill, stroke }} => Command::Circle {{ x: *x, y: *y, r: *r, fill: *fill, stroke: *stroke }},\n\
         \x20               PaintCommand::Line {{ x0, y0, x1, y1, color, width }} => Command::Line {{ x0: *x0, y0: *y0, x1: *x1, y1: *y1, color: *color, width: *width }},\n\
         \x20               PaintCommand::Text {{ text, x, y, color, size }} => Command::Text {{ text: text.clone(), x: *x, y: *y, color: *color, size: *size }},\n\
         \x20           }}).collect();\n\
         \x20           self.ui.crawrad_paint_box(id!({path})).set_commands(cx, commands);\n\
         \x20       }}\n"
    )
}
