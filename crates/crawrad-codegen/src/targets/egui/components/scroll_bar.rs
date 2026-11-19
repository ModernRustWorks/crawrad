use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ScrollBarCodegen;

impl EguiComponentCodegen for ScrollBarCodegen {
    fn class(&self) -> &'static str {
        "ScrollBar"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SCROLLBAR)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ScrollBar").expect("ScrollBar descriptor");

    // `OnChange` and `OnScroll` both answer a move of the thumb, as on
    // Makepad: the first says the position changed, the second that the user
    // scrolled
    let mut calls = String::new();
    for event in ["on_change", "on_scroll"] {
        if let Some(h) = handler(n, event) {
            calls.push_str(&format!("                self.{}();\n", identificador(&h)?));
        }
    }
    let depois = if calls.is_empty() {
        "            let _ = changed;\n".to_string()
    } else {
        format!("            if changed {{\n{calls}            }}\n")
    };

    // the same bar the designer draws (two buttons with their arrow, a
    // track and a thumb), from the shared `scroll_bar` section, and not an
    // `egui::Slider`: a slider is another control. The orientation is the
    // box's shape, the rule the canvas and the Makepad target use.
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let bounds = (self.{nome}.min, self.{nome}.max);\n\
         \x20           let steps = (self.{nome}.small_change, self.{nome}.large_change);\n\
         \x20           let enabled = self.{nome}.enabled;\n\
         \x20           let changed = scroll_bar(\n\
         \x20               ui,\n\
         \x20               \"{nome}\",\n\
         \x20               r,\n\
         \x20               &mut self.{nome}.position,\n\
         \x20               bounds,\n\
         \x20               steps,\n\
         \x20               enabled,\n\
         \x20           );\n\
         {depois}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ScrollBar,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "ScrollBar",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_SCROLLBAR: StateText = StateText {
    head: r#"
/// A `ScrollBar` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
