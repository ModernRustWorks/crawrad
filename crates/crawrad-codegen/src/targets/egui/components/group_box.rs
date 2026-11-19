use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct GroupBoxCodegen;

impl EguiComponentCodegen for GroupBoxCodegen {
    fn class(&self) -> &'static str {
        "GroupBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_GROUPBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("GroupBox").expect("GroupBox descriptor");

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           group_box_frame(ui, r, self.{nome}.text.as_str(), self.{nome}.enabled);\n\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: GroupBox,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "GroupBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_GROUPBOX: StateText = StateText {
    head: r#"
/// A `GroupBox` on the form — a container with a caption/text.
///
/// Same as `Panel` in what it holds; different in what it means. Radio buttons
/// inside it exclude one another and leave the ones outside alone, because the
/// group IS the parent — no group property, exactly like classic RAD.
///
/// Hiding it does NOT hide its children: the struct is flat. Hide them too, or
/// use `enabled`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
