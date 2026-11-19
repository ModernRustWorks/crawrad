use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ToggleCodegen;

impl EguiComponentCodegen for ToggleCodegen {
    fn class(&self) -> &'static str {
        "Toggle"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TOGGLE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-toggle-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Toggle").expect("Toggle descriptor");
    let mut calls = String::new();
    for event in ["on_click", "on_change"] {
        if let Some(h) = handler(n, event) {
            calls.push_str(&format!("                self.{}();\n", identificador(&h)?));
        }
    }
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let enabled = self.{name}.enabled;\n\
         \x20           let resp = ui.interact(r, ui.id().with((\"crawrad-toggle\", \"{name}\")), if enabled {{ egui::Sense::click() }} else {{ egui::Sense::hover() }});\n\
         \x20           let on = ui.ctx().animate_bool_responsive(resp.id, self.{name}.checked);\n\
         \x20           let track = egui::Rect::from_min_size(egui::pos2(r.min.x + 2.0, r.center().y - 8.0), egui::vec2(32.0, 16.0));\n\
         \x20           let visuals = ui.visuals().clone();\n\
         \x20           let fill = if self.{name}.checked {{ visuals.selection.bg_fill }} else {{ visuals.widgets.inactive.bg_fill }};\n\
         \x20           let fade = if enabled {{ 1.0 }} else {{ 0.4 }};\n\
         \x20           ui.painter().rect(track, 8.0, fill.gamma_multiply(fade), visuals.widgets.inactive.bg_stroke, egui::StrokeKind::Inside);\n\
         \x20           let knob = egui::pos2(egui::lerp((track.left() + 8.0)..=(track.right() - 8.0), on), track.center().y);\n\
         \x20           ui.painter().circle_filled(knob, 6.0, visuals.strong_text_color().gamma_multiply(fade));\n\
         \x20           let color = if enabled {{ visuals.text_color() }} else {{ visuals.weak_text_color() }};\n\
         \x20           ui.painter().text(egui::pos2(track.right() + 6.0, r.center().y), egui::Align2::LEFT_CENTER, self.{name}.text.as_str(), egui::TextStyle::Body.resolve(ui.style()), color);\n\
         \x20           if resp.clicked() {{\n\
         \x20               self.{name}.checked = !self.{name}.checked;\n\
         {calls}\
         \x20           }}\n\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Toggle,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Toggle",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_TOGGLE: StateText = StateText {
    head: r#"
/// A `Toggle` on the form: an on/off switch (`checked` is on) and its caption.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
