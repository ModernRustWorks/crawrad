use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ButtonCodegen;

impl EguiComponentCodegen for ButtonCodegen {
    fn class(&self) -> &'static str {
        "Button"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_BUTTON)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Button").expect("Button descriptor");

    let acao_vinculada = match n.props.get("action") {
        Some(crawrad_ir::Value::Ref(id)) if !id.trim().is_empty() => {
            ctx.doc.node(id).filter(|no| no.class == "Action")
        }
        _ => None,
    };

    let (vis_cond, en_expr, act_depois) = if let Some(act) = acao_vinculada {
        let act_nome = identificador(act.name())?;
        let act_exec = crate::targets::egui::layout::linked_action_exec(ctx.doc, act)?;
        (
            format!("self.{nome}.visible && self.{act_nome}.visible"),
            format!("self.{nome}.enabled && self.{act_nome}.enabled"),
            act_exec,
        )
    } else {
        (
            format!("self.{nome}.visible"),
            format!("self.{nome}.enabled"),
            String::new(),
        )
    };

    let miolo = format!(
        "                ui.put(r, egui::Button::new(self.{nome}.text.as_str()))\n\
                    .clicked()\n"
    );
    let btn_depois = match handler(n, "on_click") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let depois = if act_depois.is_empty() && btn_depois.is_empty() {
        "            let _ = clicked;\n".to_string()
    } else {
        format!("            if clicked {{\n{act_depois}{btn_depois}            }}\n")
    };

    // the linked Action's caption, before the control is drawn
    let caption = crate::targets::egui::layout::caption_from_action(ctx.doc, n, nome)?;
    let body = caption
        + &format!(
            "        if {vis_cond} {{\n\
                    let is_enabled = {en_expr};\n\
                    let clicked = ui\n\
                        .add_enabled_ui(is_enabled, |ui| {{\n\
{miolo}\
                        }})\n\
                        .inner;\n\
{depois}\
                }}\n"
        );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Button,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Button",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_BUTTON: StateText = StateText {
    head: r#"
/// A `Button` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
