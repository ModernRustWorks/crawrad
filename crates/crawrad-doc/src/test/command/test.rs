use super::*;
use crawrad_ir::Node;

fn com_painel() -> Document {
    let mut d = Document::new("form", "Form");
    d.node_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut p = Node::new("pnl", "Panel");
    p.set_rect(Rect::new(20.0, 30.0, 300.0, 120.0));
    d.add_child("form", p).unwrap();
    let mut b = Node::new("btn", "Button");
    b.set_rect(Rect::new(50.0, 60.0, 96.0, 32.0));
    d.add_child("form", b).unwrap();
    d
}

#[test]
fn mover_para_dentro_troca_pai_e_posicao_de_uma_vez() {
    let mut d = com_painel();
    // the button is at (50,60) in the form; inside the panel (20,30)
    // that is (30,30).
    let inverso = Command::Reparent {
        node: "btn".into(),
        parent: "pnl".into(),
        index: 0,
        rect: Rect::new(30.0, 30.0, 96.0, 32.0),
    }
    .apply(&mut d)
    .unwrap();

    assert_eq!(d.parent_of("btn").map(|s| s.as_str()), Some("pnl"));
    assert_eq!(d.node("btn").unwrap().rect().unwrap().x, 30.0);
    // and in the form it stays where it was: that is the point of the single command.
    assert_eq!(
        d.rect_in_form("btn"),
        Some(Rect::new(50.0, 60.0, 96.0, 32.0))
    );

    // the inverse restores parent AND position
    inverso.apply(&mut d).unwrap();
    assert_eq!(d.parent_of("btn").map(|s| s.as_str()), Some("form"));
    assert_eq!(
        d.node("btn").unwrap().rect(),
        Some(Rect::new(50.0, 60.0, 96.0, 32.0))
    );
}

#[test]
fn um_no_nao_pode_virar_filho_da_propria_descendencia() {
    // The tree would cease to be a tree, and `validate` would only notice
    // afterwards — with the document already corrupted.
    let mut d = com_painel();
    let r = Command::Reparent {
        node: "pnl".into(),
        parent: "pnl".into(),
        index: 0,
        rect: Rect::new(0.0, 0.0, 10.0, 10.0),
    }
    .apply(&mut d);
    assert!(r.is_err());
    d.validate().unwrap();
}

#[test]
fn set_binding_apply_and_undo() {
    use crawrad_ir::{BindingDef, BindingMode};

    let mut d = com_painel();
    let binding = BindingDef::new("ds_clientes", BindingMode::TwoWay).with_field("NOME");

    // 1. Apply binding
    let cmd = Command::SetBinding {
        node: "btn".into(),
        prop: "text".into(),
        binding: Some(binding.clone()),
    };
    let inverso = cmd.apply(&mut d).unwrap();

    assert_eq!(d.node("btn").unwrap().binding("text"), Some(&binding));

    // 2. Undo (inverse)
    inverso.apply(&mut d).unwrap();
    assert_eq!(d.node("btn").unwrap().binding("text"), None);
}

#[test]
fn rename_component_renames_derived_event_handler_and_undo_restores() {
    let mut d = com_painel();
    d.node_mut("btn").unwrap().set_name("btn_ok");
    d.node_mut("btn")
        .unwrap()
        .events
        .insert("on_click".into(), "btn_ok_click".into());

    let inverse = Command::Rename {
        node: "btn".into(),
        name: "btn_save".into(),
        handlers: true,
    }
    .apply(&mut d)
    .unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_save");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn_save_click")
    );

    inverse.apply(&mut d).unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_ok");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn_ok_click")
    );
}

#[test]
fn rename_without_handlers_keeps_default_handler_and_undo_restores_name() {
    let mut d = com_painel();
    d.node_mut("btn").unwrap().set_name("btn1");
    d.node_mut("btn")
        .unwrap()
        .events
        .insert("on_click".into(), "btn1_click".into());

    let inverse = Command::Rename {
        node: "btn".into(),
        name: "btn2".into(),
        handlers: false,
    }
    .apply(&mut d)
    .unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn2");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );

    inverse.apply(&mut d).unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn1");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );
}

#[test]
fn rename_component_renames_legacy_event_handler() {
    let mut d = com_painel();
    d.node_mut("btn").unwrap().set_name("btn_ok");
    d.node_mut("btn")
        .unwrap()
        .events
        .insert("on_click".into(), "btn_okClick".into());

    let inverse = Command::Rename {
        node: "btn".into(),
        name: "btn_save".into(),
        handlers: true,
    }
    .apply(&mut d)
    .unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_save");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn_saveClick")
    );

    inverse.apply(&mut d).unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_ok");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("btn_okClick")
    );
}

#[test]
fn rename_component_preserves_custom_event_handler() {
    let mut d = com_painel();
    d.node_mut("btn").unwrap().set_name("btn_ok");
    d.node_mut("btn")
        .unwrap()
        .events
        .insert("on_click".into(), "do_it".into());

    let inverse = Command::Rename {
        node: "btn".into(),
        name: "btn_save".into(),
        handlers: true,
    }
    .apply(&mut d)
    .unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_save");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("do_it")
    );

    inverse.apply(&mut d).unwrap();

    let node = d.node("btn").unwrap();
    assert_eq!(node.name(), "btn_ok");
    assert_eq!(
        node.events.get("on_click").map(String::as_str),
        Some("do_it")
    );
}
