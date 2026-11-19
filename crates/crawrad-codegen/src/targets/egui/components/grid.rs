use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_note::note;

pub struct GridCodegen;

impl EguiComponentCodegen for GridCodegen {
    fn class(&self) -> &'static str {
        "Grid"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_GRID)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }

    fn rola(&self) -> bool {
        true
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Grid").expect("Grid descriptor");

    let padrao = format!(
        "            {nome}: {},\n",
        crate::targets::egui::state::init_of(n, desc, ctx.doc)
    );
    if let Some(engine) = crate::targets::compartilhado::bound_grid_engine(ctx.doc, n)? {
        crate::targets::compartilhado::check_style_rules(ctx.doc, n)?;
        return Ok(bound(ctx.doc, n, nome, &engine, padrao));
    }

    // The selection before the grid is drawn is kept only when a handler
    // asks for the change; without one it would be an unused variable.
    let (antes, mut depois) = match handler(n, "on_select_cell") {
        Some(h) => {
            let h_name = identificador(&h)?;
            (
                format!("            let before_selected = self.{nome}.selected;\n"),
                format!(
                    "            if self.{nome}.selected != before_selected\n\
                     \x20               && let Some((col, row)) = self.{nome}.selected\n\
                     \x20           {{\n\
                     \x20               self.{h_name}(col, row);\n\
                     \x20           }}\n"
                ),
            )
        }
        None => (String::new(), String::new()),
    };
    // OnClick on any click, OnDblClick on the second of a double click
    for (event, flag) in [("on_click", "clicked"), ("on_dbl_click", "double")] {
        depois.push_str(&match handler(n, event) {
            Some(h) => format!(
                "            if {flag} {{\n                self.{}();\n            }}\n",
                identificador(&h)?
            ),
            None => format!("            let _ = {flag};\n"),
        });
    }

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         {antes}\
         \x20           let layout = crawrad_runtime::string_grid::GridLayout::from_props(\n\
         \x20               self.{nome}.col_count as i64,\n\
         \x20               self.{nome}.row_count as i64,\n\
         \x20               self.{nome}.fixed_cols as i64,\n\
         \x20               self.{nome}.fixed_rows as i64,\n\
         \x20               self.{nome}.default_col_width as i64,\n\
         \x20               self.{nome}.default_row_height as i64,\n\
         \x20           );\n\
         \x20           let _ = &self.{nome}.table_name;\n\
         \x20           let _ = &self.{nome}.group_bar;\n\
         \x20           let _ = &self.{nome}.cell_text;\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{nome}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           let mut clicked = false;\n\
         \x20           let mut double = false;\n\
         \x20           egui::Frame::group(w.style())\n\
         \x20               .fill(w.visuals().extreme_bg_color)\n\
         \x20               .stroke(w.visuals().widgets.inactive.bg_stroke)\n\
         \x20               .corner_radius(egui::CornerRadius::same(2))\n\
         \x20               .inner_margin(egui::Margin::ZERO)\n\
         \x20               .show(&mut w, |ui| {{\n\
         \x20                   egui::ScrollArea::both()\n\
         \x20                       .id_salt((\"rxp-grid\", \"{nome}\"))\n\
         \x20                       .max_height(r.height())\n\
         \x20                       .max_width(r.width())\n\
         \x20                       .auto_shrink([false, false])\n\
         \x20                       .show(ui, |ui| {{\n\
         \x20                           egui::Grid::new(\"grid_{nome}\")\n\
         \x20                               .striped(self.{nome}.grid_lines)\n\
         \x20                               .min_col_width(layout.col_width as f32)\n\
         \x20                               .min_row_height(layout.row_height as f32)\n\
         \x20                               .show(ui, |ui| {{\n\
         \x20                                   for row in 0..layout.row_count {{\n\
         \x20                                       for col in 0..layout.col_count {{\n\
         \x20                                           if layout.is_fixed(col, row) {{\n\
         \x20                                               let response = ui.add(egui::Label::new(egui::RichText::new(self.{nome}.cells.get(col, row)).strong()).sense(egui::Sense::click()));\n\
         \x20                                               clicked |= response.clicked();\n\
         \x20                                               double |= response.double_clicked();\n\
         \x20                                           }} else {{\n\
         \x20                                               let response = if self.{nome}.editing\n\
         \x20                                                   && self.{nome}.selected == Some((col, row))\n\
         \x20                                                   && let Some(mut buffer) = self.{nome}.editor.take()\n\
         \x20                                               {{\n\
         \x20                                                   let response = ui.add(egui::TextEdit::singleline(&mut buffer));\n\
         \x20                                                   if !response.has_focus() && !response.lost_focus() {{\n\
         \x20                                                       response.request_focus();\n\
         \x20                                                   }}\n\
         \x20                                                   if response.lost_focus() {{\n\
         \x20                                                       if !ui.input(|i| i.key_pressed(egui::Key::Escape)) {{\n\
         \x20                                                           self.{nome}.cells.set(col, row, buffer);\n\
         \x20                                                       }}\n\
         \x20                                                   }} else {{\n\
         \x20                                                       self.{nome}.editor = Some(buffer);\n\
         \x20                                                   }}\n\
         \x20                                                   response\n\
         \x20                                               }} else {{\n\
         \x20                                                   let selected = self.{nome}.selected == Some((col, row));\n\
         \x20                                                   let response = ui.selectable_label(selected, self.{nome}.cells.get(col, row));\n\
         \x20                                                   if response.clicked() {{\n\
         \x20                                                       self.{nome}.selected = Some((col, row));\n\
         \x20                                                   }}\n\
         \x20                                                   if self.{nome}.editing && response.double_clicked() {{\n\
         \x20                                                       self.{nome}.editor = Some(self.{nome}.cells.get(col, row).to_string());\n\
         \x20                                                   }}\n\
         \x20                                                   response\n\
         \x20                                               }};\n\
         \x20                                               clicked |= response.clicked();\n\
         \x20                                               double |= response.double_clicked();\n\
         \x20                                           }}\n\
         \x20                                       }}\n\
         \x20                                       ui.end_row();\n\
         \x20                                   }}\n\
         \x20                               }});\n\
         \x20                       }});\n\
         \x20               }});\n\
         {depois}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Grid,\n"),
        padrao,
        constante: String::new(),
        corpo,
        tipo_usado: "Grid",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

#[note("targets-egui-components-grid-bound")]
fn bound(
    doc: &crawrad_ir::Document,
    node: &crawrad_ir::Node,
    nome: &str,
    engine: &str,
    padrao: String,
) -> ResultadoComponente {
    let specs = crate::targets::compartilhado::column_specs_literal(doc, node);
    let lookups_code =
        crate::targets::compartilhado::grid_lookups_code(doc, node, "self.", "                ")
            // A name the generator cannot write was refused by the designer pass, which runs first.
            .unwrap_or_default();
    let (opener, target) =
        crate::targets::compartilhado::bound_grid_target(doc, node, "self.", nome);
    let open = crate::targets::compartilhado::bound_grid_open(
        opener,
        &format!("&self.{engine}.url"),
        &target,
        "                ",
        &specs,
        &lookups_code,
    );
    let mut hooks_code = String::new();
    let handlers = crate::targets::compartilhado::column_handlers(doc, node);
    if !handlers.is_empty() {
        let has_get = handlers.iter().any(|h| h.on_get_text.is_some());
        let has_draw = handlers.iter().any(|h| h.on_draw_cell.is_some());
        let has_check = handlers
            .iter()
            .any(|h| h.on_set_text.is_some() || h.on_validate.is_some());
        let has_button = handlers.iter().any(|h| h.on_button_click.is_some());
        // Both are taken from `rows` BEFORE any handler runs: a handler borrows
        // `self` mutably, so the borrow of `self.{nome}_rows` must be over by then.
        if has_get {
            hooks_code.push_str("            let mut reqs = rows.text_requests();\n");
        }
        if has_draw {
            hooks_code.push_str("            let mut style_reqs = rows.style_requests();\n");
        }
        if has_check {
            hooks_code.push_str("            let pending = rows.pending_check();\n");
        }
        if has_button {
            hooks_code.push_str("            let pending_btn = rows.pending_button();\n");
        }
        if has_get {
            let mut get_arms = String::new();
            for h in &handlers {
                if let Some(on_get) = &h.on_get_text {
                    let h_ident = identificador(on_get).unwrap_or_else(|_| on_get.clone());
                    let field_name = &h.field;
                    get_arms.push_str(&format!(
                        "                    \"{field_name}\" => self.{h_ident}(&req.row, &mut req.text),\n"
                    ));
                }
            }
            hooks_code.push_str(&format!(
                "            if !reqs.is_empty() {{\n\
                 \x20               for req in &mut reqs {{\n\
                 \x20                   match req.field.as_str() {{\n\
                 {get_arms}\
                 \x20                       _ => {{}}\n\
                 \x20                   }}\n\
                 \x20               }}\n\
                 \x20               if let Some(rows) = self.{nome}_rows.as_mut() {{\n\
                 \x20                   rows.set_texts(reqs);\n\
                 \x20               }}\n\
                 \x20           }}\n"
            ));
        }
        if has_draw {
            let mut draw_arms = String::new();
            for h in &handlers {
                if let Some(on_draw) = &h.on_draw_cell {
                    let h_ident = identificador(on_draw).unwrap_or_else(|_| on_draw.clone());
                    let field_name = &h.field;
                    draw_arms.push_str(&format!(
                        "                    \"{field_name}\" => self.{h_ident}(&req.row, &mut req.style),\n"
                    ));
                }
            }
            hooks_code.push_str(&format!(
                "            if !style_reqs.is_empty() {{\n\
                 \x20               for req in &mut style_reqs {{\n\
                 \x20                   match req.field.as_str() {{\n\
                 {draw_arms}\
                 \x20                       _ => {{}}\n\
                 \x20                   }}\n\
                 \x20               }}\n\
                 \x20               if let Some(rows) = self.{nome}_rows.as_mut() {{\n\
                 \x20                   rows.set_styles(style_reqs);\n\
                 \x20               }}\n\
                 \x20           }}\n"
            ));
        }
        if has_check {
            let mut check_arms = String::new();
            for h in &handlers {
                if h.on_set_text.is_some() || h.on_validate.is_some() {
                    let field_name = &h.field;
                    let mut body = String::new();
                    if let Some(on_set) = &h.on_set_text {
                        let set_ident = identificador(on_set).unwrap_or_else(|_| on_set.clone());
                        body.push_str(&format!(
                            "                        self.{set_ident}(&check.row, &mut cell.text);\n"
                        ));
                    }
                    if let Some(on_val) = &h.on_validate {
                        let val_ident = identificador(on_val).unwrap_or_else(|_| on_val.clone());
                        body.push_str(&format!(
                            "                        self.{val_ident}(&check.row, &cell.text, &mut cell.error);\n"
                        ));
                    }
                    check_arms.push_str(&format!(
                        "                    \"{field_name}\" => {{\n{body}                    }}\n"
                    ));
                }
            }
            hooks_code.push_str(&format!(
                "            if let Some(mut check) = pending {{\n\
                 \x20               for cell in &mut check.cells {{\n\
                 \x20                   match cell.field.as_str() {{\n\
                 {check_arms}\
                 \x20                       _ => {{}}\n\
                 \x20                   }}\n\
                 \x20               }}\n\
                 \x20               if let Some(rows) = self.{nome}_rows.as_mut() {{\n\
                 \x20                   rows.resolve_check(check);\n\
                 \x20               }}\n\
                 \x20           }}\n"
            ));
        }
        if has_button {
            let mut btn_arms = String::new();
            for h in &handlers {
                if let Some(on_btn) = &h.on_button_click {
                    let btn_ident = identificador(on_btn).unwrap_or_else(|_| on_btn.clone());
                    let field_name = &h.field;
                    btn_arms.push_str(&format!(
                        "                    \"{field_name}\" => self.{btn_ident}(&req.row, &mut req.text),\n"
                    ));
                }
            }
            hooks_code.push_str(&format!(
                "            if let Some(mut req) = pending_btn {{\n\
                 \x20               match req.field.as_str() {{\n\
                 {btn_arms}\
                 \x20                   _ => {{}}\n\
                 \x20               }}\n\
                 \x20               if let Some(rows) = self.{nome}_rows.as_mut() {{\n\
                 \x20                   rows.button_done(req);\n\
                 \x20               }}\n\
                 \x20           }}\n"
            ));
        }
    }
    let body = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let rows = self.{nome}_rows.get_or_insert_with(|| {{\n\
         \x20               let (source, columns) = {open};\n\
         \x20               let mut grid = crawrad_runtime_egui::data_grid::DataGrid::new(source, columns);\n\
         \x20               grid.set_row_height(self.{nome}.default_row_height);\n\
         \x20               grid.set_group_bar(self.{nome}.group_bar);\n\
         \x20               grid\n\
         \x20           }});\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{nome}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           rows.show(&mut w);\n\
         {hooks_code}\
         \x20       }}\n"
    );
    ResultadoComponente {
        campo: format!(
            "    pub {nome}: Grid,\n    pub {nome}_rows: Option<crawrad_runtime_egui::data_grid::DataGrid>,\n"
        ),
        padrao: format!("{padrao}            {nome}_rows: None,\n"),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Grid",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    }
}

const ESTADO_GRID: StateText = StateText {
    head: r#"
/// A `Grid` on the form — classic RAD `TStringGrid` / `TDrawGrid` (ADR 0041).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
