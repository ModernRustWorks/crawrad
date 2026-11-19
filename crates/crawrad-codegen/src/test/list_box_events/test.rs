//! The ListBox's three events on both targets: `OnClick` on any click,
//! `OnChange` only when the row is another one, `OnDblClick` on the second
//! click of a double click.

use crate::{Gerador, GeradorMakepad, GeradorRxp};
use crawrad_ir::{Document, Item, Node, Rect};

fn form_with_list(events: &[(&str, &str)]) -> Document {
    let mut doc = Document::new("form_main", "Form");
    {
        let form = doc.node_mut("form_main").expect("the form");
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("form_main");
    }
    let mut lst = Node::new("lst1", "ListBox");
    lst.set_name("list_box1");
    lst.set_rect(Rect::new(20.0, 20.0, 140.0, 150.0));
    lst.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("One"), Item::new("Two")]),
    );
    for (event, handler) in events {
        lst.events.insert((*event).into(), (*handler).into());
    }
    doc.add_child("form_main", lst).expect("add the list");
    doc
}

/// The code without whitespace, so a check reads the statements and not the
/// indentation they land at.
fn compact(code: &str) -> String {
    code.split_whitespace().collect()
}

fn designer(g: impl Gerador, doc: &Document) -> String {
    g.gerar(doc, "app", &crawrad_registry::Registry::builtin())
        .expect("codegen succeeds")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().ends_with(".designer.rs"))
        .expect("the designer")
        .conteudo
}

#[test]
fn the_catalog_declares_the_double_click() {
    let r = crawrad_registry::Registry::builtin();
    let events: Vec<&str> = r
        .get("ListBox")
        .expect("ListBox")
        .events
        .iter()
        .map(|e| e.name)
        .collect();
    for e in ["on_change", "on_click", "on_dbl_click"] {
        assert!(events.contains(&e), "{e} missing: {events:?}");
    }
}

#[test]
fn makepad_calls_the_double_click_and_changes_only_on_another_row() {
    let doc = form_with_list(&[
        ("on_click", "ListBox1Click"),
        ("on_change", "ListBox1Change"),
        ("on_dbl_click", "ListBox1DblClick"),
    ]);
    let code = designer(GeradorMakepad, &doc);
    assert!(
        code.contains("self.ui.crawrad_list_box(id!(list_box1)).double_clicked(actions).is_some()"),
        "{code}"
    );
    assert!(code.contains("self.form.ListBox1DblClick();"), "{code}");
    assert!(code.contains("self.form.ListBox1Click();"), "{code}");
    assert!(
        code.contains("let changed = self.form.list_box1.selected != Some(_val);"),
        "{code}"
    );
    assert!(
        compact(&code).contains("ifchanged{self.form.ListBox1Change();}"),
        "OnChange must wait for another row:\n{code}"
    );
}

#[test]
fn egui_calls_the_click_and_the_double_click() {
    let doc = form_with_list(&[
        ("on_click", "ListBox1Click"),
        ("on_dbl_click", "ListBox1DblClick"),
    ]);
    let code = designer(GeradorRxp, &doc);
    assert!(code.contains("double |= row.double_clicked();"), "{code}");
    assert!(
        compact(&code).contains("ifclicked{self.ListBox1Click();}"),
        "{code}"
    );
    assert!(
        compact(&code).contains("ifdouble{self.ListBox1DblClick();}"),
        "{code}"
    );
}

#[test]
fn without_handlers_nothing_is_called() {
    let doc = form_with_list(&[]);
    let mp = designer(GeradorMakepad, &doc);
    assert!(!mp.contains("double_clicked(actions)"), "{mp}");
    let eg = designer(GeradorRxp, &doc);
    assert!(
        eg.contains("let _ = clicked;") && eg.contains("let _ = double;"),
        "{eg}"
    );
}

/// `MultiSelect`: both targets hand the click and its keys to the same
/// rule, `crawrad_runtime::list_selection`, and mark the rows of `selection`.
#[test]
fn multi_select_goes_through_the_same_rule_on_both_targets() {
    let mut doc = form_with_list(&[]);
    doc.node_mut("lst1")
        .expect("the list")
        .props
        .insert("multi_select".into(), crawrad_ir::Value::Bool(true));
    let makepad = compact(&designer(GeradorMakepad, &doc));
    let egui = compact(&designer(GeradorRxp, &doc));
    for (target, code) in [("makepad", &makepad), ("egui", &egui)] {
        assert!(
            code.contains("multi_select:true"),
            "[{target}] the property is lost:
{code}"
        );
        assert!(
            code.contains("crawrad_runtime::list_selection::click("),
            "[{target}] no click rule:
{code}"
        );
        assert!(
            code.contains("crawrad_runtime::list_selection::settle("),
            "[{target}] no settle:
{code}"
        );
    }
    assert!(
        makepad.contains(".keys(actions).unwrap_or_default()"),
        "{makepad}"
    );
    assert!(
        makepad.contains("list_box.set_selection(cx,&self.form.list_box1.selection);"),
        "{makepad}"
    );
    assert!(
        egui.contains("selectable(self.list_box1.selection.contains(&i)"),
        "{egui}"
    );
    assert!(egui.contains("keys.command,keys.shift"), "{egui}");
}

/// `Sorted`: both targets sort through the same rule before drawing,
/// and the property reaches the state.
#[test]
fn sorted_goes_through_the_same_rule_on_both_targets() {
    let mut doc = form_with_list(&[]);
    doc.node_mut("lst1")
        .expect("the list")
        .props
        .insert("sorted".into(), crawrad_ir::Value::Bool(true));
    for (target, code) in [
        ("makepad", compact(&designer(GeradorMakepad, &doc))),
        ("egui", compact(&designer(GeradorRxp, &doc))),
    ] {
        assert!(
            code.contains("sorted:true"),
            "[{target}] the property is lost:\n{code}"
        );
        assert!(
            code.contains("crawrad_runtime::list_selection::sort("),
            "[{target}] no sort:\n{code}"
        );
    }
}
