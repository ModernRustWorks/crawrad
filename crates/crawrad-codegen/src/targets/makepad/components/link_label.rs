//! Makepad component implementation for `LinkLabel`: makepad's own
//! `<LinkLabel>` (a `Button` drawn as a link). Its own `url` stays empty: the
//! click opens the form's `url` through `crawrad_runtime::shell`, which refuses
//! anything but web and mail.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct LinkLabelCodegen;

impl MakepadComponentCodegen for LinkLabelCodegen {
    fn class(&self) -> &'static str {
        "LinkLabel"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("LinkLabel").expect("LinkLabel desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct LinkLabel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("LinkLabel").expect("LinkLabel desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("LinkLabel {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        // the caption follows `text`\n\
             \x20       if self.ui.link_label(id!({field})).text() != self.form.{field}.text {{\n\
             \x20           self.ui.link_label(id!({field})).set_text(cx, &self.form.{field}.text);\n\
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
        let call = match layout::handler(node, "on_click") {
            Some(h) => format!("            self.form.{}();\n", identificador(&h)?),
            None => String::new(),
        };
        Ok(format!(
            "        if self.ui.link_label(id!({field})).clicked(actions) {{\n\
             \x20           crawrad_runtime::shell::open_url(&self.form.{field}.url);\n\
             {call}\
             \x20       }}\n"
        ))
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
        let caption = layout::text(node)
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        Ok(format!(
            "            {name} = <LinkLabel> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               text: \"{caption}\"\n\
             \x20           }}\n"
        ))
    }
}
