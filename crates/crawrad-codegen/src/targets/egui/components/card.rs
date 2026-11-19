use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct CardCodegen;

impl EguiComponentCodegen for CardCodegen {
    fn class(&self) -> &'static str {
        "Card"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_CARD)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }

    #[crawrad_note::note("targets-egui-components-card-cardcodegen-envolve-filhos")]
    fn envolve_filhos(&self) -> bool {
        true
    }

    fn corpo_fecha(&self, _ctx: &ContextoEmissao) -> Result<String, Erro> {
        Ok(CLOSE.to_string())
    }
}

#[crawrad_note::note("targets-egui-components-card-close")]
const CLOSE: &str = "\
\x20               });\n\
\x20           });\n\
\x20       }\n";

#[crawrad_note::note("targets-egui-components-card-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Card").expect("Card descriptor");

    let on_click = match handler(n, "on_click") {
        Some(h) => format!("                    self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let click_handling = if !on_click.is_empty() {
        format!(
            "            if self.{name}.enabled && ui.interact(r, ui.id().with((\"crawrad-card\", \"{name}\")), egui::Sense::click()).clicked() {{\n\
             {on_click}\
             \x20           }}\n"
        )
    } else {
        String::new()
    };

    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let font = ui.style().override_font_id.clone().unwrap_or_else(|| egui::TextStyle::Button.resolve(ui.style()));\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               paint_card(\n\
         \x20                   ui.painter(),\n\
         \x20                   r,\n\
         \x20                   self.{name}.text.as_str(),\n\
         \x20                   self.{name}.subtitle.as_str(),\n\
         \x20                   self.{name}.corner_radius as f32,\n\
         \x20                   self.{name}.elevation,\n\
         \x20                   self.{name}.show_header,\n\
         \x20                   ui.visuals(),\n\
         \x20                   font,\n\
         \x20               );\n\
         \x20           }});\n\
         {click_handling}\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {{\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Card,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Card",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_CARD: StateText = StateText {
    head: r#"
/// A `Card` on the form: a modern rounded container with optional header (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
