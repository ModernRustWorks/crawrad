use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct ChipsCodegen;

impl EguiComponentCodegen for ChipsCodegen {
    fn class(&self) -> &'static str {
        "Chips"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_CHIPS)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-chips-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Chips").expect("Chips descriptor");

    let items = match n.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let const_name = format!("{}_ITEMS", name.to_uppercase());
    let constante = format!(
        "const {const_name}: &[&str] = &[\n{}];\n\n",
        items
            .iter()
            .map(|it| format!("    {},\n", literal(&it.text)))
            .collect::<String>()
    );

    let on_change = handler(n, "on_change");
    let on_click = handler(n, "on_click");

    let change_call = match on_change {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let click_call = match on_click {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let decl_flags = if !change_call.is_empty() {
        format!("            let before_index = self.{name}.item_index;\n")
    } else {
        String::new()
    };

    let flag_clicked = if !click_call.is_empty() {
        "            let mut chip_clicked = false;\n"
    } else {
        ""
    };

    let set_clicked = if !click_call.is_empty() {
        "                                chip_clicked = true;\n"
    } else {
        ""
    };

    let dispatch_click = if !click_call.is_empty() {
        format!(
            "            if chip_clicked {{\n                {}\
             \x20           }}\n",
            click_call
        )
    } else {
        String::new()
    };

    let dispatch_change = if !change_call.is_empty() {
        format!(
            "            if self.{name}.item_index != before_index {{\n                {}\
             \x20           }}\n",
            change_call
        )
    } else {
        String::new()
    };

    let remove_block = format!(
        "                                if self.{name}.removable {{\n\
         \x20                                   let cross_rect = egui::Rect::from_min_max(\n\
         \x20                                       egui::pos2(chip_rect.max.x - CHIP_PAD_X * 2.0, chip_rect.min.y),\n\
         \x20                                       chip_rect.max,\n\
         \x20                                   );\n\
         \x20                                   let cross_resp = ui.interact(cross_rect, ui.id().with((\"crawrad_chip_rm\", \"{name}\", i)), egui::Sense::click());\n\
         \x20                                   if cross_resp.clicked() {{\n\
         \x20                                       self.{name}.items.remove(i);\n\
         \x20                                       if self.{name}.item_index == i as i64 {{\n\
         \x20                                           self.{name}.item_index = -1;\n\
         \x20                                       }} else if self.{name}.item_index > i as i64 {{\n\
         \x20                                           self.{name}.item_index -= 1;\n\
         \x20                                       }}\n\
         \x20                                       {}\
         \x20                                       break;\n\
         \x20                                   }}\n\
         \x20                               }}\n",
        change_call
    );

    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{name}"),
        crate::targets::shared::Selection::Signed,
    );
    let body = format!(
        "{forget}        if self.{name}.visible {{\n\
         {decl_flags}\
         {flag_clicked}\
         \x20           let widths: Vec<f32> = self.{name}.items.iter().map(|it| chip_width(it.as_str(), self.{name}.removable)).collect();\n\
         \x20           let positions = chips_layout(&widths, r.width(), CHIP_HEIGHT, CHIP_GAP);\n\
         \x20           let font = egui::FontId::proportional(12.0);\n\
         \x20           let mut painter = ui.painter_at(r);\n\
         \x20           if !self.{name}.enabled {{\n\
         \x20               painter.multiply_opacity(0.5);\n\
         \x20           }}\n\
         \x20           for (i, (&(x, y), text)) in positions.iter().zip(self.{name}.items.iter()).enumerate() {{\n\
         \x20               let w = widths[i];\n\
         \x20               let chip_rect = egui::Rect::from_min_size(egui::pos2(r.min.x + x, r.min.y + y), egui::vec2(w, CHIP_HEIGHT));\n\
         \x20               if chip_rect.max.y > r.max.y {{\n\
         \x20                   break;\n\
         \x20               }}\n\
         \x20               let is_sel = self.{name}.item_index >= 0 && self.{name}.item_index as usize == i;\n\
         \x20               paint_chip(&painter, chip_rect, text.as_str(), is_sel, self.{name}.removable, ui.visuals(), font.clone());\n\
         \x20               if self.{name}.enabled {{\n\
         \x20                   // the chip first, its \"×\" after: in egui the area\n\
         \x20                   // registered last is on top and takes the click\n\
         \x20                   let resp = ui.interact(chip_rect, ui.id().with((\"crawrad_chip\", \"{name}\", i)), egui::Sense::click());\n\
         {remove_block}\
         \x20                   if resp.clicked() {{\n\
         \x20                       self.{name}.item_index = i as i64;\n\
         {set_clicked}\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n\
         {dispatch_click}\
         {dispatch_change}\
         \x20       }}\n"
    );

    let padrao = format!(
        "            {name}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Chips,\n"),
        padrao,
        constante,
        corpo: body,
        tipo_usado: "Chips",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_CHIPS: StateText = StateText {
    head: r#"
/// A `Chips` component on the form (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
