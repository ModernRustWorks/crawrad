use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct PaginationCodegen;

impl EguiComponentCodegen for PaginationCodegen {
    fn class(&self) -> &'static str {
        "Pagination"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_PAGINATION)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-pagination-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Pagination").expect("Pagination descriptor");

    let on_change = handler(n, "on_change");

    let change_call = match on_change {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };

    let decl_flags = if !change_call.is_empty() {
        format!("            let before_index = self.{name}.page_index;\n")
    } else {
        String::new()
    };

    let dispatch_change = if !change_call.is_empty() {
        format!(
            "            if self.{name}.page_index != before_index {{\n                {}\
             \x20           }}\n",
            change_call
        )
    } else {
        String::new()
    };

    let body = format!(
        "        if self.{name}.visible {{\n\
         {decl_flags}\
         \x20           let page_count = (self.{name}.page_count.max(1)) as usize;\n\
         \x20           let page_index = (self.{name}.page_index.max(0)) as usize;\n\
         \x20           let max_buttons = (self.{name}.max_buttons.max(1)) as usize;\n\
         \x20           let show_arrows = self.{name}.show_arrows;\n\
         \x20           let slots = pagination_slots(page_count, page_index, max_buttons, show_arrows);\n\
         \x20           let clamped_index = page_index.min(page_count.saturating_sub(1));\n\
         \x20           let font = egui::FontId::proportional(12.0);\n\
         \x20           let h = r.height();\n\
         \x20           let mut hovered = None;\n\
         \x20           let mut clicked_page: Option<usize> = None;\n\
         \x20           if self.{name}.enabled {{\n\
         \x20               let mut x = r.min.x;\n\
         \x20               for (slot_idx, slot) in slots.iter().enumerate() {{\n\
         \x20                   let label = match slot {{\n\
         \x20                       PageSlot::Prev => \"‹\".to_string(),\n\
         \x20                       PageSlot::Next => \"›\".to_string(),\n\
         \x20                       PageSlot::Gap => \"…\".to_string(),\n\
         \x20                       PageSlot::Page(p) => (p + 1).to_string(),\n\
         \x20                   }};\n\
         \x20                   let text_w = pagination_text_width(ui.painter(), &label, &font);\n\
         \x20                   let cell_w = h.max(text_w + 8.0);\n\
         \x20                   let cell_rect = egui::Rect::from_min_size(\n\
         \x20                       egui::pos2(x, r.min.y),\n\
         \x20                       egui::vec2(cell_w, h),\n\
         \x20                   );\n\
         \x20                   if cell_rect.min.x >= r.max.x {{\n\
         \x20                       break;\n\
         \x20                   }}\n\
         \x20                   let clickable = match slot {{\n\
         \x20                       PageSlot::Prev => clamped_index > 0,\n\
         \x20                       PageSlot::Next => clamped_index + 1 < page_count,\n\
         \x20                       PageSlot::Page(_) => true,\n\
         \x20                       PageSlot::Gap => false,\n\
         \x20                   }};\n\
         \x20                   if clickable {{\n\
         \x20                       let resp = ui.interact(cell_rect, ui.id().with((\"crawrad_pag\", \"{name}\", slot_idx)), egui::Sense::click());\n\
         \x20                       if resp.hovered() {{\n\
         \x20                           hovered = Some(slot_idx);\n\
         \x20                       }}\n\
         \x20                       if resp.clicked() {{\n\
         \x20                           match slot {{\n\
         \x20                               PageSlot::Prev => clicked_page = Some(clamped_index - 1),\n\
         \x20                               PageSlot::Next => clicked_page = Some(clamped_index + 1),\n\
         \x20                               PageSlot::Page(p) => clicked_page = Some(*p),\n\
         \x20                               PageSlot::Gap => {{}},\n\
         \x20                           }}\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20                   x += cell_w + PAGINATION_GAP;\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20           if let Some(p) = clicked_page {{\n\
         \x20               self.{name}.page_index = p as i64;\n\
         \x20           }}\n\
         \x20           let mut painter = ui.painter_at(r);\n\
         \x20           if !self.{name}.enabled {{\n\
         \x20               painter.multiply_opacity(0.5);\n\
         \x20           }}\n\
         \x20           paint_pagination(&painter, r, &slots, clamped_index, hovered, ui.visuals(), font);\n\
         {dispatch_change}\
         \x20       }}\n"
    );

    let padrao = format!(
        "            {name}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Pagination,\n"),
        padrao,
        constante: String::new(),
        corpo: body,
        tipo_usado: "Pagination",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_PAGINATION: StateText = StateText {
    head: r#"
/// A `Pagination` component on the form (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
