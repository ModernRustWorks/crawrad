use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ListViewCodegen;

impl EguiComponentCodegen for ListViewCodegen {
    fn class(&self) -> &'static str {
        "ListView"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_LISTVIEW)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit_list_view(ctx)
    }

    fn rola(&self) -> bool {
        true
    }
}

pub struct ListColumnCodegen;

impl EguiComponentCodegen for ListColumnCodegen {
    fn class(&self) -> &'static str {
        "ListColumn"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_LISTCOLUMN)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit_list_column(ctx)
    }
}

pub fn emit_list_view(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ListView").expect("ListView descriptor");

    let col_ids: Vec<String> = n
        .children
        .iter()
        .filter_map(|cid| ctx.doc.node(cid))
        .filter(|cn| cn.class == "ListColumn")
        .map(|cn| identificador(cn.name()))
        .collect::<Result<_, _>>()?;

    let on_change = handler(n, "on_change");
    let on_click = handler(n, "on_click");
    let on_dbl_click = handler(n, "on_dbl_click");

    let change_call = match on_change {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let click_call = match on_click {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let dbl_click_call = match on_dbl_click {
        Some(h) => format!("self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let select_item = match handler(n, "on_select_item") {
        Some(h) => Some(identificador(&h)?),
        None => None,
    };
    let column_click = match handler(n, "on_column_click") {
        Some(h) => Some(identificador(&h)?),
        None => None,
    };

    // Each flag exists only when its handler does: a flag nobody reads is a
    // warning in the generated project, which the clean gate refuses.
    let flag = |call: &str, decl: &str| {
        if call.is_empty() {
            String::new()
        } else {
            format!("            {decl}\n")
        }
    };
    let decl_flags = [
        flag(
            &change_call,
            &format!("let before_index = self.{name}.item_index;"),
        ),
        flag(&click_call, "let mut row_clicked = false;"),
        flag(&dbl_click_call, "let mut row_dbl_clicked = false;"),
        match select_item {
            Some(_) => format!("            let before_item = self.{name}.item_index;\n"),
            None => String::new(),
        },
        match column_click {
            Some(_) => "            let mut column_clicked: Option<usize> = None;\n".to_string(),
            None => String::new(),
        },
    ]
    .concat();
    // the header cell listens only when somebody wants the click
    let header_click = match column_click {
        Some(_) => format!(
            "                        if w.interact(cell, egui::Id::new((\"crawrad-lvw-col\", \"{name}\", idx)), egui::Sense::click()).clicked() {{\n\
             \x20                           column_clicked = Some(idx);\n\
             \x20                       }}\n"
        ),
        None => String::new(),
    };
    // `OnSelectItem`: the row that leaves, then the row that arrives
    let dispatch_select_item = match &select_item {
        Some(h) => format!(
            "            if self.{name}.item_index != before_item {{\n\
             \x20               if before_item >= 0 {{\n\
             \x20                   self.{h}(before_item as usize, false);\n\
             \x20               }}\n\
             \x20               if self.{name}.item_index >= 0 {{\n\
             \x20                   self.{h}(self.{name}.item_index as usize, true);\n\
             \x20               }}\n\
             \x20           }}\n"
        ),
        None => String::new(),
    };
    let dispatch_column_click = match &column_click {
        Some(h) => format!(
            "            if let Some(column) = column_clicked {{\n\
             \x20               self.{h}(column);\n\
             \x20           }}\n"
        ),
        None => String::new(),
    };
    let set = |call: &str, name: &str| {
        if call.is_empty() {
            String::new()
        } else {
            format!("                                {name} = true;\n")
        }
    };
    let set_clicked = set(&click_call, "row_clicked");
    let set_dbl_clicked = set(&dbl_click_call, "row_dbl_clicked");

    let dispatch_change = if !change_call.is_empty() {
        format!(
            "            if self.{name}.item_index != before_index {{\n                {}\
             \x20           }}\n",
            change_call
        )
    } else {
        String::new()
    };

    let dispatch_click = if !click_call.is_empty() {
        format!(
            "            if row_clicked {{\n                {}\
             \x20           }}\n",
            click_call
        )
    } else {
        String::new()
    };

    let dispatch_dbl_click = if !dbl_click_call.is_empty() {
        format!(
            "            if row_dbl_clicked {{\n                {}\
             \x20           }}\n",
            dbl_click_call
        )
    } else {
        String::new()
    };

    let mut cols_builder = String::new();
    for cid in &col_ids {
        cols_builder.push_str(&format!(
            "                    (self.{cid}.text.as_str(), self.{cid}.width as f32, self.{cid}.alignment.as_str()),\n"
        ));
    }

    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.{name}"),
        crate::targets::shared::Selection::Signed,
    );
    let body = format!(
        "{forget}        if self.{name}.visible {{\n\
         {decl_flags}\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{name}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           let columns: &[(&str, f32, &str)] = &[\n{cols_builder}\x20           ];\n\
         \x20           let font = egui::FontId::proportional(14.0);\n\
         \x20           let header_h = LIST_VIEW_HEADER_H;\n\
         \x20           let row_h = LIST_VIEW_ROW_H;\n\
         \x20           w.painter().rect(r, 0.0, w.visuals().extreme_bg_color, w.visuals().widgets.noninteractive.bg_stroke, egui::StrokeKind::Inside);\n\
         \x20           let mut top = r.min.y;\n\
         \x20           if self.{name}.show_column_headers && !columns.is_empty() {{\n\
         \x20               let mut x = r.min.x;\n\
         \x20               for (idx, &(caption, width, alignment)) in columns.iter().enumerate() {{\n\
         \x20                   let col_w = if idx + 1 == columns.len() {{ (r.max.x - x).max(width) }} else {{ width }};\n\
         \x20                   let cell = egui::Rect::from_min_size(egui::pos2(x, top), egui::vec2(col_w.min(r.max.x - x), header_h));\n\
         \x20                   if cell.width() > 0.0 {{\n\
         \x20                       paint_list_view_header_cell(w.painter(), cell, caption, alignment, w.visuals(), font.clone());\n\
         {header_click}\
         \x20                   }}\n\
         \x20                   x += width;\n\
         \x20                   if x >= r.max.x {{ break; }}\n\
         \x20               }}\n\
         \x20               top += header_h;\n\
         \x20           }}\n\
         \x20           let body_rect = egui::Rect::from_min_max(egui::pos2(r.min.x, top), r.max);\n\
         \x20           let pad = 6.0;\n\
         \x20           let num_cols = columns.len().max(1);\n\
         \x20           let total_items = self.{name}.items.len();\n\
         \x20           let mut rows_ui = w.new_child(egui::UiBuilder::new().max_rect(body_rect));\n\
         \x20           egui::ScrollArea::vertical()\n\
         \x20               .id_salt((\"rxp-listview\", \"{name}\"))\n\
         \x20               .max_height(body_rect.height())\n\
         \x20               .max_width(body_rect.width())\n\
         \x20               .show_rows(&mut rows_ui, row_h, total_items, |ui, range| {{\n
         \x20                   for row_idx in range {{\n\
         \x20                       let is_sel = self.{name}.item_index >= 0 && self.{name}.item_index as usize == row_idx;\n\
         \x20                       let (row_rect, resp) = ui.allocate_exact_size(egui::vec2(ui.available_width(), row_h), egui::Sense::click());\n\
         \x20                       if resp.clicked() {{\n\
         \x20                           self.{name}.item_index = row_idx as i64;\n\
         {set_clicked}\
         \x20                       }}\n\
         \x20                       if resp.double_clicked() {{\n\
         \x20                           self.{name}.item_index = row_idx as i64;\n\
         {set_dbl_clicked}\
         \x20                       }}\n\
         \x20                       if is_sel && self.{name}.row_select {{\n\
         \x20                           ui.painter().rect_filled(row_rect, 0.0, ui.visuals().selection.bg_fill);\n\
         \x20                       }}\n\
         \x20                       let item = &self.{name}.items[row_idx];\n\
         \x20                       let mut x = row_rect.min.x;\n\
         \x20                       for col_idx in 0..num_cols {{\n\
         \x20                           let width = if col_idx < columns.len() {{ columns[col_idx].1 }} else {{ 80.0 }};\n\
         \x20                           let col_w = if col_idx + 1 == num_cols {{ (row_rect.max.x - x).max(width) }} else {{ width }};\n\
         \x20                           let cell_rect = egui::Rect::from_min_size(egui::pos2(x, row_rect.min.y), egui::vec2(col_w.min(row_rect.max.x - x), row_h));\n\
         \x20                           if is_sel && !self.{name}.row_select && col_idx == 0 {{\n\
         \x20                               ui.painter().rect_filled(cell_rect, 0.0, ui.visuals().selection.bg_fill);\n\
         \x20                           }}\n\
         \x20                           let cell_text = if col_idx == 0 {{\n\
         \x20                               item.text.as_str()\n\
         \x20                           }} else if col_idx - 1 < item.sub_items.len() {{\n\
         \x20                               item.sub_items[col_idx - 1].as_str()\n\
         \x20                           }} else {{\n\
         \x20                               \"\"\n\
         \x20                           }};\n\
         \x20                           let align_str = if col_idx < columns.len() {{ columns[col_idx].2 }} else {{ \"taLeftJustify\" }};\n\
         \x20                           let text_color = if is_sel && (self.{name}.row_select || col_idx == 0) {{\n\
         \x20                               ui.visuals().selection.stroke.color\n\
         \x20                           }} else {{\n\
         \x20                               ui.visuals().text_color()\n\
         \x20                           }};\n\
         \x20                           let (pos, anchor) = match align_str {{\n\
         \x20                               \"taRightJustify\" => (cell_rect.right_center() + egui::vec2(-pad, 0.0), egui::Align2::RIGHT_CENTER),\n\
         \x20                               \"taCenter\" => (cell_rect.center(), egui::Align2::CENTER_CENTER),\n\
         \x20                               _ => (cell_rect.left_center() + egui::vec2(pad, 0.0), egui::Align2::LEFT_CENTER),\n\
         \x20                           }};\n\
         \x20                           let cell_painter = ui.painter().with_clip_rect(cell_rect.intersect(row_rect));\n\
         \x20                           cell_painter.text(pos, anchor, cell_text, font.clone(), text_color);\n\
         \x20                           if self.{name}.grid_lines {{\n\
         \x20                               ui.painter().line_segment([egui::pos2(cell_rect.max.x, cell_rect.min.y), egui::pos2(cell_rect.max.x, cell_rect.max.y)], ui.visuals().widgets.noninteractive.bg_stroke);\n\
         \x20                           }}\n\
         \x20                           x += col_w;\n\
         \x20                           if x >= row_rect.max.x {{ break; }}\n\
         \x20                       }}\n\
         \x20                       if self.{name}.grid_lines {{\n\
         \x20                           ui.painter().line_segment([egui::pos2(row_rect.min.x, row_rect.max.y), egui::pos2(row_rect.max.x, row_rect.max.y)], ui.visuals().widgets.noninteractive.bg_stroke);\n\
         \x20                       }}\n\
         \x20                   }}\n\
         \x20               }});\n\
         {dispatch_select_item}\
         {dispatch_change}\
         {dispatch_click}\
         {dispatch_column_click}\
         {dispatch_dbl_click}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: ListView,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "ListView",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

pub fn emit_list_column(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ListColumn").expect("ListColumn descriptor");

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: ListColumn,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: String::new(),
        tipo_usado: "ListColumn",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_LISTVIEW: StateText = StateText {
    head: r#"
/// One row of a `ListView` (#500).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListViewItem {
    pub text: String,
    pub sub_items: Vec<String>,
}

/// A `ListView` on the form: report-style list with columns and virtualized items (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};

const ESTADO_LISTCOLUMN: StateText = StateText {
    head: r#"
/// One column of a `ListView` (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
