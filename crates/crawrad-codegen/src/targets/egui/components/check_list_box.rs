use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct CheckListBoxCodegen;

impl EguiComponentCodegen for CheckListBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckListBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_CHECKLISTBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }

    fn rola(&self) -> bool {
        true
    }
}

#[crawrad_note::note("targets-egui-components-check-list-box-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx
        .reg
        .get("CheckListBox")
        .expect("CheckListBox descriptor");

    let items = match n.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let const_name = format!("{}_ITEMS", name.to_uppercase());
    let items_const = format!(
        "const {const_name}: &[&str] = &[\n{}];\n\n",
        items
            .iter()
            .map(|i| format!("    {},\n", literal(&i.text)))
            .collect::<String>()
    );

    let checked_items = match n.props.get("checked_items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let chk_const_name = format!("{}_CHECKED_ITEMS", name.to_uppercase());
    let chk_const = format!(
        "const {chk_const_name}: &[&str] = &[\n{}];\n\n",
        checked_items
            .iter()
            .map(|i| format!("    {},\n", literal(&i.text)))
            .collect::<String>()
    );

    let constante = format!("{items_const}{chk_const}");

    let on_change = handler(n, "on_change");
    let change_call = match on_change {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let dispatch_change = if !change_call.is_empty() {
        format!("                {}", change_call)
    } else {
        String::new()
    };

    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{name}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           let c = self.{name}.color;\n\
         \x20           let fill = if !self.{name}.parent_color && c.a > 0 {{\n\
         \x20               egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)\n\
         \x20           }} else {{\n\
         \x20               w.visuals().extreme_bg_color\n\
         \x20           }};\n\
         \x20           let mut toggle_idx: Option<(usize, bool)> = None;\n\
         \x20           egui::Frame::group(w.style())\n\
         \x20               .fill(fill)\n\
         \x20               .stroke(w.visuals().widgets.inactive.bg_stroke)\n\
         \x20               .corner_radius(egui::CornerRadius::same(2))\n\
         \x20               .inner_margin(egui::Margin::same(4))\n\
         \x20               .show(&mut w, |ui| {{\n\
         \x20                   egui::ScrollArea::vertical()\n\
         \x20                       .id_salt((\"rxp-checklistbox\", \"{name}\"))\n\
         \x20                       .max_height(r.height())\n\
         \x20                       .max_width(r.width())\n\
         \x20                       .auto_shrink([false, false])\n\
         \x20                       .show(ui, |ui| {{\n\
         \x20                           for (i, text) in self.{name}.items.iter().enumerate() {{\n\
         \x20                               let mut checked = self.{name}.checked_items.iter().any(|s| s == text);\n\
         \x20                               if ui.checkbox(&mut checked, text.as_str()).changed() {{\n\
         \x20                                   toggle_idx = Some((i, checked));\n\
         \x20                               }}\n\
         \x20                           }}\n\
         \x20                       }});\n\
         \x20               }});\n\
         \x20           if let Some((idx, on)) = toggle_idx {{\n\
         \x20               let item_strs: Vec<String> = self.{name}.items.iter().cloned().collect();\n\
         \x20               let mut checked_vec: Vec<String> = self.{name}.checked_items.iter().cloned().collect();\n\
         \x20               set_checked_item(&item_strs, &mut checked_vec, idx, on);\n\
         \x20               self.{name}.checked_items = Strings::from_slice(&checked_vec.iter().map(|s| s.as_str()).collect::<Vec<_>>());\n\
         {dispatch_change}\
         \x20           }}\n\
         \x20       }}\n"
    );

    let padrao = format!(
        "            {name}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: CheckListBox,\n"),
        padrao,
        constante,
        corpo: body,
        tipo_usado: "CheckListBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_CHECKLISTBOX: StateText = StateText {
    head: r#"
/// A `CheckListBox` component on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
