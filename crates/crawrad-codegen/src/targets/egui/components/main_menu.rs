use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node, Value};

#[crawrad_note::note("targets-egui-components-main-menu-bar-height")]
pub const BAR_HEIGHT: f32 = 22.0;

pub struct MainMenuCodegen;

impl EguiComponentCodegen for MainMenuCodegen {
    fn class(&self) -> &'static str {
        "MainMenu"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_MAINMENU)
    }

    fn form_strip(&self) -> bool {
        true
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let name = ctx.nome;
        let items = items_code(ctx.doc, ctx.node, 5)?;
        let body = if items.is_empty() {
            format!("        let _ = &self.{name};\n")
        } else {
            format!(
                "        // the MainMenu's bar, along the top of the form\n\
                 \x20       {{\n\
                 \x20           let _ = &self.{name};\n\
                 \x20           let bar = egui::Rect::from_min_size(ui.max_rect().min, egui::vec2(ui.max_rect().width(), {BAR_HEIGHT:.1}));\n\
                 \x20           ui.painter().rect_filled(bar, 0.0, ui.visuals().panel_fill);\n\
                 \x20           ui.scope_builder(egui::UiBuilder::new().max_rect(bar), |ui| {{\n\
                 \x20               egui::MenuBar::new().ui(ui, |ui| {{\n\
                 {items}\
                 \x20               }});\n\
                 \x20           }});\n\
                 \x20       }}\n"
            )
        };
        Ok(ResultadoComponente {
            campo: format!("    pub {name}: MainMenu,\n"),
            padrao: format!("            {name}: MainMenu::default(),\n"),
            constante: String::new(),
            corpo: body,
            tipo_usado: "MainMenu",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

#[crawrad_note::note("targets-egui-components-main-menu-menuitemcodegen")]
pub struct MenuItemCodegen;

impl EguiComponentCodegen for MenuItemCodegen {
    fn class(&self) -> &'static str {
        "MenuItem"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_MENUITEM)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let name = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("MenuItem").expect("MenuItem descriptor");
        Ok(ResultadoComponente {
            campo: format!("    pub {name}: MenuItem,\n"),
            padrao: format!(
                "            {name}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: crate::targets::egui::layout::caption_from_action(ctx.doc, n, name)?,
            tipo_usado: "MenuItem",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

#[crawrad_note::note("targets-egui-components-main-menu-action-of")]
fn action_of<'a>(doc: &'a Document, n: &Node) -> Option<&'a Node> {
    match n.props.get("action") {
        Some(Value::Ref(id)) if !id.trim().is_empty() => {
            doc.node(id).filter(|a| a.class == "Action")
        }
        _ => None,
    }
}

#[crawrad_note::note("targets-egui-components-main-menu-items-of")]
pub fn items_of<'a>(doc: &'a Document, owner: &Node) -> Vec<&'a Node> {
    owner
        .children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "MenuItem")
        .collect()
}

#[crawrad_note::note("targets-egui-components-main-menu-items-code")]
pub fn items_code(doc: &Document, owner: &Node, depth: usize) -> Result<String, Erro> {
    let p = "    ".repeat(depth);
    let mut out = String::new();
    for item in items_of(doc, owner) {
        let f = identificador(item.name())?;
        let act = action_of(doc, item);
        let act_field = act.map(|a| identificador(a.name())).transpose()?;
        let (shown, enabled) = match &act_field {
            Some(a) => (
                format!("self.{f}.visible && self.{a}.visible"),
                format!("self.{f}.enabled && self.{a}.enabled"),
            ),
            None => (format!("self.{f}.visible"), format!("self.{f}.enabled")),
        };
        let children = items_code(doc, item, depth + 3)?;
        if !children.is_empty() {
            out.push_str(&format!(
                "{p}if {shown} {{\n\
                 {p}    let mut text = if self.{f}.checked {{ format!(\"\\u{{2714}} {{}}\", self.{f}.text) }} else {{ self.{f}.text.clone() }};\n\
                 {p}    let shortcut = self.{f}.shortcut.to_string();\n\
                 {p}    if !shortcut.is_empty() {{\n\
                 {p}        text = format!(\"{{text}}    {{shortcut}}\");\n\
                 {p}    }}\n\
                 {p}    ui.add_enabled_ui({enabled}, |ui| {{\n\
                 {p}        ui.menu_button(text, |ui| {{\n\
                 {children}\
                 {p}        }});\n\
                 {p}    }});\n\
                 {p}}}\n"
            ));
            continue;
        }
        let mut calls = String::new();
        if let Some(h) = act.and_then(|a| handler(a, "on_execute")) {
            calls.push_str(&format!("{p}            self.{}();\n", identificador(&h)?));
        }
        if let Some(h) = handler(item, "on_click") {
            calls.push_str(&format!("{p}            self.{}();\n", identificador(&h)?));
        }
        let click = if calls.is_empty() {
            format!("{p}        let _ = ui.add_enabled({enabled}, button);\n")
        } else {
            format!(
                "{p}        if ui.add_enabled({enabled}, button).clicked() {{\n\
                 {calls}\
                 {p}        }}\n"
            )
        };
        out.push_str(&format!(
            "{p}if {shown} {{\n\
             {p}    if self.{f}.text == \"-\" {{\n\
             {p}        ui.separator();\n\
             {p}    }} else {{\n\
             {p}        let text = if self.{f}.checked {{ format!(\"\\u{{2714}} {{}}\", self.{f}.text) }} else {{ self.{f}.text.clone() }};\n\
             {p}        let button = egui::Button::new(text).shortcut_text(self.{f}.shortcut.to_string());\n\
             {click}\
             {p}    }}\n\
             {p}}}\n"
        ));
    }
    Ok(out)
}

const ESTADO_MAINMENU: StateText = StateText {
    head: r#"
/// A `MainMenu` on the form.
#[derive(Clone, Debug, Default)]
"#,
    tail: "",
};

const ESTADO_MENUITEM: StateText = StateText {
    head: r#"
/// One item of a menu: a `-` caption is a separator.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
