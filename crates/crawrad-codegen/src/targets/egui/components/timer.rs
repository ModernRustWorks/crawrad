use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct TimerCodegen;

impl EguiComponentCodegen for TimerCodegen {
    fn class(&self) -> &'static str {
        "Timer"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TIMER)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Timer").expect("Timer descriptor");

    let depois = match handler(n, "on_timer") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let disparo = if depois.is_empty() {
        "            let _ = fired;\n".to_string()
    } else {
        format!("            if fired {{\n{depois}            }}\n")
    };
    let corpo = format!(
        "        if self.{nome}.enabled && self.{nome}.interval > 0 {{\n\
         \x20           // Immediate mode has no clock of its own: a frame happens\n\
         \x20           // only when someone asks for one, so an idle app would\n\
         \x20           // never tick. Asking for the next frame IS the timer —\n\
         \x20           // in a classic RAD IDE the system does this for you.\n\
         \x20           let now = ui.input(|i| i.time);\n\
         \x20           let fired = timer_due(now, self.{nome}.interval, &mut self.{nome}.next_due);\n\
         \x20           ui.ctx()\n\
         \x20               .request_repaint_after(timer_wait(now, self.{nome}.next_due));\n\
         {disparo}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Timer,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Timer",
        precisa_timer: true,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_TIMER: StateText = StateText {
    head: r#"
/// A `Timer` on the form — classic RAD `TTimer`.
///
/// It is never on screen. While `enabled`, it calls your `OnTimer` handler
/// about every `interval` milliseconds. Setting `interval` to zero stops it,
/// exactly as it does in the classic RAD IDE — zero is not a very short interval.
///
/// No `visible` field, and that is on purpose: a component with no window has
/// nothing to hide.
///
/// `next_due` is bookkeeping — the deadline on egui's own clock, in seconds.
/// You do not need to touch it; clearing it to `None` restarts the count from
/// the next frame.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
