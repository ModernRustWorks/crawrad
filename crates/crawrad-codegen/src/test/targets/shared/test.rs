//! `toggle_branches`: only the branches that carry a call are emitted.

use super::toggle_branches;
use crawrad_registry::{ComponentDesc, Props};

#[test]
fn emits_only_the_branches_that_carry_a_call() {
    assert_eq!(toggle_branches("c", "", "", "  "), "");
    assert_eq!(
        toggle_branches("c", "a();\n", "", "  "),
        "  if c {\na();\n  }\n"
    );
    assert_eq!(
        toggle_branches("c", "", "b();\n", "  "),
        "  if !c {\nb();\n  }\n"
    );
    assert_eq!(
        toggle_branches("c", "a();\n", "b();\n", "  "),
        "  if c {\na();\n  } else {\nb();\n  }\n"
    );
}

#[test]
fn shortcut_piece_is_derived_for_third_party_class_and_collection() {
    let mut reg = crawrad_registry::Registry::builtin();

    let custom_action = ComponentDesc {
        inherits: None,
        class: "ThirdPartyAction",
        display: "ThirdPartyAction",
        base_name: "third_party_action",
        palette: "",
        prefix: "tpa",
        default_size: (28.0, 28.0),
        props: vec![Props::name(), Props::shortcut("")],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::NoChildren,
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    };
    reg.register(custom_action);

    let custom_action_list = ComponentDesc {
        inherits: None,
        class: "ThirdPartyActionList",
        display: "ThirdPartyActionList",
        base_name: "third_party_action_list",
        palette: "Standard",
        prefix: "tpal",
        default_size: (28.0, 28.0),
        props: vec![Props::name()],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::Collection {
            item: "ThirdPartyAction",
        },
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    };
    reg.register(custom_action_list);

    let mut doc = crawrad_ir::Document::new("Form1", "Form");
    let mut node = crawrad_ir::Node::new("list1", "ThirdPartyActionList");
    node.set_name("list1");
    doc.add_child("Form1", node).unwrap();

    let pieces = super::pecas_necessarias(&doc, &reg);
    assert!(
        pieces.contains("shortcut"),
        "third-party collection holding items with Shortcut must need shortcut piece"
    );

    // the shared text leaves it out: the Makepad form takes it from `FormBase`
    let text = super::texto_das_pecas(&doc, &reg);
    assert!(!text.contains("pub struct Shortcut"));
}

#[test]
fn shared_tree_node_literal_emits_leaves_and_branches() {
    let leaf = crawrad_ir::Item::new("Leaf");
    assert_eq!(
        super::tree_node_literal(&leaf, &[]),
        "crawrad_runtime::tree::TreeNode::leaf(\"Leaf\")"
    );

    let branch = crawrad_ir::Item::with_children("Branch", vec![leaf]);
    assert_eq!(
        super::tree_node_literal(&branch, &[]),
        "crawrad_runtime::tree::TreeNode::branch(\"Branch\", vec![crawrad_runtime::tree::TreeNode::leaf(\"Leaf\")])"
    );
}

#[test]
fn shared_tree_node_literal_emits_with_symbol_when_index_is_valid() {
    let symbols = vec!["📁".to_string(), "📄".to_string()];

    let mut leaf = crawrad_ir::Item::new("Doc");
    leaf.image_index = 1;
    assert_eq!(
        super::tree_node_literal(&leaf, &symbols),
        "crawrad_runtime::tree::TreeNode::leaf(\"Doc\").with_symbol(\"📄\")"
    );

    let mut branch = crawrad_ir::Item::with_children("Folder", vec![leaf]);
    branch.image_index = 0;
    assert_eq!(
        super::tree_node_literal(&branch, &symbols),
        "crawrad_runtime::tree::TreeNode::branch(\"Folder\", vec![crawrad_runtime::tree::TreeNode::leaf(\"Doc\").with_symbol(\"📄\")]).with_symbol(\"📁\")"
    );

    // Negative or out of bounds index emits no with_symbol
    let mut no_icon = crawrad_ir::Item::new("Plain");
    no_icon.image_index = -1;
    assert_eq!(
        super::tree_node_literal(&no_icon, &symbols),
        "crawrad_runtime::tree::TreeNode::leaf(\"Plain\")"
    );

    let mut oob = crawrad_ir::Item::new("OOB");
    oob.image_index = 99;
    assert_eq!(
        super::tree_node_literal(&oob, &symbols),
        "crawrad_runtime::tree::TreeNode::leaf(\"OOB\")"
    );
}

#[test]
fn image_symbols_resolves_icons_from_linked_image_list() {
    let mut doc = crawrad_ir::Document::new("Form1", "Form");

    let mut il = crawrad_ir::Node::new("il1", "ImageList");
    il.set_name("il1");
    let mut item0 = std::collections::BTreeMap::new();
    item0.insert("name".to_string(), crawrad_ir::Value::Str("folder".into()));
    item0.insert("symbol".to_string(), crawrad_ir::Value::Str("📁".into()));
    let mut item1 = std::collections::BTreeMap::new();
    item1.insert("name".to_string(), crawrad_ir::Value::Str("file".into()));
    item1.insert("symbol".to_string(), crawrad_ir::Value::Str("📄".into()));
    il.props.insert(
        "images_data".into(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Map(item0),
            crawrad_ir::Value::Map(item1),
        ]),
    );
    doc.add_child("Form1", il).unwrap();

    let mut tv = crawrad_ir::Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.props
        .insert("images".into(), crawrad_ir::Value::Ref("il1".into()));
    doc.add_child("Form1", tv).unwrap();

    let node = doc.node("tv1").unwrap();
    let symbols = super::image_symbols(&doc, node);
    assert_eq!(symbols, vec!["📁".to_string(), "📄".to_string()]);

    // Unknown reference yields empty
    let mut tv_bad_ref = crawrad_ir::Node::new("tv2", "TreeView");
    tv_bad_ref.props.insert(
        "images".into(),
        crawrad_ir::Value::Ref("nonexistent".into()),
    );
    assert!(super::image_symbols(&doc, &tv_bad_ref).is_empty());

    // Empty reference yields empty
    let mut tv_empty_ref = crawrad_ir::Node::new("tv3", "TreeView");
    tv_empty_ref
        .props
        .insert("images".into(), crawrad_ir::Value::Ref("".into()));
    assert!(super::image_symbols(&doc, &tv_empty_ref).is_empty());

    // Non-ImageList target yields empty
    let mut button = crawrad_ir::Node::new("btn1", "Button");
    button.set_name("btn1");
    doc.add_child("Form1", button).unwrap();
    let mut tv_wrong_target = crawrad_ir::Node::new("tv4", "TreeView");
    tv_wrong_target
        .props
        .insert("images".into(), crawrad_ir::Value::Ref("btn1".into()));
    assert!(super::image_symbols(&doc, &tv_wrong_target).is_empty());
}

#[test]
fn collapsed_preorder_and_paths_collect_non_leaf_collapsed_nodes() {
    let mut leaf = crawrad_ir::Item::new("Leaf");
    leaf.expanded = false;

    let mut sub_branch =
        crawrad_ir::Item::with_children("SubBranch", vec![crawrad_ir::Item::new("SubLeaf")]);
    sub_branch.expanded = false;

    let mut root_branch = crawrad_ir::Item::with_children("RootBranch", vec![leaf, sub_branch]);
    root_branch.expanded = false;

    let second_root = crawrad_ir::Item::new("SecondRoot");

    let items = vec![root_branch, second_root];

    // Preorder indexes:
    // 0: RootBranch (branch, collapsed: true) -> YES
    // 1: Leaf (leaf, collapsed: true) -> NO (leaf not listed)
    // 2: SubBranch (branch, collapsed: true) -> YES
    // 3: SubLeaf (leaf, collapsed: false) -> NO
    // 4: SecondRoot (leaf, collapsed: false) -> NO
    assert_eq!(super::collapsed_preorder(&items), vec![0, 2]);

    // Paths:
    // RootBranch: [0]
    // SubBranch: [0, 1]
    assert_eq!(super::collapsed_paths(&items), vec![vec![0], vec![0, 1]]);
}

#[test]
fn column_specs_literal_emits_specs_array() {
    let mut doc = crawrad_ir::Document::new("Form1", "Form");
    let mut grid = crawrad_ir::Node::new("grid1", "Grid");
    grid.set_name("grid1");

    // Without columns -> &[]
    assert_eq!(super::column_specs_literal(&doc, &grid), "&[]");

    // Add two column children, one visible, one hidden
    let mut col1 = crawrad_ir::Node::new("col1", "GridColumn");
    col1.props
        .insert("field".into(), crawrad_ir::Value::Str("total".into()));
    col1.props
        .insert("title".into(), crawrad_ir::Value::Str("Total".into()));
    col1.props
        .insert("width".into(), crawrad_ir::Value::Int(120));
    col1.props.insert(
        "alignment".into(),
        crawrad_ir::Value::Enum("taRightJustify".into()),
    );
    col1.props.insert(
        "display_format".into(),
        crawrad_ir::Value::Str("#,##0.00".into()),
    );
    col1.props.insert(
        "edit_mask".into(),
        crawrad_ir::Value::Str("(99) 9999-9999".into()),
    );
    col1.events
        .insert("on_get_text".into(), "totalGetText".into());
    col1.props
        .insert("visible".into(), crawrad_ir::Value::Bool(true));
    doc.add_child("Form1", col1).unwrap();
    grid.children.push("col1".into());

    let mut col2 = crawrad_ir::Node::new("col2", "GridColumn");
    col2.props
        .insert("field".into(), crawrad_ir::Value::Str("id".into()));
    col2.props
        .insert("visible".into(), crawrad_ir::Value::Bool(false));
    doc.add_child("Form1", col2).unwrap();
    grid.children.push("col2".into());

    let lit = super::column_specs_literal(&doc, &grid);
    assert!(lit.contains("field: \"total\".to_string()"));
    assert!(lit.contains("title: \"Total\".to_string()"));
    assert!(lit.contains("width: 120"));
    assert!(lit.contains("align: \"taRightJustify\".to_string()"));
    assert!(lit.contains("display_format: \"#,##0.00\".to_string()"));
    assert!(lit.contains("edit_mask: \"(99) 9999-9999\".to_string()"));
    assert!(lit.contains("get_text_hook: true"));
    assert!(lit.contains("set_text_hook: false"));
    assert!(lit.contains("validate_hook: false"));
    assert!(lit.contains("lookup_key: \"\".to_string()"));
    assert!(lit.contains("lookup_display: vec![]"));
    assert!(lit.contains("button_hook: false"));
    assert!(lit.contains("drop_down_count: 8"));
    assert!(lit.contains("visible: true"));
    assert!(lit.contains("word_wrap: false"));
    assert!(lit.contains("title_word_wrap: false"));
    assert!(lit.contains("field: \"id\".to_string()"));
    assert!(lit.contains("visible: false"));
}

#[test]
fn column_handlers_extracts_on_button_click() {
    let mut doc = crawrad_ir::Document::new("Form1", "Form");
    let mut grid = crawrad_ir::Node::new("grid1", "Grid");
    grid.set_name("grid1");

    let mut col = crawrad_ir::Node::new("col1", "GridColumn");
    col.props
        .insert("field".into(), crawrad_ir::Value::Str("action".into()));
    col.events
        .insert("on_button_click".into(), "onActionClick".into());
    doc.add_child("Form1", col).unwrap();
    grid.children.push("col1".into());

    let handlers = super::column_handlers(&doc, &grid);
    assert_eq!(handlers.len(), 1);
    assert_eq!(handlers[0].field, "action");
    assert_eq!(
        handlers[0].on_button_click,
        Some("onActionClick".to_string())
    );
}

#[test]
fn grid_lookups_code_resolves_table_and_sql_datasets() {
    let mut doc = crawrad_ir::Document::new("Form1", "Form");

    // Engine node
    let mut engine = crawrad_ir::Node::new("conn1", "Connection");
    engine.set_name("conn1");
    doc.add_child("Form1", engine).unwrap();

    // Dataset 1: table
    let mut ds_table = crawrad_ir::Node::new("dst_countries", "Dataset");
    ds_table.set_name("dst_countries");
    ds_table
        .props
        .insert("connection".into(), crawrad_ir::Value::Ref("conn1".into()));
    ds_table.props.insert(
        "table_name".into(),
        crawrad_ir::Value::Str("countries".into()),
    );
    doc.add_child("Form1", ds_table).unwrap();

    // Dataset 2: sql
    let mut ds_sql = crawrad_ir::Node::new("dst_cities", "Dataset");
    ds_sql.set_name("dst_cities");
    ds_sql
        .props
        .insert("connection".into(), crawrad_ir::Value::Ref("conn1".into()));
    ds_sql.props.insert(
        "sql".into(),
        crawrad_ir::Value::Str("SELECT id, name, uf FROM cities".into()),
    );
    doc.add_child("Form1", ds_sql).unwrap();

    // Grid with two lookup columns
    let mut grid = crawrad_ir::Node::new("grid1", "Grid");
    grid.set_name("grid1");

    let mut col1 = crawrad_ir::Node::new("col1", "GridColumn");
    col1.props
        .insert("field".into(), crawrad_ir::Value::Str("country_id".into()));
    col1.props.insert(
        "lookup_source".into(),
        crawrad_ir::Value::Ref("dst_countries".into()),
    );
    col1.props
        .insert("lookup_key".into(), crawrad_ir::Value::Str("id".into()));
    col1.props.insert(
        "lookup_display".into(),
        crawrad_ir::Value::Str("name;code".into()),
    );
    doc.add_child("Form1", col1).unwrap();
    grid.children.push("col1".into());

    let mut col2 = crawrad_ir::Node::new("col2", "GridColumn");
    col2.props
        .insert("field".into(), crawrad_ir::Value::Str("city_id".into()));
    col2.props.insert(
        "lookup_source".into(),
        crawrad_ir::Value::Ref("dst_cities".into()),
    );
    col2.props
        .insert("lookup_key".into(), crawrad_ir::Value::Str("id".into()));
    col2.props.insert(
        "lookup_display".into(),
        crawrad_ir::Value::Str("name".into()),
    );
    doc.add_child("Form1", col2).unwrap();
    grid.children.push("col2".into());

    // The dataset's table or SQL is read at run time, like the dataset itself does.
    let code_egui = super::grid_lookups_code(&doc, &grid, "self.", "        ").unwrap();
    assert!(code_egui.contains(
        "SqlLookup::open_sqlite(&self.conn1.url, &self.dst_countries.table, &self.dst_countries.sql, \"id\", vec![\"name\".to_string(), \"code\".to_string()])"
    ), "{code_egui}");
    assert!(
        code_egui.contains("&self.dst_cities.table, &self.dst_cities.sql"),
        "{code_egui}"
    );
    assert!(
        code_egui.contains("c.name == \"country_id\""),
        "{code_egui}"
    );
    assert!(
        code_egui.contains("crawrad_runtime::grid::LookupHandle::new(lookup)"),
        "{code_egui}"
    );

    let code_makepad = super::grid_lookups_code(&doc, &grid, "self.form.", "        ").unwrap();
    assert!(code_makepad.contains("&self.form.conn1.url"));

    // Plain grid without lookup columns returns empty code
    let plain_grid = crawrad_ir::Node::new("plain_grid", "Grid");
    assert_eq!(
        super::grid_lookups_code(&doc, &plain_grid, "self.", "        ").unwrap(),
        ""
    );
}
