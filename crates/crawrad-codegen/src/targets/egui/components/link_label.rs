use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

#[crawrad_note::note("targets-egui-components-link-label-shell")]
const SHELL: &str = include_str!("../../../../../crawrad-runtime/src/shell.rs");

pub struct LinkLabelCodegen;

impl EguiComponentCodegen for LinkLabelCodegen {
    fn class(&self) -> &'static str {
        "LinkLabel"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_LINKLABEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-link-label-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("LinkLabel").expect("LinkLabel descriptor");
    let call = match handler(n, "on_click") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let mut clicked = false;\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {{\n\
         \x20                   clicked = ui.link(self.{name}.text.as_str()).on_hover_text(self.{name}.url.as_str()).clicked();\n\
         \x20               }});\n\
         \x20           }});\n\
         \x20           if clicked {{\n\
         \x20               crawrad_ide::open_url(&self.{name}.url);\n\
         {call}\
         \x20           }}\n\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: LinkLabel,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: format!("mod crawrad_ide {{\n{SHELL}}}\n\n"),
        corpo: body,
        tipo_usado: "LinkLabel",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_LINKLABEL: StateText = StateText {
    head: r#"
/// A `LinkLabel` on the form: a caption drawn as a link; a click opens `url`
/// (web or mail only) and runs `OnClick`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
