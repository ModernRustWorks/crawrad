//! Makepad component implementation for `RadioGroup`: the runtime's
//! `CrawradRadioGroup` draws the items the form holds now and reports a click.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Item, Node};

pub struct RadioGroupCodegen;

impl MakepadComponentCodegen for RadioGroupCodegen {
    fn class(&self) -> &'static str {
        "RadioGroup"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("RadioGroup").expect("RadioGroup desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct RadioGroup {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("RadioGroup").expect("RadioGroup desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
                // the options as the emitted `Strings` takes them
                if n == "items" {
                    let items = node
                        .props
                        .get("items")
                        .map(Item::list_from)
                        .unwrap_or_default();
                    let quoted: Vec<String> =
                        items.iter().map(|it| crate::literal(&it.text)).collect();
                    format!("items: Strings::from_slice(&[{}])", quoted.join(", "))
                } else {
                    format!("{n}: {v}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("RadioGroup {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let forget = crate::targets::shared::forget_selection_past_end(
            &format!("self.form.{field}"),
            crate::targets::shared::Selection::Signed,
        );
        format!(
            "{forget}        // what the form holds now, its font the control's own or the form's\n\
             \x20       {{\n\
             \x20           let (font_size, text) = if self.form.{field}.parent_font {{ (CRAWRAD_FORM_FONT_SIZE, CRAWRAD_FORM_FONT_COLOR) }} else {{ (self.form.{field}.font.size, self.form.{field}.font.color) }};\n\
             \x20           self.ui.crawrad_radio_group(id!({field})).set_state(cx, crawrad_runtime_makepad::radio_group::GroupState {{\n\
             \x20               caption: self.form.{field}.text.clone(),\n\
             \x20               items: self.form.{field}.items.to_vec(),\n\
             \x20               index: self.form.{field}.item_index,\n\
             \x20               columns: self.form.{field}.columns,\n\
             \x20               enabled: self.form.{field}.enabled,\n\
             \x20               font_size,\n\
             \x20               text,\n\
             \x20           }});\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut calls = String::new();
        if let Some(h) = layout::handler(node, "on_click") {
            calls.push_str(&format!(
                "            self.form.{}();\n",
                identificador(&h)?
            ));
        }
        if let Some(h) = layout::handler(node, "on_change") {
            calls.push_str(&format!(
                "            if changed {{\n                self.form.{}();\n            }}\n",
                identificador(&h)?
            ));
        }
        let changed = if calls.contains("if changed") {
            format!("            let changed = self.form.{field}.item_index != i as i64;\n")
        } else {
            String::new()
        };
        Ok(format!(
            "        if let Some(i) = self.ui.crawrad_radio_group(id!({field})).picked(actions) {{\n\
             {changed}\
             \x20           self.form.{field}.item_index = i as i64;\n\
             {calls}\
             \x20       }}\n"
        ))
    }

    #[crawrad_note::note("targets-makepad-components-radio-group-radiogroupcodegen-app-state")]
    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::radio_group::CrawradRadioGroupWidgetRefExt;\n"
                .to_string(),
            ..AppState::default()
        }))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        Ok(format!(
            "            {name} = <CrawradRadioGroup> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}
