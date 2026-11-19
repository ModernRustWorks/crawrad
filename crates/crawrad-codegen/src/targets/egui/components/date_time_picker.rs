//! `DateTimePicker` in the egui target: a single-line text box shaped by
//! the date or time mask (`crawrad_runtime::mask`), validating on completion and
//! supporting caret-relative field stepping on ArrowUp / ArrowDown.

use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct DateTimePickerCodegen;

impl EguiComponentCodegen for DateTimePickerCodegen {
    fn class(&self) -> &'static str {
        "DateTimePicker"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(STATE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx
        .reg
        .get("DateTimePicker")
        .expect("DateTimePicker descriptor");

    let change = match handler(n, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!(
                "            if self.{name}.date_time != before_val {{\n\
                 \x20               self.{h_name}();\n\
                 \x20           }}\n"
            )
        }
        None => "            let _ = before_val;\n".to_string(),
    };

    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let before_val = self.{name}.date_time.clone();\n\
         \x20           let before = self.{name}.shown.clone();\n\
         \x20           let is_enabled = self.{name}.enabled;\n\
         \x20           let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&self.{name}.kind);\n\
         \x20           let mask_str = kind.mask();\n\
         \x20           if let Some(t) = crawrad_runtime::date_time::box_text(kind, &self.{name}.shown, &self.{name}.date_time) {{\n\
         \x20               self.{name}.shown = t;\n\
         \x20           }}\n\
         \x20           let res = ui.add_enabled_ui(is_enabled, |ui| {{\n\
         \x20               ui.put(r, egui::TextEdit::singleline(&mut self.{name}.shown))\n\
         \x20           }}).inner;\n\
         \x20           if res.has_focus() {{\n\
         \x20               let (up, down) = ui.input(|i| {{\n\
         \x20                   (i.key_pressed(egui::Key::ArrowUp), i.key_pressed(egui::Key::ArrowDown))\n\
         \x20               }});\n\
         \x20               if up || down {{\n\
         \x20                   let cursor = text_cursor(ui.ctx(), res.id, &self.{name}.shown);\n\
         \x20                   if let Some(iso) = crawrad_runtime::date_time::shown_to_iso(kind, &self.{name}.shown)\n\
         \x20                       && let Some(stepped) = crawrad_runtime::date_time::step_iso(\n\
         \x20                           kind,\n\
         \x20                           &iso,\n\
         \x20                           cursor,\n\
         \x20                           up,\n\
         \x20                           &self.{name}.min_date,\n\
         \x20                           &self.{name}.max_date,\n\
         \x20                       )\n\
         \x20                   {{\n\
         \x20                       self.{name}.date_time = stepped;\n\
         \x20                       self.{name}.shown = crawrad_runtime::date_time::iso_to_shown(kind, &self.{name}.date_time);\n\
         \x20                       set_text_cursor(ui.ctx(), res.id, cursor);\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20           if res.changed() {{\n\
         \x20               hold_text(ui.ctx(), res.id, &mut self.{name}.shown, |t, at| crawrad_runtime::mask::EditMask::parse(mask_str).hold_edit(&before, t, at));\n\
         \x20               if let Some(iso) = crawrad_runtime::date_time::shown_to_iso(kind, &self.{name}.shown) {{\n\
         \x20                   self.{name}.date_time = crawrad_runtime::date_time::clamp_iso(\n\
         \x20                       &iso,\n\
         \x20                       &self.{name}.min_date,\n\
         \x20                       &self.{name}.max_date,\n\
         \x20                   );\n\
         \x20               }}\n\
         \x20           }}\n\
         {change}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: DateTimePicker,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "DateTimePicker",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const STATE: StateText = StateText {
    head: r#"
/// A `DateTimePicker` on the form (#21). Stores canonical ISO-8601 strings
/// in `date_time` (`YYYY-MM-DD` or `HH:MM:SS`), shaped by its mask at display time.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
