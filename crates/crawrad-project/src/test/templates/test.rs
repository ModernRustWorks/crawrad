use super::*;
use crawrad_registry::Registry;

/// What the catalogue promises for a class, minus what `instantiate` writes externally.
///
/// Read from descriptor, not handwritten: a copied list inside the test would violate
/// rule 8 — two handwritten lists compared with each other, neither covering properties
/// added tomorrow.
fn promised_props(r: &Registry, class: &str) -> Vec<&'static str> {
    let d = r
        .get(class)
        .unwrap_or_else(|| panic!("`{class}` in catalogue"));
    let ks: Vec<&'static str> = d
        .props
        .iter()
        .map(|p| p.key)
        .filter(|k| *k != "rect" && *k != "name")
        .collect();
    assert!(
        !ks.is_empty(),
        "catalogue for `{class}` has properties — otherwise test measures nothing"
    );
    ks
}

/// Every piece of every template exists in the catalogue, **including the root**,
/// and the document comes out complete.
///
/// This is the test that pays for the template list: a mistyped class name,
/// or a component removed from the catalogue, crashes the IDE on the opening
/// screen — the first one the user sees. Here it fails beforehand.
///
/// The root entered later, through a real gap: it came from `Document::new`,
/// which used raw `Node::new`, and the form was born without the six properties
/// that `Form` has in the catalogue. Nothing broke — Inspector falls back to
/// descriptor defaults when a key is missing —, and it was precisely rule 9 left
/// open inside the module written to close it.
#[test]
fn every_template_instantiates_from_catalogue() {
    let r = Registry::builtin();
    for m in TEMPLATES {
        let doc = m
            .document(&r)
            .unwrap_or_else(|| panic!("template {} failed to instantiate from catalogue", m.name));
        assert_eq!(
            doc.walk().len(),
            m.pieces.len() + 1,
            "{}: document must have the form and each piece",
            m.name
        );
        let root = doc.node(&doc.root).expect("form is root node");
        for k in promised_props(&r, "Form") {
            assert!(
                root.props.contains_key(k),
                "{}: form was born without `{k}` — root did not go through instantiate",
                m.name
            );
        }
        // And what the template states continues taking precedence over catalogue defaults:
        // `instantiate` brings `Caption = Form1`, and the template caption is written after.
        assert_eq!(
            root.text(),
            Some(m.caption),
            "{}: template caption was lost on the way",
            m.name
        );
        doc.validate()
            .unwrap_or_else(|e| panic!("{}: invalid document — {e:?}", m.name));
    }
}

/// The blank template is the first item in the list, not a fourth copy.
#[test]
fn default_template_is_in_the_list() {
    let p = Template::default_template();
    assert!(
        TEMPLATES.iter().any(|m| std::ptr::eq(m, p)),
        "default must BE an item in the list, not a clone"
    );
    assert!(
        p.pieces.is_empty(),
        "default is empty form: what New Project always created"
    );
}

/// No template piece arrives showing its VARIABLE NAME.
///
/// The user saw this on screen before tests existed: the `Data entry` preview
/// showed `lblName`, `chkActive`, `btnSave`. It is `instantiate` doing the right
/// thing — component is born with its own name written — and template doing the wrong
/// thing, because a template is a ready starting point.
///
/// The test measures OUTPUT, not the table: it asks the prepared document if any
/// text equals the id. Written against the table, it would pass on the day someone
/// added a piece without `text`.
#[test]
fn no_piece_displays_its_own_variable_name() {
    let r = Registry::builtin();
    for m in TEMPLATES {
        let doc = m.document(&r).expect("instantiates");
        for node in doc.walk() {
            if node.id == doc.root {
                continue;
            }
            assert_ne!(
                node.text(),
                Some(node.name()),
                "{}: `{}` arrived with own name as caption — template is ready starting point, \
                 not a form to rename",
                m.name,
                node.name()
            );
        }
    }
}
