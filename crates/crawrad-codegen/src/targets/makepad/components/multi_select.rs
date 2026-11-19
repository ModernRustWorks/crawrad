//! Makepad component implementation for `MultiSelect`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Item, Node};

pub struct MultiSelectCodegen;

impl MakepadComponentCodegen for MultiSelectCodegen {
    fn class(&self) -> &'static str {
        "MultiSelect"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_multi_select")
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MultiSelect").expect("MultiSelect desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MultiSelect {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MultiSelect").expect("MultiSelect desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
                // the string lists start with the designer's texts, like Chips' items
                if n == "items" || n == "selected_items" {
                    let texts = node
                        .props
                        .get(n.as_str())
                        .map(Item::list_from)
                        .unwrap_or_default();
                    let quoted: Vec<String> =
                        texts.iter().map(|it| crate::literal(&it.text)).collect();
                    format!("{n}: Strings::from_slice(&[{}])", quoted.join(", "))
                } else {
                    format!("{n}: {v}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("MultiSelect {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        {{\n\
             \x20           let multi_select = self.ui.crawrad_multi_select(id!({field}));\n\
             \x20           multi_select.set_items(cx, self.form.{field}.items.as_slice());\n\
             \x20           multi_select.set_selected_items(cx, self.form.{field}.selected_items.as_slice());\n\
             \x20           multi_select.set_text_hint(cx, &self.form.{field}.text_hint);\n\
             \x20           multi_select.set_drop_down_count(cx, self.form.{field}.drop_down_count);\n\
             \x20           multi_select.set_enabled(cx, self.form.{field}.enabled);\n\
             \x20           multi_select.set_visible(cx, self.form.{field}.visible);\n\
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
        let mut on_change_call = String::new();
        if let Some(h) = layout::handler(node, "on_change") {
            on_change_call = format!("                self.form.{}();\n", identificador(&h)?);
        }

        let change_check = if !on_change_call.is_empty() {
            format!(
                "            if changed {{\n{}\
                 \x20           }}\n",
                on_change_call
            )
        } else {
            String::new()
        };

        let decl_changed = if !on_change_call.is_empty() {
            format!(
                "            let changed = self.form.{field}.selected_items.as_slice() != new_selected.as_slice();\n"
            )
        } else {
            String::new()
        };

        let mut code = String::new();
        code.push_str(&format!(
            "        let mse = self.ui.crawrad_multi_select(id!({field}));\n\
             \x20       if mse.changed(actions) {{\n\
             \x20           let new_selected = mse.selected_items();\n\
             {decl_changed}\
             \x20           self.form.{field}.selected_items = Strings::from_slice(&new_selected.iter().map(|s| s.as_str()).collect::<Vec<_>>());\n\
             {change_check}\
             \x20       }}\n"
        ));

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::multi_select::CrawradMultiSelectWidgetRefExt;\n"
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
            "            {name} = <CrawradMultiSelect> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}
