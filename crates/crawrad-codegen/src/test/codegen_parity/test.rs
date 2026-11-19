//! **Both code generators agree on component struct field names.**
//!
//! Egui and Makepad must not drift in the fields they emit for the
//! same component. If Egui emits `caption` and Makepad emits `text`, switching
//! targets or running on Makepad breaks the user's hand-written event code.
//!
//! Both frameworks (egui and makepad-widgets) use `text` in their native APIs,
//! and the catalogue property key is `text`. The rule of the house is:
//! "Here it is standard Rust, and from a classic RAD IDE only the IDEAS".
//!
//! This test derives from the emitted text of both generators without hard-coding
//! manual lists, ensuring that:
//! 1. Shared components have identical field sets across both targets.
//! 2. No component emits `caption` instead of `text`.

use crate::{Gerador, GeradorMakepad, GeradorRxp};
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use std::collections::{BTreeMap, BTreeSet};

fn form_with_all_components() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the form root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let mut y = 8.0;
    for desc in r.all() {
        if desc.class == "Form" || desc.class == "TabSheet" {
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let (w, h) = desc.default_size;
        let n = r
            .instantiate(desc.class, id, Rect::new(8.0, y, w, h))
            .expect("instantiate component");
        d.add_child("form_main", n).expect("add child");
        y += h + 4.0;
    }

    let pgc = r
        .suggest_id("PageControl", |i| d.node(i).is_some())
        .to_string();
    let n = r
        .instantiate("PageControl", pgc.clone(), Rect::new(8.0, y, 320.0, 200.0))
        .expect("instantiate page control");
    d.add_child("form_main", n)
        .expect("add page control to form");

    for _ in 0..2 {
        let id = r.suggest_id("TabSheet", |i| d.node(i).is_some());
        let t = r
            .instantiate("TabSheet", id, Rect::new(0.0, 24.0, 320.0, 176.0))
            .expect("instantiate tab sheet");
        d.add_child(&pgc, t).expect("add tab sheet to page control");
    }

    d.validate().expect("document is valid");
    d
}

fn extract_struct_fields(source: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut out = BTreeMap::new();
    let mut rest = source;
    while let Some(idx) = rest.find("pub struct ") {
        let after = &rest[idx + "pub struct ".len()..];
        let Some(open_brace) = after.find('{') else {
            break;
        };
        let name = after[..open_brace].trim().to_string();
        let Some(close_brace) = after[open_brace..].find('}') else {
            break;
        };
        let fields = after[open_brace + 1..open_brace + close_brace]
            .lines()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .filter_map(|line| line.split(':').next())
            .map(|col| col.trim().to_string())
            .filter(|col| !col.is_empty())
            .collect::<BTreeSet<_>>();

        if !name.is_empty() && !name.contains(' ') && name != "FormMain" && name != "App" {
            out.insert(name, fields);
        }
        rest = &after[open_brace + close_brace..];
    }
    out
}

fn generate_designer(g: impl Gerador, doc: &Document) -> String {
    g.gerar(doc, "app", &Registry::builtin())
        .expect("codegen succeeds")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("designer source exists")
        .conteudo
}

#[test]
fn both_targets_agree_on_text_property_name() {
    let doc = form_with_all_components();
    let egui_source = generate_designer(GeradorRxp, &doc);
    let makepad_source = generate_designer(GeradorMakepad, &doc);

    let egui_structs = extract_struct_fields(&egui_source);
    let makepad_structs = extract_struct_fields(&makepad_source);

    // No target should ever emit a field named 'caption'
    for (name, fields) in &egui_structs {
        assert!(
            !fields.contains("caption"),
            "Egui target emitted 'caption' on struct `{name}`. The standard is `text`."
        );
    }
    for (name, fields) in &makepad_structs {
        assert!(
            !fields.contains("caption"),
            "Makepad target emitted 'caption' on struct `{name}`. The standard is `text`."
        );
    }

    // Shared visual controls must have `text` in both targets
    let controls_with_text = [
        "Button",
        "Label",
        "Edit",
        "MaskEdit",
        "CheckBox",
        "RadioButton",
        "GroupBox",
        "Panel",
        "TabSheet",
        "ComboBox",
    ];

    for class_name in controls_with_text {
        let egui_fields = egui_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Egui missing struct {class_name}"));
        let makepad_fields = makepad_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Makepad missing struct {class_name}"));

        assert!(
            egui_fields.contains("text"),
            "Egui struct `{class_name}` must contain `text` field"
        );
        assert!(
            makepad_fields.contains("text"),
            "Makepad struct `{class_name}` must contain `text` field"
        );
    }
}

#[test]
fn shared_components_have_identical_field_sets() {
    let doc = form_with_all_components();
    let egui_source = generate_designer(GeradorRxp, &doc);
    let makepad_source = generate_designer(GeradorMakepad, &doc);

    let egui_structs = extract_struct_fields(&egui_source);
    let makepad_structs = extract_struct_fields(&makepad_source);

    // Classes where all fields are 100% identical between targets:
    let identical_classes = [
        "ActionList",
        "Action",
        "Label",
        "CheckBox",
        "RadioButton",
        "GroupBox",
        "TabSheet",
        "ListBox",
        "ComboBox",
        "Memo",
        // the StatusBar's panels
        "StatusPanel",
        // the menus
        "MenuItem",
        // the ListView
        "ListColumn",
        "ListView",
        "MessageDialog",
        "Drawer",
        "MultiSelect",
        "CheckListBox",
        "DateTimePicker",
    ];

    for class_name in identical_classes {
        let egui_fields = egui_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Egui missing struct {class_name}"));
        let makepad_fields = makepad_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Makepad missing struct {class_name}"));

        assert_eq!(
            egui_fields, makepad_fields,
            "Fields of `{class_name}` differ between Egui and Makepad targets:\n  Egui: {:?}\n  Makepad: {:?}",
            egui_fields, makepad_fields
        );
    }

    // For Button and Edit, verify that all core user fields (visible, enabled, text) are present in both:
    for class_name in ["Button", "Edit", "MaskEdit"] {
        let egui_fields = egui_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Egui missing struct {class_name}"));
        let makepad_fields = makepad_structs
            .get(class_name)
            .unwrap_or_else(|| panic!("Makepad missing struct {class_name}"));

        for required in ["visible", "enabled", "text"] {
            assert!(
                egui_fields.contains(required),
                "Egui `{class_name}` missing required field `{required}`"
            );
            assert!(
                makepad_fields.contains(required),
                "Makepad `{class_name}` missing required field `{required}`"
            );
        }
    }

    // For PageControl, verify core fields (visible, enabled, active_page) are present in both:
    let egui_pgc = egui_structs.get("PageControl").unwrap();
    let makepad_pgc = makepad_structs.get("PageControl").unwrap();
    for required in ["visible", "enabled", "active_page"] {
        assert!(egui_pgc.contains(required));
        assert!(makepad_pgc.contains(required));
    }
}
