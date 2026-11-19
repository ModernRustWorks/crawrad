//! Makepad component implementation for `CheckListBox`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Item, Node};

pub struct CheckListBoxCodegen;

impl MakepadComponentCodegen for CheckListBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckListBox"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_check_list_box")
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("CheckListBox").expect("CheckListBox desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct CheckListBox {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("CheckListBox").expect("CheckListBox desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
                if n == "items" || n == "checked_items" {
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
        format!("CheckListBox {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        {{\n\
             \x20           let check_list_box = self.ui.crawrad_check_list_box(id!({field}));\n\
             \x20           check_list_box.set_items(cx, &self.form.{field}.items);\n\
             \x20           check_list_box.set_checked(cx, &crawrad_runtime_makepad::check_list::checked_flags(&self.form.{field}.items, &self.form.{field}.checked_items));\n\
             \x20           check_list_box.set_enabled(cx, self.form.{field}.enabled);\n\
             \x20           check_list_box.set_visible(cx, self.form.{field}.visible);\n\
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

        let mut code = String::new();
        code.push_str(&format!(
            "        if let Some((i, on)) = self.ui.crawrad_check_list_box(id!({field})).toggled(actions) {{\n\
             \x20           crawrad_runtime_makepad::check_list::set_checked_item(&self.form.{field}.items, &mut self.form.{field}.checked_items, i, on);\n\
             {on_change_call}\
             \x20       }}\n"
        ));

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::check_list::CrawradCheckListBoxWidgetRefExt;\n"
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
            "            {name} = <CrawradCheckListBox> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}
