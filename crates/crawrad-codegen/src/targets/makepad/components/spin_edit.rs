//! Makepad component implementation for `SpinEdit`: a row with the
//! number in a `TextInput` (`<field>_edit`) and the step buttons
//! (`<field>_up`, `<field>_down`) at its right.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

#[crawrad_note::note("targets-makepad-components-spin-edit-steps")]
const STEPS: f32 = 16.0;

#[crawrad_note::note("targets-makepad-components-spin-edit-inset")]
const INSET: f32 = 2.0;

pub struct SpinEditCodegen;

impl MakepadComponentCodegen for SpinEditCodegen {
    fn class(&self) -> &'static str {
        "SpinEdit"
    }

    fn default_cursor(&self) -> &'static str {
        "text"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SpinEdit").expect("SpinEdit desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct SpinEdit {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("SpinEdit").expect("SpinEdit desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("SpinEdit {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        emit_sync_inner(field, false)
    }

    fn emit_sync_with_reg(
        &self,
        reg: &crawrad_registry::Registry,
        _doc: &Document,
        node: &Node,
        field: &str,
    ) -> String {
        let is_inheriting = reg.get(&node.class).and_then(|d| d.inherits).is_some();
        emit_sync_inner(field, is_inheriting)
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions_inner(node, field, false)
    }

    fn emit_actions_with_reg(
        &self,
        reg: &crawrad_registry::Registry,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let is_inheriting = reg.get(&node.class).and_then(|d| d.inherits).is_some();
        emit_actions_inner(node, field, is_inheriting)
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
        // the number's capitals in the middle, by the Edit's rule; the
        // field takes the control's height (`Fill` drew it shorter).
        let top = super::edit::text_top(node, h);
        // One piece, as on the canvas: the field's frame is the control's
        // outline and the square buttons sit inside its right edge. A button
        // beside the field drew three rounded shapes (one radius per widget,
        // so the field cannot square only its right corners). The strip sits
        // `INSET` in, clear of the field's rounded corners; the buttons' radius
        // is half a point, because a box of radius 0 draws nothing in
        // makepad 1.0.
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Overlay, align: {{ x: 1.0, y: 0.0 }}\n\
             \x20               {name}_edit = <TextInput> {{ width: Fill, height: {h}, margin: 0.0, padding: {{ top: {top:.1}, bottom: 0.0, left: 6.0, right: {right:.1} }}, empty_text: \"\" }}\n\
             \x20               {name}_steps = <View> {{\n\
             \x20                   width: {STEPS:.1}, height: Fill, flow: Down, spacing: 1.0, margin: {{ top: {INSET:.1}, right: {INSET:.1}, bottom: {INSET:.1} }}\n\
             \x20                   {name}_up = <Button> {{ width: Fill, height: Fill, margin: 0.0, padding: 0.0, text: \"+\", draw_text: {{ text_style: {{ font_size: 7.0 }} }}, draw_bg: {{ border_radius: 0.5 }} }}\n\
             \x20                   {name}_down = <Button> {{ width: Fill, height: Fill, margin: 0.0, padding: 0.0, text: \"-\", draw_text: {{ text_style: {{ font_size: 7.0 }} }}, draw_bg: {{ border_radius: 0.5 }} }}\n\
             \x20               }}\n\
             \x20           }}\n",
            right = STEPS + INSET + 6.0,
        ))
    }
}

fn emit_sync_inner(field: &str, is_inheriting: bool) -> String {
    let shown = if is_inheriting {
        format!("self.form.{field}_ext.display(self.form.{field}.value)")
    } else {
        format!("self.form.{field}.value.to_string()")
    };
    format!(
        "        // the number, typed only with EditorEnabled\n\
         \x20       {{\n\
         \x20           let ti = self.ui.text_input(id!({field}_edit));\n\
         \x20           let shown = {shown};\n\
         \x20           if ti.text() != shown {{\n\
         \x20               ti.set_text(cx, &shown);\n\
         \x20           }}\n\
         \x20           let read_only = !self.form.{field}.editor_enabled || !self.form.{field}.enabled;\n\
         \x20           if ti.is_read_only() != read_only {{\n\
         \x20               ti.set_is_read_only(cx, read_only);\n\
         \x20           }}\n\
         \x20           crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({field}_up), self.form.{field}.enabled);\n\
         \x20           crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({field}_down), self.form.{field}.enabled);\n\
         \x20       }}\n"
    )
}

fn emit_actions_inner(
    node: &Node,
    field: &str,
    is_inheriting: bool,
) -> Result<String, crate::Erro> {
    let change = match (is_inheriting, layout::handler(node, "on_change")) {
        (true, Some(h)) => format!(
            "            if self.form.{field}.value != before {{\n\
             \x20               self.form.{field}_ext.changed(self.form.{field}.value);\n\
             \x20               self.form.{}();\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        (true, None) => format!(
            "            if self.form.{field}.value != before {{\n\
             \x20               self.form.{field}_ext.changed(self.form.{field}.value);\n\
             \x20           }}\n"
        ),
        (false, Some(h)) => format!(
            "            if self.form.{field}.value != before {{\n\
             \x20               self.form.{}();\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        (false, None) => String::new(),
    };
    let before = if change.is_empty() {
        String::new()
    } else {
        format!("            let before = self.form.{field}.value;\n")
    };
    let parse_cond = if is_inheriting {
        format!("let Some(v) = self.form.{field}_ext.parse(&typed)")
    } else {
        "let Ok(v) = typed.trim().parse::<i64>()".to_string()
    };
    // a step up, a step down, or a number typed in; then the range
    Ok(format!(
        "        {{\n\
         \x20           let step = self.form.{field}.increment.max(1);\n\
         \x20           let mut moved = false;\n\
         {before}\
         \x20           if self.ui.button(id!({field}_up)).clicked(actions) {{\n\
         \x20               self.form.{field}.value = self.form.{field}.value.saturating_add(step);\n\
         \x20               moved = true;\n\
         \x20           }}\n\
         \x20           if self.ui.button(id!({field}_down)).clicked(actions) {{\n\
         \x20               self.form.{field}.value = self.form.{field}.value.saturating_sub(step);\n\
         \x20               moved = true;\n\
         \x20           }}\n\
         \x20           if let Some(typed) = self.ui.text_input(id!({field}_edit)).changed(actions)\n\
         \x20               && {parse_cond}\n\
         \x20           {{\n\
         \x20               self.form.{field}.value = v;\n\
         \x20               moved = true;\n\
         \x20           }}\n\
         \x20           // both 0: no range, as in the classic RAD IDE\n\
         \x20           if moved && (self.form.{field}.min_value != 0 || self.form.{field}.max_value != 0) {{\n\
         \x20               self.form.{field}.value = self.form.{field}.value.clamp(self.form.{field}.min_value, self.form.{field}.max_value.max(self.form.{field}.min_value));\n\
         \x20           }}\n\
         {change}\
         \x20       }}\n"
    ))
}
