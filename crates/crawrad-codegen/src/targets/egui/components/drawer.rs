use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct DrawerCodegen;

impl EguiComponentCodegen for DrawerCodegen {
    fn class(&self) -> &'static str {
        "Drawer"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_DRAWER)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }

    #[crawrad_note::note("targets-egui-components-drawer-drawercodegen-envolve-filhos")]
    fn envolve_filhos(&self) -> bool {
        true
    }

    fn corpo_fecha(&self, _ctx: &ContextoEmissao) -> Result<String, Erro> {
        Ok(CLOSE.to_string())
    }
}

#[crawrad_note::note("targets-egui-components-drawer-close")]
const CLOSE: &str = "\
\x20               });\n\
\x20           });\n\
\x20       }\n";

#[crawrad_note::note("targets-egui-components-drawer-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Drawer").expect("Drawer descriptor");

    let on_close = match handler(n, "on_close") {
        Some(h) => format!("                    self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let mut body = String::new();
    body.push_str(&format!(
        "        if self.{name}.visible && self.{name}.opened {{\n"
    ));

    // Scrim overlay over the entire form client area if overlay is true
    body.push_str(&format!(
        "            if self.{name}.overlay {{\n\
         \x20               let form_rect = ui.max_rect();\n\
         \x20               let scrim_color = ui.visuals().extreme_bg_color.gamma_multiply(0.6);\n\
         \x20               ui.painter().rect_filled(form_rect, 0.0, scrim_color);\n\
         \x20               if self.{name}.enabled && ui.interact(form_rect, ui.id().with((\"crawrad-scrim\", \"{name}\")), egui::Sense::click()).clicked() {{\n\
         \x20                   self.{name}.opened = false;\n\
         {on_close}\
         \x20               }}\n\
         \x20           }}\n"
    ));

    // Drawer panel background, header and close button
    body.push_str(&format!(
        "            let font = ui.style().override_font_id.clone().unwrap_or_else(|| egui::TextStyle::Button.resolve(ui.style()));\n\
         \x20           let _panel_click = ui.interact(r, ui.id().with((\"crawrad-drawer\", \"{name}\")), egui::Sense::click());\n\
         \x20           let close_rect = drawer_close_rect(r, self.{name}.side.as_str());\n\
         \x20           let close_resp = if self.{name}.enabled && self.{name}.show_header {{\n\
         \x20               Some(ui.interact(close_rect, ui.id().with((\"crawrad-drawer-close\", \"{name}\")), egui::Sense::click()))\n\
         \x20           }} else {{\n\
         \x20               None\n\
         \x20           }};\n\
         \x20           let close_hovered = close_resp.as_ref().is_some_and(|resp| resp.hovered());\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               paint_drawer(\n\
         \x20                   ui.painter(),\n\
         \x20                   r,\n\
         \x20                   self.{name}.text.as_str(),\n\
         \x20                   self.{name}.side.as_str(),\n\
         \x20                   self.{name}.show_header,\n\
         \x20                   close_hovered,\n\
         \x20                   ui.visuals(),\n\
         \x20                   font,\n\
         \x20               );\n\
         \x20           }});\n\
         \x20           if let Some(resp) = close_resp && resp.clicked() {{\n\
         \x20               self.{name}.opened = false;\n\
         {on_close}\
         \x20           }}\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {{\n"
    ));

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Drawer,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Drawer",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_DRAWER: StateText = StateText {
    head: r#"
/// A `Drawer` on the form: a side panel that opens over the form (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
