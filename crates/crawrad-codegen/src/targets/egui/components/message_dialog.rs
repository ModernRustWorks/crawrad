use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout;
use crate::{Erro, identificador};

pub struct MessageDialogCodegen;

impl EguiComponentCodegen for MessageDialogCodegen {
    fn class(&self) -> &'static str {
        "MessageDialog"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_MESSAGEDIALOG)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let name = ctx.nome;
        let n = ctx.node;
        let desc = ctx
            .reg
            .get("MessageDialog")
            .expect("MessageDialog descriptor");

        let on_close_stmt = match layout::handler(n, "on_close") {
            Some(h) => format!("\n            self.{}();", identificador(&h)?),
            None => String::new(),
        };

        // One `if` with a let chain: two nested ones fail the generated
        // project's clippy (`collapsible_if`).
        let body = format!(
            "        if self.{name}.answer.showing\n\
             \x20           && let Some(answer) = message_box(ui.ctx(), \"{name}\", &self.{name}.title, &self.{name}.text, &self.{name}.kind, &self.{name}.buttons)\n\
             \x20       {{\n\
             \x20           self.{name}.answer.showing = false;\n\
             \x20           self.{name}.modal_result = ModalResult::from_str_name(answer).unwrap_or_default();{on_close_stmt}\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {name}: MessageDialog,\n"),
            padrao: format!(
                "            {name}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: body,
            tipo_usado: "MessageDialog",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_MESSAGEDIALOG: StateText = StateText {
    head: r#"
/// The answer state for a message dialog.
#[derive(Clone, Debug, Default)]
pub struct MessageAnswer {
    pub showing: bool,
}

/// A `MessageDialog` on the form.
#[derive(Clone, Debug, Default)]
"#,
    tail: r#"
impl MessageDialog {
    /// Shows the message box; the answer arrives in `modal_result` and `OnClose`.
    pub fn execute(&mut self) {
        self.answer.showing = true;
    }
}
"#,
};
