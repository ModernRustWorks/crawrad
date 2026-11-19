use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct ListBoxCodegen;

impl EguiComponentCodegen for ListBoxCodegen {
    fn class(&self) -> &'static str {
        "ListBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_LISTBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }

    #[crawrad_note::note("targets-egui-components-list-box-listboxcodegen-rola")]
    fn rola(&self) -> bool {
        true
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ListBox").expect("ListBox descriptor");

    let itens = match n.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let constante = format!(
        "const {}_ITEMS: &[&str] = &[\n{}];\n\n",
        nome.to_uppercase(),
        itens
            .iter()
            .map(|i| format!("    {},\n", literal(&i.text)))
            .collect::<String>()
    );
    let padrao = format!(
        "            {nome}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );
    let depois = match handler(n, "on_change") {
        Some(h) => format!(
            "            if changed {{\n                self.{}();\n            }}\n",
            identificador(&h)?
        ),
        None => "            let _ = changed;\n".to_string(),
    };
    // OnClick on any click, OnDblClick on the second of a double click
    let mut depois = depois;
    for (event, flag) in [("on_click", "clicked"), ("on_dbl_click", "double")] {
        depois.push_str(&match handler(n, event) {
            Some(h) => format!(
                "            if {flag} {{\n                self.{}();\n            }}\n",
                identificador(&h)?
            ),
            None => format!("            let _ = {flag};\n"),
        });
    }
    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{nome}"),
        crate::targets::shared::Selection::Optional,
    );
    let corpo = format!(
        "{forget}        if self.{nome}.sorted {{\n\
         \x20           crawrad_runtime::list_selection::sort(&mut self.{nome}.items, &mut self.{nome}.selected, &mut self.{nome}.selection);\n\
         \x20       }}\n\
         \x20       crawrad_runtime::list_selection::settle(&mut self.{nome}.selection, self.{nome}.selected, self.{nome}.items.len(), self.{nome}.multi_select);\n\
         \x20       if self.{nome}.visible {{\n\
         \x20           let before = self.{nome}.selected;\n\
         \x20           let (mut clicked, mut double) = (false, false);\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{nome}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           // its own Color, or the theme's list colour\n\
         \x20           let c = self.{nome}.color;\n\
         \x20           let fill = if !self.{nome}.parent_color && c.a > 0 {{\n\
         \x20               egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)\n\
         \x20           }} else {{\n\
         \x20               w.visuals().extreme_bg_color\n\
         \x20           }};\n\
         \x20           egui::Frame::group(w.style())\n\
         \x20               .fill(fill)\n\
         \x20               .stroke(w.visuals().widgets.inactive.bg_stroke)\n\
         \x20               .corner_radius(egui::CornerRadius::same(2))\n\
         \x20               .inner_margin(egui::Margin::ZERO)\n\
         \x20               .show(&mut w, |ui| {{\n\
         \x20                   egui::ScrollArea::vertical()\n\
         \x20                       .id_salt((\"rxp-listbox\", \"{nome}\"))\n\
         \x20                       .max_height(r.height())\n\
         \x20                       .max_width(r.width())\n\
         \x20                       .auto_shrink([false, false])\n\
         \x20                       .show(ui, |ui| {{\n\
         \x20                           ui.set_width(ui.available_width());\n\
         \x20                           ui.spacing_mut().item_spacing.y = 0.0;\n\
         \x20                           for (i, text) in self.{nome}.items.iter().enumerate() {{\n\
         \x20                               let row = ui.add(\n\
         \x20                                   egui::Button::selectable(self.{nome}.selection.contains(&i), text.as_str())\n\
         \x20                                       .min_size(egui::vec2(ui.available_width(), 18.0)),\n\
         \x20                               );\n\
         \x20                               if row.clicked() {{\n\
         \x20                                   let keys = ui.input(|input| input.modifiers);\n\
         \x20                                   crawrad_runtime::list_selection::click(&mut self.{nome}.selection, self.{nome}.selected, i, self.{nome}.multi_select, keys.command, keys.shift);\n\
         \x20                                   self.{nome}.selected = Some(i);\n\
         \x20                                   clicked = true;\n\
         \x20                               }}\n\
         \x20                               double |= row.double_clicked();\n\
         \x20                           }}\n\
         \x20                       }});\n\
         \x20               }});\n\
         \x20           let changed = self.{nome}.selected != before;\n\
         {depois}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ListBox,\n"),
        padrao,
        constante,
        corpo,
        tipo_usado: "ListBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_LISTBOX: StateText = StateText {
    head: r#"
/// A `ListBox` on the form. `selected` is the index of the chosen item, or
/// `None` — classic RAD `ItemIndex` without the `-1`.
/// `items` holds the string items — classic RAD `TListBox.Items`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
