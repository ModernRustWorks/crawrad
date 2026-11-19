//! Makepad component implementation for `DateTimePicker`: a date or time
//! input box shaped by its mask (`crawrad_runtime_makepad::edit::hold_mask` over `crawrad_runtime::mask`).

use super::codegen::{AppState, EnterKey, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct DateTimePickerCodegen;

impl MakepadComponentCodegen for DateTimePickerCodegen {
    fn class(&self) -> &'static str {
        "DateTimePicker"
    }

    fn default_cursor(&self) -> &'static str {
        "text"
    }

    fn enter_key(&self) -> EnterKey {
        EnterKey::Swallowed
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("DateTimePicker").expect("DateTimePicker desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct DateTimePicker {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("DateTimePicker").expect("DateTimePicker desc");
        let mut inits = state::inits_of(node, desc);
        let shown = crawrad_ir::date_time::shown_of(node);
        inits.push((
            "shown".to_string(),
            format!("{}.to_string()", crate::literal(&shown)),
        ));
        let body = inits
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("DateTimePicker {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        {{\n\
             \x20           let ti = self.ui.text_input(id!({field}));\n\
             \x20           let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&self.form.{field}.kind);\n\
             \x20           if let Some(t) = crawrad_runtime::date_time::box_text(kind, &ti.text(), &self.form.{field}.date_time) {{\n\
             \x20               self.form.{field}.shown = t.clone();\n\
             \x20               ti.set_text(cx, &t);\n\
             \x20           }}\n\
             \x20           let read_only = !self.form.{field}.enabled;\n\
             \x20           if ti.is_read_only() != read_only {{\n\
             \x20               ti.set_is_read_only(cx, read_only);\n\
             \x20           }}\n\
             \x20       }}\n"
        )
    }

    fn app_state(&self, node: &Node, field: &str) -> Result<Option<AppState>, crate::Erro> {
        let on_change_call = match layout::handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("            self.form.{h_name}();\n")
            }
            None => String::new(),
        };
        let handle = format!(
            "        if crawrad_runtime_makepad::edit::step_date_time_input(\n\
             \x20           cx,\n\
             \x20           &self.ui.text_input(id!({field})),\n\
             \x20           event,\n\
             \x20           &self.form.{field}.kind,\n\
             \x20           &mut self.form.{field}.date_time,\n\
             \x20           &self.form.{field}.min_date,\n\
             \x20           &self.form.{field}.max_date,\n\
             \x20       ) {{\n\
             \x20           self.form.{field}.shown = crawrad_runtime::date_time::iso_to_shown(\n\
             \x20               crawrad_runtime::date_time::DateTimeKind::parse_kind(&self.form.{field}.kind),\n\
             \x20               &self.form.{field}.date_time,\n\
             \x20           );\n\
             {on_change_call}\
             \x20           self.sync_ui(cx);\n\
             \x20           return;\n\
             \x20       }}\n"
        );
        Ok(Some(AppState {
            handle,
            ..Default::default()
        }))
    }

    fn emit_actions(
        &self,
        _document: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let on_change_call = match layout::handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("            self.form.{h_name}();\n")
            }
            None => String::new(),
        };
        Ok(format!(
            "        if let Some(_val) = self.ui.text_input(id!({field})).changed(actions) {{\n\
             \x20           let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&self.form.{field}.kind);\n\
             \x20           let _val = crawrad_runtime_makepad::edit::hold_mask(cx, &self.ui.text_input(id!({field})), &self.form.{field}.shown, &_val, kind.mask()).unwrap_or(_val);\n\
             \x20           self.form.{field}.shown = _val.clone();\n\
             \x20           if let Some(iso) = crawrad_runtime::date_time::shown_to_iso(kind, &_val) {{\n\
             \x20               let clamped = crawrad_runtime::date_time::clamp_iso(&iso, &self.form.{field}.min_date, &self.form.{field}.max_date);\n\
             \x20               if self.form.{field}.date_time != clamped {{\n\
             \x20                   self.form.{field}.date_time = clamped;\n\
             {on_change_call}\
             \x20               }}\n\
             \x20           }}\n\
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
        let shown = crawrad_ir::date_time::shown_of(node);
        let top = (h - 14.0) * 0.5;
        Ok(format!(
            "            {name} = <TextInput> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               padding: {{ top: {top:.1}, bottom: 0.0, left: 6.0, right: 6.0 }}\n\
             \x20               text: \"{shown}\"\n\
             \x20           }}\n"
        ))
    }
}
