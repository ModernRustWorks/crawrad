use super::*;

fn sample() -> Vec<Item> {
    vec![
        Item::with_children(
            "Node1",
            vec![
                Item::new("SubNode1"),
                Item::with_children("SubNode2", vec![Item::new("Grandchild")]),
            ],
        ),
        Item::new("Node2"),
    ]
}

#[test]
fn tree_survives_roundtrip() {
    let before = sample();
    let after = Item::list_from(&Item::list_to(&before));
    assert_eq!(
        before, after,
        "nesting and order must roundtrip identically"
    );
    assert_eq!(Item::total(&before), 5);
}

#[test]
fn leaf_does_not_save_empty_children_list() {
    // Noise in the file is noise in the git diff.
    let Value::Map(m) = Item::new("only me").to_value() else {
        panic!("item becomes map")
    };
    assert!(!m.contains_key("children"));
}

#[test]
fn malformed_collection_does_not_become_half_item() {
    let garbage = Value::List(vec![
        Value::Str("this is not an item".into()),
        Item::new("this is").to_value(),
    ]);
    let items = Item::list_from(&garbage);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].text, "this is");
}

#[test]
fn item_without_text_is_read_as_empty_and_not_lost() {
    let mut m = BTreeMap::new();
    m.insert("children".to_string(), Item::list_to(&[Item::new("child")]));
    let i = Item::from_value(&Value::Map(m)).expect("still an item");
    assert_eq!(i.text, "");
    assert_eq!(i.children.len(), 1);
}

#[test]
fn roundtrip_of_collapsed_branch() {
    let mut branch = Item::with_children("Branch", vec![Item::new("Child")]);
    branch.expanded = false;
    let original = vec![branch];
    let val = Item::list_to(&original);
    let restored = Item::list_from(&val);
    assert_eq!(original, restored);
    assert!(!restored[0].expanded);
}

#[test]
fn map_without_expanded_reads_as_true() {
    let mut m = BTreeMap::new();
    m.insert("text".to_string(), Value::Str("Node".to_string()));
    let item = Item::from_value(&Value::Map(m)).expect("valid item");
    assert!(item.expanded);
}

#[test]
fn to_value_of_open_item_has_no_expanded_key() {
    let open_leaf = Item::new("OpenLeaf");
    assert!(open_leaf.expanded);
    let Value::Map(m_leaf) = open_leaf.to_value() else {
        panic!("item becomes map");
    };
    assert!(!m_leaf.contains_key("expanded"));

    let open_branch = Item::with_children("OpenBranch", vec![Item::new("Child")]);
    assert!(open_branch.expanded);
    let Value::Map(m_branch) = open_branch.to_value() else {
        panic!("item becomes map");
    };
    assert!(!m_branch.contains_key("expanded"));
}

#[test]
fn roundtrip_of_item_with_image_index() {
    let mut item = Item::new("IconNode");
    item.image_index = 2;
    let val = item.to_value();
    let restored = Item::from_value(&val).expect("valid item");
    assert_eq!(item, restored);
    assert_eq!(restored.image_index, 2);
}

#[test]
fn map_without_image_index_reads_as_minus_one() {
    let mut m = BTreeMap::new();
    m.insert("text".to_string(), Value::Str("Node".to_string()));
    let item = Item::from_value(&Value::Map(m)).expect("valid item");
    assert_eq!(item.image_index, -1);
}

#[test]
fn to_value_of_minus_one_image_index_has_no_image_index_key() {
    let item = Item::new("NoIcon");
    assert_eq!(item.image_index, -1);
    let Value::Map(m) = item.to_value() else {
        panic!("item becomes map");
    };
    assert!(!m.contains_key("image_index"));
}

#[test]
fn an_item_finds_its_icon_only_inside_the_list() {
    let symbols = vec!["A".to_string(), "B".to_string()];
    let mut item = Item::new("n");
    assert_eq!(item.symbol(&symbols), None);
    item.image_index = 1;
    assert_eq!(item.symbol(&symbols), Some("B"));
    item.image_index = 2;
    assert_eq!(item.symbol(&symbols), None);
}

#[test]
fn the_linked_symbols_come_from_the_named_image_list() {
    let mut doc = crate::Document::new("form1", "Form");
    let mut list = crate::Node::new("il1", "ImageList");
    let icon = |s: &str| {
        Value::Map(BTreeMap::from([
            ("name".to_string(), Value::Str("x".into())),
            ("symbol".to_string(), Value::Str(s.into())),
        ]))
    };
    list.props.insert(
        "images_data".into(),
        Value::List(vec![icon("A"), icon("B")]),
    );
    let mut impostor = crate::Node::new("pn1", "Panel");
    impostor
        .props
        .insert("images_data".into(), list.props["images_data"].clone());
    doc.add_child("form1", list).expect("the list");
    doc.add_child("form1", impostor).expect("the panel");
    let mut tree = crate::Node::new("tv1", "TreeView");
    assert!(Item::linked_symbols(&doc, &tree).is_empty());
    tree.props.insert("images".into(), Value::Ref("il1".into()));
    assert_eq!(Item::linked_symbols(&doc, &tree), ["A", "B"]);
    tree.props.insert("images".into(), Value::Ref("pn1".into()));
    assert!(
        Item::linked_symbols(&doc, &tree).is_empty(),
        "only an ImageList has icons"
    );
}
