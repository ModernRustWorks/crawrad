//! A `Grid` bound to a `Connection`: what each target writes, and the
//! crates the form then needs.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::targets::egui::GeradorRxp;
use crate::targets::makepad::GeradorMakepad;
use crate::{AlvoGerador, Gerador, component_dependencies, has_bound_grid};

/// The catalog with the `Connection` read from the real `manifest.json`.
fn catalog() -> Registry {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let manifest: Manifest = serde_json::from_str(text).expect("valid manifest");
    let mut r = Registry::builtin();
    for c in manifest.to_components().expect("the manifest converts") {
        r.register_external(c).expect("the catalog takes it");
    }
    r
}

/// A form with `conn1` (a SQLite url) and `grid1`, bound to it when `bound`.
fn form(r: &Registry, bound: bool) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
    }
    let mut engine = r
        .instantiate(
            "Connection",
            "conn1".to_string(),
            Rect::new(8.0, 8.0, 28.0, 28.0),
        )
        .expect("Connection instantiates");
    engine.set_name("conn1");
    engine
        .props
        .insert("url".to_string(), Value::Str("sqlite://shop.db".into()));
    d.add_child("form_main", engine)
        .expect("the engine goes in");

    let mut grid = r
        .instantiate(
            "Grid",
            "grid1".to_string(),
            Rect::new(16.0, 48.0, 600.0, 400.0),
        )
        .expect("Grid instantiates");
    grid.set_name("grid1");
    if bound {
        grid.props
            .insert("data_source".to_string(), Value::Ref("conn1".into()));
        grid.props
            .insert("table_name".to_string(), Value::Str("orders".into()));
    }
    d.add_child("form_main", grid).expect("the grid goes in");
    d
}

fn designer(g: &dyn Gerador, d: &Document, r: &Registry) -> String {
    g.gerar(d, "app", r)
        .expect("the target generates")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

#[test]
fn makepad_writes_the_data_grid_and_opens_the_table_once() {
    let r = catalog();
    let code = designer(&GeradorMakepad, &form(&r, true), &r);
    assert!(code.contains("grid1 = <CrawradDataGrid>"), "{code}");
    assert!(code.contains("pub grid1_opened: bool,"));
    assert!(code.contains("if !self.grid1_opened {"));
    assert!(code.contains(
        "datagrid_sql::SqlSource::open_sqlite_table(crawrad_runtime::db_url::resolve_sqlite_url(&self.form.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.form.grid1.table_name)"
    ));
    assert!(code.contains("crawrad_runtime::grid::UnavailableSource::from_error(&e)"));
    assert!(code.contains(
        "crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!(grid1)).set_source(cx, source, columns);"
    ));
}

#[test]
fn egui_writes_the_data_grid_beside_the_grid_state() {
    let r = catalog();
    let code = designer(&GeradorRxp, &form(&r, true), &r);
    assert!(
        code.contains("pub grid1_rows: Option<crawrad_runtime_egui::data_grid::DataGrid>,"),
        "{code}"
    );
    assert!(code.contains("grid1_rows: None,"));
    assert!(code.contains("table_name: \"orders\".to_string(),"));
    assert!(code.contains(
        "datagrid_sql::SqlSource::open_sqlite_table(crawrad_runtime::db_url::resolve_sqlite_url(&self.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.grid1.table_name)"
    ));
    assert!(code.contains("crawrad_runtime_egui::data_grid::DataGrid::new(source, columns)"));
    assert!(code.contains("rows.show(&mut w);"));
}

#[test]
fn an_unbound_grid_keeps_its_own_cells_in_both_targets() {
    let r = catalog();
    let doc = form(&r, false);
    let makepad = designer(&GeradorMakepad, &doc, &r);
    assert!(makepad.contains("grid1 = <CrawradStringGrid>"));
    assert!(!makepad.contains("CrawradDataGrid"));
    assert!(!makepad.contains("datagrid_sql"));
    let egui = designer(&GeradorRxp, &doc, &r);
    assert!(egui.contains("egui::Grid::new(\"grid_grid1\")"));
    assert!(!egui.contains("datagrid_sql"));
    assert!(!egui.contains("grid1_rows"));
}

#[test]
fn a_grid_pointing_at_a_deleted_engine_is_born_unbound() {
    let r = catalog();
    let mut doc = form(&r, true);
    assert_eq!(doc.remove_subtree("conn1").len(), 1, "the engine goes away");
    let makepad = designer(&GeradorMakepad, &doc, &r);
    assert!(!makepad.contains("CrawradDataGrid"), "{makepad}");
    let egui = designer(&GeradorRxp, &doc, &r);
    assert!(!egui.contains("datagrid_sql"), "{egui}");
}

#[test]
fn only_a_bound_grid_brings_the_grid_crates() {
    let r = catalog();
    let names = |doc: &Document, alvo| -> Vec<String> {
        component_dependencies(doc, alvo)
            .iter()
            .map(|d| d.nome().to_string())
            .collect()
    };
    let bound = form(&r, true);
    assert!(has_bound_grid(&bound));
    assert_eq!(
        names(&bound, AlvoGerador::Makepad),
        ["datagrid", "datagrid-sql"]
    );
    assert_eq!(
        names(&bound, AlvoGerador::Egui),
        [
            "crawrad-runtime",
            "datagrid",
            "datagrid-sql",
            "crawrad-runtime-egui"
        ]
    );
    let unbound = form(&r, false);
    assert!(!has_bound_grid(&unbound));
    // egui always asks for `crawrad-runtime`: its `main` reports a startup
    // failure through it, grid or no grid.
    assert_eq!(names(&unbound, AlvoGerador::Egui), ["crawrad-runtime"]);
    assert!(names(&unbound, AlvoGerador::Makepad).is_empty());
}

/// The grid's `GridColumn`s reach both apps as the specs the runtime
/// turns into columns; a grid without columns asks for every field (`&[]`).
#[test]
fn both_targets_write_the_grid_columns_into_the_opening_code() {
    let r = catalog();
    let mut d = form(&r, true);
    for (id, field, visible) in [("gdc1", "total", true), ("gdc2", "id", false)] {
        let mut col = r
            .instantiate(
                "GridColumn",
                id.to_string(),
                Rect::new(0.0, 0.0, 28.0, 28.0),
            )
            .expect("GridColumn instantiates");
        col.set_name(id);
        col.props
            .insert("field".to_string(), Value::Str(field.into()));
        col.props
            .insert("visible".to_string(), Value::Bool(visible));
        if id == "gdc1" {
            col.props
                .insert("null_text".to_string(), Value::Str("(empty)".into()));
            col.props
                .insert("value_map".to_string(), Value::Str("1=Yes;0=No".into()));
            col.props
                .insert("renderer".to_string(), Value::Enum("crCheckBox".into()));
            col.props.insert(
                "vertical_alignment".to_string(),
                Value::Enum("tlBottom".into()),
            );
            col.props.insert("ellipsis".to_string(), Value::Bool(true));
            col.props.insert("word_wrap".to_string(), Value::Bool(true));
            col.props
                .insert("title_word_wrap".to_string(), Value::Bool(true));
            col.props.insert(
                "title_font".to_string(),
                Value::Font(crawrad_ir::Font::new("Segoe UI", 15.0)),
            );
            col.props.insert("auto_size".to_string(), Value::Bool(true));
            col.props
                .insert("resizable".to_string(), Value::Bool(false));
            col.props.insert("movable".to_string(), Value::Bool(false));
            col.props.insert("sortable".to_string(), Value::Bool(false));
            col.props
                .insert("sort_order".to_string(), Value::Enum("soDesc".into()));
            col.props.insert("sort_priority".to_string(), Value::Int(3));
            col.props
                .insert("nulls_position".to_string(), Value::Enum("npFirst".into()));
            col.props
                .insert("groupable".to_string(), Value::Bool(false));
            col.props.insert("group_index".to_string(), Value::Int(1));
            col.props
                .insert("summary".to_string(), Value::Enum("smSum".into()));
            col.props
                .insert("summary_format".to_string(), Value::Str("#,##0.00".into()));
            col.props
                .insert("group_summary".to_string(), Value::Enum("smAvg".into()));
            col.props
                .insert("filterable".to_string(), Value::Bool(true));
            col.props
                .insert("filter_kind".to_string(), Value::Enum("fkText".into()));
            col.props.insert("readonly".to_string(), Value::Bool(true));
            col.props.insert("required".to_string(), Value::Bool(true));
            col.props
                .insert("default_value".to_string(), Value::Str("0".into()));
            col.props.insert("max_length".to_string(), Value::Int(10));
            col.props.insert("min_value".to_string(), Value::Int(1));
            col.props.insert("max_value".to_string(), Value::Int(100));
            col.props.insert("tab_stop".to_string(), Value::Bool(false));
            col.props
                .insert("editor".to_string(), Value::Enum("edList".into()));
            col.props.insert(
                "pick_list".to_string(),
                Value::Str("One\nTwo\n\nThree\n".into()),
            );
        }
        d.add_child("grid1", col)
            .expect("the column goes in the grid");
    }
    for g in [&GeradorMakepad as &dyn Gerador, &GeradorRxp] {
        let code = designer(g, &d, &r);
        assert!(
            code.contains("GridColumn::from_schema_and_design("),
            "{code}"
        );
        assert!(code.contains("field: \"total\".to_string()"), "{code}");
        assert!(code.contains("field: \"id\".to_string()"));
        assert!(code.contains("visible: false"));
        assert!(code.contains("ellipsis: true"), "{code}");
        assert!(code.contains("word_wrap: true"), "{code}");
        assert!(code.contains("title_word_wrap: true"), "{code}");
        assert!(
            code.contains("null_text: \"(empty)\".to_string()"),
            "{code}"
        );
        assert!(
            code.contains("value_map: \"1=Yes;0=No\".to_string()"),
            "{code}"
        );
        assert!(
            code.contains("renderer: \"crCheckBox\".to_string()"),
            "{code}"
        );
        assert!(
            code.contains("vertical_alignment: \"tlBottom\".to_string()"),
            "{code}"
        );
        assert!(code.contains("size: 15.0"), "{code}");
        assert!(code.contains("auto_size: true"), "{code}");
        assert!(code.contains("resizable: false"), "{code}");
        assert!(code.contains("movable: false"), "{code}");
        assert!(code.contains("sortable: false"), "{code}");
        assert!(
            code.contains("sort_order: \"soDesc\".to_string()"),
            "{code}"
        );
        assert!(code.contains("sort_priority: 3"), "{code}");
        assert!(
            code.contains("nulls_position: \"npFirst\".to_string()"),
            "{code}"
        );
        assert!(code.contains("groupable: false"), "{code}");
        assert!(code.contains("group_index: 1"), "{code}");
        assert!(code.contains("summary: \"smSum\".to_string()"), "{code}");
        assert!(
            code.contains("summary_format: \"#,##0.00\".to_string()"),
            "{code}"
        );
        assert!(
            code.contains("group_summary: \"smAvg\".to_string()"),
            "{code}"
        );
        assert!(code.contains("filterable: true"), "{code}");
        assert!(
            code.contains("filter_kind: \"fkText\".to_string()"),
            "{code}"
        );
        assert!(code.contains("read_only: true"), "{code}");
        assert!(code.contains("required: true"), "{code}");
        assert!(code.contains("default_value: \"0\".to_string()"), "{code}");
        assert!(code.contains("max_length: 10"), "{code}");
        assert!(code.contains("min_value: 1"), "{code}");
        assert!(code.contains("max_value: 100"), "{code}");
        assert!(code.contains("tab_stop: false"), "{code}");
        assert!(code.contains("editor: \"edList\".to_string()"), "{code}");
        assert!(
            code.contains(
                "pick_list: vec![\"One\".to_string(), \"Two\".to_string(), \"Three\".to_string()]"
            ),
            "{code}"
        );
        // the column is data of the grid, never a field of its own
        assert!(!code.contains("gdc1:"), "{code}");
    }
    let makepad_code = designer(&GeradorMakepad, &d, &r);
    assert!(
        makepad_code.contains(".set_row_height(self.form.grid1.default_row_height as f64);"),
        "{makepad_code}"
    );
    let egui_code = designer(&GeradorRxp, &d, &r);
    assert!(
        egui_code.contains(".set_row_height(self.grid1.default_row_height);"),
        "{egui_code}"
    );
    for g in [&GeradorMakepad as &dyn Gerador, &GeradorRxp] {
        let code = designer(g, &form(&r, true), &r);
        assert!(
            code.contains("datagrid::RowSource::schema(&source), &[])"),
            "{code}"
        );
    }
}

/// A bound grid with column hints emits tooltip DSL and hover actions; without them emits neither.
#[test]
fn bound_grid_tooltip_dsl_and_actions_only_when_hint_column_exists() {
    let r = catalog();

    // 1. A bound grid with no hint column emits no hover action (its own `Hint`
    //    may still bring the tooltip in, so only the actions are checked).
    let doc_no_hint = form(&r, true);
    let code_no_hint = designer(&GeradorMakepad, &doc_no_hint, &r);
    assert!(
        !code_no_hint.contains("header_hover"),
        "code_no_hint should not contain header_hover: {code_no_hint}"
    );

    // 2. A bound grid with one column title_hint = "Tip" contains both crawrad_tooltip and header_hover.
    let mut doc_with_hint = form(&r, true);
    let mut col = r
        .instantiate(
            "GridColumn",
            "gdc1".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("GridColumn instantiates");
    col.set_name("gdc1");
    col.props
        .insert("field".to_string(), Value::Str("total".into()));
    col.props
        .insert("title_hint".to_string(), Value::Str("Tip".into()));
    doc_with_hint
        .add_child("grid1", col)
        .expect("the column goes in the grid");

    let code_with_hint = designer(&GeradorMakepad, &doc_with_hint, &r);
    assert!(
        code_with_hint.contains("crawrad_tooltip"),
        "code_with_hint should contain crawrad_tooltip: {code_with_hint}"
    );
    assert!(
        code_with_hint.contains("header_hover"),
        "code_with_hint should contain header_hover: {code_with_hint}"
    );
}

/// Column OnGetText / OnSetText / OnValidate: both targets answer the grid's text
/// requests and its pending row check by calling the form's handlers; a grid with
/// no column handler gets none of that code.
#[test]
fn column_handlers_answer_text_requests_and_row_checks_in_both_targets() {
    let r = catalog();
    let plain = form(&r, true);
    let mut hooked = form(&r, true);
    let mut col = r
        .instantiate(
            "GridColumn",
            "gdc1".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("GridColumn instantiates");
    col.set_name("gdc1");
    col.props
        .insert("field".to_string(), Value::Str("total".into()));
    for (event, handler) in [
        ("on_get_text", "total_get_text"),
        ("on_set_text", "total_set_text"),
        ("on_validate", "total_validate"),
    ] {
        col.events.insert(event.into(), handler.into());
    }
    hooked
        .add_child("grid1", col)
        .expect("the column goes in the grid");

    for g in [&GeradorMakepad as &dyn Gerador, &GeradorRxp] {
        let code = designer(g, &hooked, &r);
        for expected in [
            "text_requests()",
            "total_get_text(&req.row, &mut req.text)",
            "pending_check()",
            "total_set_text(&check.row, &mut cell.text)",
            "total_validate(&check.row, &cell.text, &mut cell.error)",
            "resolve_check(",
            "get_text_hook: true",
            "validate_hook: true",
        ] {
            assert!(code.contains(expected), "missing `{expected}`: {code}");
        }
        // The column is not emitted, but its handlers are called: the
        // user's file must carry the stubs or the project does not build.
        let user = g
            .gerar(&hooked, "app", &r)
            .expect("the target generates")
            .into_iter()
            .find(|a| {
                let p = a.caminho.to_string_lossy();
                !p.contains("designer") && a.caminho.file_name().is_some_and(|f| f != "main.rs")
            })
            .expect("the user's file")
            .conteudo;
        for expected in [
            "pub fn total_get_text(&mut self, _row: &crawrad_runtime::grid::RowValues, _text: &mut String)",
            "pub fn total_set_text(&mut self, _row: &crawrad_runtime::grid::RowValues, _text: &mut String)",
            "pub fn total_validate(&mut self, _row: &crawrad_runtime::grid::RowValues, _value: &str, _error: &mut String)",
        ] {
            assert!(user.contains(expected), "missing `{expected}`: {user}");
        }
        let code = designer(g, &plain, &r);
        assert!(!code.contains("text_requests()"), "{code}");
        assert!(!code.contains("pending_check()"), "{code}");
    }
}

/// A column with a StyleRule child + OnDrawCell event emits style_rules and OnDrawCell
/// hooks in both targets, plus the user handler stub.
#[test]
fn column_with_style_rule_and_draw_cell_emitted_for_both_targets() {
    let r = catalog();
    let mut hooked = form(&r, true);
    let mut col = r
        .instantiate(
            "GridColumn",
            "gdc1".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("GridColumn instantiates");
    col.set_name("total");
    col.props
        .insert("field".to_string(), Value::Str("total".into()));
    col.events
        .insert("on_draw_cell".into(), "total_draw_cell".into());
    hooked
        .add_child("grid1", col)
        .expect("the column goes in the grid");

    let mut rule = r
        .instantiate(
            "StyleRule",
            "rule1".to_string(),
            Rect::new(0.0, 0.0, 0.0, 0.0),
        )
        .expect("StyleRule instantiates");
    rule.set_name("rule1");
    rule.props
        .insert("when".to_string(), Value::Str("balance < 0".into()));
    rule.props.insert(
        "font_style".to_string(),
        Value::Map(
            [("Bold".to_string(), Value::Bool(true))]
                .into_iter()
                .collect(),
        ),
    );
    hooked
        .add_child("gdc1", rule)
        .expect("the rule goes in the column");

    for g in [&GeradorMakepad as &dyn Gerador, &GeradorRxp] {
        let code = designer(g, &hooked, &r);
        for expected in [
            "style_requests()",
            "total_draw_cell(&req.row, &mut req.style)",
            "draw_cell_hook: true",
            "style_rules: vec![",
            "when: \"balance < 0\".to_string()",
            "bold: true",
        ] {
            assert!(code.contains(expected), "missing `{expected}`: {code}");
        }
        let user = g
            .gerar(&hooked, "app", &r)
            .expect("the target generates")
            .into_iter()
            .find(|a| {
                let p = a.caminho.to_string_lossy();
                !p.contains("designer") && a.caminho.file_name().is_some_and(|f| f != "main.rs")
            })
            .expect("the user's file")
            .conteudo;
        assert!(
            user.contains("pub fn total_draw_cell(&mut self, _row: &crawrad_runtime::grid::RowValues, _style: &mut crawrad_runtime::grid::CellStyle)"),
            "missing draw cell stub: {user}"
        );
    }
}

/// A style rule whose `When` does not parse fails codegen naming the grid, column, rule index, and error.
#[test]
fn invalid_style_rule_when_fails_codegen_with_context_message() {
    let r = catalog();
    let mut bad_doc = form(&r, true);
    let mut col = r
        .instantiate(
            "GridColumn",
            "gdc1".to_string(),
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("GridColumn instantiates");
    col.set_name("Balance");
    col.props
        .insert("field".to_string(), Value::Str("balance".into()));
    bad_doc
        .add_child("grid1", col)
        .expect("the column goes in the grid");

    let mut rule = r
        .instantiate(
            "StyleRule",
            "rule1".to_string(),
            Rect::new(0.0, 0.0, 0.0, 0.0),
        )
        .expect("StyleRule instantiates");
    rule.set_name("rule1");
    rule.props
        .insert("when".to_string(), Value::Str("balance".into()));
    bad_doc
        .add_child("gdc1", rule)
        .expect("the rule goes in the column");

    for g in [&GeradorMakepad as &dyn Gerador, &GeradorRxp] {
        let err = g
            .gerar(&bad_doc, "app", &r)
            .expect_err("codegen must fail for invalid style rule when");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "grid1.Balance.StyleRules[0]: expected a comparison after \"balance\" (at 7)"
            ),
            "unexpected error message: {msg}"
        );
    }
}

/// The bound form, with `dst1` (a Dataset over `conn1` with a query) between
/// the connection and the grid: Provider -> Connection -> Dataset -> Grid.
fn dataset_form(r: &Registry) -> Document {
    let mut d = form(r, true);
    let mut dst = r
        .instantiate(
            "Dataset",
            "dst1".to_string(),
            Rect::new(40.0, 8.0, 28.0, 28.0),
        )
        .expect("Dataset instantiates");
    dst.set_name("dst1");
    dst.props
        .insert("connection".to_string(), Value::Ref("conn1".into()));
    dst.props.insert(
        "sql".to_string(),
        Value::Str("SELECT name, city FROM contacts ORDER BY name".into()),
    );
    d.add_child("form_main", dst).expect("the dataset goes in");
    d.node_mut("grid1")
        .expect("the grid")
        .props
        .insert("data_source".to_string(), Value::Ref("dst1".into()));
    d
}

#[test]
fn a_grid_on_a_dataset_opens_the_dataset_query_in_both_targets() {
    let r = catalog();
    let doc = dataset_form(&r);
    let makepad = designer(&GeradorMakepad, &doc, &r);
    assert!(makepad.contains("grid1 = <CrawradDataGrid>"), "{makepad}");
    assert!(
        makepad.contains(
            "datagrid_sql::SqlSource::open_sqlite_query(crawrad_runtime::db_url::resolve_sqlite_url(&self.form.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.form.dst1.sql)"
        ),
        "{makepad}"
    );
    assert!(!makepad.contains("open_sqlite_table"), "{makepad}");
    let egui = designer(&GeradorRxp, &doc, &r);
    assert!(
        egui.contains(
            "datagrid_sql::SqlSource::open_sqlite_query(crawrad_runtime::db_url::resolve_sqlite_url(&self.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.dst1.sql)"
        ),
        "{egui}"
    );
    assert!(!egui.contains("open_sqlite_table"), "{egui}");
}

#[test]
fn a_grid_on_a_table_mode_dataset_opens_the_dataset_table_in_both_targets() {
    let r = catalog();
    let mut doc = dataset_form(&r);
    let dst = doc.node_mut("dst1").expect("the dataset");
    dst.props
        .insert("sql".to_string(), Value::Str(String::new()));
    dst.props
        .insert("table".to_string(), Value::Str("contacts".into()));
    let makepad = designer(&GeradorMakepad, &doc, &r);
    assert!(
        makepad.contains(
            "datagrid_sql::SqlSource::open_sqlite_table(crawrad_runtime::db_url::resolve_sqlite_url(&self.form.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.form.dst1.table)"
        ),
        "{makepad}"
    );
    assert!(!makepad.contains("open_sqlite_query"), "{makepad}");
    let egui = designer(&GeradorRxp, &doc, &r);
    assert!(
        egui.contains(
            "datagrid_sql::SqlSource::open_sqlite_table(crawrad_runtime::db_url::resolve_sqlite_url(&self.conn1.url, env!(\"CARGO_MANIFEST_DIR\")), &self.dst1.table)"
        ),
        "{egui}"
    );
    assert!(!egui.contains("open_sqlite_query"), "{egui}");
}

/// The table-mode form with `edit1` bound two-way to `dst1.name`: the grid and
/// the edit share the dataset.
fn grid_and_edit_form(r: &Registry) -> Document {
    let mut d = dataset_form(r);
    let dst = d.node_mut("dst1").expect("the dataset");
    dst.props
        .insert("sql".to_string(), Value::Str(String::new()));
    dst.props
        .insert("table".to_string(), Value::Str("contacts".into()));
    let mut edit = r
        .instantiate(
            "Edit",
            "edit1".to_string(),
            Rect::new(8.0, 300.0, 120.0, 24.0),
        )
        .expect("Edit instantiates");
    edit.set_name("edit1");
    edit.bindings.insert(
        "text".to_string(),
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", edit).expect("the edit goes in");
    d
}

#[test]
fn a_grid_on_an_active_dataset_moves_its_cursor_and_reloads_in_both_targets() {
    let r = catalog();
    let doc = grid_and_edit_form(&r);
    for (code, reload, rec_no) in [
        (
            designer(&GeradorMakepad, &doc, &r),
            "self.grid1_opened = false;",
            "crawrad_data_grid(&self.ui, id!(grid1)).rec_no()",
        ),
        (
            designer(&GeradorRxp, &doc, &r),
            "self.grid1_rows = None;",
            "self.grid1_rows.as_ref().map_or(0, |rows| rows.rec_no())",
        ),
    ] {
        assert!(
            code.contains("pub grid1_ds_seen: Option<(crawrad_binding::DatasetState, usize)>"),
            "{code}"
        );
        assert!(
            code.contains("if let Some(adapter) = self.dst1_adapter.as_ref()"),
            "{code}"
        );
        assert!(code.contains(reload), "{code}");
        assert!(code.contains(rec_no), "{code}");
        assert!(
            code.contains("adapter.collection().set_cursor(Some(row));"),
            "{code}"
        );
    }
}

#[test]
fn a_reloaded_grid_stands_on_the_dataset_record_without_moving_it_in_both_targets() {
    let r = catalog();
    let doc = grid_and_edit_form(&r);
    for (code, set_row, pick_row) in [
        (
            designer(&GeradorMakepad, &doc, &r),
            "crawrad_data_grid(&self.ui, id!(grid1)).set_rec_no(cx, row);",
            "let row = crawrad_runtime_makepad",
        ),
        (
            designer(&GeradorRxp, &doc, &r),
            "if let Some(grid) = self.grid1_rows.as_mut() { grid.set_rec_no(row); }",
            "let row = self.grid1_rows",
        ),
    ] {
        // A reload forgets the row it saw, so the reopened grid restores it ...
        let reload = code.find("self.grid1_rec_seen = None;").expect(&code);
        let restore = code
            .find("&& self.grid1_rec_seen.is_none() {")
            .expect(&code);
        assert!(reload < restore, "{code}");
        // ... from the dataset cursor, clamped to the last row (after a Delete) ...
        assert!(
            code.contains(
                "let row = rows.cursor().unwrap_or(0).min(rows.len().saturating_sub(1));"
            ),
            "{code}"
        );
        let set = code[restore..].find(set_row).expect(&code) + restore;
        // ... and records it as seen, so the user-pick branch does not move the cursor again.
        let seen = code[set..]
            .find("self.grid1_rec_seen = Some(row);")
            .expect(&code)
            + set;
        let pick = code[restore..].find(pick_row).expect(&code) + restore;
        assert!(set < seen && seen < pick, "{code}");
        assert!(!code.contains("self.grid1_rec_seen = Some(0);"), "{code}");
    }
}

#[test]
fn a_grid_on_a_query_dataset_does_not_follow_it() {
    let r = catalog();
    let doc = dataset_form(&r);
    for code in [
        designer(&GeradorMakepad, &doc, &r),
        designer(&GeradorRxp, &doc, &r),
    ] {
        assert!(!code.contains("grid1_ds_seen"), "{code}");
    }
}
