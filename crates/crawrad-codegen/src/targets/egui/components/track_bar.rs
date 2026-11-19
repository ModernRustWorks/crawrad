use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct TrackBarCodegen;

impl EguiComponentCodegen for TrackBarCodegen {
    fn class(&self) -> &'static str {
        "TrackBar"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TRACKBAR)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("TrackBar").expect("TrackBar descriptor");

        let on_change = match handler(n, "on_change") {
            Some(h) => format!("                    self.{}();\n", identificador(&h)?),
            None => String::new(),
        };

        let corpo = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               // a vertical bar has `Min` at the top, as the\n\
             \x20               // classic one; egui's starts its range at the bottom\n\
             \x20               let (lo, hi) = (self.{nome}.min, self.{nome}.max);\n\
             \x20               let slider = if self.{nome}.orientation == \"Vertical\" {{\n\
             \x20                   egui::Slider::new(&mut self.{nome}.position, hi..=lo).vertical()\n\
             \x20               }} else {{\n\
             \x20                   egui::Slider::new(&mut self.{nome}.position, lo..=hi)\n\
             \x20               }};\n\
             \x20               let mut resp = ui.put(r, slider);\n\
             \x20               // `PageSize`: PageDown moves toward `Max`, PageUp\n\
             \x20               // toward `Min`, as the classic bar does\n\
             \x20               if resp.has_focus() {{\n\
             \x20                   let (down, up) = ui.input(|i| {{\n\
             \x20                       (i.num_presses(egui::Key::PageDown), i.num_presses(egui::Key::PageUp))\n\
             \x20                   }});\n\
             \x20                   let moved = (down as i64 - up as i64) * self.{nome}.page_size;\n\
             \x20                   if moved != 0 {{\n\
             \x20                       let next = (self.{nome}.position + moved).clamp(lo.min(hi), hi.max(lo));\n\
             \x20                       if next != self.{nome}.position {{\n\
             \x20                           self.{nome}.position = next;\n\
             \x20                           resp.mark_changed();\n\
             \x20                       }}\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20               if resp.changed() {{\n\
             {on_change}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: TrackBar,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo,
            tipo_usado: "TrackBar",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_TRACKBAR: StateText = StateText {
    head: r#"
/// A `TrackBar` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
