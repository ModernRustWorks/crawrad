use super::*;

#[test]
fn test_named_params_insert_and_get() {
    let mut p = NamedParams::new();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);

    p.insert("id", 1i64);
    p.insert(":name", "Alice");

    assert_eq!(p.len(), 2);
    assert!(!p.is_empty());

    // Can lookup with or without leading colon
    assert_eq!(p.get("id"), Some(&Value::Integer(1)));
    assert_eq!(p.get(":id"), Some(&Value::Integer(1)));
    assert_eq!(p.get("name"), Some(&Value::Text("Alice".into())));
    assert_eq!(p.get(":name"), Some(&Value::Text("Alice".into())));
    assert_eq!(p.get("unknown"), None);

    // Overwrite existing
    p.insert("id", 2i64);
    assert_eq!(p.len(), 2);
    assert_eq!(p.get("id"), Some(&Value::Integer(2)));
}

#[test]
fn test_params_macro() {
    let p = crate::params![("a", 10i64), (":b", "test")];
    assert_eq!(p.len(), 2);
    assert_eq!(p.get("a"), Some(&Value::Integer(10)));
    assert_eq!(p.get("b"), Some(&Value::Text("test".into())));
}
