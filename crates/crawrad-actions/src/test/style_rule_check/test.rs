//! The `When` verdict the Object Inspector shows on a StyleRule.

use crate::style_rule_check::when_check;
use crawrad_ir::{Document, Node, Value};

/// form → grid1 (columns `balance`, `city`) → Balance → rule1 with `when`.
fn doc_with_rule(when: Option<&str>) -> Document {
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("grid1", "Grid"))
        .expect("grid goes in the form");
    for (id, field) in [("gdc1", "balance"), ("gdc2", "city")] {
        let mut col = Node::new(id, "GridColumn");
        col.props
            .insert("field".to_string(), Value::Str(field.to_string()));
        doc.add_child("grid1", col)
            .expect("column goes in the grid");
    }
    let mut rule = Node::new("rule1", "StyleRule");
    if let Some(w) = when {
        rule.props
            .insert("when".to_string(), Value::Str(w.to_string()));
    }
    doc.add_child("gdc1", rule)
        .expect("rule goes in the column");
    doc
}

#[test]
fn a_rule_over_the_grid_columns_is_right() {
    let doc = doc_with_rule(Some("balance < 0 or City is null"));
    assert_eq!(when_check(&doc, "rule1"), Ok(()));
}

#[test]
fn a_rule_that_does_not_parse_says_where() {
    let doc = doc_with_rule(Some("balance == 0"));
    assert_eq!(
        when_check(&doc, "rule1"),
        Err("unknown operator \"==\" (at 8)".to_string())
    );
}

#[test]
fn a_field_the_grid_does_not_have_is_named() {
    let doc = doc_with_rule(Some("status = \"B\""));
    assert_eq!(
        when_check(&doc, "rule1"),
        Err("unknown field \"status\"".to_string())
    );
}

#[test]
fn an_empty_rule_is_wrong() {
    assert_eq!(
        when_check(&doc_with_rule(None), "rule1"),
        Err("empty rule".to_string())
    );
    assert_eq!(
        when_check(&doc_with_rule(Some("   ")), "rule1"),
        Err("empty rule".to_string())
    );
}
