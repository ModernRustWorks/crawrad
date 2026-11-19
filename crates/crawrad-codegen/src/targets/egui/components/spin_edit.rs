use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct SpinEditCodegen;

impl EguiComponentCodegen for SpinEditCodegen {
    fn class(&self) -> &'static str {
        "SpinEdit"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SPINEDIT)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-spin-edit-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("SpinEdit").expect("SpinEdit descriptor");
    let is_inheriting = ctx.reg.get(&n.class).and_then(|d| d.inherits).is_some();
    let change = match (is_inheriting, handler(n, "on_change")) {
        (true, Some(h)) => format!(
            "            if self.{name}.value != before {{\n\
             \x20               self.{name}_ext.changed(self.{name}.value);\n\
             \x20               self.{}();\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        (true, None) => format!(
            "            if self.{name}.value != before {{\n\
             \x20               self.{name}_ext.changed(self.{name}.value);\n\
             \x20           }}\n"
        ),
        (false, Some(h)) => format!(
            "            if self.{name}.value != before {{\n\
             \x20               self.{}();\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        (false, None) => "            let _ = before;\n".to_string(),
    };
    let editor_block = if is_inheriting {
        format!(
            "                if self.{name}.editor_enabled {{\n\
             \x20                   // the typed text lives while focused and is read back on leave\n\
             \x20                   let id = egui::Id::new(\"{name}_text\");\n\
             \x20                   let mut text = ui.data_mut(|d| d.get_temp::<String>(id)).unwrap_or_else(|| self.{name}_ext.display(self.{name}.value));\n\
             \x20                   let resp = ui.put(field, egui::TextEdit::singleline(&mut text).id(id));\n\
             \x20                   if resp.has_focus() {{\n\
             \x20                       ui.data_mut(|d| d.insert_temp(id, text.clone()));\n\
             \x20                   }} else {{\n\
             \x20                       ui.data_mut(|d| d.remove::<String>(id));\n\
             \x20                   }}\n\
             \x20                   if resp.lost_focus() && let Some(v) = self.{name}_ext.parse(&text) {{\n\
             \x20                       self.{name}.value = v;\n\
             \x20                   }}\n\
             \x20               }} else {{\n\
             \x20                   ui.painter().rect_stroke(field, 2.0, ui.visuals().widgets.inactive.bg_stroke, egui::StrokeKind::Inside);\n\
             \x20                   ui.put(field, egui::Label::new(self.{name}_ext.display(self.{name}.value)));\n\
             \x20               }}\n"
        )
    } else {
        format!(
            "                if self.{name}.editor_enabled {{\n\
             \x20                   ui.put(field, egui::DragValue::new(&mut self.{name}.value).speed(0.0).update_while_editing(false));\n\
             \x20               }} else {{\n\
             \x20                   ui.painter().rect_stroke(field, 2.0, ui.visuals().widgets.inactive.bg_stroke, egui::StrokeKind::Inside);\n\
             \x20                   ui.put(field, egui::Label::new(self.{name}.value.to_string()));\n\
             \x20               }}\n"
        )
    };
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let before = self.{name}.value;\n\
         \x20           let step = self.{name}.increment.max(1);\n\
         \x20           // both 0: no range, as in the classic RAD IDE\n\
         \x20           let bounded = self.{name}.min_value != 0 || self.{name}.max_value != 0;\n\
         \x20           let field = egui::Rect::from_min_max(r.min, egui::pos2(r.max.x - 16.0, r.max.y));\n\
         \x20           let up = egui::Rect::from_min_max(egui::pos2(r.max.x - 16.0, r.min.y), egui::pos2(r.max.x, r.center().y));\n\
         \x20           let down = egui::Rect::from_min_max(egui::pos2(r.max.x - 16.0, r.center().y), r.max);\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               // EditorEnabled off: the number is shown, not typed\n\
         {editor_block}\
         \x20               if ui.put(up, egui::Button::new(\"+\").small()).clicked() {{\n\
         \x20                   self.{name}.value = self.{name}.value.saturating_add(step);\n\
         \x20               }}\n\
         \x20               if ui.put(down, egui::Button::new(\"-\").small()).clicked() {{\n\
         \x20                   self.{name}.value = self.{name}.value.saturating_sub(step);\n\
         \x20               }}\n\
         \x20           }});\n\
         \x20           if bounded {{\n\
         \x20               self.{name}.value = self.{name}.value.clamp(self.{name}.min_value, self.{name}.max_value.max(self.{name}.min_value));\n\
         \x20           }}\n\
         {change}\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: SpinEdit,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "SpinEdit",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_SPINEDIT: StateText = StateText {
    head: r#"
/// A `SpinEdit` on the form: a whole number, its range (`min_value` and
/// `max_value` both 0: none), the buttons' step, and whether it is typed.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
