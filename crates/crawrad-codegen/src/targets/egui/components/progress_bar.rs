use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct ProgressBarCodegen;

impl EguiComponentCodegen for ProgressBarCodegen {
    fn class(&self) -> &'static str {
        "ProgressBar"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_PROGRESSBAR)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("ProgressBar").expect("ProgressBar descriptor");

        let corpo = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               let range = (self.{nome}.max - self.{nome}.min).max(1) as f32;\n\
             \x20               let progress = ((self.{nome}.position - self.{nome}.min) as f32 / range).clamp(0.0, 1.0);\n\
             \x20               // `egui::ProgressBar` only fills smoothly from the\n\
             \x20               // left; a vertical bar or one in blocks is painted\n\
             \x20               let vertical = self.{nome}.orientation == \"Vertical\";\n\
             \x20               if vertical || !self.{nome}.smooth {{\n\
             \x20                   paint_progress(ui, r, progress, vertical, self.{nome}.smooth);\n\
             \x20               }} else {{\n\
             \x20                   ui.put(r, egui::ProgressBar::new(progress));\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: ProgressBar,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo,
            tipo_usado: "ProgressBar",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_PROGRESSBAR: StateText = StateText {
    head: r#"
/// A `ProgressBar` on the form.
#[derive(Clone, Debug)]
"#,
    tail: r#"
impl ProgressBar {
    /// `StepIt`: moves the position on by `step`; past `max` it starts over
    /// from `min`, as the classic bar does.
    pub fn step_it(&mut self) {
        self.position += self.step;
        if self.position > self.max {
            self.position = self.min;
        }
    }
}
"#,
};
