//! Makepad component implementation for `ProgressBar`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct ProgressBarCodegen;

impl MakepadComponentCodegen for ProgressBarCodegen {
    fn class(&self) -> &'static str {
        "ProgressBar"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ProgressBar").expect("ProgressBar desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        // `Step` is what `StepIt` moves by — past `max` the bar starts
        // over from `min`, as the classic one does
        format!(
            "#[derive(Debug, Clone, Default)]\npub struct ProgressBar {{\n{body}\n}}\n\n\
             impl ProgressBar {{\n\
             \x20   /// `StepIt`: moves the position on by `step`; past `max` it starts\n\
             \x20   /// over from `min`, as the classic bar does.\n\
             \x20   pub fn step_it(&mut self) {{\n\
             \x20       self.position += self.step;\n\
             \x20       if self.position > self.max {{\n\
             \x20           self.position = self.min;\n\
             \x20       }}\n\
             \x20   }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ProgressBar").expect("ProgressBar desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("ProgressBar {{ {body} }}")
    }

    #[crawrad_note::note("targets-makepad-components-progress-bar-progressbarcodegen-emit-sync")]
    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let r = doc
            .rect_in_form(&node.id)
            .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 150.0, 16.0));
        let (w, h) = (r.w, r.h);
        // the fields not drawn yet stay read, as before (written out whole:
        // the state gate reads `self.form.<field>.<prop>`)
        let reg = crawrad_registry::Registry::builtin();
        let drawn = ["position", "min", "max", "orientation"];
        let rest: Vec<String> = state::fields_of(reg.get("ProgressBar").expect("ProgressBar desc"))
            .into_iter()
            .filter(|(f, _)| !drawn.contains(&f.as_str()))
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect();
        let rest = match rest.is_empty() {
            true => String::new(),
            false => format!("        let _ = ({});\n", rest.join(", ")),
        };
        format!(
            "        let fraction = crawrad_runtime_makepad::progress::fraction(\n\
             \x20           self.form.{field}.position,\n\
             \x20           self.form.{field}.min,\n\
             \x20           self.form.{field}.max,\n\
             \x20       );\n\
             \x20       let vertical = self.form.{field}.orientation == \"Vertical\";\n\
             \x20       crawrad_runtime_makepad::progress::apply(cx, &self.ui, id!({field}), fraction, vertical, {w:.1}, {h:.1});\n\
             {rest}"
        )
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
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               align: {{ x: 0.0, y: 1.0 }},\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{ color: #1e1e1e }},\n\
             \x20               bar = <View> {{\n\
             \x20                   width: 0.0, height: Fill,\n\
             \x20                   show_bg: true,\n\
             \x20                   draw_bg: {{ color: #007acc }}\n\
             \x20               }}\n\
             \x20           }}\n"
        ))
    }
}
