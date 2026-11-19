//! What can be measured from the non-visual badge **without a window**.
//!
//! What is under test here is not aesthetics: it is **conformance with
//! `crawrad-designer`**. A badge drawn in one rectangle and a click resolved in
//! another is the interface lying — the user clicks what they see and nothing happens —,
//! and this was exactly the defect that RXP recorded in the comment of
//! `selo_nao_visual` (`crawrad-runtime-egui/src/lib.rs:2386-2389`).

use super::*;

use crawrad_designer::Designer;
use crawrad_ir::{Document, Rect as RectIr, Value};
use crawrad_runtime::Runtime;

use crate::canvas::{point_in_design, rect_on_screen};
use crate::components::{desenho_de, e_visual};
use crate::{CLASSES, MakepadRuntime, Transformacao};

/// The size that `crawrad-registry` gives to a non-visual
/// (`crawrad-registry/src/componentes/base.rs:5`, `ICONE`).
const LADO_DO_ICONE: f32 = 28.0;

/// A document with a non-visual placed on the form, at the size given
/// by the catalog.
fn documento_com_nao_visual(class: &str, id: &str, x: f32, y: f32) -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    let no = crawrad_ir::Node::new(id, class).with_prop(
        "rect",
        Value::Rect(RectIr::new(x, y, LADO_DO_ICONE, LADO_DO_ICONE)),
    );
    doc.add_child("form", no)
        .expect("the node is added to the root");
    doc
}

fn designer() -> Designer {
    Designer::with_registry(std::rc::Rc::new(crawrad_registry::Registry::builtin()))
}

// ---------------------------------------------------------------------------
// The click target — what is SEEN is what SELECTS
// ---------------------------------------------------------------------------

#[test]
fn o_selo_desenhado_e_o_retangulo_que_o_designer_seleciona() {
    // The test runs across three zoom levels on purpose. At scale 1.0 a badge of
    // FIXED size in screen points (28x28, the number given by the catalog) passes
    // by accident: it coincides with the transformed rectangle. It is at 0.5 and 2.5
    // that the mismatch appears — and it is at 0.5 and 2.5 that the user clicks and
    // nothing happens.
    let doc = documento_com_nao_visual("ImageList", "iml", 448.0, 160.0);
    let d = designer();
    let r_design = doc.rect_in_form("iml").expect("has a rect");

    for escala in [0.5, 1.0, 2.5] {
        let t = Transformacao::new(31.0, 17.0, escala);
        let selo = badge_box(rect_on_screen(&t, r_design));

        // The four corners (inside) and the center of what was DRAWN. Each
        // goes back to design units through the same transformation and is
        // offered to the designer.
        let dentro = [
            (selo.pos.x + 0.5, selo.pos.y + 0.5),
            (selo.pos.x + selo.size.x - 0.5, selo.pos.y + 0.5),
            (selo.pos.x + 0.5, selo.pos.y + selo.size.y - 0.5),
            (
                selo.pos.x + selo.size.x - 0.5,
                selo.pos.y + selo.size.y - 0.5,
            ),
            (
                selo.pos.x + selo.size.x * 0.5,
                selo.pos.y + selo.size.y * 0.5,
            ),
        ];
        for (sx, sy) in dentro {
            let (x, y) = point_in_design(&t, makepad_widgets::dvec2(sx, sy));
            assert_eq!(
                d.hit(&doc, x, y).as_deref(),
                Some("iml"),
                "at {escala}x, point ({sx}, {sy}) of the drawn badge does not select ImageList"
            );
        }

        // And the opposite: what the designer selects cannot extend outside
        // of what was drawn. One pixel beyond each edge is already the form.
        let fora = [
            (selo.pos.x - 1.5, selo.pos.y + selo.size.y * 0.5),
            (
                selo.pos.x + selo.size.x + 1.5,
                selo.pos.y + selo.size.y * 0.5,
            ),
            (selo.pos.x + selo.size.x * 0.5, selo.pos.y - 1.5),
            (
                selo.pos.x + selo.size.x * 0.5,
                selo.pos.y + selo.size.y + 1.5,
            ),
        ];
        for (sx, sy) in fora {
            let (x, y) = point_in_design(&t, makepad_widgets::dvec2(sx, sy));
            assert_ne!(
                d.hit(&doc, x, y).as_deref(),
                Some("iml"),
                "at {escala}x, point ({sx}, {sy}) is OUTSIDE the drawn badge and still selects it"
            );
        }
    }
}

#[test]
fn o_selo_encolhe_e_cresce_com_o_zoom() {
    // The corollary of the test above, stated directly: the badge geometry goes
    // through `Transformacao`, not through a fixed number of screen points.
    let r = RectIr::new(100.0, 50.0, LADO_DO_ICONE, LADO_DO_ICONE);
    let pequeno = badge_box(rect_on_screen(&Transformacao::new(0.0, 0.0, 0.5), r));
    let grande = badge_box(rect_on_screen(&Transformacao::new(0.0, 0.0, 2.0), r));
    assert_eq!(pequeno.size.x, 14.0);
    assert_eq!(grande.size.x, 56.0);
}

// ---------------------------------------------------------------------------
// What the owner saw: filled, with initials, and the name underneath
// ---------------------------------------------------------------------------

#[test]
fn as_iniciais_sao_as_maiusculas_da_classe() {
    assert_eq!(initials("ImageList"), "IL");
    assert_eq!(initials("ActionList"), "AL");
    assert_eq!(initials("Timer"), "T");
    assert_eq!(initials("Action"), "A");
}

#[test]
fn o_nome_fica_abaixo_do_selo_e_centrado_nele() {
    // "centered name" is what the owner asked for when looking at it, centered on the BADGE —
    // not on the form nor in an arbitrary box. And it stays OUTSIDE the badge: a
    // name over the initials hides both things at once.
    let selo = Rect {
        pos: makepad_widgets::dvec2(200.0, 100.0),
        size: makepad_widgets::dvec2(28.0, 28.0),
    };
    let nome = name_box(selo, 1.0);

    let centro = |r: Rect| r.pos.x + r.size.x * 0.5;
    assert!(
        (centro(nome) - centro(selo)).abs() < 1e-9,
        "the name is not centered on the badge: {nome:?} vs {selo:?}"
    );
    assert!(
        nome.pos.y >= selo.pos.y + selo.size.y,
        "the name invades the badge: {nome:?} vs {selo:?}"
    );
    assert!(
        nome.size.x > selo.size.x,
        "the name box is narrower than the badge — `NomeComprido1` would be truncated"
    );
}

#[test]
fn a_caixa_do_nome_acompanha_a_escala() {
    // The gap and line height are DESIGN measurements. If fixed in screen
    // points, at 2x the name would touch the badge and at 0.5x it would float far from it.
    let selo = Rect {
        pos: makepad_widgets::dvec2(0.0, 0.0),
        size: makepad_widgets::dvec2(56.0, 56.0),
    };
    let um = name_box(selo, 1.0);
    let dois = name_box(selo, 2.0);
    assert!(
        dois.pos.y - (selo.pos.y + selo.size.y) > um.pos.y - (selo.pos.y + selo.size.y),
        "the gap between badge and name did not follow the scale"
    );
    assert!(dois.size.y > um.size.y, "the name line did not scale");
}

#[test]
fn o_selo_do_nao_visual_espelha_as_cores_do_rxp() {
    // The same document opened in both runtimes must look the same.
    // RXP writes these four in `crawrad-runtime-egui/src/lib.rs:182-185`, and
    // this crate cannot import them (it does not depend on RXP, and shouldn't) —
    // so the value is compared here, and changing the token without changing RXP fails.
    assert_eq!(MAKEPAD_NAO_VISUAL_FUNDO, rgb(0xD0, 0xE0, 0xE8));
    assert_eq!(MAKEPAD_NAO_VISUAL_BORDA, rgb(0x40, 0x60, 0x80));
    assert_eq!(MAKEPAD_NAO_VISUAL_TEXTO, rgb(0x20, 0x40, 0x60));
    // The badge is LIGHT, and it must be: it carries its own background and therefore
    // can be read both against RXP's light paper and against the dark paper here.
    let claridade = |c: [f32; 4]| c[0] + c[1] + c[2];
    assert!(
        claridade(MAKEPAD_NAO_VISUAL_FUNDO) > claridade(MAKEPAD_PAPER),
        "the badge no longer stands out from the form paper"
    );
}

// ---------------------------------------------------------------------------
// Dispatch: the non-visual has its OWN path, not the fallback path
// ---------------------------------------------------------------------------

#[test]
fn o_nao_visual_nao_cai_no_desenho_de_reserva() {
    // The defect that the owner found visually: `ImageList` fell back to
    // `componentes::generico`, which is the dashed outline for "a file is missing here".
    // The two things are different and cannot become the same again.
    for classe in ["ActionList", "Action", "ImageList", "Timer"] {
        assert_eq!(
            desenho_de(classe).class_name(),
            classe,
            "`{classe}` fell back to placeholder drawing"
        );
        assert!(!desenho_de(classe).visual(), "`{classe}` is non-visual");
    }
    // And the contrast, so the test doesn't pass trivially: a
    // class without a file STILL falls back to the placeholder, which is its
    // desired behavior.
    assert_eq!(desenho_de("NaoExiste").class_name(), "");
    assert!(desenho_de("NaoExiste").visual());
}

#[test]
fn o_nao_visual_desenha_mas_nao_vira_widget() {
    // The two halves of the contract, which are independent: the canvas DRAWS it
    // (it is in the catalog) and the application does NOT instantiate it (outside `classes()`,
    // `crawrad-runtime/src/lib.rs:166-168`).
    for classe in ["ActionList", "Action", "ImageList", "Timer"] {
        assert!(
            !CLASSES.contains(&classe),
            "`{classe}` leaked into runtime widgets list"
        );
        assert!(!e_visual(classe));
    }
    assert!(CLASSES.contains(&"Button"));
    assert!(e_visual("Button"));
}

#[test]
fn todo_nao_visual_do_registro_e_selo_e_nao_widget() {
    // The adapter does not know the catalog at compile time
    // (`crawrad-registry` is a `dev-dependency` here, on purpose). This test is what
    // prevents the drawing catalog from lagging behind the registry — and
    // because the same response serves `no_canvas`, forgetting a new component
    // shows up on both sides at once.
    let reg = crawrad_registry::Registry::builtin();
    for d in reg.all() {
        if !d.supports("makepad") {
            continue;
        }
        assert_eq!(
            e_visual(d.class),
            d.visual,
            "`{}` is non-visual in catalogue and adapter treats it as widget (or vice versa)",
            d.class
        );
    }
}

// ---------------------------------------------------------------------------
// The gesture: the badge cannot be resized
// ---------------------------------------------------------------------------

#[test]
fn o_nao_visual_nao_recebe_alca_de_redimensionar() {
    // `Designer::hit_handle` rejects dragging on non-visuals
    // (`crawrad-designer/src/lib.rs:519`). While the adapter drew the eight
    // handles anyway, the selected `ActionList` appeared resizable when it
    // was not: the interface lying, again.
    let doc = documento_com_nao_visual("ActionList", "acl", 448.0, 160.0);
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    rt.set_selection(&["acl".to_string()]);

    assert!(rt.desenhavel("acl"), "the badge is on screen");
    assert_eq!(
        rt.rect_da_alca(),
        None,
        "non-visual cannot receive the eight handles"
    );

    let r = doc.rect_in_form("acl").expect("has a rect");
    let (ax, ay) = crawrad_designer::Handle::SE.anchor(r);
    let d = designer();
    assert_eq!(
        d.hit_handle(&doc, &["acl".to_string()], ax, ay),
        None,
        "and the designer agrees — this is the consistency expected"
    );
    assert_eq!(
        crate::canvas::cursor_for(rt.rect_da_alca(), rt.form_rect(), ax, ay),
        makepad_widgets::MouseCursor::Default,
        "the resize cursor cannot appear where nothing can be resized"
    );

    // The contrast: a Button in the same corner RECEIVES a handle.
    let mut doc = doc;
    let btn = crawrad_ir::Node::new("btn", "Button").with_prop(
        "rect",
        Value::Rect(RectIr::new(448.0, 160.0, LADO_DO_ICONE, LADO_DO_ICONE)),
    );
    doc.add_child("form", btn).expect("enters");
    rt.open(&doc).expect("reopens");
    rt.set_selection(&["btn".to_string()]);
    assert!(rt.rect_da_alca().is_some());
}

// ---------------------------------------------------------------------------
// THE LAYER: the non-visual is always in front of the visual
// ---------------------------------------------------------------------------

#[test]
fn o_nao_visual_e_desenhado_depois_de_todo_componente_visual() {
    // The owner's rule: *"the drawings of non-visual components in designer
    // mode must always stay in front of all visual components"*.
    // In his screenshot there were two badges **partially covered** — one by a
    // `Panel` and another by a `PageControl` —, and a covered badge is a
    // component that cannot be clicked to select.
    //
    // The document is constructed on purpose with the non-visual FIRST in Z-order:
    // this is the case that naive drawing gets wrong, because it would follow the document.
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    let no = |id: &str, class: &str, r: RectIr| {
        crawrad_ir::Node::new(id, class).with_prop("rect", Value::Rect(r))
    };
    // Document order: badge, panel, badge, button.
    doc.add_child(
        "form",
        no("iml", "ImageList", RectIr::new(40.0, 40.0, 28.0, 28.0)),
    )
    .expect("enters");
    doc.add_child(
        "form",
        no("pnl", "Panel", RectIr::new(20.0, 20.0, 200.0, 120.0)),
    )
    .expect("enters");
    doc.add_child(
        "form",
        no("tmr", "Timer", RectIr::new(90.0, 40.0, 28.0, 28.0)),
    )
    .expect("enters");
    doc.add_child(
        "form",
        no("btn", "Button", RectIr::new(30.0, 60.0, 120.0, 26.0)),
    )
    .expect("enters");

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");

    let pintados: Vec<&str> =
        crate::canvas::draw_order(&rt.ordem, &rt.ctls, crawrad_runtime::Mode::Design)
            .into_iter()
            .map(|id| id.as_str())
            .collect();

    // All four enter, and none disappears.
    assert_eq!(
        pintados.len(),
        4,
        "what vanishes from drawing vanishes from screen: {pintados:?}"
    );

    let primeiro_selo = pintados
        .iter()
        .position(|id| *id == "iml" || *id == "tmr")
        .expect("the badge is drawn");
    let ultimo_visual = pintados
        .iter()
        .rposition(|id| *id == "pnl" || *id == "btn")
        .expect("the visual is drawn");
    assert!(
        ultimo_visual < primeiro_selo,
        "every badge must come AFTER every visual — got {pintados:?}"
    );

    // And, between pairs, document order is preserved: `Panel` before
    // `Button` (it is the visual parent on screen) and `ImageList` before `Timer`.
    let pos = |id: &str| pintados.iter().position(|x| *x == id).expect("is there");
    assert!(pos("pnl") < pos("btn"));
    assert!(pos("iml") < pos("tmr"));
}

#[test]
fn no_preview_o_nao_visual_some_e_so_o_visual_fica() {
    // The owner saw this on screen and called it out:
    //
    // > *"in preview the non-visual components are appearing, this is WRONG,
    // > in egui they did not appear"*
    //
    // The badge is a DESIGNER OVERLAY: it exists so the user can click on an
    // `ActionList` and select it. In the application there is no one to select it, and an
    // `ImageList` draws nothing — in a classic RAD IDE it disappears when running. RXP already
    // did it right (`crawrad-runtime-egui/src/lib.rs`, `no_formulario_vivo`), and the
    // DIVERGENCE between the two adapters was the defect.
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    let no = |id: &str, class: &str, r: RectIr| {
        crawrad_ir::Node::new(id, class).with_prop("rect", Value::Rect(r))
    };
    doc.add_child(
        "form",
        no("btn", "Button", RectIr::new(10.0, 10.0, 80.0, 24.0)),
    )
    .expect("enters");
    doc.add_child(
        "form",
        no("iml", "ImageList", RectIr::new(120.0, 10.0, 28.0, 28.0)),
    )
    .expect("enters");
    doc.add_child(
        "form",
        no("acl", "ActionList", RectIr::new(160.0, 10.0, 28.0, 28.0)),
    )
    .expect("enters");

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");

    // In DESIGN all three are there — the contrast, without which a filter that
    // erased everything would pass.
    let em_design: Vec<&str> =
        crate::canvas::draw_order(&rt.ordem, &rt.ctls, crawrad_runtime::Mode::Design)
            .into_iter()
            .map(|id| id.as_str())
            .collect();
    assert_eq!(
        em_design,
        vec!["btn", "iml", "acl"],
        "in design the non-visual badge must still be drawn, and          AFTER every visual"
    );

    // In PREVIEW only what the application user would see.
    let no_preview: Vec<&str> =
        crate::canvas::draw_order(&rt.ordem, &rt.ctls, crawrad_runtime::Mode::Live)
            .into_iter()
            .map(|id| id.as_str())
            .collect();
    assert_eq!(
        no_preview,
        vec!["btn"],
        "the non-visual badge remained on screen in Preview. The form ceases to          be the application and shows designer furniture that the generated          binary does not have"
    );
}

#[test]
fn o_invisivel_nao_entra_no_desenho_de_nenhuma_das_duas_passadas() {
    // The `visible` filter lives in one place only — in `draw_order`. Two
    // passes with two filters would be two chances to forget one.
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    for (id, class) in [("btn", "Button"), ("iml", "ImageList")] {
        let n = crawrad_ir::Node::new(id, class)
            .with_prop("rect", Value::Rect(RectIr::new(10.0, 10.0, 40.0, 40.0)))
            .with_prop("visible", Value::Bool(false));
        doc.add_child("form", n).expect("enters");
    }
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    assert!(
        crate::canvas::draw_order(&rt.ordem, &rt.ctls, crawrad_runtime::Mode::Design).is_empty()
    );
}

// ---------------------------------------------------------------------------
// THE CLIP-TO-OWN-BOX TRAP — written BEFORE clipping
//
// Issue #129 specifies that a control's content stops at its box. The non-visual
// badge is the only item in the catalog that paints OUTSIDE its own rectangle on
// purpose: the name goes **under** the badge, as a classic RAD IDE shows
// `TActionList`. A blind clip would erase the name of every `ActionList`,
// `ImageList`, `Timer`, and `Action` on the form — and the name is the only thing
// that distinguishes two identical badges.
//
// These tests were created before clipping existed, and that is intentional: the order
// proves that the exception was planned, not discovered when the name disappeared.
// ---------------------------------------------------------------------------

/// The instance name is written **outside** the badge rectangle — below it.
///
/// This is what makes the exception necessary. If one day the name fits
/// inside the rectangle, this test fails and the exception can be removed.
#[test]
fn o_nome_do_nao_visual_e_escrito_fora_do_retangulo_do_selo() {
    let selo = badge_box(Rect {
        pos: dvec2(100.0, 50.0),
        size: dvec2(28.0, 28.0),
    });
    let nome = name_box(selo, 1.0);
    assert!(
        nome.pos.y >= selo.pos.y + selo.size.y,
        "the name must start BELOW the badge: badge {selo:?}, name {nome:?}"
    );
    assert!(
        nome.pos.y + nome.size.y > selo.pos.y + selo.size.y,
        "and must end outside it too"
    );
}

/// **The non-visual badge is the only item in the catalog that does not clip to its own
/// box.**
///
/// The check belongs to the component, not to a list in the canvas. This test scans the
/// entire catalog and checks both sides: the four non-visuals say *no*, and
/// **everything else says yes**. A new class that refuses clipping without a
/// reason will appear here.
///
/// Mutation that breaks it: [`Selo`]'s `clips_to_own_box` returning `true` — and,
/// on screen, the name of every `ActionList` disappears.
#[test]
fn so_o_selo_do_nao_visual_pinta_fora_da_propria_caixa() {
    for classe in crate::components::classes() {
        assert!(
            crate::components::desenho_de(classe).clips_to_own_box(),
            "{classe} is a control: its content must stop at its box"
        );
    }
    // And non-visuals — which are not in `classes()`, because they do not become widgets.
    for classe in ["ActionList", "ImageList", "Timer", "Action"] {
        let d = crate::components::desenho_de(classe);
        assert!(!d.visual(), "{classe} must be non-visual");
        assert!(
            !d.clips_to_own_box(),
            "{classe} writes the name outside the badge: clipping it erases the name"
        );
    }
}

// ---------------------------------------------------------------------------
// The class icon in the badge — the palette tile's drawing, not the initials
// ---------------------------------------------------------------------------

/// Draws the `tmr` Timer badge of a runtime and returns the probe's marks.
fn badge_marks_of_timer(looks: crawrad_ir::drawing::DeclaredLooks) -> Vec<crate::canvas::Mark> {
    let doc = documento_com_nao_visual("Timer", "tmr", 40.0, 40.0);
    let mut rt = MakepadRuntime::new();
    rt.com_desenhos(looks);
    rt.open(&doc).expect("opens");
    let mut probe = crate::canvas::Probe::default();
    let t = Transformacao::new(0.0, 0.0, 1.0);
    {
        let mut p = crate::canvas::Brush::probing(&mut probe, t);
        let scene = crate::components::Scene::new(&rt.ctls);
        let c = rt.ctl("tmr").expect("the Timer is in the runtime");
        let r = rect_on_screen(&t, doc.rect_in_form("tmr").expect("has a rect"));
        desenho_de("Timer").draw(&mut p, c, r, &scene);
    }
    probe.marks().to_vec()
}

#[test]
fn the_badge_wears_the_class_icon_when_the_class_has_one() {
    let clock = vec![
        crawrad_ir::icon::IconStep::Circle {
            x: 8.0,
            y: 8.0,
            r: 6.0,
        },
        crawrad_ir::icon::IconStep::Stroke,
    ];
    let mut looks = crawrad_ir::drawing::DeclaredLooks::default();
    looks.icons.insert("Timer".into(), clock);
    let marks = badge_marks_of_timer(looks);

    let badge_layer: Vec<_> = marks
        .iter()
        .filter(|m| m.layer == crate::canvas::Layer::Badge)
        .collect();
    assert_eq!(
        badge_layer.len(),
        2,
        "the badge box and its icon: {marks:?}"
    );
    let icon = badge_layer[1];
    assert_eq!(icon.rect.size, makepad_widgets::dvec2(16.0, 16.0));
    assert_eq!(
        icon.rect.pos,
        makepad_widgets::dvec2(46.0, 46.0),
        "centred in the 28 box"
    );
    assert_eq!(icon.color, MAKEPAD_NAO_VISUAL_TEXTO);
    assert!(
        !marks.iter().any(|m| m.text.as_deref() == Some("T")),
        "with an icon the initials are not written: {marks:?}"
    );
    assert!(
        marks.iter().any(|m| m.text.as_deref() == Some("tmr")),
        "the name still goes under the badge"
    );
}

#[test]
fn the_badge_falls_back_to_the_initials_when_the_class_has_no_icon() {
    let marks = badge_marks_of_timer(crawrad_ir::drawing::DeclaredLooks::default());
    assert_eq!(
        marks
            .iter()
            .filter(|m| m.layer == crate::canvas::Layer::Badge)
            .count(),
        1,
        "only the badge box: {marks:?}"
    );
    assert!(marks.iter().any(|m| m.text.as_deref() == Some("T")));
}

#[test]
fn the_icon_box_follows_the_scale_and_stays_inside_the_badge() {
    let badge = Rect {
        pos: makepad_widgets::dvec2(0.0, 0.0),
        size: makepad_widgets::dvec2(56.0, 56.0),
    };
    assert_eq!(
        icon_box(badge, 2.0).size,
        makepad_widgets::dvec2(32.0, 32.0)
    );
    let tiny = Rect {
        pos: makepad_widgets::dvec2(0.0, 0.0),
        size: makepad_widgets::dvec2(10.0, 10.0),
    };
    assert_eq!(icon_box(tiny, 1.0).size, makepad_widgets::dvec2(10.0, 10.0));
}

#[test]
fn a_catalogue_non_visual_with_no_drawing_is_a_badge_not_a_placeholder() {
    // A third-party `visual: false` class (portadb's `Connection`) brings no
    // Rust drawing: the flag, not its name, sends it to the badge.
    let doc = documento_com_nao_visual("Connection", "con", 40.0, 40.0);
    let mut looks = crawrad_ir::drawing::DeclaredLooks::default();
    looks.non_visual.insert("Connection".into());
    let mut rt = MakepadRuntime::new();
    rt.com_desenhos(looks);
    rt.open(&doc).expect("opens");
    let c = rt.ctl("con").expect("the Connection is in the runtime");
    assert_eq!(c.class, crate::components::non_visual::DECLARED_CLASS);
    assert_eq!(c.declared_class, "Connection");
    assert!(!e_visual(&c.class), "a badge, not a widget");

    let mut probe = crate::canvas::Probe::default();
    let t = Transformacao::new(0.0, 0.0, 1.0);
    {
        let mut p = crate::canvas::Brush::probing(&mut probe, t);
        let scene = crate::components::Scene::new(&rt.ctls);
        let r = rect_on_screen(&t, doc.rect_in_form("con").expect("has a rect"));
        desenho_de(&c.class).draw(&mut p, c, r, &scene);
    }
    let marks = probe.marks();
    assert!(marks.iter().any(|m| m.layer == crate::canvas::Layer::Badge));
    assert!(
        marks.iter().any(|m| m.text.as_deref() == Some("C")),
        "its own initials: {marks:?}"
    );
    assert!(marks.iter().any(|m| m.text.as_deref() == Some("con")));
}

#[test]
fn a_class_without_the_non_visual_flag_keeps_the_placeholder() {
    let doc = documento_com_nao_visual("Connection", "con", 40.0, 40.0);
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    assert_eq!(rt.ctl("con").expect("placed").class, "Connection");
}

#[test]
fn the_engine_bolt_inks_pixels_in_the_badge() {
    use crawrad_ir::icon::IconStep::*;
    let bolt = vec![
        MoveTo(10.0, 1.0),
        LineTo(10.0, 8.0),
        LineTo(3.5, 9.5),
        ClosePath,
        Fill,
        MoveTo(6.0, 15.0),
        LineTo(6.0, 8.0),
        LineTo(12.5, 6.5),
        ClosePath,
        Fill,
    ];
    // What `badge_icon` turns into runs: pixels whose ink is above zero.
    let inked = crawrad_ir::icon::rasterize(&bolt, crawrad_ir::icon::STROKE * 0.5, 16)
        .into_iter()
        .filter(|c| crawrad_ir::icon::ink_alpha(*c) > 0.0)
        .count();
    assert!(inked > 20, "the bolt inks only {inked} pixels");

    let mut looks = crawrad_ir::drawing::DeclaredLooks::default();
    looks.icons.insert("Timer".into(), bolt);
    let marks = badge_marks_of_timer(looks);
    assert_eq!(
        marks
            .iter()
            .filter(|m| m.layer == crate::canvas::Layer::Badge)
            .count(),
        2,
        "the badge box and the bolt: {marks:?}"
    );
}
