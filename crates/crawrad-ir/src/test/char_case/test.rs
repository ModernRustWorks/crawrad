use super::*;

#[test]
fn each_case_turns_the_text() {
    assert_eq!(char_cased("Abc", "ecUpperCase"), "ABC");
    assert_eq!(char_cased("AbC", "ecLowerCase"), "abc");
    assert_eq!(char_cased("AbC", "ecNormal"), "AbC");
    assert_eq!(char_cased("AbC", "nonsense"), "AbC");
    assert_eq!(char_cased("straße", "ecUpperCase"), "STRASSE");
}

#[test]
fn a_node_turns_by_its_own_char_case() {
    let mut n = Node::new("edt", "Edit");
    assert_eq!(cased_for(&n, "Ab"), "Ab", "no CharCase leaves it");
    n.props
        .insert("char_case".into(), Value::Enum("ecLowerCase".into()));
    assert_eq!(cased_for(&n, "Ab"), "ab");
}
