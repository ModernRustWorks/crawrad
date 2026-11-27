//! What `form.read` answers about the open form: its tree as JSON, every node
//! with its class, rect, children, properties and events. Pure over the
//! document, so it lives here; the shell adds what only it knows (the view in
//! front, whether the form is dirty).

use std::collections::HashMap;

use crawrad_note::note;
use serde_json::{Value, json};

#[note("form-json-form")]
pub fn form(doc: &crawrad_doc::Doc) -> (Value, usize) {
    let ir = doc.ir();
    let mut parent_map = HashMap::new();
    for (pid, pnode) in &ir.nodes {
        for cid in &pnode.children {
            parent_map.insert(cid.clone(), pid.clone());
        }
    }
    let rect = |n: &crawrad_ir::Node| {
        n.rect()
            .map(|r| json!({ "x": r.x, "y": r.y, "w": r.w, "h": r.h }))
    };
    let props = |n: &crawrad_ir::Node| -> HashMap<String, Value> {
        n.props
            .iter()
            .map(|(k, v)| {
                let jv = serde_json::to_value(v).unwrap_or_else(|_| json!(format!("{v:?}")));
                (k.clone(), jv)
            })
            .collect()
    };
    let root = ir.node(&ir.root).map(|r| {
        json!({
            "id": r.id,
            "name": r.name(),
            "class": r.class,
            "rect": rect(r),
            "children": r.children,
            "properties": props(r),
            "events": r.events,
        })
    });
    let nodes: Vec<Value> = ir
        .nodes
        .iter()
        .filter(|(id, _)| **id != ir.root)
        .map(|(id, node)| {
            json!({
                "id": id,
                "name": node.name(),
                "class": node.class,
                "parent": parent_map.get(id),
                "children": node.children,
                "rect": rect(node),
                "properties": props(node),
                "events": node.events,
            })
        })
        .collect();
    let count = nodes.len();
    let form = json!({
        "revision": doc.revision(),
        "selection": doc.selection(),
        "root": root,
        "nodes": nodes,
    });
    (form, count)
}

fn rect(n: &crawrad_ir::Node) -> Option<Value> {
    n.rect()
        .map(|r| json!({ "x": r.x, "y": r.y, "w": r.w, "h": r.h }))
}

#[note("form-json-components")]
pub fn components(doc: &crawrad_doc::Doc) -> Vec<Value> {
    let ir = doc.ir();
    let parents: HashMap<_, _> = ir
        .nodes
        .iter()
        .flat_map(|(pid, p)| p.children.iter().map(move |c| (c.clone(), pid.clone())))
        .collect();
    ir.nodes
        .iter()
        .filter(|(id, _)| **id != ir.root)
        .map(|(id, node)| {
            json!({ "id": id, "name": node.name(), "class": node.class,
                    "parent": parents.get(id), "rect": rect(node) })
        })
        .collect()
}

#[note("form-json-selection")]
pub fn selection(doc: &crawrad_doc::Doc) -> Value {
    let ir = doc.ir();
    let nodes: Vec<Value> = doc
        .selection()
        .iter()
        .filter_map(|id| ir.node(id).map(|n| (id, n)))
        .map(|(id, n)| json!({ "id": id, "name": n.name(), "class": n.class, "rect": rect(n) }))
        .collect();
    let count = doc.selection().len();
    json!({ "selection_ids": doc.selection(), "selected_components": nodes, "count": count })
}

#[note("form-json-component")]
pub fn component(doc: &crawrad_doc::Doc, id: &str) -> Option<(Value, String)> {
    let ir = doc.ir();
    let node = ir
        .node(id)
        .or_else(|| ir.nodes.values().find(|n| n.name() == id))?;
    let props: HashMap<&String, Value> = node
        .props
        .iter()
        .map(|(k, v)| {
            (
                k,
                serde_json::to_value(v).unwrap_or_else(|_| json!(format!("{v:?}"))),
            )
        })
        .collect();
    let answer = json!({ "id": node.id, "name": node.name(), "class": node.class,
                         "rect": rect(node), "properties": props, "events": node.events,
                         "bindings": node.bindings });
    Some((answer, node.name().to_string()))
}

/// Builds the inspection result for `form.read`, combining document IR with active view state.
pub fn form_read_answer(
    doc: Option<&crawrad_doc::Doc>,
    center_view: &str,
    is_dirty: bool,
) -> Result<crate::protocol::ToolCallResult, crate::run_tool::Answer> {
    let Some(doc) = doc else {
        return Err(crate::designer_tool::no_form());
    };
    let (mut form_val, count) = form(doc);
    form_val["center_view"] = json!(center_view);
    form_val["is_dirty"] = json!(is_dirty);
    let said = format!(
        "Active form inspected (rev: {}, {count} child nodes)",
        doc.revision()
    );
    Ok(crate::protocol::ToolCallResult::success_json(
        form_val, said,
    ))
}

#[cfg(test)]
#[path = "test/form_json/test.rs"]
mod tests;
