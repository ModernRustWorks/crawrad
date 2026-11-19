use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct MultiSelectCodegen;

impl EguiComponentCodegen for MultiSelectCodegen {
    fn class(&self) -> &'static str {
        "MultiSelect"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_MULTISELECT)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-multi-select-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("MultiSelect").expect("MultiSelect descriptor");

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

    let selected_items = match n.props.get("selected_items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let sel_const_name = format!("{}_SELECTED_ITEMS", name.to_uppercase());
    let sel_const = format!(
        "const {sel_const_name}: &[&str] = &[\n{}];\n\n",
        selected_items
            .iter()
            .map(|i| format!("    {},\n", literal(&i.text)))
            .collect::<String>()
    );

    let constante = format!("{items_const}{sel_const}");

    let on_change = handler(n, "on_change");
    let change_call = match on_change {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let decl_flags = if !change_call.is_empty() {
        format!(
            "            let before_selected = self.{name}.selected_items.as_slice().to_vec();\n"
        )
    } else {
        String::new()
    };

    let dispatch_change = if !change_call.is_empty() {
        format!(
            "            if self.{name}.selected_items.as_slice() != before_selected.as_slice() {{\n                {}\
             \x20           }}\n",
            change_call
        )
    } else {
        String::new()
    };

    let body = format!(
        "        if self.{name}.visible {{\n\
         {decl_flags}\
         \x20           let item_strs: Vec<String> = self.{name}.items.iter().cloned().collect();\n\
         \x20           let selected_strs: Vec<String> = self.{name}.selected_items.iter().cloned().collect();\n\
         \x20           let font = egui::FontId::proportional(12.0);\n\
         \x20           let summary = multi_select_summary(\n\
         \x20               &item_strs,\n\
         \x20               &selected_strs,\n\
         \x20               &self.{name}.text_hint,\n\
         \x20               |text| {{\n\
         \x20                   ui.painter().layout_no_wrap(\n\
         \x20                       text.to_owned(),\n\
         \x20                       font.clone(),\n\
         \x20                       ui.visuals().text_color(),\n\
         \x20                   ).size().x <= (r.width() - 24.0).max(0.0)\n\
         \x20               }},\n\
         \x20           );\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{name}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           w.spacing_mut().interact_size.y = r.height();\n\
         \x20           let row_h = r.height().max(20.0);\n\
         \x20           let drop_h = row_h * (self.{name}.drop_down_count.max(1) as f32) + 8.0;\n\
         \x20           let mut toggle_idx: Option<usize> = None;\n\
         \x20           egui::ComboBox::from_id_salt(\"{name}\")\n\
         \x20               .selected_text(summary)\n\
         \x20               .width(r.width())\n\
         \x20               .height(drop_h)\n\
         \x20               .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)\n\
         \x20               .show_ui(&mut w, |ui| {{\n\
         \x20                   for (i, text) in self.{name}.items.iter().enumerate() {{\n\
         \x20                       let mut checked = self.{name}.selected_items.iter().any(|s| s == text);\n\
         \x20                       let resp = ui.checkbox(&mut checked, text.as_str());\n\
         \x20                       if resp.clicked() {{\n\
         \x20                           toggle_idx = Some(i);\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }});\n\
         \x20           if let Some(idx) = toggle_idx {{\n\
         \x20               let mut sel_vec: Vec<String> = self.{name}.selected_items.iter().cloned().collect();\n\
         \x20               toggle_selected(&item_strs, &mut sel_vec, idx);\n\
         \x20               self.{name}.selected_items = Strings::from_slice(&sel_vec.iter().map(|s| s.as_str()).collect::<Vec<_>>());\n\
         \x20           }}\n\
         {dispatch_change}\
         \x20       }}\n"
    );

    let padrao = format!(
        "            {name}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: MultiSelect,\n"),
        padrao,
        constante,
        corpo: body,
        tipo_usado: "MultiSelect",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_MULTISELECT: StateText = StateText {
    head: r#"
/// A `MultiSelect` component on the form (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
