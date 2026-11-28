//! The `designer.*` tools: what they ask of the form and what they answer.
//! Pure of the window, so they live here; the shell lends the open form and
//! runs the catalogue's actions through [`Designer`], and writes the audit line.

use serde_json::{Value, json};

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;
use crawrad_note::note;

#[note("designer-tool-designer")]
pub trait Designer {
    #[note("designer-tool-designer-doc")]
    fn doc(&self) -> Option<&crawrad_doc::Doc>;

    #[note("designer-tool-designer-doc-mut")]
    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc>;

    #[note("designer-tool-designer-run")]
    fn run(&mut self, name: &str, args: &Value) -> Result<(), String>;

    #[note("designer-tool-designer-prop-keys")]
    fn prop_keys(&self, class: &str) -> Vec<&'static str>;
}

#[note("designer-tool-no-form")]
pub fn no_form() -> Answer {
    let said = "No visual form document is currently open in the IDE designer.";
    (ToolCallResult::error_text("no-form", said), "error")
}

fn error(code: &str, said: impl Into<String>) -> Answer {
    (ToolCallResult::error_text(code, said.into()), "error")
}

fn missing(key: &str) -> Answer {
    error("invalid-params", format!("Missing '{key}' parameter"))
}

fn not_found(id: &str) -> Answer {
    let said = format!("Component '{id}' not found in active form.");
    error("component-not-found", said)
}

#[note("designer-tool-resolve")]
pub fn resolve(doc: &crawrad_doc::Doc, id: &str) -> Option<String> {
    let ir = doc.ir();
    ir.node(id)
        .or_else(|| ir.nodes.values().find(|n| n.name() == id))
        .map(|n| n.id.clone())
}

#[note("designer-tool-select")]
pub fn select(doc: Option<&crawrad_doc::Doc>, id: &str) -> Result<String, Answer> {
    let Some(doc) = doc else {
        return Err(no_form());
    };
    resolve(doc, id).ok_or_else(|| {
        let said = format!("Component '{id}' not found in active form.");
        (
            ToolCallResult::error_text("unknown-component", said),
            "error",
        )
    })
}

#[note("designer-tool-selected")]
pub fn selected(doc: &crawrad_doc::Doc, id: &str) -> Answer {
    let ir = doc.ir();
    let name = ir
        .node(id)
        .map(|n| n.name().to_string())
        .unwrap_or_default();
    let data = json!({
        "id": id,
        "name": name,
        "selection": [id],
        "status": "ok"
    });
    let said = format!("Selected component '{id}' ('{name}').");
    (ToolCallResult::success_json(data, said), "applied")
}

fn revision(d: &impl Designer) -> u64 {
    d.doc().map(|d| d.revision()).unwrap_or(0)
}

#[note("designer-tool-list-components")]
pub fn list_components(doc: Option<&crawrad_doc::Doc>) -> Answer {
    let Some(doc) = doc else {
        return no_form();
    };
    let components = crate::form_json::components(doc);
    let said = format!("Listed {} components in active form.", components.len());
    let count = components.len();
    let data = json!({ "components": components, "count": count });
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("designer-tool-get-selection")]
pub fn get_selection(doc: Option<&crawrad_doc::Doc>) -> Answer {
    let Some(doc) = doc else {
        return no_form();
    };
    let found = crate::form_json::selection(doc);
    let said = format!("{} components currently selected.", found["count"]);
    (ToolCallResult::success_json(found, said), "applied")
}

#[note("designer-tool-get-component-properties")]
pub fn get_component_properties(doc: Option<&crawrad_doc::Doc>, args: &Value) -> Answer {
    let Some(id) = crate::cargo_tool::nonempty(args, "id") else {
        // NAMING THE ID, not just demanding one: `designer.add_component`
        // answers "Added Grid component 'grid1'", so the caller has the value
        // and only has to guess the field it goes in. Saying `name` is wrong
        // and not saying what is right costs a round trip every time.
        return error(
            "invalid-params",
            "Missing 'id' parameter: the component's id, as `designer.add_component`              reported it (for example `grid1`) or as `designer.list_components` lists it",
        );
    };
    let Some(doc) = doc else {
        return no_form();
    };
    let Some((found, name)) = crate::form_json::component(doc, id) else {
        return not_found(id);
    };
    let said = format!("Properties retrieved for component '{name}'.");
    (ToolCallResult::success_json(found, said), "applied")
}

#[note("designer-tool-add-component")]
pub fn add_component(d: &mut impl Designer, args: &Value) -> Answer {
    let Some(class) = crate::cargo_tool::nonempty(args, "class") else {
        return error("invalid-params", "Missing or empty 'class' parameter");
    };
    let parent = args
        .get("parent")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty());
    let parent_id = match parent {
        Some(p) => {
            let Some(doc) = d.doc() else {
                return no_form();
            };
            let Some(id) = resolve(doc, p) else {
                let said = format!("Parent component '{p}' does not exist in active form.");
                return error("parent-not-found", said);
            };
            Some(id)
        }
        None => None,
    };
    let mut add = json!({ "class": class });
    if let Some(p) = &parent_id {
        add["parent"] = json!(p);
    }
    for key in ["x", "y", "w", "h"] {
        if let Some(v) = args.get(key) {
            add[key] = v.clone();
        }
    }
    if let Err(e) = d.run("component.add", &add) {
        return error("add-component-error", e);
    }
    let Some(new_id) = d.doc().and_then(|doc| doc.selection().first().cloned()) else {
        return error(
            "add-component-error",
            "Failed to retrieve newly added component ID",
        );
    };

    let mut name = new_id.clone();
    let custom = args
        .get("name")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty() && *s != new_id);
    if let Some(custom) = custom {
        let rename = json!({ "node": new_id.clone(), "name": custom });
        if let Err(e) = d.run("component.rename", &rename) {
            return error("rename-error", e);
        }
        name = custom.to_string();
    }
    if let Some(props) = args.get("properties").and_then(Value::as_object) {
        for (k, v) in props {
            let set = json!({ "nodes": [new_id.clone()], "key": k, "value": v });
            if let Err(e) = d.run("component.set_property", &set) {
                return error("set-property-error", e);
            }
        }
    }

    let rev = revision(d);
    let rect = d
        .doc()
        .and_then(|doc| doc.ir().node(&new_id))
        .and_then(|n| n.rect());
    let (rx, ry, rw, rh) = rect.map(|r| (r.x, r.y, r.w, r.h)).unwrap_or_default();
    let parent =
        parent_id.unwrap_or_else(|| d.doc().map(|doc| doc.ir().root.clone()).unwrap_or_default());
    let data = json!({
        "id": new_id,
        "name": name,
        "class": class,
        "parent": parent,
        "rect": { "x": rx, "y": ry, "w": rw, "h": rh },
        "revision": rev,
        "status": "ok"
    });
    let said = format!("Added {class} component '{new_id}' (rev {rev}).");
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("designer-tool-set-component-properties")]
pub fn set_component_properties(d: &mut impl Designer, args: &Value) -> Answer {
    let Some(id) = args.get("id").and_then(Value::as_str) else {
        return missing("id");
    };
    let Some(props) = args.get("properties").and_then(Value::as_object) else {
        return error("invalid-params", "Missing 'properties' object");
    };
    let Some(doc) = d.doc() else {
        return no_form();
    };
    let Some(node_id) = resolve(doc, id) else {
        return not_found(id);
    };
    let class = doc
        .ir()
        .node(&node_id)
        .map(|n| n.class.clone())
        .unwrap_or_default();
    let keys = d.prop_keys(&class);
    let mut out = crate::properties::Outcome::default();
    for (k, v) in props {
        if let Some(why) = crate::properties::unknown_key(&class, &keys, k) {
            out.refuse(k, why);
            continue;
        }
        let set = json!({ "nodes": [node_id.clone()], "key": k, "value": v });
        match d.run("component.set_property", &set) {
            Ok(()) => out.apply(k),
            Err(e) => out.refuse(k, e),
        }
    }
    let (data, said) = out.answer(&node_id, props.len(), revision(d));
    if out.failed() {
        return error("property-refused", said);
    }
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("designer-tool-delete-component")]
pub fn delete_component(d: &mut impl Designer, args: &Value) -> Answer {
    let Some(id) = args.get("id").and_then(Value::as_str) else {
        return missing("id");
    };
    let Some(doc) = d.doc() else {
        return no_form();
    };
    let Some(node_id) = resolve(doc, id) else {
        return not_found(id);
    };
    if node_id == doc.ir().root {
        return error(
            "cannot-delete-root",
            "Cannot delete the root form component.",
        );
    }
    if let Some(doc) = d.doc_mut() {
        doc.select_one(&node_id);
    }
    if let Err(e) = d.run("edit.delete", &json!({})) {
        return error("apply-error", e);
    }
    let rev = revision(d);
    let data = json!({ "deleted_id": node_id, "revision": rev, "status": "ok" });
    let said = format!("Deleted component '{node_id}' from form (rev {rev}).");
    (ToolCallResult::success_json(data, said), "applied")
}

#[note("designer-tool-wired")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wired {
    /// The component's id.
    pub node: String,
    /// The component's name, as the user reads it.
    pub name: String,
    /// The event bound.
    pub event: String,
    /// The handler it calls: the one asked for, or the default name.
    pub handler: String,
}

#[note("designer-tool-wire-event")]
pub fn wire_event(d: &mut impl Designer, args: &Value) -> Result<Wired, Answer> {
    let Some(id) = args.get("id").and_then(Value::as_str) else {
        return Err(missing("id"));
    };
    let Some(event) = args.get("event").and_then(Value::as_str) else {
        return Err(missing("event"));
    };
    let Some(doc) = d.doc() else {
        return Err(no_form());
    };
    let Some(node) = resolve(doc, id) else {
        return Err(not_found(id));
    };
    let name = doc
        .ir()
        .node(&node)
        .map(|n| n.name().to_string())
        .unwrap_or_default();
    let handler = match args.get("handler").and_then(Value::as_str) {
        Some(h) => h.to_string(),
        None => crawrad_ir::default_handler_name(&name, event),
    };
    let bind = json!({ "node": node.clone(), "event": event, "handler": handler.clone() });
    d.run("event.bind", &bind)
        .map_err(|e| error("apply-error", e))?;
    let event = event.to_string();
    Ok(Wired {
        node,
        name,
        event,
        handler,
    })
}

#[note("designer-tool-wired-2")]
pub fn wired(
    w: &Wired,
    rev: u64,
    file: Option<String>,
    line: Option<usize>,
    generated: bool,
    note: Option<String>,
) -> Answer {
    let data = json!({
        "id": w.node,
        "event": w.event,
        "handler": w.handler,
        "stub_generated": generated,
        "file": file,
        "line": line,
        "note": note,
        "revision": rev,
        "status": "ok"
    });
    let (event, handler, name) = (&w.event, &w.handler, &w.name);
    let said = format!("Wired event '{event}' to handler '{handler}' on '{name}' (rev {rev}).");
    (ToolCallResult::success_json(data, said), "applied")
}

/// Parses the required `size` argument for `designer.set_grid_size`.
pub fn set_grid_size_arg(args: &Value) -> Result<f32, ToolCallResult> {
    let Some(size) = args.get("size").and_then(|v| v.as_f64()) else {
        return Err(ToolCallResult::error_text(
            "bad-args",
            "designer.set_grid_size needs `size`",
        ));
    };
    Ok(size as f32)
}

/// Parses the required `code` argument for `designer.set_language`.
pub fn set_language_arg(args: &Value) -> Result<&str, ToolCallResult> {
    let Some(code) = args.get("code").and_then(|v| v.as_str()) else {
        return Err(ToolCallResult::error_text(
            "bad-args",
            "designer.set_language needs `code`",
        ));
    };
    Ok(code)
}

/// Builds the error response when an unknown language code is requested in `designer.set_language`.
pub fn unknown_language_error(code: &str, valids: Vec<String>) -> ToolCallResult {
    let mut list = vec!["en".to_string()];
    list.extend(valids);
    let valid_list = list.join(", ");
    ToolCallResult::error_text(
        "unknown-language",
        format!("Unknown language '{code}'. Valid languages: {valid_list}"),
    )
}

/// Dispatches inspector and selection read queries on the active form.
pub fn dispatch_inspect(
    name: &str,
    doc: Option<&crawrad_doc::Doc>,
    args: &Value,
) -> Option<Answer> {
    match name {
        "designer.list_components" => Some(list_components(doc)),
        "designer.get_selection" => Some(get_selection(doc)),
        "designer.get_component_properties" => Some(get_component_properties(doc, args)),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/designer_tool/test.rs"]
mod tests;
