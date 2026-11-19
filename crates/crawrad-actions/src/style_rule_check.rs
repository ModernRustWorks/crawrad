//! Is a `StyleRule`'s `When` right? The Object Inspector's answer.
//!
//! The same verdict the user reads in the box: green when the expression parses
//! and names only fields the grid has, red with the reason otherwise. Codegen
//! refuses an expression that does not parse; a field the grid does not show is
//! told here, while the user types, because at run time it just never matches.

use crawrad_ir::{Document, Value};

/// The class of a conditional style rule, a child of a `GridColumn`.
pub const STYLE_RULE_CLASS: &str = "StyleRule";

/// Checks the `when` of the `StyleRule` node `rule`: `Ok` when it parses and
/// every field it names is the `field` of a column of the rule's grid.
///
/// # Errors
///
/// The reason a person reads: `empty rule`, the parse error with its position,
/// or `unknown field "x"`.
pub fn when_check(doc: &Document, rule: &str) -> Result<(), String> {
    let when = match doc.node(rule).and_then(|n| n.props.get("when")) {
        Some(Value::Str(s)) => s.trim(),
        _ => "",
    };
    if when.is_empty() {
        return Err("empty rule".to_string());
    }
    let expr =
        crawrad_ir::style_rule::parse(when).map_err(|e| format!("{} (at {})", e.message, e.at))?;
    let fields = grid_fields(doc, rule);
    for f in expr.fields() {
        if !fields.iter().any(|g| g.eq_ignore_ascii_case(f)) {
            return Err(format!("unknown field \"{f}\""));
        }
    }
    Ok(())
}

/// The `field` of every column of the grid that owns `rule` (rule → column → grid).
fn grid_fields(doc: &Document, rule: &str) -> Vec<String> {
    let Some(grid) = doc
        .parent_of(rule)
        .and_then(|column| doc.parent_of(column))
        .and_then(|g| doc.node(g))
    else {
        return Vec::new();
    };
    grid.children
        .iter()
        .filter_map(|id| doc.node(id))
        .filter(|c| c.class == crawrad_ir::grid_columns::GRID_COLUMN_CLASS)
        .filter_map(|c| match c.props.get("field") {
            Some(Value::Str(f)) if !f.trim().is_empty() => Some(f.trim().to_string()),
            _ => None,
        })
        .collect()
}
