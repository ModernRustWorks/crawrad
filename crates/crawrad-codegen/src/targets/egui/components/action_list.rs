use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ActionListCodegen;

impl EguiComponentCodegen for ActionListCodegen {
    fn class(&self) -> &'static str {
        "ActionList"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_ACTIONLIST)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir_action_list(ctx)
    }
}

pub fn emitir_action_list(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ActionList").expect("ActionList descriptor");

    let corpo = format!("        let _ = (self.{nome}.enabled, &self.{nome}.images);\n");

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ActionList,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "ActionList",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

pub struct ActionCodegen;

impl EguiComponentCodegen for ActionCodegen {
    fn class(&self) -> &'static str {
        "Action"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_ACTION)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir_action(ctx)
    }
}

pub fn emitir_action(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Action").expect("Action descriptor");

    let mut updates = String::new();
    if let Some(h) = handler(n, "on_update") {
        let fn_nome = identificador(&h)?;
        updates.push_str(&format!("            self.{fn_nome}();\n"));
    }

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         {updates}\
         \x20           let _ = (\n\
         \x20               self.{nome}.enabled,\n\
         \x20               &self.{nome}.text,\n\
         \x20               &self.{nome}.hint,\n\
         \x20               &self.{nome}.shortcut,\n\
         \x20               &self.{nome}.category,\n\
         \x20               self.{nome}.image_index,\n\
         \x20               self.{nome}.checked,\n\
         \x20               &self.{nome}.dataset,\n\
         \x20               &self.{nome}.dataset_operation,\n\
         \x20           );\n\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Action,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Action",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_ACTIONLIST: StateText = StateText {
    head: r#"
/// An `ActionList` on the form — classic RAD `TActionList`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};

const ESTADO_ACTION: StateText = StateText {
    head: r#"
/// An `Action` on the form — classic RAD `TAction`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
