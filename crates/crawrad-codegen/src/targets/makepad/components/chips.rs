//! Makepad component implementation for `Chips`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Item, Node};

pub struct ChipsCodegen;

impl MakepadComponentCodegen for ChipsCodegen {
    fn class(&self) -> &'static str {
        "Chips"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_chips")
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Chips").expect("Chips desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Chips {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Chips").expect("Chips desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
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
        format!("Chips {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let forget = crate::targets::shared::forget_selection_past_end(
            &format!("self.form.{field}"),
            crate::targets::shared::Selection::Signed,
        );
        format!(
            "{forget}        {{\n\
             \x20           let chips = self.ui.crawrad_chips(id!({field}));\n\
             \x20           chips.set_items(cx, &self.form.{field}.items);\n\
             \x20           chips.set_item_index(cx, self.form.{field}.item_index);\n\
             \x20           chips.set_removable(cx, self.form.{field}.removable);\n\
             \x20           chips.set_enabled(cx, self.form.{field}.enabled);\n\
             \x20           chips.set_visible(cx, self.form.{field}.visible);\n\
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
        let mut on_click_call = String::new();
        if let Some(h) = layout::handler(node, "on_click") {
            on_click_call = format!("                self.form.{}();\n", identificador(&h)?);
        }
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
            format!("            let changed = self.form.{field}.item_index != i as i64;\n")
        } else {
            String::new()
        };

        let removed_change = if !on_change_call.is_empty() {
            format!(
                "            self.form.{}();\n",
                identificador(&layout::handler(node, "on_change").unwrap())?
            )
        } else {
            String::new()
        };

        let mut code = String::new();

        code.push_str(&format!(
            "        if let Some(i) = self.ui.crawrad_chips(id!({field})).selected(actions) {{\n\
             {decl_changed}\
             \x20           self.form.{field}.item_index = i as i64;\n\
             {on_click_call}\
             {change_check}\
             \x20       }}\n"
        ));

        code.push_str(&format!(
            "        if let Some(i) = self.ui.crawrad_chips(id!({field})).removed(actions)\n\
             \x20           && i < self.form.{field}.items.len()\n\
             \x20       {{\n\
             \x20           self.form.{field}.items.remove(i);\n\
             \x20           if self.form.{field}.item_index == i as i64 {{\n\
             \x20               self.form.{field}.item_index = -1;\n\
             \x20           }} else if self.form.{field}.item_index > i as i64 {{\n\
             \x20               self.form.{field}.item_index -= 1;\n\
             \x20           }}\n\
             {removed_change}\
             \x20       }}\n"
        ));

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::chips::CrawradChipsWidgetRefExt;\n".to_string(),
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
            "            {name} = <CrawradChips> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}
