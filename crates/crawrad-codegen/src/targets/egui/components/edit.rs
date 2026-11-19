use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct EditCodegen;

impl EguiComponentCodegen for EditCodegen {
    fn class(&self) -> &'static str {
        "Edit"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_EDIT)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    emit_text_box(ctx, "Edit", "")
}

#[crawrad_note::note("targets-egui-components-edit-emit-text-box")]
pub fn emit_text_box(
    ctx: &ContextoEmissao,
    class: &'static str,
    after_change: &str,
) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx
        .reg
        .get(class)
        .expect("a text box class is in the catalogue");

    let mut event_dispatches = String::new();
    if let Some(h) = handler(n, "on_change") {
        let h_name = identificador(&h)?;
        event_dispatches.push_str(&format!(
            "            if res.changed() {{\n                self.{h_name}();\n            }}\n"
        ));
    }
    // `OnEnter` / `OnExit` are the control router's (`control_events`), as
    // every other class's: egui reports `lost_focus` on two passes running
    // when the focus moves mid-pass (a Tab), and the router fires on
    // the change of the focus, once.
    if let Some(h) = handler(n, "on_return") {
        let h_name = identificador(&h)?;
        event_dispatches.push_str(&format!(
            "            if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {{\n                self.{h_name}();\n            }}\n"
        ));
    }
    if let Some(h) = handler(n, "on_key_press") {
        let h_name = identificador(&h)?;
        event_dispatches.push_str(&format!(
            "            if res.has_focus() {{\n\
             \x20               let typed_chars: Vec<char> = ui.input(|i| {{\n\
             \x20                   i.events.iter().filter_map(|e| match e {{\n\
             \x20                       egui::Event::Text(t) => Some(t.chars().collect::<Vec<_>>()),\n\
             \x20                       _ => None,\n\
             \x20                   }}).flatten().collect()\n\
             \x20               }});\n\
             \x20               for ch in typed_chars {{\n\
             \x20                   self.{h_name}(ch);\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
    }

    event_dispatches.push_str(&crate::targets::egui::control_keys::dispatch(n)?);

    let before_edit = if class == "MaskEdit" {
        format!("            let before = self.{nome}.text.clone();\n")
    } else {
        String::new()
    };

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           // its own Color, or the theme's box colour\n\
         \x20           let c = self.{nome}.color;\n\
         \x20           let fill = if !self.{nome}.parent_color && c.a > 0 {{\n\
         \x20               egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)\n\
         \x20           }} else {{\n\
         \x20               ui.visuals().extreme_bg_color\n\
         \x20           }};\n\
         \x20           let char_limit = if self.{nome}.max_length > 0 {{\n\
         \x20               self.{nome}.max_length as usize\n\
         \x20           }} else {{\n\
         \x20               usize::MAX\n\
         \x20           }};\n\
         \x20           let halign = match self.{nome}.alignment.as_str() {{\n\
         \x20               \"taCenter\" => egui::Align::Center,\n\
         \x20               \"taRightJustify\" => egui::Align::Max,\n\
         \x20               _ => egui::Align::Min,\n\
         \x20           }};\n\
         {before_edit}\
         \x20           let res = ui\n\
         \x20               .add_enabled_ui(is_enabled, |ui| {{\n\
         \x20                   ui.put(\n\
         \x20                       r,\n\
         \x20                       egui::TextEdit::singleline(&mut self.{nome}.text)\n\
         \x20                           .interactive(!self.{nome}.readonly)\n\
         \x20                           .password(!self.{nome}.password_char.is_empty())\n\
         \x20                           .hint_text(&self.{nome}.text_hint)\n\
         \x20                           .char_limit(char_limit)\n\
         \x20                           .horizontal_align(halign)\n\
         \x20                           .background_color(fill),\n\
         \x20                   )\n\
         \x20               }})\n\
         \x20               .inner;\n\
         \x20           if res.changed() && self.{nome}.numbers_only {{\n\
         \x20               self.{nome}.text.retain(|c| c.is_ascii_digit());\n\
         \x20           }}\n\
         \x20           // CharCase holds the typed text to its case\n\
         \x20           if res.changed() {{\n\
         \x20               self.{nome}.text = char_cased(&self.{nome}.text, &self.{nome}.char_case);\n\
         \x20           }}\n\
         {after_change}\
         {event_dispatches}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: {class},\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: class,
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_EDIT: StateText = StateText {
    head: r#"
/// An `Edit` on the form. `text` is what the user typed — read and write it
/// freely.
#[derive(Clone, Debug, Default)]
"#,
    tail: "",
};
