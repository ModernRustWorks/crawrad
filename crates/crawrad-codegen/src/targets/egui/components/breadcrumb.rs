use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};
use crawrad_ir::Item;

pub struct BreadcrumbCodegen;

impl EguiComponentCodegen for BreadcrumbCodegen {
    fn class(&self) -> &'static str {
        "Breadcrumb"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_BREADCRUMB)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-breadcrumb-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Breadcrumb").expect("Breadcrumb descriptor");

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
        "            let mut clicked_idx: Option<usize> = None;\n"
    } else {
        ""
    };

    let set_clicked = if !click_call.is_empty() {
        "                                clicked_idx = Some(i);\n"
    } else {
        ""
    };

    let dispatch_click = if !click_call.is_empty() {
        format!(
            "            if clicked_idx.is_some() {{\n                {}\
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

    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{name}"),
        crate::targets::shared::Selection::Signed,
    );
    let body = format!(
        "{forget}        if self.{name}.visible {{\n\
{decl_flags}\
{flag_clicked}\
         \x20           let item_strs: Vec<&str> = self.{name}.items.iter().map(|it| it.as_str()).collect();\n\
         \x20           let font = egui::FontId::proportional(12.0);\n\
         \x20           let mut hovered = None;\n\
         \x20           if self.{name}.style == \"bsChevron\" {{\n\
         \x20               let widths: Vec<f32> = item_strs.iter().map(|it| breadcrumb_text_width(ui.painter(), it, &font)).collect();\n\
         \x20               let pads = ChevronPads::default();\n\
         \x20               let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, pads);\n\
         \x20               if self.{name}.enabled && !item_strs.is_empty() {{\n\
         \x20                   for (i, (&(x, w), _)) in positions.iter().zip(item_strs.iter()).enumerate() {{\n\
         \x20                       let seg_rect = egui::Rect::from_min_size(egui::pos2(r.min.x + x, r.min.y), egui::vec2(w, r.height()));\n\
         \x20                       if seg_rect.min.x >= r.max.x {{\n\
         \x20                           break;\n\
         \x20                       }}\n\
         \x20                       let resp = ui.interact(seg_rect, ui.id().with((\"crawrad_brc\", \"{name}\", i)), egui::Sense::click());\n\
         \x20                       if resp.hovered() {{\n\
         \x20                           hovered = Some(i);\n\
         \x20                       }}\n\
         \x20                       if resp.clicked() {{\n\
         \x20                           self.{name}.item_index = i as i64;\n\
{set_clicked}\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20               let mut painter = ui.painter_at(r);\n\
         \x20               if !self.{name}.enabled {{\n\
         \x20                   painter.multiply_opacity(0.5);\n\
         \x20               }}\n\
         \x20               let count = item_strs.len();\n\
         \x20               let active_idx = if self.{name}.item_index >= 0 && (self.{name}.item_index as usize) < count {{\n\
         \x20                   self.{name}.item_index as usize\n\
         \x20               }} else if count > 0 {{\n\
         \x20                   count - 1\n\
         \x20               }} else {{\n\
         \x20                   0\n\
         \x20               }};\n\
         \x20               let sc = self.{name}.segment_color;\n\
         \x20               let cc = self.{name}.current_color;\n\
         \x20               paint_breadcrumb_chevron(\n\
         \x20                   &painter,\n\
         \x20                   r,\n\
         \x20                   &item_strs,\n\
         \x20                   active_idx,\n\
         \x20                   hovered,\n\
         \x20                   egui::Color32::from_rgba_unmultiplied(sc.r, sc.g, sc.b, sc.a).into(),\n\
         \x20                   egui::Color32::from_rgba_unmultiplied(cc.r, cc.g, cc.b, cc.a).into(),\n\
         \x20                   font,\n\
         \x20               );\n\
         \x20           }} else {{\n\
         \x20               let widths: Vec<f32> = item_strs.iter().map(|it| breadcrumb_text_width(ui.painter(), it, &font)).collect();\n\
         \x20               let sep_w = breadcrumb_text_width(ui.painter(), &self.{name}.separator, &font);\n\
         \x20               let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);\n\
         \x20               if self.{name}.enabled && item_strs.len() > 1 {{\n\
         \x20                   for (i, (&x, &w)) in positions.iter().zip(widths.iter()).enumerate().take(item_strs.len() - 1) {{\n\
         \x20                       let seg_rect = egui::Rect::from_min_size(egui::pos2(r.min.x + x, r.min.y), egui::vec2(w, r.height()));\n\
         \x20                       if seg_rect.min.x >= r.max.x {{\n\
         \x20                           break;\n\
         \x20                       }}\n\
         \x20                       let resp = ui.interact(seg_rect, ui.id().with((\"crawrad_brc\", \"{name}\", i)), egui::Sense::click());\n\
         \x20                       if resp.hovered() {{\n\
         \x20                           hovered = Some(i);\n\
         \x20                       }}\n\
         \x20                       if resp.clicked() {{\n\
         \x20                           self.{name}.item_index = i as i64;\n\
{set_clicked}\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20               let mut painter = ui.painter_at(r);\n\
         \x20               if !self.{name}.enabled {{\n\
         \x20                   painter.multiply_opacity(0.5);\n\
         \x20               }}\n\
         \x20               paint_breadcrumb(&painter, r, &item_strs, &self.{name}.separator, hovered, ui.visuals(), font);\n\
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
        campo: format!("    pub {name}: Breadcrumb,\n"),
        padrao,
        constante,
        corpo: body,
        tipo_usado: "Breadcrumb",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_BREADCRUMB: StateText = StateText {
    head: r#"
/// A `Breadcrumb` component on the form (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
