use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct PanelCodegen;

impl EguiComponentCodegen for PanelCodegen {
    fn class(&self) -> &'static str {
        "Panel"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_PANEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Panel").expect("Panel descriptor");

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           // The container draws the background; the children come later,\n\
         \x20           // because the emission order puts the parent before them.\n\
         \x20           //\n\
         \x20           // Disabled does not vanish, it dims — what a classic RAD IDE does with\n\
         \x20           // a disabled panel. While nothing read `enabled`, writing it\n\
         \x20           // did nothing and nothing warned.\n\
         \x20           let (outline, caption_color) = if self.{nome}.enabled {{\n\
         \x20               (\n\
         \x20                   ui.visuals().widgets.noninteractive.fg_stroke.color,\n\
         \x20                   ui.visuals().text_color(),\n\
         \x20               )\n\
         \x20           }} else {{\n\
         \x20               (\n\
         \x20                   ui.visuals().widgets.noninteractive.bg_stroke.color,\n\
         \x20                   ui.visuals().weak_text_color(),\n\
         \x20               )\n\
         \x20           }};\n\
         \x20           // its own Color, or the theme's panel colour\n\
         \x20           let c = self.{nome}.color;\n\
         \x20           let fill = if !self.{nome}.parent_color && c.a > 0 {{\n\
         \x20               egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)\n\
         \x20           }} else {{\n\
         \x20               ui.visuals().extreme_bg_color\n\
         \x20           }};\n\
         \x20           ui.painter().rect_filled(r, 2.0, fill);\n\
         \x20           ui.painter().rect_stroke(\n\
         \x20               r,\n\
         \x20               2.0,\n\
         \x20               egui::Stroke::new(self.{nome}.border_width, outline),\n\
         \x20               egui::StrokeKind::Inside,\n\
         \x20           );\n\
         \x20           if !self.{nome}.text.is_empty() {{\n\
         \x20               ui.painter().text(\n\
         \x20                   egui::pos2(r.min.x + 6.0, r.min.y + 9.0),\n\
         \x20                   egui::Align2::LEFT_CENTER,\n\
         \x20                   self.{nome}.text.as_str(),\n\
         \x20                   egui::FontId::proportional(12.0),\n\
         \x20                   caption_color,\n\
         \x20               );\n\
         \x20           }}\n\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Panel,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Panel",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_PANEL: StateText = StateText {
    head: r#"
/// A `Panel` on the form — a container.
///
/// It draws the background; what sits inside are fields of the form itself,
/// emitted right after. Hiding the panel does NOT hide its children: in the classic RAD IDE
/// that comes from the window hierarchy, and here the struct is flat. Hide the
/// children too, or use `enabled` instead.
/// `border_width` is classic RAD `TPanel.BorderWidth` — the thickness of the
/// outline, in points. Zero draws no outline at all.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
