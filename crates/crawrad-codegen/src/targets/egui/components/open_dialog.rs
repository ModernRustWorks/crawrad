use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct OpenDialogCodegen;

impl EguiComponentCodegen for OpenDialogCodegen {
    fn class(&self) -> &'static str {
        "OpenDialog"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_OPENDIALOG)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("OpenDialog").expect("OpenDialog descriptor");

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: OpenDialog,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: String::new(),
            tipo_usado: "OpenDialog",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_OPENDIALOG: StateText = StateText {
    head: r#"
/// An `OpenDialog` on the form.
#[derive(Clone, Debug, Default)]
"#,
    tail: r#"
impl OpenDialog {
    pub fn execute(&mut self) -> bool {
        let req = crawrad_runtime::file_dialog::FileDialogRequest {
            title: self.title.clone(),
            filter: self.filter.clone(),
            initial_dir: self.initial_dir.clone(),
            file_name: self.file_name.clone(),
            default_ext: self.default_ext.clone(),
            multi: false,
        };
        if let Some(paths) = crawrad_runtime::file_dialog::pick_open(&req) {
            self.files = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();
            self.file_name = self.files.first().cloned().unwrap_or_default();
            true
        } else {
            false
        }
    }
}
"#,
};
