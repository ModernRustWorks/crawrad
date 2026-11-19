use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct SaveDialogCodegen;

impl EguiComponentCodegen for SaveDialogCodegen {
    fn class(&self) -> &'static str {
        "SaveDialog"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SAVEDIALOG)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("SaveDialog").expect("SaveDialog descriptor");

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: SaveDialog,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: String::new(),
            tipo_usado: "SaveDialog",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_SAVEDIALOG: StateText = StateText {
    head: r#"
/// A `SaveDialog` on the form.
#[derive(Clone, Debug, Default)]
"#,
    tail: r#"
impl SaveDialog {
    pub fn execute(&mut self) -> bool {
        let req = crawrad_runtime::file_dialog::FileDialogRequest {
            title: self.title.clone(),
            filter: self.filter.clone(),
            initial_dir: self.initial_dir.clone(),
            file_name: self.file_name.clone(),
            default_ext: self.default_ext.clone(),
            multi: false,
        };
        if let Some(path) = crawrad_runtime::file_dialog::pick_save(&req) {
            self.file_name = path.to_string_lossy().into_owned();
            true
        } else {
            false
        }
    }
}
"#,
};
