use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct LabelCodegen;

impl EguiComponentCodegen for LabelCodegen {
    fn class(&self) -> &'static str {
        "Label"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_LABEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Label").expect("Label descriptor");

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           // alignment, layout, wrap and fit, read at run time\n\
         \x20           let halign = match self.{nome}.alignment.as_str() {{\n\
         \x20               \"taCenter\" => egui::Align::Center,\n\
         \x20               \"taRightJustify\" => egui::Align::Max,\n\
         \x20               _ => egui::Align::Min,\n\
         \x20           }};\n\
         \x20           let valign = match self.{nome}.layout.as_str() {{\n\
         \x20               \"tlCenter\" => egui::Align::Center,\n\
         \x20               \"tlBottom\" => egui::Align::Max,\n\
         \x20               _ => egui::Align::Min,\n\
         \x20           }};\n\
         \x20           let wrap = if self.{nome}.word_wrap && !self.{nome}.auto_size {{\n\
         \x20               egui::TextWrapMode::Wrap\n\
         \x20           }} else {{\n\
         \x20               egui::TextWrapMode::Extend\n\
         \x20           }};\n\
         \x20           let area = if self.{nome}.auto_size {{\n\
         \x20               egui::Rect::from_min_size(r.min, egui::vec2(f32::INFINITY, f32::INFINITY))\n\
         \x20           }} else {{\n\
         \x20               r\n\
         \x20           }};\n\
         \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
         \x20               ui.scope_builder(\n\
         \x20                   egui::UiBuilder::new()\n\
         \x20                       .max_rect(area)\n\
         \x20                       .layout(egui::Layout::top_down(halign).with_main_align(valign)),\n\
         \x20                   |ui| {{\n\
         \x20                       ui.add(egui::Label::new(self.{nome}.text.as_str()).wrap_mode(wrap));\n\
         \x20                   }},\n\
         \x20               );\n\
         \x20           }});\n\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Label,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Label",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_LABEL: StateText = StateText {
    head: r#"
/// A `Label` on the form. The designer writes the initial values; from then
/// on they are yours: `self.lbl_name.text = "Other".into();`.
/// `alignment` / `layout`: where the text sits (`taCenter`, `tlBottom`…);
/// `word_wrap`: a long line breaks; `auto_size`: the box fits the text.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
