use super::codegen::{EguiComponentCodegen, StateText};
use super::main_menu::{items_code, items_of};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Document;

pub struct PopupMenuCodegen;

impl EguiComponentCodegen for PopupMenuCodegen {
    fn class(&self) -> &'static str {
        "PopupMenu"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_POPUPMENU)
    }

    #[crawrad_note::note("targets-egui-components-popup-menu-popupmenucodegen-emit")]
    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let name = ctx.nome;
        let items = items_code(ctx.doc, ctx.node, 6)?;
        let body = if items.is_empty() {
            format!("        let _ = &self.{name};\n")
        } else {
            format!(
                "        // the PopupMenu, open where a secondary click asked for it\n\
                 \x20       {{\n\
                 \x20           let _ = &self.{name};\n\
                 \x20           let id = egui::Id::new((\"crawrad_popup\", {key}));\n\
                 \x20           if let Some(at) = ui.ctx().data(|d| d.get_temp::<egui::Pos2>(id)) {{\n\
                 \x20               egui::Popup::new(id, ui.ctx().clone(), egui::PopupAnchor::Position(at), ui.layer_id())\n\
                 \x20                   .kind(egui::PopupKind::Menu)\n\
                 \x20                   .layout(egui::Layout::top_down_justified(egui::Align::Min))\n\
                 \x20                   .style(egui::containers::menu::menu_style)\n\
                 \x20                   .open_memory(None)\n\
                 \x20                   .show(|ui| {{\n\
                 {items}\
                 \x20                   }});\n\
                 \x20           }}\n\
                 \x20       }}\n",
                key = literal(name)
            )
        };
        Ok(ResultadoComponente {
            campo: format!("    pub {name}: PopupMenu,\n"),
            padrao: format!("            {name}: PopupMenu::default(),\n"),
            constante: String::new(),
            corpo: body,
            tipo_usado: "PopupMenu",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

#[crawrad_note::note("targets-egui-components-popup-menu-popup-trigger")]
pub fn popup_trigger(doc: &Document, control: &str) -> Result<String, Erro> {
    let mut arms = Vec::new();
    for m in doc.nodes.values() {
        if m.class != "PopupMenu" || items_of(doc, m).is_empty() {
            continue;
        }
        let f = identificador(m.name())?;
        let on_popup = match handler(m, "on_popup") {
            Some(h) => format!("                self.{}();\n", identificador(&h)?),
            None => String::new(),
        };
        arms.push((
            literal(&f),
            format!(
                "{on_popup}\
                 \x20               let id = egui::Id::new((\"crawrad_popup\", {}));\n\
                 \x20               ui.ctx().data_mut(|d| d.insert_temp(id, at));\n\
                 \x20               egui::Popup::open_id(ui.ctx(), id);\n",
                literal(&f)
            ),
        ));
    }
    if arms.is_empty() {
        return Ok(String::new());
    }
    let open = if let [(name, body)] = arms.as_slice() {
        format!(
            "            if self.{control}.popup_menu == {name} {{\n\
             {body}\
             \x20           }}\n"
        )
    } else {
        let mut m = format!("            match self.{control}.popup_menu.as_str() {{\n");
        for (name, body) in &arms {
            m.push_str(&format!(
                "                {name} => {{\n\
                 {body}\
                 \x20               }}\n"
            ));
        }
        m.push_str("                _ => {}\n            }\n");
        m
    };
    Ok(format!(
        "        if self.{control}.visible && ui.rect_contains_pointer(r) && ui.input(|i| i.pointer.secondary_clicked()) {{\n\
         \x20           let at = ui.input(|i| i.pointer.interact_pos()).unwrap_or(r.center());\n\
         {open}\
         \x20       }}\n"
    ))
}

const ESTADO_POPUPMENU: StateText = StateText {
    head: r#"
/// A `PopupMenu` on the form.
#[derive(Clone, Debug, Default)]
"#,
    tail: "",
};
