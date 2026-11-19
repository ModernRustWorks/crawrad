//! **A THIRD-PARTY COMPONENT LEAVES THE PALETTE AND LANDS ON THE FORM.**
//!
//! # What this test locked, and why it was missing
//!
//! The third-party component chain was proven at both ends: the
//! `manifest.json` enters the catalog, and the saved form becomes code that
//! compiles and draws. **The middle — the gesture — was never measured.**
//!
//! And it is not a detail: if the palette does not offer the class, or if the creation
//! port rejects it, the installed component exists in the catalog but cannot
//! be placed on the form. Everything else would be useless work.
//!
//! # Why headless, and not just on screen
//!
//! The gesture involves three things that are pure functions: the Toolbox list
//! (`palette::rows`), the container choice (`Designer::parent_for_drop`), and the
//! creation port (`component.add`, ADR 0005). None of them requires a
//! window — and what is measured here is measured in milliseconds, on every `cargo
//! test`, instead of once, by eye.
//!
//! The eye verified it as well: `Dial`, installed by the IDE from an external folder,
//! clicked in the Toolbox and dropped onto the form, on 2026-09-18.

use crawrad_actions::{Args, Author, Context, Project};
use crawrad_designer::Designer;
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Doc;
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::ide_app::palette::{self, Row};

/// The target of the Makepad shell — the same one the Toolbox uses.
const ALVO: &str = crawrad_runtime_makepad::ALVO;

/// The catalog with packages from THIS repository, read from real files.
fn catalogo() -> Registry {
    let mut r = Registry::builtin();
    for texto in [
        include_str!("../../../../../components/gauge-crawrad/manifest.json"),
        include_str!("../../../../../components/portadb-crawrad/manifest.json"),
    ] {
        let m: Manifest = serde_json::from_str(texto).expect("valid JSON");
        for c in m.to_components().expect("convert") {
            r.register_external(c).expect("register");
        }
    }
    r
}

/// A form with an inner `Panel`, to measure dropping INSIDE.
fn documento() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let mut p = r
        .instantiate(
            "Panel",
            "pnl1".to_string(),
            Rect::new(40.0, 40.0, 300.0, 200.0),
        )
        .expect("instantiate");
    p.set_name("pnl1");
    d.add_child("form_main", p).expect("enters");
    d.validate().expect("valid");
    d
}

// ---------------------------------------------------------------------------
// 1 — THE PALETTE OFFERS
// ---------------------------------------------------------------------------

/// **The third-party component appears in the Toolbox, in the group declared by the manifest.**
#[test]
fn o_terceiro_aparece_na_paleta_no_grupo_dele() {
    let r = catalogo();
    let linhas = palette::rows(&r, ALVO, "", false);
    let mut grupo_atual = None;
    let mut onde = None;
    for l in &linhas {
        match l {
            Row::Section(s) => grupo_atual = Some(*s),
            Row::Item { class, .. } if *class == "Gauge" => onde = grupo_atual,
            _ => {}
        }
    }
    assert_eq!(
        onde,
        Some("Additional"),
        "Gauge did not end up in the group declared by manifest.json"
    );
}

/// **The filter finds the third-party component by name**, just like any native component.
#[test]
fn o_filtro_acha_o_terceiro() {
    let r = catalogo();
    let classes: Vec<&str> = palette::rows(&r, ALVO, "gau", false)
        .iter()
        .filter_map(Row::class)
        .collect();
    assert_eq!(classes, vec!["Gauge"], "filter did not find only Gauge");
}

/// **What does not declare THIS target does not appear**, which is the filter doing
/// its job.
///
/// Without it, a component that only knows how to draw in Makepad would appear in the
/// palette of someone targeting egui, and the user would design a form that their target
/// cannot produce.
#[test]
fn o_terceiro_de_outro_alvo_nao_aparece() {
    use crawrad_registry::external::{ExternalChildren, ExternalComponent, ExternalProp};
    let mut r = catalogo();
    r.register_external(ExternalComponent {
        inherits: None,
        class: "SoMakepad".to_string(),
        display: "So Makepad".to_string(),
        base_name: "so_makepad".to_string(),
        palette: "Additional".to_string(),
        prefix: "sm".to_string(),
        default_size: (80.0, 24.0),
        props: vec![ExternalProp {
            key: "name".to_string(),
            display: "Name".to_string(),
            default: Value::Str(String::new()),
            category: crawrad_registry::Category::Identity,
            kind: crawrad_registry::external::ExternalKind::Text,
            readonly: false,
            bindable: false,
            options: Vec::new(),
            ref_class: None,
            tree_collection: false,
            item_class: None,
            multiline: false,
            hidden_when: None,
        }],
        events: Vec::new(),
        targets: vec!["makepad".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: false,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
        crate_module: String::new(),
        desenho: Vec::new(),
        icon: Vec::new(),
    })
    .expect("port accepts");

    let no_makepad: Vec<&str> = palette::rows(&r, "makepad", "", false)
        .iter()
        .filter_map(Row::class)
        .collect();
    assert!(
        no_makepad.contains(&"SoMakepad"),
        "it declares makepad and did not appear in the Makepad palette"
    );
    let no_egui: Vec<&str> = palette::rows(&r, "egui", "", false)
        .iter()
        .filter_map(Row::class)
        .collect();
    assert!(
        !no_egui.contains(&"SoMakepad"),
        "it appeared in the palette of a target it does not declare"
    );
}

// ---------------------------------------------------------------------------
// 2 — DROPPING CREATES
// ---------------------------------------------------------------------------

/// **Dropping the third-party component onto the form creates a node of its class.**
///
/// Via the single mutation port (`component.add`, ADR 0005) — the same one
/// the mouse and the agent use.
#[test]
fn soltar_o_terceiro_no_formulario_cria_o_no() {
    let r = catalogo();
    let (tx, _) = soltar(&r, "Gauge", None, 100.0, 50.0);
    let mut d = Doc::new(documento());
    d.apply(tx).expect("transaction succeeds");
    let novo = d
        .ir()
        .nodes
        .values()
        .find(|n| n.class == "Gauge")
        .expect("Gauge was created in the document");
    assert_eq!(
        novo.rect().map(|r| (r.x, r.y)),
        Some((100.0, 50.0)),
        "it was not created where it was dropped"
    );
    assert_eq!(
        novo.rect().map(|r| (r.w, r.h)),
        Some((140.0, 48.0)),
        "default size is not what manifest.json declares"
    );
}

/// **Dropping INSIDE a container places it inside.**
///
/// This is the missing half in Makepad until 2026-09-17, applying to third-party
/// components exactly as it does to native ones: in a classic RAD IDE, dropping inside a `Panel`
/// is how a form is built.
#[test]
fn soltar_o_terceiro_dentro_de_um_painel_o_poe_dentro() {
    let r = catalogo();
    let doc = documento();
    // The point is inside the `Panel` (40,40 300x200).
    let designer = Designer::with_registry(std::rc::Rc::new(catalogo()));
    let (pai, rx, ry) = designer.parent_for_drop(&doc, 100.0, 100.0);
    assert_eq!(
        pai.as_str(),
        "pnl1",
        "parent_for_drop did not find the panel"
    );

    let (tx, _) = soltar(&r, "Gauge", Some(&pai), rx, ry);
    let mut d = Doc::new(doc);
    d.apply(tx).expect("transaction succeeds");
    let novo = d
        .ir()
        .nodes
        .values()
        .find(|n| n.class == "Gauge")
        .expect("Gauge was created");
    assert_eq!(
        d.ir().parent_of(&novo.id).map(String::as_str),
        Some("pnl1"),
        "it was created as SIBLING of the panel, not inside it"
    );
}

/// **A third-party non-visual component also leaves the palette.**
///
/// It has no rectangle on the application screen, yet it has an icon in the
/// designer and enters via the same gesture — which is what separates a component
/// from a figure.
#[test]
fn o_nao_visual_de_terceiro_tambem_sai_da_paleta() {
    let r = catalogo();
    let classes: Vec<&str> = palette::rows(&r, ALVO, "", false)
        .iter()
        .filter_map(Row::class)
        .collect();
    assert!(
        classes.contains(&"Connection"),
        "Connection is not in the palette: {classes:?}"
    );
    let (tx, _) = soltar(&r, "Connection", None, 8.0, 8.0);
    let mut d = Doc::new(documento());
    d.apply(tx).expect("transaction succeeds");
    assert!(
        d.ir().nodes.values().any(|n| n.class == "Connection"),
        "dropping the non-visual created nothing"
    );
}

/// **A class missing from the catalog is rejected with its name in the message.**
///
/// This is the twin of the tests above, and it makes the pair conclusive: without it,
/// `component.add` could be accepting everything, and the tests above would still pass.
#[test]
fn classe_que_nao_existe_e_recusada_dizendo_o_nome() {
    let r = catalogo();
    let doc = documento();
    let area = Clipboard::default();
    let d = Doc::new(doc);
    let ctx = Context {
        doc: Some(&d),
        registry: &r,
        clipboard: &area,
        args: Args::text("class", "NaoExisteIsso"),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let efeito = (crawrad_actions::by_id("component.add")
        .expect("action exists")
        .execute)(&ctx);
    assert!(efeito.transaction.is_none(), "created a nonexistent class");
    assert_eq!(
        efeito.status.as_deref(),
        Some("unknown class: NaoExisteIsso"),
        "rejection does not name the class"
    );
}

/// Runs `component.add` as the shell runs it when dropping from the palette.
fn soltar(
    r: &Registry,
    classe: &str,
    pai: Option<&str>,
    x: f32,
    y: f32,
) -> (crawrad_doc::Transaction, Option<String>) {
    let d = Doc::new(documento());
    let area = Clipboard::default();
    let mut args = Args::text("class", classe)
        .and("x", Value::Float(x as f64))
        .and("y", Value::Float(y as f64));
    if let Some(p) = pai {
        args = args.and("parent", Value::Str(p.to_string()));
    }
    let ctx = Context {
        doc: Some(&d),
        registry: r,
        clipboard: &area,
        args,
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let efeito = (crawrad_actions::by_id("component.add")
        .expect("action exists")
        .execute)(&ctx);
    let status = efeito.status.clone();
    (
        efeito.transaction.unwrap_or_else(|| {
            panic!("dropping `{classe}` did not produce a transaction: {status:?}")
        }),
        status,
    )
}

/// A web project greys what cannot run in the browser (both SQLite providers),
/// with the reason as the tooltip, and does not arm it; desktop projects are untouched.
#[test]
fn web_project_greys_sqlite_providers_with_reason() {
    let r = catalogo();
    let web = palette::rows(&r, ALVO, "", true);
    for class in ["SqliteProvider", "SqliteSqlxProvider"] {
        let row = web
            .iter()
            .find(|l| matches!(l, Row::Item { class: c, .. } if *c == class))
            .expect("listed, not hidden");
        assert_eq!(row.class(), None, "{class} must not arm in a web project");
        assert!(
            row.hint()
                .is_some_and(|h| h.contains("needs a desktop project"))
        );
    }
    let memory = web
        .iter()
        .find(|l| {
            matches!(
                l,
                Row::Item {
                    class: "MemoryProvider",
                    ..
                }
            )
        })
        .expect("memory provider");
    assert_eq!(memory.class(), Some("MemoryProvider"));
    let desktop = palette::rows(&r, ALVO, "", false);
    assert!(desktop.iter().all(|l| l.hint().is_none()));
}
