//! Makepad component implementation for `MainMenu`.

use super::codegen::MakepadComponentCodegen;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct MainMenuCodegen;

impl MakepadComponentCodegen for MainMenuCodegen {
    fn class(&self) -> &'static str {
        "MainMenu"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MainMenu").expect("MainMenu desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MainMenu {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MainMenu").expect("MainMenu desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("MainMenu {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MainMenu").expect("MainMenu desc");
        let fields = state::fields_of(desc);
        if fields.is_empty() {
            return String::new();
        }
        let reads = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        // one field is no tuple (`unused_parens`)
        if fields.len() == 1 {
            format!("        let _ = {reads};\n")
        } else {
            format!("        let _ = ({reads});\n")
        }
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }
}

#[crawrad_note::note("targets-makepad-components-main-menu-menuitemcodegen")]
pub struct MenuItemCodegen;

impl MakepadComponentCodegen for MenuItemCodegen {
    fn class(&self) -> &'static str {
        "MenuItem"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MenuItem").expect("MenuItem desc");
        // the linked Action is resolved by the generator, as a Button's
        let body = state::fields_of(desc)
            .iter()
            .filter(|(n, _)| n != "action")
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MenuItem {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MenuItem").expect("MenuItem desc");
        let body = state::inits_of(node, desc)
            .iter()
            .filter(|(n, _)| n != "action")
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("MenuItem {{ {body} }}")
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let f = field;
        let shown = format!(
            "        if self.ui.widget(id!({f})).visible() != self.form.{f}.visible {{\n\
             \x20           self.ui.widget(id!({f})).set_visible(cx, self.form.{f}.visible);\n\
             \x20       }}\n"
        );
        // a separator is a line: whether it is there is all it shows
        if crate::targets::makepad::menus::is_separator(node) {
            return format!(
                "{shown}\
                 \x20       let _ = (&self.form.{f}.text, self.form.{f}.checked, &self.form.{f}.shortcut, self.form.{f}.enabled);\n"
            );
        }
        let on_bar = doc
            .parent_of(&node.id)
            .and_then(|p| doc.node(p))
            .is_some_and(|p| p.class == "MainMenu");
        // in a list the check mark has its room even when absent, so the
        // captions line up; on the bar it takes none
        let text = if on_bar {
            format!(
                "let text = crawrad_runtime_makepad::menu::entry(&self.form.{f}.text, self.form.{f}.checked, &self.form.{f}.shortcut.to_string());"
            )
        } else if crate::targets::makepad::menus::items_of(doc, node).is_empty() {
            format!(
                "let text = crawrad_runtime_makepad::menu::caption(&self.form.{f}.text, self.form.{f}.checked, &self.form.{f}.shortcut.to_string());"
            )
        } else {
            // an item with items of its own says it opens a sub-menu
            format!(
                "let text = crawrad_runtime_makepad::menu::caption(&self.form.{f}.text, self.form.{f}.checked, &self.form.{f}.shortcut.to_string()) + \"  \\u{{203A}}\";"
            )
        };
        format!(
            "        {{\n\
             \x20           {text}\n\
             \x20           let b = self.ui.button(id!({f}));\n\
             \x20           if b.text() != text {{\n\
             \x20               b.set_text(cx, &text);\n\
             \x20           }}\n\
             \x20       }}\n\
             {shown}\
             \x20       crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({f}), self.form.{f}.enabled);\n"
        )
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // a separator is a line, with no click
        if crate::targets::makepad::menus::is_separator(node) {
            return Ok(String::new());
        }
        // one `if` for the click: what it does to the lists, then OnClick
        let mut body =
            crate::targets::makepad::menus::item_click(doc, emitted, node)?.unwrap_or_default();
        if let Some(h) = crate::targets::makepad::layout::handler(node, "on_click") {
            body.push_str(&format!(
                "            self.form.{}();\n",
                crate::identificador(&h)?
            ));
        }
        if body.is_empty() {
            return Ok(String::new());
        }
        Ok(format!(
            "        if self.ui.button(id!({field})).clicked(actions) {{\n{body}        }}\n"
        ))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        // the menus write it, in the bar or in a list (`menus::dsl`)
        Ok(String::new())
    }
}
