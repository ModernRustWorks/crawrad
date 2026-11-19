//! Makepad component implementation for `Grid`.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util;
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;

pub struct GridCodegen;

impl MakepadComponentCodegen for GridCodegen {
    fn class(&self) -> &'static str {
        "Grid"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        emit_sync(doc, node, field)
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(doc, node, field, emitted)
    }

    fn app_state(&self, node: &Node, field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(bound_app_state(node, field).unwrap_or_else(|| {
            AppState {
                uses: "use crawrad_runtime_makepad::string_grid::CrawradStringGridWidgetRefExt;\n"
                    .to_string(),
                ..AppState::default()
            }
        })))
    }

    fn emit_dsl(
        &self,
        doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        if crate::targets::compartilhado::bound_grid_engine(doc, node)?.is_some() {
            crate::targets::compartilhado::check_style_rules(doc, node)?;
            return emit_bound_dsl(node, x, y, w, h);
        }
        emit_dsl(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::grid();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    for (class, nome, ty) in crate::targets::makepad::state::EXTRA_FIELDS {
        if *class == "Grid" {
            fields.push(util::format_field(nome, ty));
        }
    }
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Grid {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);

    let desc = crawrad_registry::components::grid();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    inits.push(format!("cells: {}", crate::grid_cells_init(node)));
    // The target-only fields (`state::EXTRA_FIELDS`) start empty.
    inits.push("..Default::default()".to_string());
    format!("Grid {{\n{}\n}}", inits.join(",\n    "))
}

pub fn emit_sync(doc: &Document, node: &Node, field: &str) -> String {
    // A reference the generator cannot name was already refused by the DSL
    // pass, which runs first; here it only means "not bound".
    if let Ok(Some(engine)) = crate::targets::compartilhado::bound_grid_engine(doc, node) {
        return emit_bound_sync(doc, node, field, &engine);
    }
    format!(
        "        let _ = self.form.{field}.visible;\n\
         \x20       let _ = self.form.{field}.enabled;\n\
         \x20       let _ = &self.form.{field}.hint;\n\
         \x20       self.ui.crawrad_string_grid(id!({field})).set_grid(cx, crawrad_runtime::string_grid::GridLayout::from_props(self.form.{field}.col_count, self.form.{field}.row_count, self.form.{field}.fixed_cols, self.form.{field}.fixed_rows, self.form.{field}.default_col_width, self.form.{field}.default_row_height), &self.form.{field}.cells, self.form.{field}.grid_lines, self.form.{field}.editing);\n\
         \x20       let _ = &self.form.{field}.data_source;\n\
         \x20       let _ = &self.form.{field}.table_name;\n\
         \x20       let _ = &self.form.{field}.group_bar;\n\
         \x20       let _ = &self.form.{field}.cell_text;\n"
    )
}

#[crawrad_note::note("targets-makepad-components-grid-bound-app-state")]
fn bound_app_state(node: &Node, field: &str) -> Option<AppState> {
    match node.props.get("data_source") {
        Some(Value::Ref(id)) if !id.is_empty() => Some(AppState {
            // No `use`: the sync names the trait in full, so a grid whose
            // engine was deleted leaves no unused import behind.
            uses: String::new(),
            field: format!("    #[rust]\n    pub {field}_opened: bool,\n"),
            handle: String::new(),
        }),
        _ => None,
    }
}

#[note("targets-makepad-components-grid-emit-bound-sync")]
fn emit_bound_sync(doc: &Document, node: &Node, field: &str, engine: &str) -> String {
    let specs = crate::targets::compartilhado::column_specs_literal(doc, node);
    let lookups_code =
        crate::targets::compartilhado::grid_lookups_code(doc, node, "self.form.", "            ")
            // As in `emit_sync`: a name the generator cannot write was refused by the DSL pass.
            .unwrap_or_default();
    let (opener, target) =
        crate::targets::compartilhado::bound_grid_target(doc, node, "self.form.", field);
    let open = crate::targets::compartilhado::bound_grid_open(
        opener,
        &format!("&self.form.{engine}.url"),
        &target,
        "            ",
        &specs,
        &lookups_code,
    );
    format!(
        "        if !self.{field}_opened {{\n\
         \x20           self.{field}_opened = true;\n\
         \x20           let (source, columns) = {open};\n\
         \x20           crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).set_source(cx, source, columns);\n\
         \x20           crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).set_row_height(self.form.{field}.default_row_height as f64);\n\
         \x20           crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).set_group_bar(cx, self.form.{field}.group_bar);\n\
         \x20       }}\n"
    )
}

#[crawrad_note::note("targets-makepad-components-grid-emit-bound-dsl")]
fn emit_bound_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    Ok(format!(
        "        {name} = <CrawradDataGrid> {{\n\
         \x20           margin: {{left: {x}, top: {y}}}\n\
         \x20           width: {w}, height: {h}\n\
         \x20       }}\n"
    ))
}

/// Whether this grid has any column with a non-empty `title_hint` or `cell_hint = true`.
pub fn has_hint(doc: &Document, node: &Node) -> bool {
    crawrad_ir::grid_columns::ColumnSpec::of_grid(doc, node)
        .iter()
        .any(|c| !c.title_hint.is_empty() || c.cell_hint)
}

/// Whether any bound grid in the emitted nodes has a column hint.
pub fn any_bound_grid_has_hint(doc: &Document, emitted: &[&Node]) -> bool {
    emitted.iter().any(|n| {
        n.class == "Grid"
            && crate::targets::compartilhado::bound_grid_engine(doc, n)
                .ok()
                .flatten()
                .is_some()
            && has_hint(doc, n)
    })
}

pub fn emit_actions(
    doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    if crate::targets::compartilhado::bound_grid_engine(doc, node)?.is_some() {
        let mut code = String::new();
        if has_hint(doc, node) {
            code.push_str(&format!(
                "        if let Some((_col, hint, pos)) = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).header_hover(actions) {{\n\
                 \x20           self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, pos + makepad_widgets::dvec2(12.0, 20.0), &hint);\n\
                 \x20       }} else if let Some((_row, _col, text, pos)) = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).cell_hover(actions) {{\n\
                 \x20           self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, pos + makepad_widgets::dvec2(12.0, 20.0), &text);\n\
                 \x20       }} else if crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).hover_out(actions) {{\n\
                 \x20           self.ui.tooltip(id!(crawrad_tooltip)).hide(cx);\n\
                 \x20       }}\n"
            ));
        }
        let handlers = crate::targets::compartilhado::column_handlers(doc, node);
        if !handlers.is_empty() {
            let has_get = handlers.iter().any(|h| h.on_get_text.is_some());
            let has_draw = handlers.iter().any(|h| h.on_draw_cell.is_some());
            let has_check = handlers
                .iter()
                .any(|h| h.on_set_text.is_some() || h.on_validate.is_some());
            if has_get {
                let mut get_arms = String::new();
                for h in &handlers {
                    if let Some(on_get) = &h.on_get_text {
                        let h_ident = identificador(on_get)?;
                        let field_name = &h.field;
                        get_arms.push_str(&format!(
                            "                    \"{field_name}\" => self.form.{h_ident}(&req.row, &mut req.text),\n"
                        ));
                    }
                }
                code.push_str(&format!(
                    "        let mut reqs = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).text_requests();\n\
                     \x20       if !reqs.is_empty() {{\n\
                     \x20           for req in &mut reqs {{\n\
                     \x20               match req.field.as_str() {{\n\
                     {get_arms}\
                     \x20                   _ => {{}}\n\
                     \x20               }}\n\
                     \x20           }}\n\
                     \x20           crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).set_texts(cx, reqs);\n\
                     \x20       }}\n"
                ));
            }
            if has_draw {
                let mut draw_arms = String::new();
                for h in &handlers {
                    if let Some(on_draw) = &h.on_draw_cell {
                        let h_ident = identificador(on_draw)?;
                        let field_name = &h.field;
                        draw_arms.push_str(&format!(
                            "                    \"{field_name}\" => self.form.{h_ident}(&req.row, &mut req.style),\n"
                        ));
                    }
                }
                code.push_str(&format!(
                    "        let mut style_reqs = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).style_requests();\n\
                     \x20       if !style_reqs.is_empty() {{\n\
                     \x20           for req in &mut style_reqs {{\n\
                     \x20               match req.field.as_str() {{\n\
                     {draw_arms}\
                     \x20                   _ => {{}}\n\
                     \x20               }}\n\
                     \x20           }}\n\
                     \x20           crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).set_styles(cx, style_reqs);\n\
                     \x20       }}\n"
                ));
            }
            if has_check {
                let mut check_arms = String::new();
                for h in &handlers {
                    if h.on_set_text.is_some() || h.on_validate.is_some() {
                        let field_name = &h.field;
                        let mut body = String::new();
                        if let Some(on_set) = &h.on_set_text {
                            let set_ident = identificador(on_set)?;
                            body.push_str(&format!(
                                "                        self.form.{set_ident}(&check.row, &mut cell.text);\n"
                            ));
                        }
                        if let Some(on_val) = &h.on_validate {
                            let val_ident = identificador(on_val)?;
                            body.push_str(&format!(
                                "                        self.form.{val_ident}(&check.row, &cell.text, &mut cell.error);\n"
                            ));
                        }
                        check_arms.push_str(&format!(
                            "                    \"{field_name}\" => {{\n{body}                    }}\n"
                        ));
                    }
                }
                code.push_str(&format!(
                    "        if let Some(mut check) = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).pending_check() {{\n\
                     \x20           for cell in &mut check.cells {{\n\
                     \x20               match cell.field.as_str() {{\n\
                     {check_arms}\
                     \x20                   _ => {{}}\n\
                     \x20               }}\n\
                     \x20           }}\n\
                     \x20           crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).resolve_check(cx, check);\n\
                     \x20       }}\n"
                ));
            }
            let has_button = handlers.iter().any(|h| h.on_button_click.is_some());
            if has_button {
                let mut btn_arms = String::new();
                for h in &handlers {
                    if let Some(on_btn) = &h.on_button_click {
                        let btn_ident = identificador(on_btn)?;
                        let field_name = &h.field;
                        btn_arms.push_str(&format!(
                            "                    \"{field_name}\" => self.form.{btn_ident}(&req.row, &mut req.text),\n"
                        ));
                    }
                }
                code.push_str(&format!(
                    "        if let Some(mut req) = crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).pending_button() {{\n\
                     \x20           match req.field.as_str() {{\n\
                     {btn_arms}\
                     \x20               _ => {{}}\n\
                     \x20           }}\n\
                     \x20           crawrad_runtime_makepad::data_grid::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({field})).button_done(cx, req);\n\
                     \x20       }}\n"
                ));
            }
        }
        return Ok(code);
    }
    let mut code = String::new();
    let handler_call = match layout::handler(node, "on_select_cell") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}(col, row);\n")
        }
        None => String::new(),
    };
    code.push_str(&format!(
        "        if let Some((col, row)) = self.ui.crawrad_string_grid(id!({field})).cell_selected(actions) {{\n\
         \x20           self.form.{field}.selected = Some((col, row));\n\
         {handler_call}\
         \x20       }}\n"
    ));
    code.push_str(&format!(
        "        if let Some((col, row, text)) = self.ui.crawrad_string_grid(id!({field})).cell_edited(actions) {{\n\
         \x20           self.form.{field}.cells.set(col, row, text);\n\
         \x20       }}\n"
    ));
    for (event, method) in [("on_click", "clicked"), ("on_dbl_click", "double_clicked")] {
        if let Some(h) = layout::handler(node, event) {
            let handler = identificador(&h)?;
            code.push_str(&format!(
                "        if self.ui.crawrad_string_grid(id!({field})).{method}(actions) {{\n\
                 \x20           self.form.{handler}();\n\
                 \x20       }}\n"
            ));
        }
    }
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    Ok(format!(
        "        {name} = <CrawradStringGrid> {{\n\
         \x20           margin: {{left: {x}, top: {y}}}\n\
         \x20           width: {w}, height: {h}\n\
         \x20       }}\n"
    ))
}
