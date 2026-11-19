use crawrad_ir::Value;

#[test]
fn catalogue_fields_skips_common_and_by_hand_fields() {
    let desc = crawrad_registry::components::edit();
    let fields = super::catalogue_fields(&desc, &["visible", "enabled", "text"]);
    let joined = fields.join("\n");

    assert!(!joined.contains("pub name"));
    assert!(!joined.contains("pub rect"));
    assert!(!joined.contains("pub action"));
    assert!(!joined.contains("pub visible"));
    assert!(!joined.contains("pub enabled"));
    assert!(!joined.contains("pub text:"));
    assert!(joined.contains("pub text_hint:"));
}

#[test]
fn catalogue_fields_by_hand_handling() {
    let desc = crawrad_registry::components::edit();

    let skipped = super::catalogue_fields(&desc, &["visible", "enabled", "text", "text_hint"]);
    assert!(!skipped.iter().any(|f| f.contains("text_hint")));

    let emitted = super::catalogue_fields(&desc, &["visible", "enabled", "text"]);
    assert!(emitted.iter().any(|f| f.contains("text_hint")));
}

#[test]
fn catalogue_inits_value_when_present_and_default_otherwise() {
    let desc = crawrad_registry::components::edit();

    let mut node_with_val = crawrad_ir::Node::new("edit1", "Edit");
    let custom_val = Value::Str("abc".into());
    node_with_val
        .props
        .insert("text_hint".to_string(), custom_val.clone());

    let inits_with_val =
        super::catalogue_inits(&node_with_val, &desc, &["visible", "enabled", "text"]);
    let expected_custom = format!("text_hint: {}", super::value_to_rust_literal(&custom_val));
    assert!(inits_with_val.contains(&expected_custom));

    let node_default = crawrad_ir::Node::new("edit2", "Edit");
    let inits_default =
        super::catalogue_inits(&node_default, &desc, &["visible", "enabled", "text"]);
    let prop_text_hint = desc.props.iter().find(|p| p.key == "text_hint").unwrap();
    let expected_default = format!("text_hint: {}", super::prop_default_literal(prop_text_hint));
    assert!(inits_default.contains(&expected_default));
}
