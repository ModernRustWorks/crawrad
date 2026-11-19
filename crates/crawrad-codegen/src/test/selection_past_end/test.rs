//! A handler that shrinks a list leaves no selection past its end.
//!
//! Every class the catalog gives both `items` and `item_index` is a list with a
//! selected row, and both targets must put that row back to none once the
//! collection ends before it. The classes come from the catalog, so a list
//! added tomorrow is held to the same rule without anyone remembering it.

use crate::{Gerador, GeradorMakepad, GeradorRxp};
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

fn lists(r: &Registry) -> Vec<&'static str> {
    r.all()
        .filter(|d| {
            let has = |k: &str| d.props.iter().any(|p| p.key == k);
            has("items") && has("item_index")
        })
        .map(|d| d.class)
        .collect()
}

fn form_with(r: &Registry, class: &str) -> (Document, String) {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the form root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
    }
    let id = r.suggest_id(class, |i| d.node(i).is_some());
    let (w, h) = r.get(class).expect("a catalog class").default_size;
    let n = r
        .instantiate(class, id.clone(), Rect::new(8.0, 8.0, w, h))
        .expect("instantiate");
    let name = n.name().to_string();
    d.add_child("form_main", n).expect("add child");
    d.validate().expect("document is valid");
    (d, name)
}

fn designer(g: impl Gerador, doc: &Document) -> String {
    g.gerar(doc, "app", &Registry::builtin())
        .expect("codegen succeeds")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("designer source exists")
        .conteudo
}

#[test]
fn the_catalog_has_the_lists_this_rule_was_written_for() {
    let r = Registry::builtin();
    let found = lists(&r);
    for class in [
        "ListBox",
        "ComboBox",
        "Chips",
        "RadioGroup",
        "ListView",
        "Breadcrumb",
    ] {
        assert!(
            found.contains(&class),
            "{class} is no longer a list: {found:?}"
        );
    }
}

#[test]
fn every_list_forgets_a_selection_past_its_end_on_both_targets() {
    let r = Registry::builtin();
    for class in lists(&r) {
        let (doc, name) = form_with(&r, class);
        for (target, source, owner) in [
            ("egui", designer(GeradorRxp, &doc), format!("self.{name}")),
            (
                "makepad",
                designer(GeradorMakepad, &doc),
                format!("self.form.{name}"),
            ),
        ] {
            let optional = format!("if {owner}.selected.is_some_and(|i| i >= {owner}.items.len())");
            let signed = format!("if {owner}.item_index >= {owner}.items.len() as i64");
            assert!(
                source.contains(&optional) || source.contains(&signed),
                "{class} on {target} keeps a selection past the end of its list:\n{source}"
            );
        }
    }
}
