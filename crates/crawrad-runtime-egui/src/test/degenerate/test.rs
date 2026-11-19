//! **The degenerate rectangle kills no one.**
//!
//! This originated from a panic observed on the owner's screen: the IDE closed by itself
//! when he shrank the form and its border *met* a component.
//! The message was always the same —
//! `min > max, or either was NaN. min = 8.0, max = 5.0` —, and came from
//! `f32::clamp`, which **panics** when the floor does not fit within the ceiling:
//!
//! ```ignore
//! let thumb_w = (track_w * 0.3).clamp(8.0, track_w); // track_w could be 4
//! ```
//!
//! The contract is that of a classic RAD IDE, and the owner verified by looking: shrinking the
//! form over a component **clips** the component. It keeps its
//! size, the form border passes over it, and what stays outside is not
//! drawn. Nothing shrinks to a floor, nothing hangs, and **nothing dies**.
//!
//! So the fix is not a minimum size: it is for DRAWING to handle width 4,
//! width 0, and negative width. What guarantees this is geometry, at the source —
//! [`entre`] and [`barra_de_rolagem`] —, and not a patch in each branch of the
//! `match`. The four patches were literally the defect: the same line
//! copied in design-horizontal, design-vertical, live-horizontal, and live-vertical.
//!
//! Each test here was proven by mutation: undoing the fix makes it
//! red. What it measures is written in its own name.

use super::*;
use crawrad_ir::Node;

/// The measurement form and the component inside it.
fn documento(classe: &str, r: Rect, forma: Rect) -> Document {
    let mut d = Document::new("form", "Form");
    d.nodes.get_mut("form").unwrap().set_rect(forma);
    let mut n = Node::new("alvo", classe);
    n.set_rect(r);
    n.set_text("Xy");
    d.add_child("form", n).unwrap();
    d
}

/// The size of the surface where everything here is measured, in screen points.
const TELA: Vec2 = Vec2::new(900.0, 700.0);

/// A rectangle that was actually painted: what was requested, the color, and what
/// remained of it after the clipping it exited with.
#[derive(Debug, Clone, Copy)]
struct Pintado {
    pedido: ERect,
    visivel: ERect,
    cor: Color32,
}

/// Runs a real frame, without a window, and collects the painted rectangles.
///
/// The measurement is always what came out of egui, not what was requested: it is the same
/// discipline as `varredura` — requesting correctly does not prove having drawn correctly. The
/// `Shape::Vec` is flattened because in **live** mode egui widgets arrive
/// nested, and without flattening not a single `Shape::Rect` comes out of the bar.
fn pintado(d: &Document, zoom: f32, modo: Mode) -> Vec<Pintado> {
    let ctx = egui::Context::default();
    let mut rt = EguiRuntime::new(ctx.clone());
    rt.set_viewport(
        ViewportPx {
            x: 0,
            y: 0,
            w: TELA.x as i32,
            h: TELA.y as i32,
        },
        Scale(1.0),
    );
    rt.open(d).unwrap();
    rt.set_zoom(zoom);
    rt.set_mode(modo);
    let mut colhido = Vec::new();
    // THREE frames, with the clock advancing one second between them.
    //
    // Two reasons, both measured here. The first is that of `varredura`: in the
    // debut frame egui has not measured any text yet. The second is **live**
    // mode, and it cost an hour: the live form lives in an
    // `egui::Area`, and the `Area` starts with `fade_in` — in the first frame it has
    // `opacity_factor == 0.0`, and egui's `Painter::add` replaces EVERY shape with
    // `Shape::Noop` in this case. Without the clock advancing, Preview returned seven
    // `Noop`s and no rectangles, and the color of whatever survived would be dimmed
    // by partial opacity — which would ruin finding the thumb by color.
    for quadro in 0..3 {
        let entrada = egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, TELA)),
            time: Some(quadro as f64),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        rt.frame();
        let mut saida = ctx.end_pass();
        // See the `quadro` note in `tests.rs`: egui requires texture deltas.
        saida.textures_delta.clear();
        colhido.clear();
        for c in &saida.shapes {
            achatar_retangulos(&c.shape, c.clip_rect, &mut colhido);
        }
    }
    colhido
}

fn achatar_retangulos(s: &egui::Shape, corte: ERect, saida: &mut Vec<Pintado>) {
    match s {
        egui::Shape::Vec(v) => {
            for s in v {
                achatar_retangulos(s, corte, saida);
            }
        }
        egui::Shape::Rect(r) => saida.push(Pintado {
            pedido: r.rect,
            visivel: r.rect.intersect(corte),
            cor: r.fill,
        }),
        _ => {}
    }
}

/// The scroll bar thumb, which is the only rectangle painted with filled [`PAINEL_BORDA`].
/// Searching by COLOR is what makes this measurement valid in both modes: in
/// Preview the bar shares output with real egui widgets.
fn polegares(formas: &[Pintado]) -> Vec<ERect> {
    formas
        .iter()
        .filter(|f| f.cor == PAINEL_BORDA)
        .map(|f| f.pedido)
        .collect()
}

// ------------------------------------------------------------------ path 1
//
// The owner's gesture: the form shrinks and the border meets the component.

/// The form shrinks over the anchored component, and the IDE remains alive.
///
/// **This is the owner's path, reconstructed.** `Anchors::ajustar` from `crawrad-designer`
/// has no floor — `w += dw` and that's it —, so a component anchored to
/// both left AND right thins down to zero and goes negative as the form's
/// right border moves over it. The calculation is intentionally copied here:
/// the designer is not a dependency of this crate, and the test must
/// reproduce the gesture, not call the code that executes it.
///
/// Boundary MEASURED before the fix, with the component starting at 240×120:
/// `dw = -232` (the bar at 8×0) passed and `dw = -233` (7×0) died. After the
/// fix, all 301 steps pass — including those with negative width.
///
/// Mutation: restoring `(pista).max(4.0)` and `clamp(8.0, pista)` to
/// `barra_de_rolagem`, this test panics at step 233.
#[test]
fn encolher_o_formulario_por_cima_do_componente_nao_mata_a_ide() {
    let inicial = Rect::new(60.0, 40.0, 240.0, 120.0);
    for modo in [Mode::Design, Mode::Live] {
        let mut passo = 0.0f32;
        while passo <= 300.0 {
            // Left+right and top+bottom anchor: the component follows both
            // borders, which is what `ajustar` does with `w += dw`.
            let r = Rect::new(inicial.x, inicial.y, inicial.w - passo, inicial.h - passo);
            let forma = Rect::new(0.0, 0.0, 400.0 - passo, 300.0 - passo);
            let d = documento("ScrollBar", r, forma);
            let formas = pintado(&d, 1.0, modo);
            // As long as the bar still has width, its thumb keeps
            // appearing: the fix is for the drawing to HANDLE the shrunken rectangle,
            // not for drawing to give up on it.
            if r.w > 0.0 && r.h > 0.0 {
                assert!(
                    !polegares(&formas).is_empty(),
                    "the bar disappeared with form at {forma:?} and bar at {r:?}"
                );
            }
            passo += 1.0;
        }
    }
}

/// What the form border does to the component is **clip**, not
/// shrink — the classic RAD IDE contract that the owner verified by looking.
///
/// The proof is the paint: with the bar crossing the form's right border,
/// no painted rectangle extends beyond that border, **and** the component still
/// has the size given by the document — the sum of the remaining pieces covers
/// exactly up to the wall, not a rectangle shrunk down to a floor.
///
/// Mutation that this test BITES — measured, not assumed: shrinking the drawing
/// rectangle instead of clipping it, which is the classic "wrong fix". A
/// `w.min(100.0)` in `Transformacao::na_tela` turns the test red with
/// *"a barra não foi pedida no tamanho do documento"*.
///
/// **Beware of the mutation it does NOT bite**, and this is written because the
/// previous comment promised the opposite and the promise was false: a size
/// FLOOR (`w.max(8.0)`) leaves all 83 tests GREEN. The reason is the fixture — the
/// bar here is 150x20, and a floor of 8 does not touch anything above 8. Anyone
/// who wants to check a floor needs another fixture, with a side below the floor.
/// This test checks CLIPPING versus SHRINKING, and nothing else.
#[test]
fn encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe() {
    // The bar starts inside and ends 100 points AFTER the right wall.
    let forma = Rect::new(0.0, 0.0, 200.0, 150.0);
    let r = Rect::new(150.0, 40.0, 150.0, 20.0);
    let d = documento("ScrollBar", r, forma);
    let formas = pintado(&d, 1.0, Mode::Design);

    // The bar box is requested WHOLE, at the size the document gave — which
    // proves that nobody shrank it down to a floor.
    let inteira = ERect::from_min_size(Pos2::new(REGUA + r.x, REGUA + r.y), Vec2::new(r.w, r.h));
    let achada = formas.iter().find(|f| {
        (f.pedido.min - inteira.min).length() < 0.01
            && (f.pedido.size() - inteira.size()).length() < 0.01
    });
    let achada = achada
        .unwrap_or_else(|| panic!("the bar was not requested at document size ({inteira:?})"));

    // And what appears of it stops at the form wall: the form CLIPS.
    //
    // The cut falls at exact `parede`, not at `parede + META_DA_BORDA`: the half-border
    // margin belongs to COMPONENT clipping by its own rectangle, and the
    // form clipping — which comes first and is tighter — does not have it.
    let parede = REGUA + forma.w;
    assert!(
        (achada.visivel.max.x - parede).abs() < 0.01,
        "bar paint ({:?}) was not clipped at form wall at x={parede}",
        achada.visivel
    );
    // The request goes past the wall and the paint does not: the two halves of the contract.
    assert!(
        achada.pedido.max.x > parede,
        "bar did not even reach wall — scenario does not measure clipping"
    );
}

// ------------------------------------------------------------------ path 2
//
// The resize handle dragged to the floor, and any size below it.

/// No `ScrollBar` size kills drawing — neither zero nor 39.
///
/// The geometry calculation is swept ENTIRELY — the 41×41 sizes from 0 to 40 —, and the
/// real frame runs on the sizes that matter. The separation is about cost:
/// geometry is arithmetic and comes for free; an egui frame costs 8 ms, and
/// 3,362 of them would add half a minute to the suite for whoever works here every day.
///
/// The list of those that run the frame includes both edges MEASURED before the
/// fix: the long axis at **39** screen points died and at **40** lived
/// (that is `39 - 2×16 = 7`, and 7 cannot fit the thumb of 8), and on the short axis
/// **8** lived while **7** died. Dragging the handle to the resize floor of 8
/// reached it in **one** gesture.
///
/// And the measurement is not just "it didn't die": the thumb is never larger than the track
/// it moves on. That was exactly what `clamp` was trying to express and couldn't.
///
/// Mutation: restoring `clamp(8.0, pista)`, dies right at the first case (0×0).
#[test]
fn a_barra_de_rolagem_aguenta_qualquer_tamanho() {
    for w in 0..=40 {
        for h in 0..=40 {
            let r = ERect::from_min_size(Pos2::ZERO, Vec2::new(w as f32, h as f32));
            let b = barra_de_rolagem(r, 50.0);
            let deitada = r.width() > r.height();
            let pista = if deitada {
                r.width() - b.inicio.width() - b.fim.width()
            } else {
                r.height() - b.inicio.height() - b.fim.height()
            };
            let polegar = if deitada {
                b.polegar.width()
            } else {
                b.polegar.height()
            };
            assert!(
                polegar <= pista.max(0.0) + 0.01,
                "at {w}x{h} thumb ({polegar}) is larger than track ({pista})"
            );
        }
    }
    // And the full frame, which is where the panic appeared, at the measured edges.
    const BEIRADAS: [f32; 9] = [0.0, 1.0, 4.0, 7.0, 8.0, 16.0, 32.0, 39.0, 40.0];
    for modo in [Mode::Design, Mode::Live] {
        for w in BEIRADAS {
            for h in BEIRADAS {
                let d = documento(
                    "ScrollBar",
                    Rect::new(20.0, 20.0, w, h),
                    Rect::new(0.0, 0.0, 300.0, 200.0),
                );
                let _ = pintado(&d, 1.0, modo);
            }
        }
    }
}

// ------------------------------------------------------------------ path 3
//
// Zoom: the calculation runs in SCREEN points, so the threshold moves with it.

/// Zooming out does not kill a normal-sized bar.
///
/// This path requires no gesture on the component: a `ScrollBar`
/// of 40×20 — which easily passes at 100% — becomes 10×5 on screen at 25%, and that was
/// the same panic. MEASURED before the fix: at 25%, sides 40, 60, 80, and
/// 120 died (all those resulting in under 160 design points); at 50%, 40 and 60; at 75%,
/// 40. **A `Ctrl -` in the owner's IDE closed the window.**
///
/// Mutation: restoring `clamp`, dies at the first pair (25%, 40 points).
#[test]
fn dar_zoom_para_fora_nao_mata_a_barra_de_rolagem() {
    for z in crawrad_runtime::ZOOMS {
        for lado in [40.0f32, 60.0, 80.0, 120.0, 160.0, 200.0] {
            let d = documento(
                "ScrollBar",
                Rect::new(10.0, 10.0, lado, lado / 2.0),
                Rect::new(0.0, 0.0, 400.0, 300.0),
            );
            let formas = pintado(&d, z, Mode::Design);
            assert!(
                !polegares(&formas).is_empty(),
                "the {lado}-point bar disappeared with zoom at {z}"
            );
        }
    }
}

// ------------------------------------------------------------------ path 4
//
// The palette tile and template preview, which shrink the ENTIRE form.

/// A form preview fits in any tile, even one with a side of zero.
///
/// `pintar_dentro` shrinks the entire form into `area`, and the
/// resulting scale multiplies each component. With a 400×300 form
/// and a 100×60 `ScrollBar`, MEASURED before the fix: side 159
/// died, side 160 lived. And side 0 died for another reason — zero scale
/// becomes zero font size, and `epaint` responds `Bad px_scale_factor: 0`.
///
/// Mutation: removing `if area.width() <= 0.0 …` from `pintar_dentro`, side 0
/// dies again; restoring the bar's `clamp`, sides 1 to 159 die.
#[test]
fn a_previa_do_formulario_cabe_em_qualquer_ladrilho() {
    let d = documento(
        "ScrollBar",
        Rect::new(10.0, 10.0, 100.0, 60.0),
        Rect::new(0.0, 0.0, 400.0, 300.0),
    );
    // A `Label` as well: it carries the text, and text is what died on
    // zero scale. Without it side 0 would pass without proving anything.
    let mut d = d;
    let mut l = Node::new("rotulo", "Label");
    l.set_rect(Rect::new(10.0, 100.0, 200.0, 16.0));
    l.set_text("Any label");
    d.add_child("form", l).unwrap();

    for lado in [0u32, 1, 8, 32, 100, 159, 160, 320] {
        let ctx = egui::Context::default();
        let entrada = egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, TELA)),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        let p = ctx.layer_painter(egui::LayerId::background());
        let area = ERect::from_min_size(Pos2::new(10.0, 10.0), Vec2::splat(lado as f32));
        pintar_amostra_do_formulario(&p, area, &d);
        pintar_amostra(&p, area, &d);
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        let _ = lado;
    }
}

// ------------------------------------------------------------------ the golden rule
//
// Designer drawing and Preview drawing come from the SAME calculation.

/// The designer and Preview paint the same thumb, in the same position on the bar.
///
/// Previously there were two copies of the calculation, and two copies diverge on the first fix
/// that only one receives — which is precisely what
/// ADR 0013 bought by
/// choosing a single adapter. Today both call [`barra_de_rolagem`], and what
/// this test measures is that they keep calling it.
///
/// The comparison is RELATIVE to the component corner, because design has a ruler and
/// Preview does not: the same thumb falls on different screen points, and the
/// offset within the bar is what must match.
///
/// Mutation: changing the thumb fraction in ONE of the two branches (restoring the hand-
/// written calculation in one of them) turns this test red.
#[test]
fn o_designer_e_o_preview_pintam_a_mesma_barra() {
    let r = Rect::new(40.0, 60.0, 180.0, 18.0);
    let mut d = documento("ScrollBar", r, Rect::new(0.0, 0.0, 400.0, 300.0));
    d.node_mut("alvo")
        .unwrap()
        .props
        .insert("position".into(), Value::Int(70));

    let canto_design = Pos2::new(REGUA + r.x, REGUA + r.y);
    let canto_vivo = Pos2::new(r.x, r.y);

    let de_design = polegares(&pintado(&d, 1.0, Mode::Design));
    let de_vivo = polegares(&pintado(&d, 1.0, Mode::Live));
    assert_eq!(de_design.len(), 1, "design: {de_design:?}");
    assert_eq!(de_vivo.len(), 1, "live: {de_vivo:?}");

    let a = de_design[0].translate(-canto_design.to_vec2());
    let b = de_vivo[0].translate(-canto_vivo.to_vec2());
    assert!(
        (a.min - b.min).length() < 0.01 && (a.size() - b.size()).length() < 0.01,
        "designer thumb ({a:?}) does not match Preview thumb ({b:?})"
    );
}
