//! Makepad component implementation for `Splitter`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct SplitterCodegen;

impl MakepadComponentCodegen for SplitterCodegen {
    fn class(&self) -> &'static str {
        "Splitter"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Splitter").expect("Splitter desc");
        // `offset` and `press` come from `state::EXTRA_FIELDS`
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Splitter {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Splitter").expect("Splitter desc");
        let mut inits: Vec<String> = state::inits_of(node, desc)
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect();
        inits.push("offset: 0.0".to_string());
        inits.push("press: 0.0".to_string());
        format!("Splitter {{ {} }}", inits.join(", "))
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!("        let _ = (self.form.{field}.auto_snap, self.form.{field}.beveled);\n")
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // press, drag (offset = where it started + how far the finger
        // went, within MinSize of both neighbours), and OnMoved on release
        let split = crate::targets::splitters::split_of(doc, field);
        let vertical = split.as_ref().is_none_or(|s| s.vertical);
        let axis = if vertical { "x" } else { "y" };
        let lo = match split.as_ref().and_then(|s| s.before.as_ref()) {
            Some((_, len)) => format!("self.form.{field}.min_size as f32 - {len:.1}"),
            None => "f32::NEG_INFINITY".to_string(),
        };
        let hi = match split.as_ref().and_then(|s| s.after.as_ref()) {
            Some((_, len)) => format!("{len:.1} - self.form.{field}.min_size as f32"),
            None => "f32::INFINITY".to_string(),
        };
        let moved = match layout::handler(node, "on_moved") {
            Some(h) => format!("            self.form.{}();\n", identificador(&h)?),
            None => String::new(),
        };
        Ok(format!(
            "        if self.ui.view(id!({field})).finger_down(actions).is_some() {{\n\
             \x20           self.form.{field}.press = self.form.{field}.offset;\n\
             \x20       }}\n\
             \x20       if let Some(fm) = self.ui.view(id!({field})).finger_move(actions) {{\n\
             \x20           let moved = self.form.{field}.press + (fm.abs.{axis} - fm.abs_start.{axis}) as f32;\n\
             \x20           self.form.{field}.offset = moved.clamp({lo}, {hi});\n\
             \x20       }}\n\
             \x20       if self.ui.view(id!({field})).finger_up(actions).is_some() {{\n\
             {moved}\
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
        // a View tests the pointer only with a cursor; the splitter's
        // shows which way it drags
        let cursor = if h >= w { "ColResize" } else { "RowResize" };
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{ color: #252538 }}\n\
             \x20               cursor: {cursor}\n\
             \x20           }}\n"
        ))
    }
}
