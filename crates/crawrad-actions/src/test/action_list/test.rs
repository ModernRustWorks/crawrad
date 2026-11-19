//! Unit tests for action_list collection mode and helpers.

use super::*;
use crawrad_ir::{Document, Node, Rect, Value};
use crawrad_registry::Registry;

#[test]
fn test_items_labels_fallbacks_and_indices() {
    let mut doc = Document::new("form_main", "Form");
    let mut grid = Node::new("grid1", "Grid");
    grid.set_rect(Rect::new(0.0, 0.0, 300.0, 200.0));
    doc.add_child("form_main", grid).unwrap();

    // 0: title takes priority
    let mut col0 = Node::new("col_0", "GridColumn");
    col0.props
        .insert("title".into(), Value::Str("Customer Name".into()));
    col0.props
        .insert("field".into(), Value::Str("cust_name".into()));
    col0.props
        .insert("text".into(), Value::Str("ignored_text".into()));
    doc.add_child("grid1", col0).unwrap();

    // 1: field fallback when title is missing
    let mut col1 = Node::new("col_1", "GridColumn");
    col1.props
        .insert("field".into(), Value::Str("cust_email".into()));
    col1.props
        .insert("text".into(), Value::Str("ignored_text".into()));
    doc.add_child("grid1", col1).unwrap();

    // 2: text fallback when title and field are missing
    let mut col2 = Node::new("col_2", "GridColumn");
    col2.props
        .insert("text".into(), Value::Str("Phone Number".into()));
    doc.add_child("grid1", col2).unwrap();

    // 3: node name fallback when all props are missing
    let col3 = Node::new("col_3", "GridColumn");
    doc.add_child("grid1", col3).unwrap();

    // 4: title is empty string -> falls back to field
    let mut col4 = Node::new("col_4", "GridColumn");
    col4.props.insert("title".into(), Value::Str("".into()));
    col4.props
        .insert("field".into(), Value::Str("status_code".into()));
    doc.add_child("grid1", col4).unwrap();

    let rows = items(&doc, "grid1");
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0].id, "col_0");
    assert_eq!(rows[0].label, "0  Customer Name");

    assert_eq!(rows[1].id, "col_1");
    assert_eq!(rows[1].label, "1  cust_email");

    assert_eq!(rows[2].id, "col_2");
    assert_eq!(rows[2].label, "2  Phone Number");

    assert_eq!(rows[3].id, "col_3");
    assert_eq!(rows[3].label, "3  col_3");

    assert_eq!(rows[4].id, "col_4");
    assert_eq!(rows[4].label, "4  status_code");
}

#[test]
fn test_selected_accepts_non_action_child() {
    let mut doc = Document::new("form_main", "Form");
    let grid = Node::new("grid1", "Grid");
    doc.add_child("form_main", grid).unwrap();

    let col0 = Node::new("col_0", "GridColumn");
    doc.add_child("grid1", col0).unwrap();

    let other_btn = Node::new("btn_1", "Button");
    doc.add_child("form_main", other_btn).unwrap();

    // Direct child of grid1 (GridColumn class) is accepted
    assert_eq!(
        selected(&doc, "grid1", &["col_0".to_string()]),
        Some("col_0".to_string())
    );

    // Non-child of grid1 is rejected
    assert_eq!(selected(&doc, "grid1", &["btn_1".to_string()]), None);

    // Empty selection returns None
    assert_eq!(selected(&doc, "grid1", &[]), None);

    // Non-existent id returns None
    assert_eq!(selected(&doc, "grid1", &["no_such".to_string()]), None);
}

#[test]
fn test_is_flat_collection_and_title() {
    let reg = Registry::builtin();
    assert!(is_flat_collection(&reg, "Grid"));
    assert!(!is_flat_collection(&reg, "Button"));
    assert!(!is_flat_collection(&reg, "NonExistent"));

    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid_customers", "Grid"))
        .unwrap();
    doc.add_child("form_main", Node::new("action_list1", "ActionList"))
        .unwrap();

    assert_eq!(
        title(&doc, &reg, "grid_customers"),
        "grid_customers.Columns"
    );
    assert_eq!(
        title(&doc, &reg, "action_list1"),
        "Editing form_main.action_list1"
    );
}

#[test]
fn test_move_rules_and_state() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid1", "Grid"))
        .unwrap();
    for id in ["col_0", "col_1", "col_2"] {
        doc.add_child("grid1", Node::new(id, "GridColumn")).unwrap();
    }
    let sel = |id: &str| vec![id.to_string()];

    assert!(!can_move(&doc, "grid1", &sel("col_0"), -1));
    assert!(can_move(&doc, "grid1", &sel("col_0"), 1));
    assert!(can_move(&doc, "grid1", &sel("col_1"), -1));
    assert!(can_move(&doc, "grid1", &sel("col_1"), 1));
    assert!(can_move(&doc, "grid1", &sel("col_2"), -1));
    assert!(!can_move(&doc, "grid1", &sel("col_2"), 1));
    assert!(!can_move(&doc, "grid1", &[], 1));

    let cols = vec!["id".to_string()];
    let st = state(&doc, &reg, "grid1", &sel("col_1"), Some(&cols));
    assert_eq!(st.title, "grid1.Columns");
    assert_eq!(st.buttons, COLLECTION_BUTTONS);
    assert!(!st.categories);
    assert_eq!(st.list_label, "Items:");
    assert_eq!(st.footer, COLLECTION_FOOTER);
    assert!(st.can_delete && st.can_up && st.can_down && st.can_add_fields);

    // the grid has no design-time table: "Add all fields" is not offered
    assert_eq!(state(&doc, &reg, "grid1", &[], None).buttons[5], "");
    // a design-time table with no fields: offered but off
    assert!(!state(&doc, &reg, "grid1", &[], Some(&[])).can_add_fields);
}

#[test]
fn test_action_list_mode_is_unchanged() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("acl", "ActionList"))
        .unwrap();
    let st = state(&doc, &reg, "acl", &[], None);
    assert_eq!(st.buttons, ACTION_LIST_BUTTONS);
    assert!(st.categories);
    assert_eq!(st.list_label, "Actions:");
    assert_eq!(st.footer, "");
    let (id, _) = add_args(&doc, &reg, "acl").unwrap();
    assert_eq!(id, ACTION_NEW);
}

#[test]
fn test_add_args_puts_the_item_class_inside_the_owner() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid1", "Grid"))
        .unwrap();
    doc.add_child("form_main", Node::new("btn1", "Button"))
        .unwrap();
    let (id, args) = add_args(&doc, &reg, "grid1").unwrap();
    assert_eq!(id, "component.add");
    assert_eq!(args.as_text("class"), Some("GridColumn"));
    assert_eq!(args.as_text("parent"), Some("grid1"));
    assert!(add_args(&doc, &reg, "btn1").is_none());
}

#[test]
fn test_items_when_prop_fallback_for_style_rules() {
    let mut doc = Document::new("form_main", "Form");
    let grid = Node::new("grid1", "Grid");
    doc.add_child("form_main", grid).unwrap();
    let col = Node::new("col_0", "GridColumn");
    doc.add_child("grid1", col).unwrap();

    let mut rule0 = Node::new("rule_0", "StyleRule");
    rule0
        .props
        .insert("when".into(), Value::Str("balance < 0".into()));
    doc.add_child("col_0", rule0).unwrap();

    let rule1 = Node::new("rule_1", "StyleRule");
    doc.add_child("col_0", rule1).unwrap();

    let rows = items(&doc, "col_0");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].label, "0  balance < 0");
    assert_eq!(rows[1].label, "1  rule_1");
}

#[test]
fn test_breadcrumb_and_state_with_owners() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid_customers", "Grid"))
        .unwrap();
    let mut col = Node::new("col_balance", "GridColumn");
    col.props
        .insert("name".into(), Value::Str("Balance".into()));
    doc.add_child("grid_customers", col).unwrap();

    let owners = vec!["grid_customers".to_string(), "col_balance".to_string()];
    let crumbs = breadcrumb(&doc, &reg, &owners);
    assert_eq!(
        crumbs,
        vec![
            "grid_customers.Columns".to_string(),
            "Balance.StyleRules".to_string(),
        ]
    );

    let st_single = state(&doc, &reg, "grid_customers", &[], None);
    assert!(!st_single.back);
    assert_eq!(st_single.breadcrumb, vec!["grid_customers.Columns"]);

    let st_nested = state_with_owners(&doc, &reg, &owners, &[], None);
    assert!(st_nested.back);
    assert_eq!(st_nested.breadcrumb, crumbs);
    assert_eq!(st_nested.crumb_prev, "grid_customers.Columns › ");
    assert_eq!(st_single.crumb_prev, "");
}

#[test]
fn opening_a_child_goes_one_level_down_and_back_returns() {
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid1", "Grid"))
        .expect("grid goes in the form");
    doc.add_child("grid1", Node::new("gdc1", "GridColumn"))
        .expect("column goes in the grid");
    let top = owners_after_open(&doc, &[], "grid1");
    assert_eq!(top, vec!["grid1".to_string()]);
    let down = owners_after_open(&doc, &top, "gdc1");
    assert_eq!(down, vec!["grid1".to_string(), "gdc1".to_string()]);
    // An unrelated list starts over.
    assert_eq!(owners_after_open(&doc, &down, "grid1"), top);
    assert_eq!(
        owners_after_back(&down),
        Some((top.clone(), "gdc1".to_string()))
    );
    assert_eq!(owners_after_back(&top), None);
}
