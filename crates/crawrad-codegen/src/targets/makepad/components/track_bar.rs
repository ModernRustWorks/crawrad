//! Makepad component implementation for `TrackBar`.
//!
//! A real slider: the emitted widget follows `makepad.look`
//! (`SliderMinimal`, `Slider` or `SliderRound`) with the TrackBar's range,
//! whole steps and its orientation. The sync moves it to the form's `position`;
//! dragging it writes the position back and calls `OnChange`
//! (`SliderRef::slided`, `:2894`). It used to be a painted bar that never moved.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node, Value};

#[crawrad_note::note("targets-makepad-components-track-bar-int")]
fn int(node: &Node, key: &str, default: i64) -> i64 {
    match node.props.get(key) {
        Some(Value::Int(i)) => *i,
        _ => default,
    }
}

pub struct TrackBarCodegen;

impl MakepadComponentCodegen for TrackBarCodegen {
    fn class(&self) -> &'static str {
        "TrackBar"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("TrackBar").expect("TrackBar desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct TrackBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("TrackBar").expect("TrackBar desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("TrackBar {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("TrackBar").expect("TrackBar desc");
        let fields = state::fields_of(desc);
        if fields.is_empty() {
            return String::new();
        }
        // the range and the orientation are the design's (the DSL); the
        // position follows the form at run time
        let reads = fields
            .iter()
            .filter(|(f, _)| f != "position")
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "        self.ui.slider(id!({field})).set_value(cx, self.form.{field}.position as f64);\n\
             \x20       let _ = ({reads});\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let on_change = match layout::handler(node, "on_change") {
            Some(h) => format!("            self.form.{}();\n", identificador(&h)?),
            None => String::new(),
        };
        Ok(format!(
            "        if let Some(_val) = self.ui.slider(id!({field})).slided(actions) {{\n\
             \x20           self.form.{field}.position = _val.round() as i64;\n\
             {on_change}\
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
        let widget = match node.props.get("makepad.look") {
            Some(Value::Enum(l) | Value::Str(l)) if l == "Box" => "Slider",
            Some(Value::Enum(l) | Value::Str(l)) if l == "Round" => "SliderRound",
            _ => "SliderMinimal",
        };
        let (min, max) = (int(node, "min", 0), int(node, "max", 10));
        // the slider takes `f64`s: written as such, not as whole numbers
        let position = int(node, "position", min).clamp(min, max.max(min)) as f64;
        let (min, max) = (min as f64, max as f64);
        let vertical =
            matches!(node.props.get("orientation"), Some(Value::Enum(o)) if o == "Vertical");
        let axis = if vertical { "Vertical" } else { "Horizontal" };
        // `SliderRound` keeps 75 px on its left for a label the TrackBar
        // never has (`slider.rs:1141`): the pill takes the whole width
        let label = if widget == "SliderRound" {
            "\n                draw_bg: { label_size: 0.0 },"
        } else {
            ""
        };
        Ok(format!(
            "            {name} = <{widget}> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               text: \"\", precision: 0,\n\
             \x20               min: {min:.1}, max: {max:.1}, step: 1.0, default: {position:.1},\n\
             \x20               axis: {axis},{label}\n\
             \x20           }}\n"
        ))
    }
}

#[cfg(test)]
#[path = "../../../test/track_bar_look/test.rs"]
mod tests;
