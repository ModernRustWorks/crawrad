use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct BevelCodegen;

impl EguiComponentCodegen for BevelCodegen {
    fn class(&self) -> &'static str {
        "Bevel"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_BEVEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-bevel-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Bevel").expect("Bevel descriptor");
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               paint_bevel(ui.painter(), r, &self.{name}.bevel_shape, self.{name}.bevel_style == \"Raised\", ui.visuals());\n\
         \x20           }});\n\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Bevel,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Bevel",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_BEVEL: StateText = StateText {
    head: r#"
/// A `Bevel` on the form: a sunk or raised box, frame or line.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
