use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct BitBtnCodegen;

impl EguiComponentCodegen for BitBtnCodegen {
    fn class(&self) -> &'static str {
        "BitBtn"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_BITBTN)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("BitBtn").expect("BitBtn descriptor");

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
        "{}                ui.put(r, egui::Button::new(atoms))\n\
                    .clicked()\n",
        crate::targets::egui::glyph::atoms_code(ctx.doc, n, nome, "                ")?
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
        campo: format!("    pub {nome}: BitBtn,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "BitBtn",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

pub fn bit_btn_caption_and_modal_result(node: &crawrad_ir::Node) -> (String, &'static str) {
    let kind = match node.props.get("kind") {
        Some(crawrad_ir::Value::Enum(k)) => k.as_str(),
        _ => "bkCustom",
    };
    let (std_caption, default_mr) = match kind {
        "bkOK" => (Some("OK"), "mrOk"),
        "bkCancel" => (Some("Cancel"), "mrCancel"),
        "bkHelp" => (Some("Help"), "mrNone"),
        "bkClose" => (Some("Close"), "mrClose"),
        "bkYes" => (Some("Yes"), "mrYes"),
        "bkNo" => (Some("No"), "mrNo"),
        _ => (None, "mrNone"),
    };
    let text = if let Some(cap) = std_caption {
        let raw = node.text();
        if raw.is_none_or(|t| t.is_empty() || t == "BitBtn" || t == node.name()) {
            cap.to_string()
        } else {
            crate::sem_acelerador(raw.unwrap_or(node.name()))
        }
    } else {
        crate::sem_acelerador(node.text().unwrap_or(node.name()))
    };
    let mr = match node.props.get("modal_result") {
        Some(crawrad_ir::Value::Enum(s)) => match s.as_str() {
            "mrOk" => "mrOk",
            "mrCancel" => "mrCancel",
            "mrYes" => "mrYes",
            "mrNo" => "mrNo",
            "mrClose" => "mrClose",
            "mrNone" => "mrNone",
            _ => default_mr,
        },
        _ => default_mr,
    };
    (text, mr)
}

const ESTADO_BITBTN: StateText = StateText {
    head: r#"
/// A `BitBtn` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
