//! The members offered after `self.<component>.`, checked against what the
//! generators really write.

use super::*;
use crawrad_codegen::{Gerador, GeradorMakepad, GeradorRxp, SUFIXO_GERADO};
use crawrad_ir::{Node, Rect};

/// A form with a Button and an Edit, as the designer would build it.
fn form() -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name("form_main");
    }
    let mut btn = Node::new("btn1", "Button");
    btn.set_name("btn_ok");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    d.add_child("form_main", btn).unwrap();
    let mut edt = Node::new("edt1", "Edit");
    edt.set_name("edt_name");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    d.add_child("form_main", edt).unwrap();
    d
}

/// The `.designer.rs` a generator writes for [`form`].
fn designer(generator: &dyn Gerador) -> String {
    generator
        .gerar(&form(), "probe", &Registry::builtin())
        .unwrap()
        .into_iter()
        .find(|f| f.caminho.to_string_lossy().ends_with(SUFIXO_GERADO))
        .expect("a .designer.rs")
        .conteudo
}

fn labels(members: &[Member]) -> Vec<&str> {
    members.iter().map(|m| m.label.as_str()).collect()
}

/// **On a Makepad project, `self.btn_ok.` offers the Button's generated
/// fields — and not one the code does not have.** The hand list this replaces
/// offered `hint`, `rect`, `anchors`, `align`, `margins`: none of them is a
/// field, and accepting one broke the build.
#[test]
fn makepad_button_offers_exactly_its_generated_fields() {
    let source = designer(&GeradorMakepad);
    assert_eq!(field_class(&source, "btn_ok").as_deref(), Some("Button"));
    let reg = Registry::builtin();
    let found = members(&reg, Some(&source), "Button", "");
    // `hint` and `show_hint` are real fields (the tooltip reads
    // them); the font and colour pairs are fields too
    assert_eq!(
        labels(&found),
        [
            "visible",
            "enabled",
            "text",
            "hint",
            "show_hint",
            // the pointer over the button
            "cursor",
            // the PopupMenu a secondary click shows
            "popup_menu",
            "font",
            "parent_font",
            "color",
            "parent_color"
        ]
    );
    // The Tab sequence is the form's, so neither key is a field
    for ghost in [
        "rect",
        "anchors",
        "align",
        "margins",
        "tab_order",
        "tab_stop",
    ] {
        assert!(!labels(&found).contains(&ghost), "{ghost} is not a field");
    }
}

/// **The fields are read from the project's own `.designer.rs`**, so the
/// answer follows the project, not a guess about it. The egui
/// state comes from the catalogue too, in its order. (Previously the Makepad Button
/// also carried a `tab_order` field; the Tab sequence is the form's now.)
#[test]
fn the_fields_follow_the_projects_target() {
    let source = designer(&GeradorRxp);
    let reg = Registry::builtin();
    let found = members(&reg, Some(&source), "Button", "");
    assert_eq!(
        labels(&found),
        [
            "enabled",
            "visible",
            "hint",
            "show_hint",
            // the pointer over the button
            "cursor",
            "popup_menu",
            "text",
            "font",
            "parent_font",
            "color",
            "parent_color"
        ]
    );
    assert_eq!(
        class_fields(Some(&source), "Button").unwrap().1,
        FieldSource::Designer
    );
}

/// **The class comes from the form, never from the name.** `btn_ok` is not
/// guessed to be a Button: a field the form does not declare has no class.
#[test]
fn the_class_comes_from_the_form_not_the_name() {
    let source = designer(&GeradorMakepad);
    assert_eq!(field_class(&source, "edt_name").as_deref(), Some("Edit"));
    assert_eq!(field_class(&source, "btn_undeclared"), None);
    assert_eq!(
        field_class(&source, "text"),
        None,
        "a component's own field"
    );
    let live = form();
    assert_eq!(class_in_form(&live, "btn_ok"), Some("Button"));
    assert_eq!(class_in_form(&live, "btn_undeclared"), None);
}

/// **A class just dropped on the form, not generated yet, still gets its
/// fields — from the same generator, and it says so.**
#[test]
fn a_class_not_generated_yet_falls_back_to_the_generator() {
    let source = designer(&GeradorMakepad);
    assert!(
        struct_fields(&source, "CheckBox").is_none(),
        "not on this form"
    );
    let (fields, from) = class_fields(Some(&source), "CheckBox").unwrap();
    assert_eq!(from, FieldSource::Generator);
    assert!(fields.iter().any(|f| f.name == "checked" && f.ty == "bool"));
}

/// **Every class the Makepad generator emits answers with the fields it
/// emits.** A class whose struct the parser could not read would offer
/// nothing in silence; this names it.
#[test]
fn every_generated_class_is_readable() {
    let mut unreadable = Vec::new();
    for class in crawrad_codegen::targets::makepad::components::classes() {
        let def = crawrad_codegen::targets::makepad::components::for_class(class)
            .unwrap()
            .struct_definition();
        if def.trim().is_empty() {
            continue;
        }
        // The text may carry helper structs too (`ImageItem` with
        // `ImageList`): every field line must be read by some struct.
        let field_lines = def
            .lines()
            .filter(|l| l.starts_with("    pub ") && l.trim_end().ends_with(','))
            .count();
        let read: usize = def
            .lines()
            .filter_map(|l| l.trim().strip_prefix("pub struct "))
            .filter_map(|l| l.strip_suffix(" {"))
            .map(|name| struct_fields(&def, name).map_or(0, |f| f.len()))
            .sum();
        let own = struct_fields(&def, class).map_or(0, |f| f.len());
        if own == 0 || read != field_lines {
            unreadable.push(format!("{class}: own {own}, read {read} of {field_lines}"));
        }
    }
    assert!(unreadable.is_empty(), "{unreadable:#?}");
}

/// **The documentation is the Object Inspector's: `text` is the Caption.**
#[test]
fn a_field_is_documented_as_the_inspector_names_it() {
    let source = designer(&GeradorMakepad);
    let reg = Registry::builtin();
    let found = members(&reg, Some(&source), "Button", "te");
    assert_eq!(labels(&found), ["text"]);
    assert_eq!(found[0].detail, "String");
    let doc = found[0].documentation.as_deref().unwrap_or_default();
    assert!(doc.starts_with("Caption · Appearance"), "{doc}");
}

/// **The parser reads CRLF, docs above a field, and stops at the struct's
/// end — a field of the NEXT struct is not taken.**
#[test]
fn struct_fields_reads_crlf_docs_and_stops_at_the_end() {
    let source = "pub struct A {\r\n    /// Shown or not.\r\n    pub visible: bool,\r\n    pub items: Option<usize>,\r\n}\r\n\r\npub struct B {\r\n    pub other: i64,\r\n}\r\n";
    let fields = struct_fields(source, "A").unwrap();
    assert_eq!(
        fields,
        vec![
            Field {
                name: "visible".into(),
                ty: "bool".into(),
                doc: Some("Shown or not.".into())
            },
            Field {
                name: "items".into(),
                ty: "Option<usize>".into(),
                doc: None
            },
        ]
    );
    assert!(struct_fields(source, "C").is_none());
}
