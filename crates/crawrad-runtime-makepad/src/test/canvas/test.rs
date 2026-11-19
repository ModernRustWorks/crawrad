//! What can be measured in the canvas **without a window**.
//!
//! Five things, and they are the five on which drawing depends: coordinate
//! conversion, gesture translation, cursor selection, WHERE the content of
//! each control comes from, and WHO can receive handles.
//!
//! No test here reads a field that it just wrote itself. What is under test
//! is always **conformance with `crawrad-designer`** — when the two readings
//! diverge, the interface lies, and that is what is asserted.

use super::*;
use crawrad_designer::{Designer, Handle};
use crawrad_ir::{Document, Value};
use crawrad_runtime::{Axis, HANDLE_TARGET, Runtime};

fn t(ox: f32, oy: f32, escala: f32) -> Transformacao {
    Transformacao::new(ox, oy, escala)
}

/// A document with a `ctl` node of the given class and rectangle, child of the root.
fn documento_de_um_no(class: &str, r: RectIr, props: &[(&str, Value)]) -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    let mut no = crawrad_ir::Node::new("ctl", class).with_prop("rect", Value::Rect(r));
    for (k, v) in props {
        no.props.insert((*k).to_string(), v.clone());
    }
    doc.add_child("form", no)
        .expect("the node is added to the root");
    doc
}

fn documento_de_uma_caixa(r: RectIr) -> Document {
    documento_de_um_no("Button", r, &[])
}

// ---------------------------------------------------------------------------
// Coordinate conversion — ADR 0019, a single transformation
// ---------------------------------------------------------------------------

#[test]
fn o_retangulo_na_tela_soma_a_origem_e_multiplica_pela_escala() {
    let r = rect_on_screen(&t(100.0, 40.0, 2.0), RectIr::new(10.0, 5.0, 80.0, 24.0));
    assert_eq!(r.pos.x, 120.0);
    assert_eq!(r.pos.y, 50.0);
    assert_eq!(r.size.x, 160.0);
    assert_eq!(r.size.y, 48.0);
}

#[test]
fn tela_para_design_desfaz_design_para_tela() {
    // Without this, a drag with zoom accumulates error and the component escapes
    // the pointer. The round trip must be the SAME calculation in reverse.
    let tr = t(37.0, -12.0, 1.5);
    for (x, y) in [(0.0, 0.0), (10.0, 320.0), (-40.0, 17.5)] {
        let (sx, sy) = tr.screen_point(x, y);
        let (dx, dy) = point_in_design(&tr, dvec2(sx as f64, sy as f64));
        assert!((dx - x).abs() < 1e-3, "x {x} -> {sx} -> {dx}");
        assert!((dy - y).abs() < 1e-3, "y {y} -> {sy} -> {dy}");
    }
}

#[test]
fn a_escala_do_canvas_e_o_zoom_vezes_o_fator_do_dispositivo() {
    use crawrad_runtime::{Runtime, Scale, ViewportPx};
    let mut rt = MakepadRuntime::new();
    rt.set_viewport(
        ViewportPx {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        Scale(2.0),
    );
    rt.set_zoom(1.5);
    assert_eq!(rt.escala(), 3.0);
    // And the canvas transform MUST use this number — otherwise drawing
    // measures one thing and the gesture measures another.
    assert_eq!(t(0.0, 0.0, rt.escala()).medida(10.0), 30.0);
}

// ---------------------------------------------------------------------------
// Gesture translation — and nothing beyond it
// ---------------------------------------------------------------------------

#[test]
fn o_gesto_chega_ao_runtime_em_unidades_de_design() {
    // The canvas origin is at (200, 100) and the zoom is 2: a click at screen
    // pixel (400, 300) is point (100, 100) on the form.
    let tr = t(200.0, 100.0, 2.0);
    let abs = dvec2(400.0, 300.0);

    let v = translate(
        Touch::Down {
            ctrl: true,
            double: false,
        },
        &tr,
        abs,
    );
    assert_eq!(
        v,
        vec![RuntimeEvent::PointerDown {
            x: 100.0,
            y: 100.0,
            ctrl: true
        }]
    );

    assert_eq!(
        translate(Touch::Move { free: false }, &tr, abs),
        vec![RuntimeEvent::PointerMove {
            x: 100.0,
            y: 100.0,
            free: false
        }]
    );
    assert_eq!(
        translate(Touch::Up, &tr, abs),
        vec![RuntimeEvent::PointerUp { x: 100.0, y: 100.0 }]
    );
    assert_eq!(
        translate(Touch::Cancel, &tr, abs),
        vec![RuntimeEvent::Cancel]
    );
}

#[test]
fn o_ctrl_e_o_duplo_clique_atravessam_a_traducao() {
    let tr = t(0.0, 0.0, 1.0);
    let sem = translate(
        Touch::Down {
            ctrl: false,
            double: false,
        },
        &tr,
        dvec2(5.0, 6.0),
    );
    assert_eq!(
        sem,
        vec![RuntimeEvent::PointerDown {
            x: 5.0,
            y: 6.0,
            ctrl: false
        }]
    );
    // A double click does not REPLACE down: the designer needs both, because
    // selection happens on pointer down and the editor opens after it.
    let duplo = translate(
        Touch::Down {
            ctrl: false,
            double: true,
        },
        &tr,
        dvec2(5.0, 6.0),
    );
    assert_eq!(duplo.len(), 2);
    assert_eq!(duplo[1], RuntimeEvent::PointerDblClick { x: 5.0, y: 6.0 });
}

#[test]
fn a_traducao_nao_inventa_evento() {
    // One touch = one event (two only on double click). If someone ever puts
    // logic here, the count changes.
    let tr = t(0.0, 0.0, 1.0);
    for toque in [Touch::Move { free: false }, Touch::Up, Touch::Cancel] {
        assert_eq!(translate(toque, &tr, dvec2(0.0, 0.0)).len(), 1);
    }
}

// ---------------------------------------------------------------------------
// The handle and the cursor — conformance with the designer is under test
// ---------------------------------------------------------------------------

#[test]
fn a_alca_do_adapter_cai_onde_o_designer_a_procura() {
    // The designer resolves clicks on handles via `Handle::anchor` + `handle_size`;
    // the adapter chooses the cursor via `handle_at_point`. If the two disagree,
    // the symptom is "the cursor changes but the handle does not grab" — and nobody
    // connects that to the cause. Here both readings are compared point by point.
    let r = RectIr::new(40.0, 30.0, 120.0, 60.0);
    for h in Handle::ALL {
        let (ax, ay) = h.anchor(r);
        assert_eq!(
            handle_at_point(r, ax, ay),
            Some(h.axis()),
            "handle {h:?} at point ({ax}, {ay})"
        );
    }
}

#[test]
fn o_alvo_da_alca_tem_a_folga_do_designer_inteira() {
    // The historical defect is not the handle in the wrong place: it is the TARGET
    // with HALF the tolerance (`crawrad-runtime-egui/src/lib.rs:1617-1623`). The
    // cursor appeared in a smaller crown than the area that actually hits, dragging
    // worked anyway, and nobody reports this.
    //
    // Measuring only at the exact anchor does not catch it: `ALCA_LADO/2` would pass.
    // That is why the test walks to the EDGE of the target on all four sides.
    let r = RectIr::new(40.0, 30.0, 120.0, 60.0);
    let folga = HANDLE_TARGET / 2.0;
    for h in Handle::ALL {
        let (ax, ay) = h.anchor(r);
        for (dx, dy) in [
            (folga - 0.5, 0.0),
            (-(folga - 0.5), 0.0),
            (0.0, folga - 0.5),
            (0.0, -(folga - 0.5)),
        ] {
            assert_eq!(
                handle_at_point(r, ax + dx, ay + dy),
                Some(h.axis()),
                "handle {h:?} missed the target at ({dx}, {dy}) — clearance smaller than ALCA_ALVO/2?"
            );
        }
        // And the designer agrees at the same point: ITS tolerance governs.
        let d = Designer::default();
        let doc = documento_de_uma_caixa(r);
        assert_eq!(
            d.hit_handle(&doc, &["ctl".to_string()], ax + folga - 0.5, ay)
                .map(|hd| hd.axis()),
            Some(h.axis()),
            "the designer and adapter disagree at the target edge of handle {h:?}"
        );
    }
}

#[test]
fn fora_do_alvo_da_alca_nao_ha_alca() {
    let r = RectIr::new(40.0, 30.0, 120.0, 60.0);
    // The center of the component is not a handle at all.
    assert_eq!(handle_at_point(r, 100.0, 60.0), None);
    // And just beyond the edge of the target, neither.
    let (ax, ay) = Handle::NW.anchor(r);
    let fora = HANDLE_TARGET / 2.0 + 0.5;
    assert_eq!(handle_at_point(r, ax - fora, ay), None);
    assert_eq!(handle_at_point(r, ax, ay - fora), None);
}

#[test]
fn o_formulario_tem_tres_alcas_e_nao_oito() {
    // Mirrors `Designer::hit_form_handle`. While the form got all eight, five
    // invisible handles captured clicks.
    let r = RectIr::new(0.0, 0.0, 480.0, 320.0);
    for h in [Handle::E, Handle::S, Handle::SE] {
        let (ax, ay) = h.anchor(r);
        assert_eq!(handle_at_point_in_form(r, ax, ay), Some(h.axis()));
    }
    for h in [Handle::NW, Handle::N, Handle::NE, Handle::W, Handle::SW] {
        let (ax, ay) = h.anchor(r);
        assert_eq!(
            handle_at_point_in_form(r, ax, ay),
            None,
            "the form cannot have handle {h:?}"
        );
    }
}

#[test]
fn cada_alca_pede_a_seta_do_seu_eixo() {
    let r = RectIr::new(40.0, 30.0, 120.0, 60.0);
    let vazio = RectIr::new(0.0, 0.0, 0.0, 0.0);
    let esperado = [
        (Handle::NW, MouseCursor::NwseResize),
        (Handle::N, MouseCursor::NsResize),
        (Handle::NE, MouseCursor::NeswResize),
        (Handle::E, MouseCursor::EwResize),
        (Handle::SE, MouseCursor::NwseResize),
        (Handle::S, MouseCursor::NsResize),
        (Handle::SW, MouseCursor::NeswResize),
        (Handle::W, MouseCursor::EwResize),
    ];
    for (h, c) in esperado {
        let (ax, ay) = h.anchor(r);
        assert_eq!(cursor_for(Some(r), vazio, ax, ay), c, "handle {h:?}");
    }
    // Far from any handle, the cursor is the default one.
    assert_eq!(
        cursor_for(Some(r), vazio, 100.0, 60.0),
        MouseCursor::Default
    );
}

#[test]
fn a_alca_do_componente_ganha_da_alca_do_formulario() {
    // Precedence is only EXERCISED when both handles fall on the same point with
    // DIFFERENT AXES. The previous fixture placed `SE` of the component over `SE`
    // of the form: both are `Nwse`, and reversing the order in `cursor_for`
    // changed nothing — the test passed both ways.
    //
    // Here the EAST handle of the form (`We`) falls exactly on the
    // SOUTHEAST handle of the component (`Nwse`):
    //
    //   form E       = (x+w+f, y+h/2) = (203,5 ; 46,5)
    //   component SE = (x+w+f, y+h+f) = (203,5 ; 46,5)
    let form = RectIr::new(0.0, 0.0, 200.0, 93.0);
    let ctl = RectIr::new(120.0, 3.0, 80.0, 40.0);
    let (ax, ay) = Handle::SE.anchor(ctl);
    assert_eq!(
        Handle::E.anchor(form),
        (ax, ay),
        "the fixture lost the match it exists to measure"
    );

    // Both geometries respond, and with different axes.
    assert_eq!(handle_at_point(ctl, ax, ay), Some(Axis::Nwse));
    assert_eq!(handle_at_point_in_form(form, ax, ay), Some(Axis::We));

    // The COMPONENT wins, as in `Designer::pointer_down`, which tests the
    // handle of the selection before that of the form.
    assert_eq!(
        cursor_for(Some(ctl), form, ax, ay),
        MouseCursor::NwseResize,
        "reversing the order in `cursor_for` must break here"
    );

    // Without a selection, the form answers for its own.
    assert_eq!(cursor_for(None, form, ax, ay), MouseCursor::EwResize);
    let (fx, fy) = Handle::S.anchor(form);
    assert_eq!(cursor_for(None, form, fx, fy), MouseCursor::NsResize);
}

// ---------------------------------------------------------------------------
// The tab strip — the copy of the constant must remain a copy
// ---------------------------------------------------------------------------

#[test]
fn a_tira_de_abas_espelha_o_catalogo() {
    // The tab drawing and `Designer::hit_aba` perform the SAME calculation in
    // opposite directions. If the catalog constant changes and this copy does not,
    // the tab appears in one place and the click finds it in another.
    assert_eq!(ALTURA_DA_TIRA_DE_ABAS, crawrad_registry::TAB_STRIP_HEIGHT);
    assert_eq!(LARGURA_DA_ABA, crawrad_registry::TAB_WIDTH);
    assert_eq!(LARGURA_DO_MAIS, crawrad_registry::PLUS_BUTTON_WIDTH);
    assert_eq!(
        crate::shared_geometry::TAB_WIDTH,
        crawrad_registry::TAB_WIDTH
    );
    assert_eq!(
        crate::shared_geometry::TAB_SCROLL_BUTTON_WIDTH,
        crawrad_registry::TAB_SCROLL_BUTTON_WIDTH
    );
}

#[test]
fn the_tab_window_copy_answers_like_the_catalog() {
    // #13: the runtime carries its own copy of the window rule, because it
    // does not depend on the catalog. A grid of inputs keeps the copy a copy.
    for width in [40.0, 60.0, 150.0, 320.0, 500.0] {
        for shown in 0..9 {
            for active in 0..9 {
                for plus in [false, true] {
                    let reserved = if plus { LARGURA_DO_MAIS } else { 0.0 };
                    let ours = crate::shared_geometry::tab_window(width, shown, active, reserved);
                    let theirs = crawrad_registry::tab_window(width, shown, active, plus);
                    assert_eq!(
                        ours,
                        (theirs.first, theirs.count, theirs.arrows),
                        "width {width}, shown {shown}, active {active}, plus {plus}"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Line-based content — WHERE it comes from
// ---------------------------------------------------------------------------

#[test]
fn o_memo_desenha_o_texto_e_nao_a_colecao_vazia() {
    // MEASURED in the owner's `form_main.crawform`: `Memo1.props.items` is `None` and the
    // content lives in `props.text`. While the canvas sent `c.itens` to draw,
    // the `Memo` box came up BLANK on the screen — the owner saw this in the
    // screenshot. RXP already read `text.lines()` (`crawrad-runtime-egui/src/lib.rs:2601`).
    let doc = documento_de_um_no(
        "Memo",
        RectIr::new(10.0, 10.0, 200.0, 90.0),
        &[("text", Value::Str("one\ntwo\nthree\nfour".into()))],
    );
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let c = rt.ctl("ctl").expect("the Memo is in the runtime");

    // The cause, made explicit: the collection is empty, and drawing came from it.
    assert!(
        c.itens.is_empty(),
        "Memo has no collection — this is where the blank box came from"
    );

    let linhas = control_lines(c);
    assert_eq!(linhas.len(), 4, "four lines of text, four lines to draw");
    assert_eq!(linhas[0], ContentLine::leaf(0, "one"));
    assert_eq!(linhas[3], ContentLine::leaf(0, "four"));
}

#[test]
fn o_treeview_achata_a_arvore_com_o_nivel_de_cada_item() {
    let filho = |t: &str| {
        Value::Map(
            [("text".to_string(), Value::Str(t.into()))]
                .into_iter()
                .collect(),
        )
    };
    let itens = Value::List(vec![Value::Map(
        [
            ("text".to_string(), Value::Str("Node1".into())),
            (
                "children".to_string(),
                Value::List(vec![filho("Sub1"), filho("Sub2")]),
            ),
        ]
        .into_iter()
        .collect(),
    )]);
    let doc = documento_de_um_no(
        "TreeView",
        RectIr::new(0.0, 0.0, 200.0, 90.0),
        &[("items", itens)],
    );
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let linhas = control_lines(rt.ctl("ctl").expect("in runtime"));
    // `Node1` has children, so it is a BRANCH: this is where the `[-]` requested
    // by the owner comes from — *"without it the tree is an indented list"*. Once
    // the tree is flattened, nobody knows who had children anymore, so the answer
    // travels with the line.
    assert_eq!(
        linhas,
        vec![
            ContentLine::branch(0, "Node1"),
            ContentLine::leaf(1, "Sub1"),
            ContentLine::leaf(1, "Sub2"),
        ]
    );
}

#[test]
fn o_numero_de_linhas_sai_da_caixa_e_nao_de_uma_constante() {
    // Six lines of 15 fit exactly in a box of 90; in an 89 box the last one is
    // left out. A fixed `take(12)` was wrong on both ends, and the overflow spilled
    // over the neighbor (`crawrad-runtime-egui/src/lib.rs:2595-2600`).
    assert_eq!(lines_that_fit(90.0, 15.0), 6);
    assert_eq!(lines_that_fit(89.0, 15.0), 5);
    // A box smaller than a single line does not draw half a line.
    assert_eq!(lines_that_fit(14.0, 15.0), 0);
    // And degenerate height does not become division by zero.
    assert_eq!(lines_that_fit(90.0, 0.0), 0);
}

// ---------------------------------------------------------------------------
// The tab strip — back and forth must match
// ---------------------------------------------------------------------------

#[test]
fn onde_a_aba_aparece_e_onde_o_clique_a_encontra() {
    // `Designer::hit_aba` resolves clicks via
    // `((x - r.x) / LARGURA_DA_ABA).floor()` (`crawrad-designer/src/lib.rs:465`).
    // `aba_i` is that calculation in reverse, and the center of each drawn tab must
    // round-trip back to its index.
    let r = RectIr::new(40.0, 20.0, 320.0, 200.0);
    for i in 0..3 {
        let aba = aba_i(r, i).unwrap_or_else(|| panic!("tab {i} fits in 320pt"));
        assert_eq!(aba.y, r.y, "the strip starts at the top of the control");
        assert!(aba.h <= ALTURA_DA_TIRA_DE_ABAS);
        let centro = aba.x + aba.w / 2.0;
        let volta = ((centro - r.x) / LARGURA_DA_ABA).floor() as usize;
        assert_eq!(volta, i, "tab {i} is drawn where the click finds {volta}");
    }
    // The fifth does not fit in 320pt — and drawing must stop, not leak out.
    assert_eq!(aba_i(r, 4), None);
}

#[test]
fn tab_scroll_buttons_sit_to_the_left_of_plus_button() {
    let r = RectIr::new(40.0, 20.0, 320.0, 200.0);
    let lx = r.x + crawrad_registry::tab_scroll_left_x(r.w, true);
    let plus = plus_rectangle(r);
    assert_eq!(
        lx + 2.0 * crawrad_registry::TAB_SCROLL_BUTTON_WIDTH,
        plus.x,
        "scroll buttons must end where the plus button begins"
    );
}

#[test]
fn o_mais_fica_encostado_na_borda_direita() {
    // Against the RIGHT edge, not after the last tab: it used to be there, and with
    // 92pt per tab in a 320 control the fifth slot started outside the control —
    // `PageControl` was stuck at four pages, since "+" is the only way to create
    // one (`crawrad-designer/src/lib.rs:459-462`).
    let r = RectIr::new(40.0, 20.0, 320.0, 200.0);
    let mais = plus_rectangle(r);
    assert_eq!(mais.x, r.x + r.w - LARGURA_DO_MAIS);
    assert_eq!(mais.x + mais.w, r.x + r.w);
}

#[test]
fn o_selo_fica_acima_do_componente_e_nao_por_cima_dele() {
    // A badge on top of the component hides what it numbers.
    let r = Rect {
        pos: dvec2(100.0, 200.0),
        size: dvec2(80.0, 24.0),
    };
    let selo = badge_rect(r, 1.0);
    assert!(
        selo.pos.y + selo.size.y <= r.pos.y,
        "the badge invaded the component"
    );
    assert_eq!(selo.pos.x, r.pos.x);
}

/// #112: the badge grows with the zoom, as the number inside it does, and it
/// still sits above the component it numbers.
#[test]
fn the_tab_order_badge_scales_with_the_canvas() {
    let r = Rect {
        pos: dvec2(100.0, 200.0),
        size: dvec2(80.0, 24.0),
    };
    let at_one = badge_rect(r, 1.0);
    let at_two = badge_rect(r, 2.0);
    assert_eq!(at_two.size, at_one.size * 2.0, "the badge ignored the zoom");
    assert_eq!(at_two.pos.y + at_two.size.y, r.pos.y);
    assert_eq!(at_two.pos.x, r.pos.x);
}

/// The tab-order badge is a circle, never an oval: its corner is half its height.
#[test]
fn the_tab_order_badge_is_square_so_it_draws_a_circle() {
    let r = Rect {
        pos: dvec2(100.0, 200.0),
        size: dvec2(80.0, 24.0),
    };
    let badge = badge_rect(r, 1.5);
    assert_eq!(badge.size.x, badge.size.y, "an oval badge");
}

// ---------------------------------------------------------------------------
// Handle over a node NOT on screen — the interface lying
// ---------------------------------------------------------------------------

/// A `PageControl` with two pages, and a `ListBox` on the second.
fn documento_com_pagina_fechada() -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 640.0, 480.0)),
        );
    }
    doc.add_child(
        "form",
        crawrad_ir::Node::new("pc", "PageControl")
            .with_prop("rect", Value::Rect(RectIr::new(20.0, 20.0, 320.0, 200.0)))
            .with_prop("active_page", Value::Int(0)),
    )
    .expect("the PageControl enters");
    for (id, i) in [("ts1", 0), ("ts2", 1)] {
        doc.add_child(
            "pc",
            crawrad_ir::Node::new(id, "TabSheet")
                .with_prop("rect", Value::Rect(RectIr::new(0.0, 24.0, 320.0, 176.0)))
                .with_prop("text", Value::Str(format!("Page{}", i + 1))),
        )
        .expect("the page enters");
    }
    doc.add_child(
        "ts2",
        crawrad_ir::Node::new("lst", "ListBox")
            .with_prop("rect", Value::Rect(RectIr::new(10.0, 10.0, 120.0, 80.0))),
    )
    .expect("the ListBox enters the closed page");
    doc
}

#[test]
fn nao_ha_alca_sobre_no_que_nao_esta_na_tela() {
    // The `ListBox` lives on the SECOND page, and the FIRST is open. The node
    // exists in the document, remains in `ctls` (the tab needs its caption) and
    // does **not** go to the screen.
    //
    // While adornments swept `ctls`, the result was eight handles floating over
    // the open page, with nothing underneath, and the designer refusing the drag.
    // RXP closed the same gap in `crawrad-runtime-egui/src/lib.rs:1828`.
    let mut doc = documento_com_pagina_fechada();
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    rt.set_selection(&["lst".to_string()]);

    assert!(rt.ctl("lst").is_some(), "the node remains in `ctls`");
    assert!(!rt.desenhavel("lst"), "but its page is closed");
    assert!(rt.desenhavel("pc"), "the PageControl is indeed on screen");

    // The proof that this matters: at the SE corner of the hidden node, GEOMETRY
    // says "handle" and the DESIGNER refuses dragging. The two conflicting answers
    // are the defect.
    let r = doc.rect_in_form("lst").expect("has rectangle");
    let (ax, ay) = Handle::SE.anchor(r);
    assert_eq!(
        handle_at_point(r, ax, ay),
        Some(Axis::Nwse),
        "pure geometry says there is a handle there"
    );
    let d = Designer::with_registry(std::rc::Rc::new(crawrad_registry::Registry::builtin()));
    assert_eq!(
        d.hit_handle(&doc, &["lst".to_string()], ax, ay),
        None,
        "and the designer refuses the drag — this is the discrepancy"
    );

    // The canvas sides with the designer: neither drawn handle nor cursor.
    assert_eq!(
        rt.rect_da_alca(),
        None,
        "no rectangle receives the 8 handles"
    );
    assert_eq!(
        cursor_for(rt.rect_da_alca(), rt.form_rect(), ax, ay),
        MouseCursor::Default,
        "the resize cursor cannot appear where nothing can be dragged"
    );

    // And the contrast, so the test does not pass by being always empty: with the
    // page switched, the SAME node gets handles again.
    if let Some(pc) = doc.node_mut("pc") {
        pc.props.insert("active_page".into(), Value::Int(1));
    }
    rt.open(&doc).expect("reopens");
    rt.set_selection(&["lst".to_string()]);
    assert!(rt.desenhavel("lst"));
    assert_eq!(rt.rect_da_alca(), Some(r));
    assert_eq!(
        cursor_for(rt.rect_da_alca(), rt.form_rect(), ax, ay),
        MouseCursor::NwseResize
    );
}

#[test]
fn o_formulario_recebe_alca_mesmo_sem_estar_em_ordem() {
    // The root never enters `ordem` — it is the surface, not a child. If
    // `desenhavel` rejected it, the three form handles would disappear too.
    let doc = documento_de_uma_caixa(RectIr::new(10.0, 10.0, 80.0, 24.0));
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    assert!(rt.desenhavel("form"));
    rt.set_selection(&["form".to_string()]);
    // But it does not get the EIGHT: its handles are three, and come from another path.
    assert_eq!(rt.rect_da_alca(), None);
}

#[test]
fn a_node_with_derived_geometry_gets_no_handle_nor_resize_cursor() {
    let doc = documento_com_pagina_fechada();
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    // TabSheet1 ("ts1") is the active page of the PageControl
    rt.set_selection(&["ts1".to_string()]);
    assert!(rt.desenhavel("ts1"), "the active tab is on screen");
    let c = rt.ctl("ts1").expect("ts1 is in ctls");
    assert!(c.derived_geometry, "TabSheet has derived geometry");

    // The designer refuses hit_handle for a node with derived geometry:
    let d = Designer::with_registry(std::rc::Rc::new(crawrad_registry::Registry::builtin()));
    let r = doc.rect_in_form("ts1").expect("has rectangle");
    let (ax, ay) = Handle::SE.anchor(r);
    assert_eq!(
        d.hit_handle(&doc, &["ts1".to_string()], ax, ay),
        None,
        "designer refuses handle on node with derived geometry"
    );

    // And the runtime agrees: rect_da_alca is None and cursor is Default
    assert_eq!(
        rt.rect_da_alca(),
        None,
        "node with derived geometry cannot receive resize handles"
    );
    assert_eq!(
        cursor_for(rt.rect_da_alca(), rt.form_rect(), ax, ay),
        MouseCursor::Default,
        "node with derived geometry cannot change cursor to resize"
    );
}

// The canvas DSL gate — the bug that fails in SILENCE — does not live here:
// it requires a child process to capture the Makepad log, and is in
// `tests/dsl_gate.rs` (`probe_canvas`), which instantiates this widget for real.

// ---------------------------------------------------------------------------
// THE LAYER STACK — the defect that appeared on the owner's screen
// ---------------------------------------------------------------------------

#[test]
fn a_pilha_de_camadas_cobre_todo_primitivo_uma_vez_so() {
    // The `match` in `open_layers` is exhaustive, so adding a primitive without
    // giving it a layer does not compile. What this test covers is the other side:
    // duplicating or forgetting an entry IN THE LIST, which compiles and breaks the screen.
    let todas = [
        Layer::Background,
        Layer::GridDot,
        Layer::Box,
        Layer::Picture,
        Layer::Text,
        Layer::Stroke,
        Layer::Arrow,
        Layer::Check,
        Layer::Badge,
        Layer::BadgeText,
        Layer::Dashed,
        Layer::Chrome,
        Layer::ChromeArrow,
        Layer::Handle,
        Layer::Ruler,
        Layer::RulerText,
    ];
    assert_eq!(LAYERS.len(), todas.len());
    for c in todas {
        assert_eq!(
            LAYERS.iter().filter(|x| **x == c).count(),
            1,
            "{c:?} must appear exactly once in the stack"
        );
    }
}

#[test]
fn a_pilha_de_camadas_protege_o_que_ja_quebrou() {
    // Each pair here is a defect that WAS on screen. Changing `LAYERS` without
    // reading this brings them back, and the `assert` indicates which.
    let nivel = |c: Layer| {
        LAYERS
            .iter()
            .position(|x| *x == c)
            .expect("every layer is in the stack")
    };

    // MEASURED in the owner's IDE: `form_main.crawform` starts with a `Label`, which
    // draws text and does not draw a box. Without the stack, the text layer opened
    // first and **every box covered all text** — both buttons came out EMPTY,
    // and in the first one the accelerator underline was left alone in the middle.
    assert!(
        nivel(Layer::Box) < nivel(Layer::Picture),
        "a picture sits above the component box"
    );
    assert!(
        nivel(Layer::Picture) < nivel(Layer::Text),
        "a picture sits below the component text"
    );
    assert!(
        nivel(Layer::Picture) < nivel(Layer::Dashed),
        "a picture sits below the marquee"
    );
    assert!(
        nivel(Layer::Box) < nivel(Layer::Text),
        "the component box cannot cover its text"
    );

    // And the accelerator underline belongs to TEXT: it must go on top.
    assert!(
        nivel(Layer::Text) < nivel(Layer::Stroke),
        "the `&OK` underline must stay above the caption"
    );

    // The grid is the artboard, and artboards stay at the bottom. While the dot
    // layer came later, dots appeared INSIDE `Button` and `Edit` —
    // counted in the screenshot, `#7a7f9a` over `#3b3b54`.
    assert!(
        nivel(Layer::GridDot) < nivel(Layer::Box),
        "the grid cannot appear inside components"
    );
    assert!(nivel(Layer::Background) < nivel(Layer::GridDot));

    // Owner's rule: *"the drawings of non-visual components in designer mode
    // must always stay in front of all visual components"*.
    for visual in [
        Layer::Box,
        Layer::Picture,
        Layer::Text,
        Layer::Stroke,
        Layer::Arrow,
        Layer::Check,
    ] {
        assert!(
            nivel(visual) < nivel(Layer::Badge),
            "the non-visual badge must stay in front of {visual:?}"
        );
    }

    // And badge initials on top of the badge — otherwise the badge hides what it
    // writes. That is what happened in the IDE: the box appeared and `AL` did not.
    assert!(
        nivel(Layer::Badge) < nivel(Layer::BadgeText),
        "badge initials must stay above its box"
    );

    // Designer adornments stay above drawing: the handle of a selected
    // component cannot disappear behind the badge of a neighboring non-visual.
    assert!(nivel(Layer::BadgeText) < nivel(Layer::Dashed));
    assert!(nivel(Layer::Dashed) < nivel(Layer::Handle));

    // CONTAINER CHROME, and the two halves of issue #129. The owner saw the
    // marquee of `button3` crossing the entire `ScrollBox1` bar while the FACE
    // of the same button stopped at it: *the same rectangle drawn under two rules*.
    //
    // 1. The bar covers the child's box — already held by emission order,
    //    and now holds by layer stack, which is stronger: emission only resolves
    //    ties WITHIN a single call.
    assert!(
        nivel(Layer::Box) < nivel(Layer::Chrome),
        "the container bar must cover the box of its children"
    );
    // 2. And covers the MARQUEE too, which is decoration — nobody grabs it.
    assert!(
        nivel(Layer::Dashed) < nivel(Layer::Chrome),
        "the selection border must stop where the component face stops"
    );
    // 3. But does **not** cover the HANDLE, and the reason is different: the handle
    //    IS what gets grabbed, and `Designer::hit_handle` does not know scrollbars
    //    exist. A hidden yet draggable handle is the interface lying — the exact
    //    opposite of what #111 fixed.
    assert!(
        nivel(Layer::Chrome) < nivel(Layer::Handle),
        "the handle cannot disappear under the bar: it is what the user grabs"
    );
    // The chrome arrow accompanies its box, as `Seta` accompanies `Caixa`:
    // without this, the button triangle renders behind the button itself.
    assert!(nivel(Layer::Chrome) < nivel(Layer::ChromeArrow));
    assert!(nivel(Layer::ChromeArrow) < nivel(Layer::Handle));

    // And the RULER is on top of everything, including adornments: it **measures**
    // what is underneath it, and a component over the number would erase the only
    // thing it is meant to tell.
    assert_eq!(
        nivel(Layer::RulerText),
        LAYERS.len() - 1,
        "the ruler number is the last step"
    );
    assert!(nivel(Layer::Handle) < nivel(Layer::Ruler));
    assert!(nivel(Layer::Ruler) < nivel(Layer::RulerText));
}

// ---------------------------------------------------------------------------
// CLIPPING BY THE CONTAINER
//
// The contract is a classic RAD IDE, and the owner sent a screenshot: shrinking
// the form over the components in a classic RAD IDE caused the parent to CLIP
// the child. The component keeps its size, the parent border overlaps it, and
// what falls outside is not drawn — *"a classic RAD IDE overlaps part of the comp"*.
// Nothing freezes, nothing shrinks.
//
// The rule asserted by these tests is the same that RXP already tests in
// `crawrad-runtime-egui/src/test/degenerado/test.rs`
// (`encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe`): **clipping
// never shrinks the draw rectangle**.
// ---------------------------------------------------------------------------

/// The form, `ScrollBox1`, and `button3` measured by the owner.
///
/// The numbers are from his report: `ScrollBox1` at (464, 488, 200, 150) —
/// right 664 — and `button3` at (576, 562, 96, 32) in the form, i.e.
/// right 672. **Leaks 8 points past the parent border**, and this leak is what
/// the tests measure.
fn documento_do_vazamento() -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 800.0, 700.0)),
        );
    }
    doc.add_child(
        "form",
        crawrad_ir::Node::new("ScrollBox1", "ScrollBox")
            .with_prop("rect", Value::Rect(RectIr::new(464.0, 488.0, 200.0, 150.0))),
    )
    .expect("the container enters the root");
    // The child rectangle is relative to the PARENT (`crawrad-ir/src/lib.rs:358`):
    // 464 + 112 = 576, and 488 + 74 = 562.
    doc.add_child(
        "ScrollBox1",
        crawrad_ir::Node::new("button3", "Button")
            .with_prop("rect", Value::Rect(RectIr::new(112.0, 74.0, 96.0, 32.0))),
    )
    .expect("the child enters the container");
    doc
}

fn runtime_do_vazamento() -> MakepadRuntime {
    let mut rt = MakepadRuntime::new();
    rt.open(&documento_do_vazamento())
        .expect("the document opens");
    rt
}

/// How much of `r` survives `corte`.
fn visivel(r: RectIr, corte: RectIr) -> RectIr {
    let x = r.x.max(corte.x);
    let y = r.y.max(corte.y);
    let direita = (r.x + r.w).min(corte.x + corte.w);
    let baixo = (r.y + r.h).min(corte.y + corte.h);
    RectIr::new(x, y, (direita - x).max(0.0), (baixo - y).max(0.0))
}

/// The component overflows the container, the container CLIPS it, and its
/// rectangle remains intact underneath.
///
/// The two halves of the contract, and both are required: without the first,
/// shrinking the component to fit would pass; without the second, the scenario
/// would not measure clipping at all.
///
/// Mutation that breaks it: making `clip_of` return the intersection — or drawing
/// passing the already clipped rectangle instead of `c.rect` — and the full
/// rectangle assertion fails.
#[test]
fn o_conteiner_recorta_o_componente_e_nao_o_encolhe() {
    let rt = runtime_do_vazamento();
    let pai = rt
        .ctl("ScrollBox1")
        .expect("the container is in the model")
        .rect;
    let c = rt.ctl("button3").expect("the child is in the model");

    // The rectangle is requested IN FULL, at the size and place given by the document.
    assert_eq!(c.rect, RectIr::new(576.0, 562.0, 96.0, 32.0));
    let pedido = Clipped::of_control(c, rt.form_rect());
    assert_eq!(
        pedido.item, c.rect,
        "the canvas asks the component for the ENTIRE rectangle; the brush does the clipping"
    );

    // And what clips is the clip rect, which is the PARENT's rectangle.
    let corte = clip_of(c, rt.form_rect());
    assert_eq!(corte, pai);
    assert_eq!(pedido.clip, Some(corte));

    // The scenario measures real clipping: the child extends past the parent's wall.
    let parede = pai.x + pai.w;
    assert!(
        c.rect.x + c.rect.w > parede,
        "the button did not even reach the container wall — the scenario does not measure clipping"
    );

    // And what shows of it stops exactly at the wall: 8 points stay outside.
    let tinta = visivel(c.rect, corte);
    assert!((tinta.x + tinta.w - parede).abs() < 0.001);
    assert!((c.rect.x + c.rect.w - parede - 8.0).abs() < 0.001);

    // What clipping removes is INK, not size.
    assert_eq!(c.rect.w, 96.0);
    assert_eq!(c.rect.h, 32.0);
}

/// Shrinking the FORM clips the component, and does not shrink it.
///
/// Port of `encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe` from
/// RXP. The form is a container like any other: whatever sits loose in it has
/// `recorte: None` (`crawrad-runtime/src/base.rs:154-157`, which only checks the
/// parent when it is not the root), and [`clip_of`] returns *the form*.
///
/// Mutation that breaks it: replacing `unwrap_or(form)` with a clip rect that
/// nothing applies — the bar then paints 100 points off the canvas.
#[test]
fn encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe() {
    // The bar starts inside and ends 100 points PAST the right wall.
    let forma = RectIr::new(0.0, 0.0, 200.0, 150.0);
    let r = RectIr::new(150.0, 40.0, 150.0, 20.0);
    let mut doc = documento_de_um_no("ScrollBar", r, &[]);
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert("rect".into(), Value::Rect(forma));
    }
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("the document opens");

    let c = rt.ctl("ctl").expect("the node is in the model");
    assert_eq!(c.clip, None, "what is loose in the form has no parent");

    // The rectangle is requested IN FULL — proving that nothing shrank it.
    assert_eq!(c.rect, r);
    assert_eq!(Clipped::of_control(c, rt.form_rect()).item, r);

    // And what appears of it stops at the form wall.
    let corte = clip_of(c, rt.form_rect());
    assert_eq!(corte, forma);
    let parede = forma.x + forma.w;
    let tinta = visivel(c.rect, corte);
    assert!(
        (tinta.x + tinta.w - parede).abs() < 0.001,
        "the bar ink ({tinta:?}) was not clipped at the form wall at x={parede}"
    );
    assert!(
        c.rect.x + c.rect.w > parede,
        "the bar did not even reach the wall — the scenario does not measure clipping"
    );
}

/// **The handle is clipped along with the component.**
///
/// Without this, the defect that died twice before returns via another path:
/// handle drawn over a part of the component that is not on screen, and
/// `Designer::hit_handle` refusing the drag — the interface lying. RXP
/// writes the same rule in `crawrad-runtime-egui/src/lib.rs:1820-1830`.
///
/// Mutation that breaks it: returning no clip on handles — the assertion fails,
/// and with it the southeast handle floats outside the container again.
#[test]
fn a_alca_e_recortada_pelo_mesmo_retangulo_que_recorta_o_componente() {
    let mut rt = runtime_do_vazamento();
    rt.set_selection(&["button3".to_string()]);
    let a = adornments(&rt);

    let pai = rt
        .ctl("ScrollBox1")
        .expect("the container is in the model")
        .rect;
    assert_eq!(
        a.handles.clip,
        Some(pai),
        "the handle must be clipped by the component's container"
    );

    // And geometry remains that of `crawrad-runtime` — eight handles, intact
    // (ADR 0019: no new coordinate calculations).
    let c = rt.ctl("button3").expect("the child is in the model").rect;
    assert_eq!(
        a.handles.item,
        crawrad_runtime::handles(c.x, c.y, c.w, c.h).to_vec(),
        "clipping cannot change WHERE the handle sits"
    );

    // At least one of them falls outside the parent — which clipping removes,
    // and is the half of the contract proving that the scenario measures something.
    let parede = pai.x + pai.w;
    assert!(
        a.handles.item.iter().any(|(x, _)| *x > parede),
        "no handle went past the container wall — the scenario does not measure anything"
    );
}

/// The selection outline follows the handle: same component, same clip.
#[test]
fn o_contorno_da_selecao_e_recortado_como_o_componente() {
    let mut rt = runtime_do_vazamento();
    rt.set_selection(&["button3".to_string()]);
    let a = adornments(&rt);
    let pai = rt
        .ctl("ScrollBox1")
        .expect("the container is in the model")
        .rect;
    assert_eq!(a.selection.len(), 1);
    assert_eq!(a.selection[0].clip, Some(pai));
    assert_eq!(
        a.selection[0].item,
        rt.ctl("button3").expect("the child is in the model").rect,
        "the outline is for the ENTIRE rectangle — the brush does the clipping"
    );
}

/// The FORM is clipped by nothing.
///
/// Its outline sits exactly on the edge, and clipping it to its own
/// rectangle would erase half of the stroke.
#[test]
fn o_formulario_selecionado_nao_e_recortado() {
    let mut rt = runtime_do_vazamento();
    rt.set_selection(&["form".to_string()]);
    let a = adornments(&rt);
    assert_eq!(a.selection.len(), 1);
    assert_eq!(a.selection[0].clip, None);
    assert_eq!(a.handles.clip, None);
    // And they remain the THREE for the form.
    assert_eq!(a.handles.item.len(), 3);
}

/// The tab order badge is clipped by the CONTAINER, not by the form.
///
/// The divergence from RXP is measured and deliberate: there the badge stays
/// INSIDE the component rectangle (`crawrad-runtime-egui/src/lib.rs:1926`), here
/// it sits ABOVE the top edge ([`badge_rect`]) so it does not hide what it
/// numbers. Clipping it by the form wall would erase the badge for the entire top
/// row — a component at `y = 0` would lose its number.
#[test]
fn o_selo_da_ordem_de_tabulacao_segue_o_recorte_do_conteiner() {
    let mut rt = runtime_do_vazamento();
    rt.set_badges(&[
        ("button3".to_string(), "1".to_string()),
        ("ScrollBox1".to_string(), "0".to_string()),
    ]);
    let a = adornments(&rt);
    let pai = rt
        .ctl("ScrollBox1")
        .expect("the container is in the model")
        .rect;

    let do_filho = a
        .badges
        .iter()
        .find(|s| s.item.1 == "1")
        .expect("the child badge was read");
    assert_eq!(do_filho.clip, Some(pai));

    // And anything unparented on the form has no clip at all: its badge is born
    // ABOVE the component, and the form wall would erase it.
    let do_conteiner = a
        .badges
        .iter()
        .find(|s| s.item.1 == "0")
        .expect("the container badge was read");
    assert_eq!(do_conteiner.clip, None);
}

// ---------------------------------------------------------------------------
// EMISSION ORDER
//
// The owner, looking at the evidence for #111: *"the `ScrollBox` clipped the
// components inside it well, but there is a detail: the components are overlapping
// the scrollbar. In a classic RAD IDE the scrollbar is still on top."*
//
// The fix is NOT subtracting the bar from the clip rect. In Makepad the bar is
// OVERLAID, not non-client area: `ScrollBars::begin` reserves no gutter and `end`
// draws the bar after the content (`makepad-widgets-1.0.0/src/scroll_bars.rs:245-250`).
// Reserving a gutter here would make the designer diverge from the generated app,
// which uses the framework's `ScrollBars`. What a classic RAD IDE and Makepad
// agree on is what the owner pointed out: **the bar stays on top**, and that is all
// these tests assert.
// ---------------------------------------------------------------------------

/// A `ScrollBox` with a child and a SIBLING after it, in the form.
///
/// The sibling exists to measure the reach of the overlay: the container bar
/// sits above ITS children, not above the entire form.
fn documento_com_irmao() -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 800.0, 700.0)),
        );
    }
    doc.add_child(
        "form",
        crawrad_ir::Node::new("ScrollBox1", "ScrollBox")
            .with_prop("rect", Value::Rect(RectIr::new(20.0, 20.0, 200.0, 150.0))),
    )
    .expect("the container enters the root");
    doc.add_child(
        "ScrollBox1",
        crawrad_ir::Node::new("button3", "Button")
            .with_prop("rect", Value::Rect(RectIr::new(10.0, 10.0, 96.0, 32.0))),
    )
    .expect("the child enters the container");
    doc.add_child(
        "form",
        crawrad_ir::Node::new("Edit1", "Edit")
            .with_prop("rect", Value::Rect(RectIr::new(300.0, 20.0, 120.0, 24.0))),
    )
    .expect("the sibling enters the root");
    doc
}

/// Frame steps as names, so the `assert` reports what broke.
fn passos(rt: &MakepadRuntime) -> Vec<String> {
    let ordem: Vec<NodeId> = rt.ordem.clone();
    draw_program(
        &draw_order(&ordem, &rt.ctls, crawrad_runtime::Mode::Design),
        &rt.ctls,
    )
    .into_iter()
    .map(|s| match s {
        Step::Paint(id) => format!("paint {id}"),
        Step::Overlay(id) => format!("overlay {id}"),
    })
    .collect()
}

/// **The container bar is emitted AFTER its children.**
///
/// This is issue #131 in full. Before this, `ScrollBox` painted itself in one go —
/// surface and bars — and every child drawn next passed over the bar; this is what
/// the owner saw in the screenshot for #111.
///
/// Mutation that breaks it: having `ScrollBox`'s `draws_over_children` return
/// `false` — the overlay step disappears and the bar goes back under `button3`.
#[test]
fn a_barra_do_conteiner_e_emitida_depois_dos_filhos_dele() {
    let mut rt = MakepadRuntime::new();
    rt.open(&documento_do_vazamento())
        .expect("the document opens");
    let v = passos(&rt);

    let i_conteiner = v
        .iter()
        .position(|s| s == "paint ScrollBox1")
        .expect("the container is painted");
    let i_filho = v
        .iter()
        .position(|s| s == "paint button3")
        .expect("the child is painted");
    let i_barra = v
        .iter()
        .position(|s| s == "overlay ScrollBox1")
        .expect("the container bar has its own step");

    assert!(
        i_conteiner < i_filho,
        "the container surface remains BEHIND children: {v:?}"
    );
    assert!(
        i_filho < i_barra,
        "the container bar must come AFTER the child — that was the defect: {v:?}"
    );
}

/// Overlay closes upon leaving the subtree, **before the sibling**.
///
/// The other half, distinguishing *"the bar sits over its children"* from
/// *"the bar sits over the form"*. In a classic RAD IDE, a sibling higher in Z-order
/// does cover the neighbor's scrollbar, because it is another window on top.
///
/// Mutation that breaks it: removing subtree closing and flushing all overlays at
/// the end of the frame — the bar then crosses `Edit1`.
#[test]
fn a_sobreposicao_do_conteiner_fecha_antes_do_irmao() {
    let mut rt = MakepadRuntime::new();
    rt.open(&documento_com_irmao()).expect("the document opens");
    let v = passos(&rt);

    assert_eq!(
        v,
        vec![
            "paint ScrollBox1".to_string(),
            "paint button3".to_string(),
            "overlay ScrollBox1".to_string(),
            "paint Edit1".to_string(),
        ],
        "the container bar must close when leaving its subtree"
    );
}

/// Anything that does not paint over children is emitted **only once**.
///
/// Without this, an overlay step for every component would double frame cost and
/// open the door for duplicate drawing to go unnoticed.
#[test]
fn quem_nao_pinta_por_cima_e_emitido_uma_vez_so() {
    let mut rt = MakepadRuntime::new();
    rt.open(&documento_com_irmao()).expect("the document opens");
    let v = passos(&rt);
    for id in ["button3", "Edit1"] {
        assert_eq!(
            v.iter().filter(|s| *s == &format!("paint {id}")).count(),
            1,
            "{id} must be painted once: {v:?}"
        );
        assert!(
            !v.contains(&format!("overlay {id}")),
            "{id} does not paint over anything: {v:?}"
        );
    }
    // And every component in order appears — the overlay must not SWALLOW
    // any step.
    assert_eq!(
        v.iter().filter(|s| s.starts_with("paint ")).count(),
        rt.ordem.len()
    );
}

/// A container with no children still keeps its bar.
///
/// Edge case measured because the stack closes on absence: an empty `ScrollBox`
/// never sees an ID from outside the subtree before the end of the frame, and
/// the final flush is what saves it.
#[test]
fn o_conteiner_vazio_nao_perde_a_barra() {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 400.0, 300.0)),
        );
    }
    doc.add_child(
        "form",
        crawrad_ir::Node::new("ScrollBox1", "ScrollBox")
            .with_prop("rect", Value::Rect(RectIr::new(10.0, 10.0, 100.0, 80.0))),
    )
    .expect("the container enters the root");
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("the document opens");
    assert_eq!(
        passos(&rt),
        vec![
            "paint ScrollBox1".to_string(),
            "overlay ScrollBox1".to_string()
        ]
    );
}

// ---------------------------------------------------------------------------
// CLIPPING BY OWN BOX
//
// *"Did you see the item label overlapped, right?"* — the `delta` of a `ListBox` painted
// BELOW the bottom border of the `ListBox` itself, over the container bar below.
// RXP closes the same gap in `crawrad-runtime-egui/src/lib.rs:2466`.
//
// This is a SECOND clip, and it does not replace the first: `clip_of` returns
// *as far as the PARENT allows to show* and `content_clip_of` returns
// *as far as the control's own ink may fall*. The two nest, intersected by
// Makepad's turtle stack.
// ---------------------------------------------------------------------------

/// The content of a control is clipped by its **full box**.
///
/// Mutation that breaks it: `content_clip_of` returning the rectangle already discounted
/// for borders, or `None` — in the former case the outline disappears, in the latter
/// `delta` leaks again.
#[test]
fn o_conteudo_e_recortado_pela_propria_caixa_do_controle() {
    let rt = runtime_do_vazamento();
    let c = rt.ctl("button3").expect("the child is in the model");
    assert_eq!(
        content_clip_of(c),
        Some(c.rect),
        "the content clip is the ENTIRE box — neither shrunk nor enlarged"
    );
}

/// **The non-visual badge is not clipped**, and this was the trap that preceded it.
///
/// The instance name is written below the badge; clipping would erase the name of
/// every `ActionList`, `ImageList`, `Timer`, and `Action`. The test measuring WHERE
/// the name lands is in `src/test/non_visual/test.rs` and was written **before**
/// this clip existed.
///
/// Mutation that breaks it: removing `clips_to_own_box` from `Selo`, or making
/// `content_clip_of` ignore the component's response.
#[test]
fn o_selo_do_nao_visual_escapa_do_recorte_da_propria_caixa() {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(RectIr::new(0.0, 0.0, 400.0, 300.0)),
        );
    }
    doc.add_child(
        "form",
        crawrad_ir::Node::new("ActionList1", "ActionList")
            .with_prop("rect", Value::Rect(RectIr::new(40.0, 40.0, 28.0, 28.0))),
    )
    .expect("the non-visual enters the root");
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("the document opens");

    let c = rt.ctl("ActionList1").expect("the badge is in the model");
    assert_eq!(
        content_clip_of(c),
        None,
        "the badge writes the name OUTSIDE the rectangle: clipping it erases the name"
    );

    // And clipping by the PARENT still holds for it — the badge does not escape
    // the form, only its own box. The two rules are independent.
    assert_eq!(clip_of(c, rt.form_rect()), rt.form_rect());
}

/// The two clips are **independent**: the parent's does not disappear because of the new one.
///
/// Without this, swapping one for the other would pass — and #111 would regress silently.
#[test]
fn o_recorte_do_pai_e_o_da_propria_caixa_nao_se_substituem() {
    let rt = runtime_do_vazamento();
    let c = rt.ctl("button3").expect("the child is in the model");
    let pai = rt
        .ctl("ScrollBox1")
        .expect("the container is in the model")
        .rect;

    assert_eq!(clip_of(c, rt.form_rect()), pai, "parent clip (#111)");
    assert_eq!(content_clip_of(c), Some(c.rect), "own box clip (#129)");
    assert_ne!(
        clip_of(c, rt.form_rect()),
        c.rect,
        "in the measured scenario both MUST be different, otherwise the test measures nothing"
    );
}

/// **#582** — a caption wider than its box is cut where the app cuts it:
/// "Project dashboard" in a 100-wide Label showed whole in the designer.
#[test]
fn a_caption_is_cut_at_its_box() {
    let width = |t: &str| t.chars().count() as f64 * 7.0;
    assert_eq!(
        super::brush::prefix_that_fits("Project dashboard", 100.0, width),
        "Project dashbo"
    );
    assert_eq!(super::brush::prefix_that_fits("OK", 100.0, width), "OK");
    assert_eq!(super::brush::prefix_that_fits("ação", 14.0, width), "aç");
}

#[test]
fn derived_geometry_matches_the_catalogue() {
    let registry = crawrad_registry::Registry::builtin();
    let mut from_catalogue: Vec<&str> = registry
        .all()
        .filter(|d| d.derived_geometry)
        .map(|d| d.class)
        .collect();
    from_catalogue.sort_unstable();
    let mut in_runtime = crawrad_runtime::DERIVED_GEOMETRY.to_vec();
    in_runtime.sort_unstable();
    assert_eq!(in_runtime, from_catalogue);
}

/// #122: a `ScrollBox` with `AutoScroll` off scrolls on no axis, even with
/// both bar properties on.
#[test]
fn a_scroll_box_without_auto_scroll_has_no_axis() {
    let mut doc = documento_do_vazamento();
    if let Some(n) = doc.node_mut("ScrollBox1") {
        n.props.insert("horz_scroll_bar".into(), Value::Bool(true));
        n.props.insert("vert_scroll_bar".into(), Value::Bool(true));
    }
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("the document opens");
    let on = rt.ctl("ScrollBox1").expect("the box is in the model");
    assert!(on.barra_h && on.barra_v, "AutoScroll defaults to on");

    if let Some(n) = doc.node_mut("ScrollBox1") {
        n.props.insert("auto_scroll".into(), Value::Bool(false));
    }
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("the document opens");
    let off = rt.ctl("ScrollBox1").expect("the box is in the model");
    assert!(!off.barra_h && !off.barra_v, "AutoScroll off still scrolls");
}

/// `Sorted`: the designer draws the items in the order the generated
/// app keeps them, without reordering what was typed.
#[test]
fn a_sorted_list_box_is_drawn_in_order() {
    let items = crawrad_ir::Item::list_to(&[
        crawrad_ir::Item::new("Pear"),
        crawrad_ir::Item::new("apple"),
        crawrad_ir::Item::new("Fig"),
    ]);
    let doc = documento_de_um_no(
        "ListBox",
        RectIr::new(10.0, 10.0, 120.0, 90.0),
        &[("items", items), ("sorted", Value::Bool(true))],
    );
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let c = rt.ctl("ctl").expect("the ListBox is in the runtime");
    let lines = control_lines(c);
    assert_eq!(lines[0], ContentLine::leaf(0, "apple"));
    assert_eq!(lines[2], ContentLine::leaf(0, "Pear"));
    assert_eq!(c.itens[0].text, "Pear", "what was typed stays as typed");
}

// ---------------------------------------------------------------------------
// SNAP MADE VISIBLE
// ---------------------------------------------------------------------------

#[test]
fn snap_off_draws_the_grid_faint_and_snap_on_strong() {
    let points = [(8.0, 8.0), (16.0, 8.0)];
    for (strong, want) in [(true, MAKEPAD_GRID), (false, MAKEPAD_GRID_FAINT)] {
        let mut probe = Probe::default();
        Brush::probing(&mut probe, t(0.0, 0.0, 1.0)).grid(&points, 1.0, strong);
        let colors: Vec<_> = probe.marks().iter().map(|m| m.color).collect();
        assert_eq!(colors, [want, want], "snap {strong}");
    }
    assert_ne!(
        MAKEPAD_GRID, MAKEPAD_GRID_FAINT,
        "ON and OFF must look different"
    );
}

#[test]
fn a_snapped_move_outlines_the_landing_and_the_pointer_and_marks_the_rulers() {
    let landing = RectIr::new(96.0, 112.0, 96.0, 30.0);
    let pointer = RectIr::new(99.0, 115.0, 96.0, 30.0);
    let a = Adornments {
        ghost: Some((landing, pointer)),
        ..Adornments::default()
    };
    let mut probe = Probe::default();
    let e = RULER_THICKNESS;
    let mut p = Brush::probing(&mut probe, t(e, e, 1.0));
    draw_adornments(&mut p, &a);
    let area = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(800.0, 600.0),
    };
    draw_ruler(&mut p, area, None, Some(landing));
    let marks = probe.marks();
    let dashed: Vec<_> = marks
        .iter()
        .filter(|m| m.layer == Layer::Dashed)
        .map(|m| (m.rect.pos.x, m.color))
        .collect();
    let (lx, px) = (f64::from(96.0 + e), f64::from(99.0 + e));
    assert_eq!(dashed, [(px, MAKEPAD_SNAP_POINTER), (lx, MAKEPAD_SNAP)]);
    let numbers: Vec<_> = marks
        .iter()
        .filter(|m| m.color == MAKEPAD_SNAP && m.text.is_some())
        .filter_map(|m| m.text.clone())
        .collect();
    assert_eq!(
        numbers,
        ["96", "112"],
        "the rulers say where the edges snapped"
    );
}

#[test]
fn no_ghost_no_accent_on_the_rulers() {
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(RULER_THICKNESS, RULER_THICKNESS, 1.0));
    let area = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(800.0, 600.0),
    };
    draw_ruler(&mut p, area, None, None);
    assert!(probe.marks().iter().all(|m| m.color != MAKEPAD_SNAP));
}

/// #538: the strip along the form is the drawing's declaration, not a class
/// name in the canvas — a MainMenu with an item paints its bar, a Button
/// paints none.
#[test]
fn form_strip_is_decided_by_drawing_catalogue_declaration() {
    let mut doc = documento_de_um_no("MainMenu", RectIr::new(8.0, 8.0, 28.0, 28.0), &[]);
    doc.add_child(
        "ctl",
        crawrad_ir::Node::new("item1", "MenuItem").with_prop("text", Value::Str("File".into())),
    )
    .expect("the item hangs on the menu");
    doc.add_child(
        "form",
        crawrad_ir::Node::new("btn1", "Button")
            .with_prop("rect", Value::Rect(RectIr::new(10.0, 60.0, 80.0, 24.0))),
    )
    .expect("the button hangs on the form");
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let form = RectIr::new(0.0, 0.0, 640.0, 480.0);
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(0.0, 0.0, 1.0));
    let scene = Scene::new(&rt.ctls);
    let menu = rt.ctl("ctl").expect("the menu is in the runtime");
    let button = rt.ctl("btn1").expect("the button is in the runtime");
    assert!(desenho_de("MainMenu").draw_form_strip(&mut p, menu, form, &scene));
    assert!(!desenho_de("Button").draw_form_strip(&mut p, button, form, &scene));
}

// ---------------------------------------------------------------------------
// PICTURES ON THE CANVAS
// ---------------------------------------------------------------------------

#[test]
fn an_image_is_registered_in_the_catalogue_and_classes() {
    assert!(crate::components::classes().contains(&"Image"));
    assert_eq!(desenho_de("Image").class_name(), "Image");
    assert!(crate::components::e_visual("Image"));
}

#[test]
fn an_empty_image_draws_a_thin_frame_and_no_generic_dashed_box() {
    let doc = documento_de_um_no("Image", RectIr::new(10.0, 10.0, 100.0, 80.0), &[]);
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let c = rt.ctl("ctl").expect("Image is in runtime");

    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(0.0, 0.0, 1.0));
    let scene = Scene::new(&rt.ctls);
    let r = p.na_tela(c.rect);
    desenho_de("Image").draw(&mut p, c, r, &scene);

    let marks = probe.marks();
    assert!(
        marks
            .iter()
            .any(|m| m.layer == Layer::Box && m.border == Some(MAKEPAD_BORDER)),
        "empty Image must draw a frame on Layer::Box"
    );
    assert!(
        marks.iter().all(|m| m.layer != Layer::Dashed),
        "empty Image is a normal state, must never draw generic dashed box"
    );
}

#[test]
fn a_bit_btn_with_undecodable_glyph_falls_back_to_empty_slot() {
    let doc = documento_de_um_no(
        "BitBtn",
        RectIr::new(10.0, 10.0, 96.0, 32.0),
        &[(
            "glyph",
            Value::Picture(crawrad_ir::Picture::from_bytes(b"corrupt".to_vec())),
        )],
    );
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let c = rt.ctl("ctl").expect("BitBtn is in runtime");

    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(0.0, 0.0, 1.0));
    let scene = Scene::new(&rt.ctls);
    let r = p.na_tela(c.rect);
    desenho_de("BitBtn").draw(&mut p, c, r, &scene);

    let marks = probe.marks();
    assert!(
        marks
            .iter()
            .any(|m| m.layer == Layer::Box && m.color == MAKEPAD_WIDGET_INPUT),
        "undecodable glyph falls back to empty slot frame"
    );
}

/// An 8x4 red PNG: what an `Image` or a `Glyph` holds in the tests below.
const PNG_8X4: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x04, 0x08, 0x06, 0x00, 0x00, 0x00, 0xB3, 0xCD, 0x7E,
    0xF0, 0x00, 0x00, 0x00, 0x15, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0xFC, 0xCF, 0xC0, 0xF0,
    0x9F, 0x01, 0x0F, 0x60, 0xC2, 0x27, 0x49, 0x1D, 0x05, 0x00, 0x04, 0xA0, 0x02, 0x06, 0xF3, 0x17,
    0xDB, 0x29, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
];

fn picture_marks(class: &str, rect: RectIr, props: &[(&str, Value)]) -> Vec<Rect> {
    let doc = documento_de_um_no(class, rect, props);
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("opens");
    let c = rt.ctl("ctl").expect("the control is in the runtime");
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(0.0, 0.0, 1.0));
    let scene = Scene::new(&rt.ctls);
    let r = p.na_tela(c.rect);
    desenho_de(class).draw(&mut p, c, r, &scene);
    probe
        .marks()
        .iter()
        .filter(|m| m.layer == Layer::Picture)
        .map(|m| m.rect)
        .collect()
}

/// #544: an Image paints its picture by the generated app's rules — its own
/// size top-left, stretched to the control, or as big as fits and centred.
#[test]
fn an_image_paints_its_picture_by_the_fit_rules() {
    let pic = || {
        (
            "picture",
            Value::Picture(crawrad_ir::Picture::from_bytes(PNG_8X4.to_vec())),
        )
    };
    let rect = RectIr::new(10.0, 10.0, 100.0, 100.0);
    let own = picture_marks("Image", rect, &[pic()]);
    assert_eq!(own.len(), 1, "one picture painted");
    assert_eq!(
        (own[0].size.x, own[0].size.y),
        (8.0, 4.0),
        "its own pixel size"
    );

    let stretched = picture_marks("Image", rect, &[pic(), ("stretch", Value::Bool(true))]);
    assert_eq!((stretched[0].size.x, stretched[0].size.y), (100.0, 100.0));

    let fitted = picture_marks(
        "Image",
        rect,
        &[
            pic(),
            ("proportional", Value::Bool(true)),
            ("center", Value::Bool(true)),
        ],
    );
    assert_eq!(
        (fitted[0].size.x, fitted[0].size.y),
        (100.0, 50.0),
        "aspect kept"
    );
    assert_eq!(fitted[0].pos.y - own[0].pos.y, 25.0, "centred vertically");
}

/// #544: a BitBtn's Glyph is painted in its 16-point slot; with two glyphs in
/// the strip only the first one shows (4x4 of the 8x4 strip, scaled to 16).
#[test]
fn a_bit_btn_paints_the_first_glyph_in_its_slot() {
    let glyph = (
        "glyph",
        Value::Picture(crawrad_ir::Picture::from_bytes(PNG_8X4.to_vec())),
    );
    let rect = RectIr::new(10.0, 10.0, 96.0, 32.0);
    let two = picture_marks(
        "BitBtn",
        rect,
        &[glyph.clone(), ("num_glyphs", Value::Int(2))],
    );
    assert_eq!(two.len(), 1);
    assert_eq!(
        (two[0].size.x, two[0].size.y),
        (16.0, 16.0),
        "a square glyph fills the slot"
    );
    let one = picture_marks("BitBtn", rect, &[glyph]);
    assert_eq!(
        (one[0].size.x, one[0].size.y),
        (16.0, 8.0),
        "the whole 8x4 strip, aspect kept"
    );
}

#[test]
fn overflow_adornment_draws_a_frame_with_pin_error_color() {
    let over = RectIr::new(10.0, 10.0, 50.0, 30.0);
    let a = Adornments {
        overflow: vec![over],
        ..Adornments::default()
    };
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, t(0.0, 0.0, 1.0));
    draw_adornments(&mut p, &a);

    let marks = probe.marks();
    assert!(
        marks
            .iter()
            .any(|m| m.layer == Layer::Stroke && m.color == MAKEPAD_PIN_ERROR[0]),
        "overflow outline must draw with MAKEPAD_PIN_ERROR[0] red color"
    );
}

#[test]
fn a_control_cut_off_marks_the_frame_edge_along_its_span() {
    let form = RectIr::new(0.0, 0.0, 390.0, 844.0);
    // wholly past the right edge: the bar sits on the edge, at the control's height
    assert_eq!(
        overflow_marks(form, RectIr::new(408.0, 228.0, 200.0, 96.0)),
        vec![RectIr::new(386.0, 228.0, 4.0, 96.0)]
    );
    // past the bottom only: a bar along the bottom edge
    assert_eq!(
        overflow_marks(form, RectIr::new(10.0, 800.0, 100.0, 100.0)),
        vec![RectIr::new(10.0, 840.0, 100.0, 4.0)]
    );
    // inside: no mark
    assert!(overflow_marks(form, RectIr::new(24.0, 24.0, 100.0, 24.0)).is_empty());
    // past both edges: two bars
    assert_eq!(
        overflow_marks(form, RectIr::new(300.0, 800.0, 200.0, 100.0)).len(),
        2
    );
}
