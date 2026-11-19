//! "Add all fields" of a grid's columns editor.
//!
//! One `GridColumn` per field of the grid's table, with the defaults the
//! field's type asks for: numbers line up on the right, a whole number is
//! narrow, a real number gets the thousands mask. Fields that already have a
//! column are left alone, so pressing it again adds only what is new.

use crate::{Action, Context, Effect, Need, Param, ParamKind, Place, ViewAffinity};
use crawrad_doc::{Command, Transaction};
use crawrad_ir::grid_columns::{GRID_COLUMN_CLASS, title_from_field};
use crawrad_ir::{Rect, Value};
use crawrad_note::note;

#[note("grid-columns-action-add-all-fields")]
pub const ACTION_ADD_ALL_FIELDS: &str = "collection.add_all_fields";

/// The catalogue entry of "Add all fields".
pub(crate) fn add_all_fields_action() -> Action {
    Action {
        id: ACTION_ADD_ALL_FIELDS,
        label: "Add all fields",
        hint: "Add one column per field of the grid's table",
        group: "Edit",
        place: Place::TOP,
        shortcut: None,
        params: &[
            Param {
                name: "node",
                kind: ParamKind::String,
                description: "the grid whose columns are added",
                list: false,
                required: true,
                domain: None,
            },
            Param {
                name: "fields",
                kind: ParamKind::String,
                description: "the table's field names, in order",
                list: true,
                required: true,
                domain: None,
            },
            Param {
                name: "kinds",
                kind: ParamKind::String,
                description: "each field's type (INTEGER, REAL, TEXT...), same order",
                list: true,
                required: false,
                domain: None,
            },
        ],
        view: ViewAffinity::Design,
        enabled: Need::Form.only(),
        execute: |c| {
            let Some(grid) = c.args.as_text("node") else {
                return Effect::none().with_status("add all fields: `node` is required");
            };
            add_all_fields(
                c,
                grid,
                &c.args.as_texts("fields"),
                &c.args.as_texts("kinds"),
            )
        },
    }
}

#[note("grid-columns-add-all-fields")]
pub fn add_all_fields(c: &Context, grid: &str, fields: &[&str], kinds: &[&str]) -> Effect {
    let Some(ir) = c.ir() else {
        return Effect::none();
    };
    let Some(node) = ir.node(grid) else {
        return Effect::none().with_status(format!("no such component: `{grid}`"));
    };
    let takes_columns = c
        .registry
        .get(&node.class)
        .is_some_and(|d| d.item_class() == Some(GRID_COLUMN_CLASS));
    if !takes_columns {
        return Effect::none().with_status(format!("`{}` has no grid columns", node.class));
    }
    if fields.is_empty() {
        return Effect::none().with_status("add all fields: the table has no fields");
    }
    let has_column = |field: &str| {
        node.children
            .iter()
            .filter_map(|id| ir.node(id))
            .any(|n| matches!(n.props.get("field"), Some(Value::Str(f)) if f == field))
    };
    let mut tx = Transaction::new("Add all fields", c.author.clone());
    let mut created: Vec<String> = Vec::new();
    let mut index = node.children.len();
    for (i, field) in fields.iter().enumerate() {
        if field.is_empty() || has_column(field) {
            continue;
        }
        // The columns made here are not in the document yet: `created` counts too.
        let id = c.registry.suggest_id(GRID_COLUMN_CLASS, |n| {
            ir.node(n).is_some() || ir.node_by_name(n).is_some() || created.iter().any(|k| k == n)
        });
        let (w, h) = crawrad_registry::ICON_SIZE;
        let Some(mut col) =
            c.registry
                .instantiate(GRID_COLUMN_CLASS, id.clone(), Rect::new(0.0, 0.0, w, h))
        else {
            return Effect::none().with_status("add all fields: no GridColumn in the catalogue");
        };
        col.props
            .insert("field".into(), Value::Str(field.to_string()));
        col.props
            .insert("title".into(), Value::Str(title_from_field(field)));
        if let Some(kind) = kinds.get(i).copied()
            && !kind.is_empty()
        {
            col.props
                .insert("field_type".into(), Value::Str(kind.to_string()));
        }
        match kinds.get(i).copied() {
            Some("INTEGER") => {
                col.props
                    .insert("alignment".into(), Value::Enum("taRightJustify".into()));
                col.props.insert("width".into(), Value::Int(70));
            }
            Some("REAL") => {
                col.props
                    .insert("alignment".into(), Value::Enum("taRightJustify".into()));
                col.props
                    .insert("display_format".into(), Value::Str("#,##0.00".into()));
            }
            Some("BOOLEAN") => {
                col.props
                    .insert("renderer".into(), Value::Enum("crCheckBox".into()));
            }
            _ => {}
        }
        tx.push(Command::AddNode {
            parent: grid.to_string(),
            node: col,
            index,
        });
        created.push(id);
        index += 1;
    }
    if created.is_empty() {
        return Effect::none().with_status("every field already has a column");
    }
    Effect::tx(tx)
}

#[cfg(test)]
#[path = "test/grid_columns/test.rs"]
mod tests;
