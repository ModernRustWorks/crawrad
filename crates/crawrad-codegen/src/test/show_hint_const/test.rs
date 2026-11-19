//! **The form's `ShowHint` constant is declared exactly when it is read**.
//!
//! Every catalogue class with a `Hint`, alone on a form with a hint set, on
//! both targets: an unread `CRAWRAD_FORM_SHOW_HINT` is a warning in the user's
//! project, a read one without its declaration does not compile. Measured on
//! 03/11/2026: deciding by the catalogue declared it unread for `Action`, which
//! declares `Hint` and draws no tooltip (a `TabSheet` lives in a `PageControl`).
use crate::{Gerador, GeradorMakepad, GeradorRxp};
use crawrad_ir::{Document, Rect, Value};

#[test]
fn the_show_hint_constant_is_declared_exactly_when_read() {
    let reg = crawrad_registry::Registry::builtin();
    let mut bad = Vec::new();
    for desc in reg.all() {
        if desc.class == "Form" || !desc.props.iter().any(|p| p.key == "hint") {
            continue;
        }
        let mut d = Document::new("form_main", "Form");
        d.node_mut("form_main")
            .unwrap()
            .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        let (w, h) = desc.default_size;
        let Some(mut n) = reg.instantiate(desc.class, "probe1", Rect::new(8.0, 8.0, w, h)) else {
            continue;
        };
        n.set_name("probe1");
        n.props.insert("hint".into(), Value::Str("x".into()));
        if desc.class == "TabSheet" {
            let (pw, ph) = reg.get("PageControl").unwrap().default_size;
            let mut pc = reg
                .instantiate("PageControl", "pages1", Rect::new(8.0, 8.0, pw, ph))
                .unwrap();
            pc.set_name("pages1");
            d.add_child("form_main", pc).unwrap();
            if d.add_child("pages1", n).is_err() {
                continue;
            }
        } else if d.add_child("form_main", n).is_err() {
            continue;
        }
        for (t, files) in [
            ("egui", GeradorRxp.gerar(&d, "p", &reg)),
            ("makepad", GeradorMakepad.gerar(&d, "p", &reg)),
        ] {
            let Ok(files) = files else { continue };
            let all: String = files.iter().map(|f| f.conteudo.clone()).collect();
            let declared = all.contains("const CRAWRAD_FORM_SHOW_HINT");
            let read = all.contains("CRAWRAD_FORM_SHOW_HINT &&");
            if declared != read {
                bad.push(format!("{} {t}", desc.class));
            }
        }
    }
    assert!(bad.is_empty(), "declared and read disagree: {bad:?}");
}
