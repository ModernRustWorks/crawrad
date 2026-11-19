//! The `designer.*` tools on a fake IDE: what they ask the catalogue to run,
//! and what they answer.

use super::{
    Designer, add_component, delete_component, select, selected, set_component_properties,
    wire_event,
};
use crawrad_ir::{Document, Node};
use serde_json::{Value, json};

/// A form and the actions run on it; `component.add` adds a Button1 and
/// selects it, as the catalogue does.
struct Fake {
    doc: Option<crawrad_doc::Doc>,
    ran: Vec<(String, Value)>,
}

impl Fake {
    fn with_button() -> Self {
        let mut d = Document::new("form_main", "Form");
        d.add_child("form_main", Node::new("Button1", "Button"))
            .expect("a child");
        let doc = Some(crawrad_doc::Doc::new(d));
        Self {
            doc,
            ran: Vec::new(),
        }
    }

    fn names(&self) -> Vec<&str> {
        self.ran.iter().map(|(n, _)| n.as_str()).collect()
    }
}

impl Designer for Fake {
    fn doc(&self) -> Option<&crawrad_doc::Doc> {
        self.doc.as_ref()
    }

    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> {
        self.doc.as_mut()
    }

    fn run(&mut self, name: &str, args: &Value) -> Result<(), String> {
        self.ran.push((name.to_string(), args.clone()));
        if let (Some(doc), "component.add") = (&self.doc, name) {
            let mut ir = doc.ir().clone();
            ir.add_child("form_main", Node::new("Button2", "Button"))
                .expect("a child");
            let mut doc = crawrad_doc::Doc::new(ir);
            doc.select_one("Button2");
            self.doc = Some(doc);
        }
        Ok(())
    }

    fn prop_keys(&self, _class: &str) -> Vec<&'static str> {
        vec!["caption"]
    }
}

fn code(answer: &(crate::protocol::ToolCallResult, &str)) -> Value {
    answer
        .0
        .data
        .as_ref()
        .map(|d| d["code"].clone())
        .unwrap_or_default()
}

#[test]
fn with_no_form_every_change_says_so_and_runs_nothing() {
    let mut f = Fake {
        doc: None,
        ran: Vec::new(),
    };
    let id = json!({ "id": "Button1", "event": "clicked", "properties": {} });
    assert_eq!(code(&delete_component(&mut f, &id)), json!("no-form"));
    assert_eq!(
        code(&set_component_properties(&mut f, &id)),
        json!("no-form")
    );
    assert_eq!(
        code(&wire_event(&mut f, &id).unwrap_err()),
        json!("no-form")
    );
    assert!(f.ran.is_empty());
}

#[test]
fn delete_selects_by_name_and_never_takes_the_form() {
    let mut f = Fake::with_button();
    let a = delete_component(&mut f, &json!({ "id": "form_main" }));
    assert_eq!(code(&a), json!("cannot-delete-root"));
    let a = delete_component(&mut f, &json!({ "id": "Button1" }));
    assert_eq!(a.1, "applied");
    assert_eq!(f.names(), ["edit.delete"]);
    let selected = f.doc.as_ref().map(|d| d.selection().to_vec());
    assert_eq!(selected, Some(vec!["Button1".to_string()]));
}

#[test]
fn an_unknown_key_is_refused_without_running_and_a_known_one_runs() {
    let mut f = Fake::with_button();
    let args = json!({ "id": "Button1", "properties": { "caption": "OK", "colour": 1 } });
    let a = set_component_properties(&mut f, &args);
    assert_eq!(
        a.1, "applied",
        "one key applied: the call answers key by key"
    );
    assert_eq!(f.names(), ["component.set_property"]);
    assert_eq!(f.ran[0].1["key"], "caption");
    let only_unknown = json!({ "id": "Button1", "properties": { "colour": 1 } });
    let a = set_component_properties(&mut f, &only_unknown);
    assert_eq!(code(&a), json!("property-refused"));
    assert_eq!(f.ran.len(), 1);
}

#[test]
fn wire_binds_the_default_handler_name() {
    let mut f = Fake::with_button();
    let w = wire_event(&mut f, &json!({ "id": "Button1", "event": "clicked" })).expect("bound");
    assert_eq!(
        w.handler,
        crawrad_ir::default_handler_name("Button1", "clicked")
    );
    assert_eq!(f.names(), ["event.bind"]);
    assert_eq!(f.ran[0].1["handler"], json!(w.handler));
}

#[test]
fn add_passes_the_geometry_then_renames_and_sets() {
    let mut f = Fake::with_button();
    let args = json!({ "class": "Button", "parent": "form_main", "x": 8, "name": "Ok",
        "properties": { "caption": "OK" } });
    let a = add_component(&mut f, &args);
    assert_eq!(a.1, "applied");
    assert_eq!(
        f.names(),
        [
            "component.add",
            "component.rename",
            "component.set_property"
        ]
    );
    assert_eq!(
        f.ran[0].1,
        json!({ "class": "Button", "parent": "form_main", "x": 8 })
    );
    let data = a.0.data.expect("the answer");
    assert_eq!(
        (data["id"].clone(), data["name"].clone()),
        (json!("Button2"), json!("Ok"))
    );
    let a = add_component(&mut f, &json!({ "class": "Button", "parent": "Nope" }));
    assert_eq!(code(&a), json!("parent-not-found"));
}

#[test]
fn select_finds_component_and_formats_selection() {
    let f = Fake::with_button();
    let doc = f.doc.as_ref().unwrap();

    // Unknown component refuses
    let missing = select(Some(doc), "MissingBtn").unwrap_err();
    assert_eq!(missing.0.data.unwrap()["code"], json!("unknown-component"));

    // No form open refuses
    let no_form = select(None, "Button1").unwrap_err();
    assert_eq!(no_form.0.data.unwrap()["code"], json!("no-form"));

    // Known component resolves by id or name
    let found = select(Some(doc), "Button1").unwrap();
    assert_eq!(found, "Button1");

    let (res, outcome) = selected(doc, &found);
    assert_eq!(outcome, "applied");
    assert_eq!(res.data.unwrap()["selection"], json!(["Button1"]));
}
