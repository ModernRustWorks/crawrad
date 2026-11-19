use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct ComboBoxCodegen;

impl EguiComponentCodegen for ComboBoxCodegen {
    fn class(&self) -> &'static str {
        "ComboBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_COMBOBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ComboBox").expect("ComboBox descriptor");

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
    let call = |event: &str| -> Result<Option<String>, Erro> {
        handler(n, event)
            .map(|h| identificador(&h).map(|h| format!("self.{h}();")))
            .transpose()
    };
    // `OnSelect`: the item the user picked, the same one or not
    let select = call("on_select")?;
    let select_call = match &select {
        Some(c) => format!("            if picked {{\n                {c}\n            }}\n"),
        None => "            let _ = picked;\n".to_string(),
    };
    let change_call = match call("on_change")? {
        Some(c) => format!("            if changed {{\n                {c}\n            }}\n"),
        None => "            let _ = changed;\n".to_string(),
    };
    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{nome}"),
        crate::targets::shared::Selection::Optional,
    );
    let forget = forget + &crate::targets::shared::text_follows_selection(&format!("self.{nome}"));
    let corpo = format!(
        "{forget}        if self.{nome}.sorted {{\n\
         \x20           crawrad_runtime::list_selection::sort(&mut self.{nome}.items, &mut self.{nome}.selected, &mut []);\n\
         \x20       }}\n\
         \x20       if self.{nome}.visible {{\n\
         \x20           let before = self.{nome}.selected;\n\
         \x20           let text_before = self.{nome}.text.clone();\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{nome}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           w.spacing_mut().interact_size.y = r.height();\n\
         \x20           let row_h = r.height().max(20.0);\n\
         \x20           let drop_h = if self.{nome}.drop_down_count == 0 {{\n\
         \x20               f32::INFINITY\n\
         \x20           }} else {{\n\
         \x20               row_h * (self.{nome}.drop_down_count.max(1) as f32) + 8.0\n\
         \x20           }};\n\
         \x20           let mut picked = false;\n\
         \x20           if self.{nome}.style == \"DropDown\" {{\n\
         \x20               let caret_w = 22.0f32;\n\
         \x20               let text_rect = egui::Rect::from_min_max(r.min, egui::pos2(r.max.x - caret_w, r.max.y));\n\
         \x20               let caret_rect = egui::Rect::from_min_max(egui::pos2(r.max.x - caret_w, r.min.y), r.max);\n\
         \x20               let text_res = w.put(text_rect, egui::TextEdit::singleline(&mut self.{nome}.text));\n\
         \x20               let caret_res = w.put(caret_rect, egui::Button::new(\"▾\"));\n\
         \x20               let popup_id = ui.make_persistent_id(concat!({id}, \"_popup\"));\n\
         \x20               let mut is_open = ui.data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));\n\
         \x20               if text_res.changed() {{\n\
         \x20                   is_open = true;\n\
         \x20               }}\n\
         \x20               if caret_res.clicked() {{\n\
         \x20                   is_open = !is_open;\n\
         \x20               }}\n\
         \x20               let filtered = crawrad_runtime::list_selection::matching(&self.{nome}.items, &self.{nome}.text);\n\
         \x20               let mut chosen = None;\n\
         \x20               if text_res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {{\n\
         \x20                   match filtered.first() {{\n\
         \x20                       Some(first) if is_open => chosen = Some(*first),\n\
         \x20                       _ => {{\n\
         \x20                           self.{nome}.selected = self.{nome}.items.iter().position(|t| t == &self.{nome}.text);\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20                   is_open = false;\n\
         \x20               }}\n\
         \x20               egui::Popup::from_response(&text_res)\n\
         \x20                   .open_bool(&mut is_open)\n\
         \x20                   .width(r.width())\n\
         \x20                   .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)\n\
         \x20                   .show(|ui| {{\n\
         \x20                       egui::ScrollArea::vertical().max_height(drop_h).show(ui, |ui| {{\n\
         \x20                           for i in &filtered {{\n\
         \x20                               let text = self.{nome}.items[*i].as_str();\n\
         \x20                               if ui.selectable_label(self.{nome}.selected == Some(*i), text).clicked() {{\n\
         \x20                                   chosen = Some(*i);\n\
         \x20                               }}\n\
         \x20                           }}\n\
         \x20                       }});\n\
         \x20                   }});\n\
         \x20               if let Some(i) = chosen {{\n\
         \x20                   self.{nome}.selected = Some(i);\n\
         \x20                   self.{nome}.text = self.{nome}.items[i].clone();\n\
         \x20                   picked = true;\n\
         \x20                   is_open = false;\n\
         \x20               }}\n\
         \x20               ui.data_mut(|d| d.insert_temp(popup_id, is_open));\n\
         \x20           }} else {{\n\
         \x20               let label = self.{nome}\n\
         \x20                   .selected\n\
         \x20                   .and_then(|i| self.{nome}.items.get(i).map(|s| s.as_str()))\n\
         \x20                   .unwrap_or(\"\");\n\
         \x20               egui::ComboBox::from_id_salt({id})\n\
         \x20                   .selected_text(label)\n\
         \x20                   .width(r.width())\n\
         \x20                   .height(drop_h)\n\
         \x20                   .show_ui(&mut w, |ui| {{\n\
         \x20                       for (i, text) in self.{nome}.items.iter().enumerate() {{\n\
         \x20                           if ui\n\
         \x20                               .selectable_value(\n\
         \x20                                   &mut self.{nome}.selected,\n\
         \x20                                   Some(i),\n\
         \x20                                   text.as_str(),\n\
         \x20                               )\n\
         \x20                               .clicked()\n\
         \x20                           {{\n\
         \x20                               picked = true;\n\
         \x20                           }}\n\
         \x20                       }}\n\
         \x20                   }});\n\
         \x20               if picked\n\
         \x20                   && let Some(item_text) = self.{nome}.selected.and_then(|i| self.{nome}.items.get(i).cloned())\n\
         \x20               {{\n\
         \x20                   self.{nome}.text = item_text;\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20           let changed = self.{nome}.selected != before || self.{nome}.text != text_before;\n\
         {select_call}\
         {change_call}\
         \x20       }}\n",
        id = literal(nome),
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ComboBox,\n"),
        padrao,
        constante,
        corpo,
        tipo_usado: "ComboBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_COMBOBOX: StateText = StateText {
    head: r#"
/// A `ComboBox` on the form. `selected` is the index of the chosen item, or
/// `None` — classic RAD `ItemIndex` without the `-1`.
/// `items` holds the string choices — classic RAD `TComboBox.Items`.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
