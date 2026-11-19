use super::*;
use crawrad_ir::Item;

fn sample_tree() -> ItemsEditorState {
    let items = vec![
        Item::with_children("Root1", vec![Item::new("Child1"), Item::new("Child2")]),
        Item::new("Root2"),
    ];
    ItemsEditorState {
        node_id: "tv1".into(),
        node_class: "TreeView".into(),
        node_name: "tree_view1".into(),
        items,
        selected: Some(vec![0]),
        is_tree: true,
        has_images: false,
    }
}

#[test]
fn run_each_command() {
    let mut state = sample_tree();

    // 1. NewItem creates sibling after Root1
    run(&mut state, ItemsCommand::NewItem);
    assert_eq!(state.items.len(), 3);
    assert_eq!(state.selected, Some(vec![1]));

    // 2. NewSubItem creates child under the newly created sibling
    run(&mut state, ItemsCommand::NewSubItem);
    assert_eq!(state.items[1].children.len(), 1);
    assert_eq!(state.selected, Some(vec![1, 0]));

    // 3. MoveDown moves the item down
    state.selected = Some(vec![0]);
    run(&mut state, ItemsCommand::MoveDown);
    assert_eq!(state.selected, Some(vec![1]));
    assert_eq!(state.items[1].text, "Root1");

    // 4. MoveUp moves it back up
    run(&mut state, ItemsCommand::MoveUp);
    assert_eq!(state.selected, Some(vec![0]));
    assert_eq!(state.items[0].text, "Root1");

    // 5. Delete removes the selected item
    run(&mut state, ItemsCommand::Delete);
    assert_eq!(state.items.len(), 2);
}

#[test]
fn test_set_selected_text() {
    let mut state = sample_tree();
    state.selected = Some(vec![0, 1]);
    set_selected_text(&mut state, "RenamedChild".into());
    assert_eq!(state.items[0].children[1].text, "RenamedChild");

    state.selected = None;
    set_selected_text(&mut state, "Ignored".into());
    assert_eq!(state.items[0].children[1].text, "RenamedChild");
}

#[test]
fn test_selected_expanded_leaf_and_none() {
    let mut state = sample_tree();

    // No selection -> None
    state.selected = None;
    assert_eq!(selected_expanded(&state), None);

    // Selected leaf -> None (a leaf has nothing to fold)
    state.selected = Some(vec![0, 0]);
    assert_eq!(selected_expanded(&state), None);

    // Selected branch with children -> Some(true)
    state.selected = Some(vec![0]);
    assert_eq!(selected_expanded(&state), Some(true));
}

#[test]
fn test_set_selected_expanded_round_trip() {
    let mut state = sample_tree();

    // On a leaf item, setting expanded has no effect
    state.selected = Some(vec![0, 0]);
    set_selected_expanded(&mut state, false);
    assert_eq!(selected_expanded(&state), None);

    // On a branch item with children
    state.selected = Some(vec![0]);
    assert_eq!(selected_expanded(&state), Some(true));
    set_selected_expanded(&mut state, false);
    assert_eq!(selected_expanded(&state), Some(false));
    assert!(!state.items[0].expanded);

    // Round-trip through Item::list_to
    let val = Item::list_to(&state.items);
    let crawrad_ir::Value::List(list) = &val else {
        panic!("expected Value::List");
    };
    let crawrad_ir::Value::Map(map) = &list[0] else {
        panic!("expected Value::Map for Root1");
    };
    assert_eq!(
        map.get("expanded"),
        Some(&crawrad_ir::Value::Bool(false)),
        "a collapsed branch writes \"expanded\": false"
    );

    // Reading back restores expanded: false
    let back = Item::list_from(&val);
    assert!(!back[0].expanded);
    // Leaf child remains expanded: true by default
    assert!(back[0].children[0].expanded);
}

#[test]
fn test_has_images_from_catalog() {
    let registry = crawrad_registry::Registry::builtin();
    let tree_node = crawrad_ir::Node::new("tv1", "TreeView");
    let tree_state = ItemsEditorState::from_node("tv1", &tree_node, &registry);
    assert!(tree_state.has_images);

    let list_node = crawrad_ir::Node::new("lb1", "ListBox");
    let list_state = ItemsEditorState::from_node("lb1", &list_node, &registry);
    assert!(!list_state.has_images);
}

#[test]
fn test_selected_fields_scenarios() {
    let mut state = sample_tree();
    state.has_images = true;

    // 1. No selection
    state.selected = None;
    assert_eq!(
        selected_fields(&state),
        SelectedFields {
            can_add_child: false,
            text: None,
            expanded: None,
            image_index: None,
        }
    );

    // 2. Leaf item (Child1: leaf, image_index defaults to -1)
    state.selected = Some(vec![0, 0]);
    assert_eq!(
        selected_fields(&state),
        SelectedFields {
            can_add_child: true,
            text: Some("Child1".into()),
            expanded: None,
            image_index: Some(-1),
        }
    );

    // 3. Branch item (Root1: branch, expanded defaults to true)
    state.selected = Some(vec![0]);
    assert_eq!(
        selected_fields(&state),
        SelectedFields {
            can_add_child: true,
            text: Some("Root1".into()),
            expanded: Some(true),
            image_index: Some(-1),
        }
    );

    // 4. ListBox (has_images is false)
    let mut list_state = sample_tree();
    list_state.has_images = false;
    list_state.selected = Some(vec![0]);
    assert_eq!(
        selected_fields(&list_state),
        SelectedFields {
            can_add_child: true,
            text: Some("Root1".into()),
            expanded: Some(true),
            image_index: None,
        }
    );
}

#[test]
fn test_edit_selected_image_index() {
    let mut state = sample_tree();
    state.has_images = true;
    state.selected = Some(vec![0]);

    // "2" parses as integer >= -1 -> sets to 2
    edit_selected(&mut state, ItemEdit::ImageIndex("2".into()));
    assert_eq!(selected_fields(&state).image_index, Some(2));

    // "" empty string -> sets to -1
    edit_selected(&mut state, ItemEdit::ImageIndex("".into()));
    assert_eq!(selected_fields(&state).image_index, Some(-1));

    // Reset to 2, then "-1" sets to -1
    edit_selected(&mut state, ItemEdit::ImageIndex("2".into()));
    assert_eq!(selected_fields(&state).image_index, Some(2));
    edit_selected(&mut state, ItemEdit::ImageIndex("-1".into()));
    assert_eq!(selected_fields(&state).image_index, Some(-1));

    // Reset to 2, then invalid "abc" leaves unchanged
    edit_selected(&mut state, ItemEdit::ImageIndex("2".into()));
    assert_eq!(selected_fields(&state).image_index, Some(2));
    edit_selected(&mut state, ItemEdit::ImageIndex("abc".into()));
    assert_eq!(selected_fields(&state).image_index, Some(2));

    // "-5" is < -1, so leaves unchanged
    edit_selected(&mut state, ItemEdit::ImageIndex("-5".into()));
    assert_eq!(selected_fields(&state).image_index, Some(2));

    // When has_images is false, image index edit is ignored
    state.has_images = false;
    edit_selected(&mut state, ItemEdit::ImageIndex("4".into()));
    assert_eq!(state.items[0].image_index, 2);
}

#[test]
fn test_edit_selected_text_and_expanded() {
    let mut state = sample_tree();
    state.selected = Some(vec![0]);

    edit_selected(&mut state, ItemEdit::Text("UpdatedRoot".into()));
    assert_eq!(selected_fields(&state).text, Some("UpdatedRoot".into()));

    edit_selected(&mut state, ItemEdit::Expanded(false));
    assert_eq!(selected_fields(&state).expanded, Some(false));
}

#[test]
fn test_round_trip_writes_image_index() {
    let mut state = sample_tree();
    state.has_images = true;
    state.selected = Some(vec![0]);

    edit_selected(&mut state, ItemEdit::ImageIndex("3".into()));
    assert_eq!(state.items[0].image_index, 3);

    let val = Item::list_to(&state.items);
    let crawrad_ir::Value::List(list) = &val else {
        panic!("expected Value::List");
    };
    let crawrad_ir::Value::Map(map) = &list[0] else {
        panic!("expected Value::Map for Root1");
    };
    assert_eq!(
        map.get("image_index"),
        Some(&crawrad_ir::Value::Int(3)),
        "image_index must be serialized into the Value::Map"
    );

    let restored = Item::list_from(&val);
    assert_eq!(restored[0].image_index, 3);
}
