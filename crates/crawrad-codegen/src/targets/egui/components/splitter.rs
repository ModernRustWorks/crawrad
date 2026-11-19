use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct SplitterCodegen;

impl EguiComponentCodegen for SplitterCodegen {
    fn class(&self) -> &'static str {
        "Splitter"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SPLITTER)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("Splitter").expect("Splitter descriptor");

        // A drag moves `offset` within MinSize of both neighbours; the
        // neighbours' boxes add it (`targets::splitters`), and letting go
        // calls OnMoved
        let split = crate::targets::splitters::split_of(ctx.doc, nome);
        let vertical = split.as_ref().is_none_or(|s| s.vertical);
        let (axis, cursor) = if vertical {
            ("x", "ResizeHorizontal")
        } else {
            ("y", "ResizeVertical")
        };
        let lo = match split.as_ref().and_then(|s| s.before.as_ref()) {
            Some((_, len)) => format!("self.{nome}.min_size as f32 - {len:.1}"),
            None => "f32::NEG_INFINITY".to_string(),
        };
        let hi = match split.as_ref().and_then(|s| s.after.as_ref()) {
            Some((_, len)) => format!("{len:.1} - self.{nome}.min_size as f32"),
            None => "f32::INFINITY".to_string(),
        };
        let moved = match crate::targets::egui::layout::handler(n, "on_moved") {
            Some(h) => format!(
                "                if resp.drag_stopped() {{\n                    self.{}();\n                }}\n",
                crate::identificador(&h)?
            ),
            None => String::new(),
        };
        let body = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           let _ = (self.{nome}.auto_snap, self.{nome}.beveled);\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               let bg = ui.visuals().widgets.noninteractive.bg_fill;\n\
             \x20               let stroke = ui.visuals().widgets.noninteractive.bg_stroke;\n\
             \x20               ui.painter().rect(r, 0.0, bg, stroke, egui::StrokeKind::Inside);\n\
             \x20               let resp = ui.interact(r, egui::Id::new((\"splitter\", \"{nome}\")), egui::Sense::drag());\n\
             \x20               if resp.hovered() || resp.dragged() {{\n\
             \x20                   ui.ctx().set_cursor_icon(egui::CursorIcon::{cursor});\n\
             \x20               }}\n\
             \x20               if resp.dragged() {{\n\
             \x20                   let moved = self.{nome}.offset + resp.drag_delta().{axis};\n\
             \x20                   self.{nome}.offset = moved.clamp({lo}, {hi});\n\
             \x20               }}\n\
             {moved}\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: Splitter,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: body,
            tipo_usado: "Splitter",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_SPLITTER: StateText = StateText {
    head: r#"
/// A `Splitter` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
