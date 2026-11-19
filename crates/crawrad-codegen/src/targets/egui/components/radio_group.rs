use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct RadioGroupCodegen;

impl EguiComponentCodegen for RadioGroupCodegen {
    fn class(&self) -> &'static str {
        "RadioGroup"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_RADIOGROUP)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-radio-group-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("RadioGroup").expect("RadioGroup descriptor");

    let items = match n.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let constant = format!(
        "const {}_ITEMS: &[&str] = &[\n{}];\n\n",
        name.to_uppercase(),
        items
            .iter()
            .map(|i| format!("    {},\n", literal(&i.text)))
            .collect::<String>()
    );
    let mut calls = String::new();
    if let Some(h) = handler(n, "on_click") {
        calls.push_str(&format!("                self.{}();\n", identificador(&h)?));
    }
    if let Some(h) = handler(n, "on_change") {
        calls.push_str(&format!(
            "                if changed {{\n                    self.{}();\n                }}\n",
            identificador(&h)?
        ));
    }
    let after = if calls.contains("if changed") {
        format!(
            "            if let Some(i) = picked {{\n\
             \x20               let changed = self.{name}.item_index != i;\n\
             \x20               self.{name}.item_index = i;\n\
             {calls}\
             \x20           }}\n"
        )
    } else {
        format!(
            "            if let Some(i) = picked {{\n\
             \x20               self.{name}.item_index = i;\n\
             {calls}\
             \x20           }}\n"
        )
    };
    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{name}"),
        crate::targets::shared::Selection::Signed,
    );
    let body = format!(
        "{forget}        if self.{name}.visible {{\n\
         \x20           let enabled = self.{name}.enabled;\n\
         \x20           group_box_frame(ui, r, self.{name}.text.as_str(), enabled);\n\
         \x20           let columns = self.{name}.columns.max(1) as usize;\n\
         \x20           let rows = self.{name}.items.len().div_ceil(columns).max(1);\n\
         \x20           let inner = egui::Rect::from_min_max(r.min + egui::vec2(8.0, 18.0), r.max - egui::vec2(6.0, 6.0));\n\
         \x20           let cell = egui::vec2(inner.width() / columns as f32, (inner.height() / rows as f32).min(22.0));\n\
         \x20           let mut picked = None;\n\
         \x20           ui.add_enabled_ui(enabled, |ui| {{\n\
         \x20               for (i, item) in self.{name}.items.iter().enumerate() {{\n\
         \x20                   let at = inner.min + egui::vec2((i / rows) as f32 * cell.x, (i % rows) as f32 * cell.y);\n\
         \x20                   let on = self.{name}.item_index == i as i64;\n\
         \x20                   if ui.put(egui::Rect::from_min_size(at, cell), egui::RadioButton::new(on, item.as_str())).clicked() {{\n\
         \x20                       picked = Some(i as i64);\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }});\n\
         {after}\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: RadioGroup,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: constant,
        corpo: body,
        tipo_usado: "RadioGroup",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_RADIOGROUP: StateText = StateText {
    head: r#"
/// A `RadioGroup` on the form: its options, the chosen one (`item_index`,
/// -1 for none) and how many columns they are laid out in.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
