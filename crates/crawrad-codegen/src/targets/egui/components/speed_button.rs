use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct SpeedButtonCodegen;

impl EguiComponentCodegen for SpeedButtonCodegen {
    fn class(&self) -> &'static str {
        "SpeedButton"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SPEEDBUTTON)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("SpeedButton").expect("SpeedButton descriptor");

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

    // Down draws it pressed; Flat leaves the frame for the pointer
    // on a ToolBar, the bar's Flat makes it flat too
    let unframed = match crate::targets::egui::glyph::tool_bar_of(ctx.doc, n)? {
        Some(bar) => format!("!(self.{nome}.flat || self.{bar}.flat)"),
        None => format!("!self.{nome}.flat"),
    };
    let miolo = format!(
        "{}                ui.put(r, egui::Button::new(atoms).selected(self.{nome}.down).frame_when_inactive({unframed}))\n\
                    .clicked()\n",
        crate::targets::egui::glyph::atoms_code(ctx.doc, n, nome, "                ")?
    );

    let btn_depois = match handler(n, "on_click") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let calls = format!("{act_depois}{btn_depois}");
    let depois =
        crate::targets::speed_groups::click_block(ctx.doc, n, nome, "self.", "clicked", &calls)
            .replace("\n        ", "\n            ");
    let depois = format!("    {depois}");

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
        campo: format!("    pub {nome}: SpeedButton,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "SpeedButton",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_SPEEDBUTTON: StateText = StateText {
    head: r#"
/// A `SpeedButton` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
