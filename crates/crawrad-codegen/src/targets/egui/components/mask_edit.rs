//! `MaskEdit` in the egui target: an `Edit` whose every change is
//! shaped by its `EditMask` again (`crawrad_runtime::mask`) before `OnChange`
//! runs; the cursor is moved through the `mask_edit` piece of
//! `desenho_comum.rs`.

use super::codegen::{EguiComponentCodegen, StateText};
use super::edit::emit_text_box;
use super::{ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct MaskEditCodegen;

impl EguiComponentCodegen for MaskEditCodegen {
    fn class(&self) -> &'static str {
        "MaskEdit"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(STATE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let name = ctx.nome;
        let reshape = format!(
            "            // the mask shapes what was typed; the cursor goes to\n\
             \x20           // the next editable position\n\
             \x20           if res.changed() {{\n\
             \x20               hold_text(ui.ctx(), res.id, &mut self.{name}.text, |t, at| crawrad_runtime::mask::EditMask::parse(&self.{name}.edit_mask).hold_edit(&before, t, at));\n\
             \x20           }}\n"
        );
        emit_text_box(ctx, "MaskEdit", &reshape)
    }
}

const STATE: StateText = StateText {
    head: r#"
/// A `MaskEdit` on the form. `text` is what the box shows — the typed
/// characters laid into `edit_mask`, blanks where nothing was typed;
/// `crawrad_runtime::mask::EditMask::parse(&edit_mask).text(&text)` is the
/// value with or without the literals, as the mask's `;1`/`;0` says.
#[derive(Clone, Debug, Default)]
"#,
    tail: "",
};
