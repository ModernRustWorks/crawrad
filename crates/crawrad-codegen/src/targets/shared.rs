//! **WHAT BOTH TARGETS CARVE** from the same compiled file.
//!
//! There are two such files, and both exist for the same reason: the designer
//! and the application must agree.
//!
//! | file | where it is compiled | what it carries |
//! |---|---|---|
//! | `crawrad-runtime-egui/src/desenho_comum.rs` | the egui preview | drawing, and only the egui target reads it |
//! | `crawrad-runtime-makepad/src/shared_geometry.rs` | the designer canvas | arithmetic, and BOTH targets read it |
//!
//! Carving by marker — `[CRAWRAD:INICIO nome]` / `[CRAWRAD:FIM nome]` — lives here
//! because it gained two clients. It was born inside the egui target, and was
//! `crate::targets::egui::recorte` until the Makepad target needed the same
//! `fraction` that egui carved out: one target reaching into the other's module to
//! find a text function is the kind of dependency that only surfaces when it is already
//! widespread.

use crawrad_note::note;
#[note("targets-shared-recorte")]
pub fn recorte(texto: &str, nome: &str) -> String {
    let abre = format!("// [CRAWRAD:INICIO {nome}]\n");
    let fecha = format!("// [CRAWRAD:FIM {nome}]");
    let comum = texto.replace("\r\n", "\n");
    let i = comum
        .find(&abre)
        .unwrap_or_else(|| panic!("the shared file has no `{nome}` section"))
        + abre.len();
    let j = comum[i..]
        .find(&fecha)
        .unwrap_or_else(|| panic!("the `{nome}` section of the shared file does not close"))
        + i;
    comum[i..j].to_string()
}

#[note("targets-shared-geometria")]
pub const GEOMETRIA: &str = include_str!("../../../crawrad-component/src/fraction.rs");

#[note("targets-shared-trecho-da-geometria")]
pub fn trecho_da_geometria(nome: &str) -> String {
    recorte(GEOMETRIA, nome)
}

/// **THE FIVE COLOR ROLES**, as the designer canvas knows them.
///
/// Compiled, not text: `live_design!` and `ui.visuals()` require different
/// forms, and what passes through is the VALUE.
pub mod tema {
    include!("../../../crawrad-runtime-makepad/src/shared_theme.rs");
}

#[note("targets-shared-form-hint-text")]
pub fn form_hint_text(root: &crawrad_ir::Node) -> Option<&str> {
    let show = !matches!(
        root.props.get("show_hint"),
        Some(crawrad_ir::Value::Bool(false))
    );
    match root.props.get("hint") {
        Some(crawrad_ir::Value::Str(h)) if show && !h.is_empty() => Some(h),
        _ => None,
    }
}

#[note("targets-shared-hex")]
pub fn hex(cor: [f32; 4]) -> String {
    let c = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    dsl_colour(&format!(
        "#{:02X}{:02X}{:02X}",
        c(cor[0]),
        c(cor[1]),
        c(cor[2])
    ))
}

#[note("targets-shared-dsl-colour")]
pub fn dsl_colour(hex: &str) -> String {
    match hex.strip_prefix('#') {
        Some(digits) if digits.starts_with(|c: char| c.is_ascii_digit()) => format!("#x{digits}"),
        _ => hex.to_string(),
    }
}

#[note("targets-shared-eventos-do-declarado")]
pub const EVENTOS_DO_DECLARADO: &[&str] = &["on_click"];

#[note("targets-shared-ligacoes-do-declarado")]
pub fn ligacoes_do_declarado(
    node: &crawrad_ir::Node,
) -> Result<Vec<(String, String)>, crate::Erro> {
    let mut v = Vec::new();
    for evento in EVENTOS_DO_DECLARADO {
        let Some(h) = node
            .events
            .get(*evento)
            .filter(|h| !h.trim().is_empty())
            .cloned()
        else {
            continue;
        };
        v.push(((*evento).to_string(), crate::identificador(&h)?));
    }
    Ok(v)
}

#[note("targets-shared-nome-apontado")]
pub fn nome_apontado(doc: &crawrad_ir::Document, id: &str) -> Option<String> {
    if id.is_empty() {
        return None;
    }
    doc.nodes.get(id).map(|n| n.name().to_string())
}

#[note("targets-shared-literal-de-referencia")]
pub fn literal_de_referencia(doc: &crawrad_ir::Document, id: &str) -> String {
    match nome_apontado(doc, id) {
        Some(nome) => format!("{}.to_string()", crate::literal(&nome)),
        None => "String::new()".to_string(),
    }
}

// ---------------------------------------------------------------------------
// THE BOUND GRID: the same opening code in both targets
// ---------------------------------------------------------------------------

#[note("targets-shared-bound-grid-engine")]
pub fn bound_grid_engine(
    doc: &crawrad_ir::Document,
    node: &crawrad_ir::Node,
) -> Result<Option<String>, crate::Erro> {
    // Through a Dataset, the url is still its Connection's (Provider ->
    // Connection -> Dataset -> Grid).
    match crawrad_ir::design_rows::grid_binding(doc, node) {
        Some(binding) => Ok(Some(crate::identificador(binding.connection.name())?)),
        None => Ok(None),
    }
}

/// How a bound grid opens: `(SqlSource opener, its second argument)`.
///
/// Bound to a Dataset, the dataset's `sql`, or its `table` when the sql is
/// empty (table mode; the grid's `table_name` is ignored); bound to a Connection, the grid's own `table_name`. `prefix` is
/// how the target reaches the form's fields (`self.form.` / `self.`).
#[note("targets-shared-bound-grid-target")]
pub fn bound_grid_target(
    doc: &crawrad_ir::Document,
    node: &crawrad_ir::Node,
    prefix: &str,
    grid_field: &str,
) -> (&'static str, String) {
    let dataset = crawrad_ir::design_rows::grid_binding(doc, node)
        .and_then(|b| b.dataset)
        // a name the generator cannot write was refused by the designer pass
        .and_then(|d| Some((d, crate::identificador(d.name()).ok()?)));
    match dataset {
        // Table mode: no sql, a table -> the dataset's table, editable.
        Some((d, dataset)) if dataset_table_mode(d) => {
            ("open_sqlite_table", format!("&{prefix}{dataset}.table"))
        }
        Some((_, dataset)) => ("open_sqlite_query", format!("&{prefix}{dataset}.sql")),
        None => (
            "open_sqlite_table",
            format!("&{prefix}{grid_field}.table_name"),
        ),
    }
}

/// Whether a Dataset node is in table mode: no `sql`, a `table` (editable).
pub fn dataset_table_mode(dataset: &crawrad_ir::Node) -> bool {
    let set = |key: &str| matches!(dataset.props.get(key), Some(crawrad_ir::Value::Str(s)) if !s.trim().is_empty());
    !set("sql") && set("table")
}

#[note("targets-shared-bound-grid-open")]
pub fn bound_grid_open(
    opener: &str,
    url: &str,
    table: &str,
    indent: &str,
    specs: &str,
    lookups_code: &str,
) -> String {
    // A relative SQLite file is the project's, never the working directory's.
    // The openers take `impl AsRef<str>`: the owned `String` goes in as is, since
    // a `&` there trips clippy's `needless_borrows_for_generic_args` in the app.
    let url = resolved_url(url);
    let url = url.strip_prefix('&').unwrap_or(&url);
    // `mut` only when edLookup columns get their lookup after the build.
    let columns = if lookups_code.is_empty() {
        "columns"
    } else {
        "mut columns"
    };
    format!(
        "match datagrid_sql::SqlSource::{opener}({url}, {table}) {{\n\
         {indent}    Ok(source) => {{\n\
         {indent}        let {columns} = crawrad_runtime::grid::GridColumn::from_schema_and_design(datagrid::RowSource::schema(&source), {specs});\n\
         {lookups_code}\
         {indent}        (std::sync::Arc::new(source) as std::sync::Arc<dyn datagrid::RowSource>, columns)\n\
         {indent}    }}\n\
         {indent}    Err(e) => (\n\
         {indent}        std::sync::Arc::new(crawrad_runtime::grid::UnavailableSource::from_error(&e)) as std::sync::Arc<dyn datagrid::RowSource>,\n\
         {indent}        Vec::new(),\n\
         {indent}    ),\n\
         {indent}}}"
    )
}

fn font_literal(f: &crawrad_ir::Font) -> String {
    format!(
        "crawrad_runtime::grid::Font {{ name: {}.to_string(), size: {:.1}, bold: {}, italic: {}, underline: {}, strike_out: {}, color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }} }}",
        crate::literal(&f.name),
        f.size,
        f.bold,
        f.italic,
        f.underline,
        f.strike_out,
        f.color.r,
        f.color.g,
        f.color.b,
        f.color.a,
    )
}

/// Refuses a grid whose StyleRules hold a `When` that does not parse: the form does not
/// compile with a bad rule.
///
/// # Errors
///
/// The first rule that does not parse, named `grid.Column.StyleRules[i]` with the reason.
pub fn check_style_rules(
    doc: &crawrad_ir::Document,
    grid: &crawrad_ir::Node,
) -> Result<(), crate::Erro> {
    fn children_of<'a>(
        doc: &'a crawrad_ir::Document,
        n: &'a crawrad_ir::Node,
        class: &'static str,
    ) -> impl Iterator<Item = &'a crawrad_ir::Node> {
        n.children
            .iter()
            .filter_map(move |id| doc.node(id))
            .filter(move |c| c.class == class)
    }
    for column in children_of(doc, grid, crawrad_ir::grid_columns::GRID_COLUMN_CLASS) {
        for (i, rule) in children_of(doc, column, "StyleRule").enumerate() {
            let when = match rule.props.get("when") {
                Some(crawrad_ir::Value::Str(s)) => s.as_str(),
                _ => "",
            };
            if let Err(e) = crawrad_ir::style_rule::parse(when) {
                return Err(crate::Erro::Message(format!(
                    "{}.{}.StyleRules[{i}]: {} (at {})",
                    grid.name(),
                    column.name(),
                    e.message,
                    e.at
                )));
            }
        }
    }
    Ok(())
}

#[note("targets-shared-column-specs-literal")]
pub fn column_specs_literal(doc: &crawrad_ir::Document, grid: &crawrad_ir::Node) -> String {
    let specs: Vec<String> = crawrad_ir::grid_columns::ColumnSpec::of_grid(doc, grid)
        .iter()
        .map(|s| {
            let pick_list_items: Vec<String> = s
                .pick_list
                .iter()
                .map(|item| format!("{}.to_string()", crate::literal(item)))
                .collect();
            let pick_list_literal = format!("vec![{}]", pick_list_items.join(", "));
            let lookup_display_items: Vec<String> = s
                .lookup_display
                .iter()
                .map(|item| format!("{}.to_string()", crate::literal(item)))
                .collect();
            let lookup_display_literal = format!("vec![{}]", lookup_display_items.join(", "));
            let rule_literals: Vec<String> = s
                .style_rules
                .iter()
                .map(|r| {
                    format!(
                        "crawrad_runtime::grid::StyleRuleSpec {{ when: {}.to_string(), color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }}, text_color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }}, bold: {}, italic: {} }}",
                        crate::literal(&r.when),
                        r.color.r,
                        r.color.g,
                        r.color.b,
                        r.color.a,
                        r.text_color.r,
                        r.text_color.g,
                        r.text_color.b,
                        r.text_color.a,
                        r.bold,
                        r.italic,
                    )
                })
                .collect();
            let style_rules_literal = format!("vec![{}]", rule_literals.join(", "));
            format!(
                "crawrad_runtime::grid::ColumnSpec {{ field: {}.to_string(), title: {}.to_string(), width: {}, align: {}.to_string(), display_format: {}.to_string(), visible: {}, null_text: {}.to_string(), boolean_texts: {}.to_string(), value_map: {}.to_string(), renderer: {}.to_string(), title_alignment: {}.to_string(), min_width: {}, max_width: {}, color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }}, text_color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }}, title_color: crawrad_runtime::grid::Color {{ r: {}, g: {}, b: {}, a: {} }}, vertical_alignment: {}.to_string(), padding: crawrad_runtime::grid::Edges {{ left: {:.1}, top: {:.1}, right: {:.1}, bottom: {:.1} }}, ellipsis: {}, word_wrap: {}, title_word_wrap: {}, title_hint: {}.to_string(), cell_hint: {}, title_font: {}, font: {}, auto_size: {}, resizable: {}, movable: {}, sortable: {}, sort_order: {}.to_string(), sort_priority: {}, nulls_position: {}.to_string(), groupable: {}, group_index: {}, summary: {}.to_string(), summary_format: {}.to_string(), group_summary: {}.to_string(), filterable: {}, filter_kind: {}.to_string(), read_only: {}, required: {}, default_value: {}.to_string(), max_length: {}, min_value: {}, max_value: {}, tab_stop: {}, editor: {}.to_string(), pick_list: {}, edit_mask: {}.to_string(), get_text_hook: {}, set_text_hook: {}, validate_hook: {}, field_type: {}.to_string(), lookup_key: {}.to_string(), lookup_display: {}, button_hook: {}, drop_down_count: {}, draw_cell_hook: {}, style_rules: {} }}",
                crate::literal(&s.field),
                crate::literal(&s.title),
                s.width,
                crate::literal(&s.align),
                crate::literal(&s.display_format),
                s.visible,
                crate::literal(&s.null_text),
                crate::literal(&s.boolean_texts),
                crate::literal(&s.value_map),
                crate::literal(&s.renderer),
                crate::literal(&s.title_alignment),
                s.min_width,
                s.max_width,
                s.color.r,
                s.color.g,
                s.color.b,
                s.color.a,
                s.text_color.r,
                s.text_color.g,
                s.text_color.b,
                s.text_color.a,
                s.title_color.r,
                s.title_color.g,
                s.title_color.b,
                s.title_color.a,
                crate::literal(&s.vertical_alignment),
                s.padding.left,
                s.padding.top,
                s.padding.right,
                s.padding.bottom,
                s.ellipsis,
                s.word_wrap,
                s.title_word_wrap,
                crate::literal(&s.title_hint),
                s.cell_hint,
                font_literal(&s.title_font),
                font_literal(&s.font),
                s.auto_size,
                s.resizable,
                s.movable,
                s.sortable,
                crate::literal(&s.sort_order),
                s.sort_priority,
                crate::literal(&s.nulls_position),
                s.groupable,
                s.group_index,
                crate::literal(&s.summary),
                crate::literal(&s.summary_format),
                crate::literal(&s.group_summary),
                s.filterable,
                crate::literal(&s.filter_kind),
                s.read_only,
                s.required,
                crate::literal(&s.default_value),
                s.max_length,
                s.min_value,
                s.max_value,
                s.tab_stop,
                crate::literal(&s.editor),
                pick_list_literal,
                crate::literal(&s.edit_mask),
                s.get_text_hook,
                s.set_text_hook,
                s.validate_hook,
                crate::literal(&s.field_type),
                crate::literal(&s.lookup_key),
                lookup_display_literal,
                s.button_hook,
                s.drop_down_count,
                s.draw_cell_hook,
                style_rules_literal,
            )
        })
        .collect();
    format!("&[{}]", specs.join(", "))
}

/// Handler information for a GridColumn's OnGetText, OnSetText, OnValidate, OnButtonClick, and OnDrawCell events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnHandlerInfo {
    pub field: String,
    pub on_get_text: Option<String>,
    pub on_set_text: Option<String>,
    pub on_validate: Option<String>,
    pub on_button_click: Option<String>,
    pub on_draw_cell: Option<String>,
}

/// Gathers any column event handlers defined on grid columns.
pub fn column_handlers(
    doc: &crawrad_ir::Document,
    grid: &crawrad_ir::Node,
) -> Vec<ColumnHandlerInfo> {
    grid.children
        .iter()
        .filter_map(|id| doc.node(id))
        .filter(|n| n.class == "GridColumn")
        .filter_map(|n| {
            let field = match n.props.get("field") {
                Some(crawrad_ir::Value::Str(s) | crawrad_ir::Value::Enum(s)) => s.clone(),
                _ => String::new(),
            };
            let on_get_text = n
                .events
                .get("on_get_text")
                .filter(|h| !h.trim().is_empty())
                .cloned();
            let on_set_text = n
                .events
                .get("on_set_text")
                .filter(|h| !h.trim().is_empty())
                .cloned();
            let on_validate = n
                .events
                .get("on_validate")
                .filter(|h| !h.trim().is_empty())
                .cloned();
            let on_button_click = n
                .events
                .get("on_button_click")
                .filter(|h| !h.trim().is_empty())
                .cloned();
            let on_draw_cell = n
                .events
                .get("on_draw_cell")
                .filter(|h| !h.trim().is_empty())
                .cloned();
            if on_get_text.is_some()
                || on_set_text.is_some()
                || on_validate.is_some()
                || on_button_click.is_some()
                || on_draw_cell.is_some()
            {
                Some(ColumnHandlerInfo {
                    field,
                    on_get_text,
                    on_set_text,
                    on_validate,
                    on_button_click,
                    on_draw_cell,
                })
            } else {
                None
            }
        })
        .collect()
}

/// The code that gives each edLookup column of `grid` its lookup right after its columns
/// are built: the column's `LookupSource` dataset opened over its engine, reading the
/// dataset's `table`/`sql` at run time like the dataset itself. A column whose dataset is not
/// wired gets nothing and edits as text. `form` is how the generated code reaches the form
/// (`self.` in egui, `self.form.` in Makepad).
pub fn grid_lookups_code(
    doc: &crawrad_ir::Document,
    grid: &crawrad_ir::Node,
    form: &str,
    indent: &str,
) -> Result<String, crate::Erro> {
    let text = |n: &crawrad_ir::Node, key: &str| match n.props.get(key) {
        Some(crawrad_ir::Value::Str(s) | crawrad_ir::Value::Enum(s)) => s.trim().to_string(),
        _ => String::new(),
    };
    let mut code = String::new();
    for col in grid
        .children
        .iter()
        .filter_map(|id| doc.node(id))
        .filter(|n| n.class == "GridColumn")
    {
        let Some(crawrad_ir::Value::Ref(dataset_id)) = col.props.get("lookup_source") else {
            continue;
        };
        let Some(conn) = bound_dataset_connection(doc, dataset_id)? else {
            continue;
        };
        let display: Vec<String> = text(col, "lookup_display")
            .split(';')
            .map(str::trim)
            .filter(|d| !d.is_empty())
            .map(|d| format!("{}.to_string()", crate::literal(d)))
            .collect();
        code.push_str(&format!(
            "{indent}        if let Ok(lookup) = datagrid_sql::SqlLookup::open_sqlite(&{form}{engine}.url, &{form}{dataset}.table, &{form}{dataset}.sql, {key}, vec![{display}])\n\
             {indent}            && let Some(col) = columns.iter_mut().find(|c| c.name == {field})\n\
             {indent}        {{\n\
             {indent}            col.lookup = crawrad_runtime::grid::LookupHandle::new(lookup);\n\
             {indent}        }}\n",
            engine = conn.engine_name,
            dataset = conn.dataset_name,
            key = crate::literal(&text(col, "lookup_key")),
            display = display.join(", "),
            field = crate::literal(&text(col, "field")),
        ));
    }
    Ok(code)
}

#[note("targets-shared-bounddatasetconnection")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundDatasetConnection {
    /// Identifier of the `Dataset` node in the generated code (e.g. `dst1`).
    pub dataset_name: String,
    /// Identifier of the `Connection` node in the generated code (e.g. `conn1`).
    pub engine_name: String,
}

#[note("targets-shared-bound-dataset-connection")]
pub fn bound_dataset_connection(
    doc: &crawrad_ir::Document,
    dataset_id: &str,
) -> Result<Option<BoundDatasetConnection>, crate::Erro> {
    if dataset_id.is_empty() {
        return Ok(None);
    }
    let Some(dataset_node) = doc.node(dataset_id) else {
        return Ok(None);
    };
    if dataset_node.class != "Dataset" {
        return Ok(None);
    }
    let dataset_name = crate::identificador(dataset_node.name())?;
    let Some(crawrad_ir::Value::Ref(engine_id)) = dataset_node.props.get("connection") else {
        return Ok(None);
    };
    if engine_id.is_empty() {
        return Ok(None);
    }
    let Some(engine_node) = doc.node(engine_id) else {
        return Ok(None);
    };
    if engine_node.class != "Connection" {
        return Ok(None);
    }
    let engine_name = crate::identificador(engine_node.name())?;
    Ok(Some(BoundDatasetConnection {
        dataset_name,
        engine_name,
    }))
}

#[note("targets-shared-bound-dataset-open")]
pub fn bound_dataset_open(engine_url_expr: &str, dataset_table_expr: &str) -> String {
    let url = resolved_url(engine_url_expr);
    format!("crawrad_binding::PortaDbBridge::open_table({url}, {dataset_table_expr})")
}

/// The url expression, with a relative SQLite file found next to the exe or in
/// the project folder the app was built from (`crawrad_runtime::db_url`).
fn resolved_url(url_expr: &str) -> String {
    format!(
        "&crawrad_runtime::db_url::resolve_sqlite_url({url_expr}, env!(\"CARGO_MANIFEST_DIR\"))"
    )
}

// ---------------------------------------------------------------------------
// THE STOCK PIECES A FORM NEEDS
// ---------------------------------------------------------------------------

#[note("targets-shared-peca-edges")]
pub const PECA_EDGES: &str = include_str!("../../../crawrad-component/src/pieces/edges.rs");

#[note("targets-shared-peca-color")]
pub const PECA_COLOR: &str = include_str!("../../../crawrad-component/src/pieces/color.rs");

#[note("targets-shared-peca-datetime")]
pub const PECA_DATETIME: &str = include_str!("../../../crawrad-component/src/pieces/date_time.rs");

#[note("targets-shared-peca-filepath")]
pub const PECA_FILEPATH: &str = include_str!("../../../crawrad-component/src/pieces/file_path.rs");

#[note("targets-shared-peca-cursor")]
pub const PECA_CURSOR: &str = include_str!("../../../crawrad-component/src/pieces/cursor.rs");

#[note("targets-shared-peca-modal-result")]
pub const PECA_MODAL_RESULT: &str =
    include_str!("../../../crawrad-component/src/pieces/modal_result.rs");

#[note("targets-shared-peca-shortcut")]
pub const PECA_SHORTCUT: &str = include_str!("../../../crawrad-component/src/pieces/shortcut.rs");

#[note("targets-shared-peca-font")]
pub const PECA_FONT: &str = include_str!("../../../crawrad-component/src/pieces/font.rs");

#[note("targets-shared-peca-picture")]
pub const PECA_PICTURE: &str = include_str!("../../../crawrad-component/src/pieces/picture.rs");

#[note("targets-shared-pecas-necessarias")]
pub fn pecas_necessarias(
    doc: &crawrad_ir::Document,
    reg: &crawrad_registry::Registry,
) -> std::collections::BTreeSet<&'static str> {
    let mut out = std::collections::BTreeSet::new();
    for node in doc.nodes.values() {
        let Some(desc) = reg.get(&node.class) else {
            continue;
        };
        if !desc.emits_code {
            continue;
        }
        // the form's own properties (its Color, Position, WindowState)
        // are read by the generator, not kept in a struct: no piece for them
        if node.class == "Form" {
            continue;
        }
        if node.class == "MessageDialog" {
            out.insert("modal_result");
        }
        let mut descs = vec![desc];
        if let Some(item_name) = desc.item_class()
            && let Some(item_desc) = reg.get(item_name)
        {
            descs.push(item_desc);
        }
        for d in descs {
            for prop in &d.props {
                if crate::emission::why_not_set(prop.key, prop.category).is_some() {
                    continue;
                }
                match prop.kind {
                    crawrad_registry::PropKind::Edges => {
                        out.insert("edges");
                    }
                    crawrad_registry::PropKind::Color => {
                        out.insert("color");
                    }
                    crawrad_registry::PropKind::DateTime => {
                        out.insert("date_time");
                    }
                    crawrad_registry::PropKind::FilePath => {
                        out.insert("file_path");
                    }
                    crawrad_registry::PropKind::Cursor => {
                        out.insert("cursor");
                    }
                    crawrad_registry::PropKind::ModalResult => {
                        out.insert("modal_result");
                    }
                    crawrad_registry::PropKind::Shortcut => {
                        out.insert("shortcut");
                    }
                    crawrad_registry::PropKind::Font => {
                        out.insert("font");
                    }
                    crawrad_registry::PropKind::Picture => {
                        out.insert("picture");
                    }
                    _ => {}
                }
            }
        }
    }
    out
}

#[note("targets-shared-texto-das-pecas")]
pub fn texto_das_pecas(doc: &crawrad_ir::Document, reg: &crawrad_registry::Registry) -> String {
    let mut out = String::new();
    for peca in pecas_necessarias(doc, reg) {
        let texto = match peca {
            "edges" => PECA_EDGES,
            "color" => PECA_COLOR,
            "date_time" => PECA_DATETIME,
            "file_path" => PECA_FILEPATH,
            "cursor" => PECA_CURSOR,
            "modal_result" => PECA_MODAL_RESULT,
            // the Makepad form takes `Shortcut` from `FormBase`; egui adds it (`ui_rs`)
            "shortcut" => "",
            "font" => PECA_FONT,
            "picture" => PECA_PICTURE,
            outro => unreachable!("peca sem texto: {outro}"),
        };
        out.push('\n');
        out.push_str(&texto.replace("\r\n", "\n"));
    }
    unique_uses(&out)
}

#[note("targets-shared-unique-uses")]
pub fn unique_uses(text: &str) -> String {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        if line.starts_with("use ") && !seen.insert(line) {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[note("targets-shared-literal-da-peca")]
pub fn literal_da_peca(v: &crawrad_ir::Value) -> Option<String> {
    match v {
        crawrad_ir::Value::Edges(e) => Some(format!(
            "Edges {{ left: {:.1}, top: {:.1}, right: {:.1}, bottom: {:.1} }}",
            e.left, e.top, e.right, e.bottom
        )),
        crawrad_ir::Value::Color(c) => Some(format!(
            "Color {{ r: {}, g: {}, b: {}, a: {} }}",
            c.r, c.g, c.b, c.a
        )),
        crawrad_ir::Value::DateTime(dt) => Some(format!(
            "DateTime::new({}, {}, {}, {}, {}, {})",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
        )),
        crawrad_ir::Value::FilePath(fp) => {
            let lit = crate::literal(&fp.path);
            Some(format!("FilePath::new({lit})"))
        }
        crawrad_ir::Value::Cursor(c) => Some(format!("Cursor::{c:?}")),
        crawrad_ir::Value::ModalResult(m) => Some(format!("ModalResult::{m:?}")),
        crawrad_ir::Value::Shortcut(sc) => {
            if sc.is_none() {
                Some("Shortcut::NONE".to_string())
            } else {
                Some(format!(
                    "Shortcut {{ key: Key::{:?}, ctrl: {}, alt: {}, shift: {}, meta: {} }}",
                    sc.key, sc.ctrl, sc.alt, sc.shift, sc.meta
                ))
            }
        }
        crawrad_ir::Value::Font(f) => Some(format!(
            "Font {{ name: {}.to_string(), size: {:.1}, bold: {}, italic: {}, underline: {}, strike_out: {}, color: [{}, {}, {}, {}] }}",
            crate::literal(&f.name),
            f.size,
            f.bold,
            f.italic,
            f.underline,
            f.strike_out,
            f.color.r,
            f.color.g,
            f.color.b,
            f.color.a
        )),
        crawrad_ir::Value::Picture(p) => {
            if p.is_empty() {
                if let Some(src) = &p.source {
                    Some(format!(
                        "Picture {{ bytes: Vec::new(), source: Some({}.to_string()) }}",
                        crate::literal(src)
                    ))
                } else {
                    Some("Picture::empty()".to_string())
                }
            } else {
                let bytes_lit = format!("vec!{:?}", p.bytes);
                let src_lit = match &p.source {
                    Some(s) => format!("Some({}.to_string())", crate::literal(s)),
                    None => "None".to_string(),
                };
                Some(format!(
                    "Picture {{ bytes: {}, source: {} }}",
                    bytes_lit, src_lit
                ))
            }
        }
        _ => None,
    }
}

#[note("targets-shared-look-consts")]
pub fn look_consts(
    doc: &crawrad_ir::Document,
    reg: &crawrad_registry::Registry,
    emitted: &[&crawrad_ir::Node],
) -> String {
    // the controls this form EMITS, and not every node of the document: a
    // derived form's override entry names an inherited control that
    // the base draws, and a constant for it would never be read
    let has = |key: &str| {
        emitted.iter().any(|n| {
            n.id != doc.root
                && reg
                    .get(&n.class)
                    .is_some_and(|d| d.props.iter().any(|p| p.key == key))
        })
    };
    let Some(root) = doc.node(&doc.root) else {
        return String::new();
    };
    let mut s = String::new();
    if has("font") {
        let (size, c) = match root.props.get("font") {
            Some(crawrad_ir::Value::Font(f)) => (f.size, f.color),
            _ => (0.0, crawrad_ir::Color::transparent()),
        };
        s.push_str(&format!(
            "\n/// The form's font size, for a control whose `ParentFont` is on; 0 = the theme's.\n\
             const CRAWRAD_FORM_FONT_SIZE: f32 = {size:?};\n\
             /// The form's text colour, the same way; alpha 0 = the theme's.\n\
             const CRAWRAD_FORM_FONT_COLOR: [u8; 4] = [{}, {}, {}, {}];\n",
            c.r, c.g, c.b, c.a
        ));
    }
    s
}

#[note("targets-shared-with-show-hint-const")]
pub fn with_show_hint_const(doc: &crawrad_ir::Document, code: String) -> String {
    if !code.contains("CRAWRAD_FORM_SHOW_HINT &&") {
        return code;
    }
    let show = !doc.node(&doc.root).is_some_and(|r| {
        matches!(
            r.props.get("show_hint"),
            Some(crawrad_ir::Value::Bool(false))
        )
    });
    format!(
        "{code}
/// The form's `ShowHint` (#139): off, no control on it shows its hint.
         const CRAWRAD_FORM_SHOW_HINT: bool = {show};
"
    )
}

#[note("targets-shared-standard-colors-const")]
pub fn standard_colors_const() -> String {
    let rows: String = crawrad_ir::Color::STANDARD
        .iter()
        .map(|(n, c)| {
            format!(
                "    ({}, Color::rgb({}, {}, {})),\n",
                crate::literal(n),
                c.r,
                c.g,
                c.b
            )
        })
        .collect();
    format!(
        "const CRAWRAD_STANDARD_COLORS: [(&str, Color); {}] = [\n{rows}];\n\n",
        crawrad_ir::Color::STANDARD.len()
    )
}

#[note("targets-shared-toggle-branches")]
pub fn toggle_branches(cond: &str, when_true: &str, when_false: &str, indent: &str) -> String {
    match (when_true.is_empty(), when_false.is_empty()) {
        (true, true) => String::new(),
        (false, true) => format!("{indent}if {cond} {{\n{when_true}{indent}}}\n"),
        (true, false) => format!("{indent}if !{cond} {{\n{when_false}{indent}}}\n"),
        (false, false) => {
            format!("{indent}if {cond} {{\n{when_true}{indent}}} else {{\n{when_false}{indent}}}\n")
        }
    }
}

#[cfg(test)]
#[path = "../test/targets/shared/test.rs"]
mod test;

#[note("targets-shared-selection")]
#[derive(Debug, Clone, Copy)]
pub enum Selection {
    /// `selected: Option<usize>` — `ListBox`, `ComboBox`.
    Optional,
    /// `item_index: i64`, `-1` for none — `Chips`, `RadioGroup`, `ListView`.
    Signed,
}

#[note("targets-shared-forget-selection-past-end")]
pub fn forget_selection_past_end(owner: &str, selection: Selection) -> String {
    match selection {
        Selection::Optional => format!(
            "        if {owner}.selected.is_some_and(|i| i >= {owner}.items.len()) {{\n\
             \x20           {owner}.selected = None;\n\
             \x20       }}\n"
        ),
        Selection::Signed => format!(
            "        if {owner}.item_index >= {owner}.items.len() as i64 {{\n\
             \x20           {owner}.item_index = -1;\n\
             \x20       }}\n"
        ),
    }
}

/// An empty `Text` shows the selected item: the editable `ComboBox`
/// opens on the item its `ItemIndex` names, as in the classic RAD IDE.
///
/// # Examples
///
/// ```ignore
/// let code = text_follows_selection("self.form.combo_box1");
/// assert!(code.contains("text.is_empty()"));
/// ```
pub fn text_follows_selection(owner: &str) -> String {
    format!(
        "        if {owner}.text.is_empty()\n\
         \x20           && let Some(item) = {owner}.selected.and_then(|i| {owner}.items.get(i).cloned())\n\
         \x20       {{\n\
         \x20           {owner}.text = item;\n\
         \x20       }}\n"
    )
}

#[note("targets-shared-rust-type-of")]
pub fn rust_type_of(default: &crawrad_ir::Value) -> &'static str {
    match default {
        crawrad_ir::Value::Bool(_) => "bool",
        crawrad_ir::Value::Float(_) => "f32",
        crawrad_ir::Value::Int(_) => "i64",
        crawrad_ir::Value::List(_) => "Strings",
        crawrad_ir::Value::DateTime(_) => "DateTime",
        crawrad_ir::Value::FilePath(_) => "FilePath",
        crawrad_ir::Value::Cursor(_) => "Cursor",
        crawrad_ir::Value::ModalResult(_) => "ModalResult",
        crawrad_ir::Value::Shortcut(_) => "Shortcut",
        crawrad_ir::Value::Font(_) => "Font",
        crawrad_ir::Value::Color(_) => "Color",
        crawrad_ir::Value::Picture(_) => "Picture",
        _ => "String",
    }
}

#[note("targets-shared-image-symbols")]
pub fn image_symbols(doc: &crawrad_ir::Document, node: &crawrad_ir::Node) -> Vec<String> {
    crawrad_ir::Item::linked_symbols(doc, node)
}

#[note("targets-shared-tree-node-literal")]
pub fn tree_node_literal(item: &crawrad_ir::Item, symbols: &[String]) -> String {
    let text = crate::literal(&item.text);
    let mut lit = if item.children.is_empty() {
        format!("crawrad_runtime::tree::TreeNode::leaf({text})")
    } else {
        let children: Vec<String> = item
            .children
            .iter()
            .map(|c| tree_node_literal(c, symbols))
            .collect();
        format!(
            "crawrad_runtime::tree::TreeNode::branch({text}, vec![{}])",
            children.join(", ")
        )
    };
    if let Some(symbol) = item.symbol(symbols) {
        lit.push_str(&format!(".with_symbol({})", crate::literal(symbol)));
    }
    lit
}

#[note("targets-shared-collapsed-preorder")]
pub fn collapsed_preorder(items: &[crawrad_ir::Item]) -> Vec<usize> {
    fn walk(item: &crawrad_ir::Item, level: usize, counter: &mut usize, out: &mut Vec<usize>) {
        let idx = *counter;
        *counter += 1;

        if !item.children.is_empty() && !item.expanded {
            out.push(idx);
        }

        if level < 32 {
            for child in &item.children {
                walk(child, level + 1, counter, out);
            }
        }
    }

    let mut out = Vec::new();
    let mut counter = 0;
    for item in items {
        walk(item, 0, &mut counter, &mut out);
    }
    out
}

#[note("targets-shared-collapsed-paths")]
pub fn collapsed_paths(items: &[crawrad_ir::Item]) -> Vec<Vec<usize>> {
    fn walk(
        item: &crawrad_ir::Item,
        level: usize,
        current_path: &[usize],
        out: &mut Vec<Vec<usize>>,
    ) {
        if !item.children.is_empty() && !item.expanded {
            out.push(current_path.to_vec());
        }

        if level < 32 {
            for (i, child) in item.children.iter().enumerate() {
                let mut next_path = current_path.to_vec();
                next_path.push(i);
                walk(child, level + 1, &next_path, out);
            }
        }
    }

    let mut out = Vec::new();
    for (i, item) in items.iter().enumerate() {
        walk(item, 0, &[i], &mut out);
    }
    out
}
