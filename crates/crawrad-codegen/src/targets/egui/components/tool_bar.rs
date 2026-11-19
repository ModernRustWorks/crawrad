use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct ToolBarCodegen;

impl EguiComponentCodegen for ToolBarCodegen {
    fn class(&self) -> &'static str {
        "ToolBar"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TOOLBAR)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("ToolBar").expect("ToolBar descriptor");
        let corpo = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               ui.painter().rect_filled(r, 0.0, ui.visuals().window_fill());\n\
             \x20               // a bar that is not Flat draws its lower edge\n\
             \x20               if !self.{nome}.flat {{\n\
             \x20                   ui.painter().hline(r.x_range(), r.bottom() - 0.5, ui.visuals().widgets.noninteractive.bg_stroke);\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        );
        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: ToolBar,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo,
            tipo_usado: "ToolBar",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_TOOLBAR: StateText = StateText {
    head: r#"
/// A `ToolBar` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
