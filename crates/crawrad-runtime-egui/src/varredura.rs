//! **The clipping sweep**: no component paints outside its own
//! rectangle.
//!
//! Originated from a defect seen on screen: a `Memo` 90 points high with
//! twenty lines inside spilled text over the neighboring `ListBox` and
//! outside the form area. The question that came with it is what this file
//! answers, and it dictates the SHAPE of the test:
//!
//! > *"Do other components with items do this?"*
//!
//! A list of cases would answer today's question and allow component
//! 17 to be born with the same defect. So here the test is a **catalog
//! sweep**: whatever enters the `egui` target palette enters this measurement
//! without anyone writing a line, and is born with clipping enforced.
//!
//! The measurement is what was **painted**, not what was requested: each shape
//! from egui's output is valued by the bounding box of the paint it emits, already clipped by
//! the `clip_rect` it was emitted with — which is exactly what the GPU will clip.
//! Measuring the requested rectangle would only prove that we requested correctly.

use super::*;
use crawrad_ir::{Item, Node};

/// The measured component's rectangle, in design units.
///
/// The height matches the reported defect — 90 —, and the width is intentionally
/// narrow: long captions must hit the side wall as well.
fn caixa() -> Rect {
    Rect::new(40.0, 60.0, 140.0, 90.0)
}

/// The form is large and the component sits in the middle of it: if the only thing
/// containing the leak were clipping by the form, it would not be
/// measured — the test would pass by positional accident.
fn moldura() -> Rect {
    Rect::new(0.0, 0.0, 480.0, 360.0)
}

/// A caption that does not fit into 140 points of width, by far.
const LEGENDA_LONGA: &str =
    "A caption way too long to fit in this box by any means, not even with good will";

/// A name that does not fit under a 28-point badge.
const NOME_LONGO: &str = "ActionListDoFormularioPrincipalComNomeComprido";

/// Thirty lines — the owner's case had twenty, and twenty already overflowed.
fn texto_de_trinta_linhas() -> String {
    (1..=30)
        .map(|i| format!("Line {i} of text that the user typed"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Thirty items, flat or in a tree as dictated by the catalog.
///
/// What determines whether the collection has children is the registry's `colecao_em_arvore`,
/// not the class name: it is the same rule the item editor learned the
/// hard way.
fn trinta_itens(em_arvore: bool) -> Value {
    let itens: Vec<Item> = (1..=30)
        .map(|i| {
            let texto = format!("Item {i} with a very long label");
            if em_arvore {
                Item::with_children(&texto, vec![Item::new(format!("Child of {i}"))])
            } else {
                Item::new(&texto)
            }
        })
        .collect();
    Item::list_to(&itens)
}

/// How much the tab strip takes from the top of the `PageControl`.
const ALTURA_DA_ABA: f32 = 22.0;

/// A measurement scenario: the document and **where**, on the form, the measured
/// component is located.
///
/// Both things are emitted together intentionally. Asking the document later where
/// the component ended up would make the test's expectation depend on the same
/// code being measured; whoever builds the scenario knows where it was placed, and writes it.
struct Cenario {
    doc: Document,
    /// In design units, relative to the FORM's origin.
    caixa: Rect,
}

/// The measurement document: the form and, when `com_alvo`, the component
/// loaded with content.
///
/// Both documents originate here, and that is what makes the subtraction honest: the
/// frame, the ruler, and the paper are painted identically in both, so what remains
/// in the difference is paint from the component and nobody else.
///
/// The scenario is **generic and read from the catalog** — text if the descriptor has
/// text, items if it has a collection, a child breaching the wall if it has
/// a children area. The exceptions are geometrical, not drawing-related: the
/// `TabSheet` is the client area of a `PageControl`, the `Action` lives in the
/// collection of an `ActionList`, the `PageControl` receives pages instead of an
/// arbitrary child, and the non-visual gets the badge size from the catalog — stretching
/// a 28-point icon to 140 would measure a non-existent component.
fn cenario(reg: &crawrad_registry::Registry, classe: &str, com_conteudo: bool) -> Cenario {
    let mut d = Document::new("form", "Form");
    {
        let raiz = d.node_mut("form").unwrap();
        raiz.set_rect(moldura());
        raiz.set_name("form");
    }
    let desc = reg.get(classe).expect("catalog class in registry");
    let r = caixa();

    // (owner, node rectangle relative to owner, rectangle on the form)
    let (dono, alvo_r, no_formulario) = match classe {
        "TabSheet" => {
            let mut pc = Node::new("hospede", "PageControl");
            pc.set_rect(r);
            d.add_child("form", pc).unwrap();
            let corpo = Rect::new(0.0, ALTURA_DA_ABA, r.w, r.h - ALTURA_DA_ABA);
            (
                "hospede",
                corpo,
                Rect::new(r.x, r.y + ALTURA_DA_ABA, corpo.w, corpo.h),
            )
        }
        "Action" => {
            let mut al = Node::new("hospede", "ActionList");
            al.set_name("hospede");
            al.set_rect(Rect::new(300.0, 60.0, 28.0, 28.0));
            d.add_child("form", al).unwrap();
            let selo = Rect::new(0.0, 0.0, desc.default_size.0, desc.default_size.1);
            ("hospede", selo, Rect::new(300.0, 60.0, selo.w, selo.h))
        }
        _ if !desc.visual => {
            let selo = Rect::new(r.x, r.y, desc.default_size.0, desc.default_size.1);
            ("form", selo, selo)
        }
        _ => ("form", r, r),
    };
    // The component enters BOTH documents; what changes is the content. Thus the
    // subtraction isolates the CONTENT paint, and the component's own frame —
    // which falls exactly on the box edge — cancels out instead of
    // showing up as a half-point leak on every row of the table.
    let mut n = Node::new("alvo", classe);
    // The NAME is content for the non-visual: its badge has the name written
    // under the icon, and a long name is how the badge overflows.
    // For the rest, the name does not go on screen, and the line changes nothing.
    n.set_name(if com_conteudo { NOME_LONGO } else { "alvo" });
    n.set_rect(alvo_r);
    // Without content means without ANYTHING: an out-of-the-box `Button1` already has the name written
    // inside, and that caption would enter both sides of the calculation.
    n.set_text("");
    if !com_conteudo && desc.prop("col_count").is_some() {
        n.props.insert("col_count".into(), Value::Int(0));
        n.props.insert("row_count".into(), Value::Int(0));
    }
    // An empty Pagination is one page and no arrows; the sweep's box is
    // narrower than two of its square cells, so the content is the arrows and
    // the cells that follow
    if !com_conteudo && desc.prop("page_count").is_some() {
        n.props.insert("page_count".into(), Value::Int(1));
        n.props.insert("show_arrows".into(), Value::Bool(false));
    }
    if !com_conteudo && desc.prop("beveled").is_some() {
        n.props.insert("beveled".into(), Value::Bool(false));
    }
    // An empty Drawer has no header (and no caption)
    if !com_conteudo && classe == "Drawer" {
        n.props.insert("show_header".into(), Value::Bool(false));
    }
    // An empty MultiSelect has no placeholder hint
    if !com_conteudo && desc.prop("text_hint").is_some() {
        n.props
            .insert("text_hint".into(), Value::Str(String::new()));
    }
    // A DateTimePicker's content is its value; without one it shows no text
    if desc.prop("date_time").is_some() {
        let value = if com_conteudo { "2000-01-01" } else { "" };
        n.props.insert("date_time".into(), Value::Str(value.into()));
    }
    // A Bevel's content is its lines; a spacer draws none
    if desc.prop("bevel_shape").is_some() {
        let shape = if com_conteudo { "Frame" } else { "Spacer" };
        n.props
            .insert("bevel_shape".into(), Value::Enum(shape.into()));
    }
    if com_conteudo {
        if desc.prop("beveled").is_some() {
            n.props.insert("beveled".into(), Value::Bool(true));
        }
        if desc.prop("col_count").is_some() {
            n.props.insert("col_count".into(), Value::Int(10));
            n.props.insert("row_count".into(), Value::Int(10));
        }
        // The caption of `TabSheet` **does not belong to it**: what draws it is the
        // tab strip of `PageControl`, above the page body. Varying it here would make
        // the subtraction credit to the page paint that belongs to the owner — 197 points
        // of non-existent leakage. Derived geometry is the indicator, and it
        // comes from the catalog.
        if (desc.prop("text").is_some() || desc.prop("simple_text").is_some())
            && !desc.derived_geometry
        {
            // `Memo` is the only multiline text component, and what specifies this is the
            // catalog: `multiline` on the `text` property.
            if desc.prop("text").is_some_and(|p| p.multiline) {
                n.set_text(texto_de_trinta_linhas());
            } else {
                n.set_text(LEGENDA_LONGA);
                if desc.prop("simple_text").is_some() {
                    n.props
                        .insert("simple_text".into(), Value::Str(LEGENDA_LONGA.into()));
                }
            }
        }
        if let Some(p) = desc.prop("items") {
            n.props
                .insert("items".into(), trinta_itens(p.tree_collection));
            n.props.insert("item_index".into(), Value::Int(0));
        }
        if desc.prop("position").is_some() {
            n.props.insert("position".into(), Value::Int(50));
        }
        // Pages as content (a Pagination's), with numbers wide enough to widen their cells
        if desc.prop("page_count").is_some() {
            n.props.insert("page_count".into(), Value::Int(1_000_000));
            n.props.insert("page_index".into(), Value::Int(500_000));
            n.props.insert("show_arrows".into(), Value::Bool(true));
        }
        // A number as content (a SpinEdit's), as long as it gets
        if desc.prop("value").is_some() {
            n.props.insert("value".into(), Value::Int(-1_234_567_890));
        }
        // A colour outside the standard ones (a ColorBox's), named "Custom"
        if desc.prop("selected").is_some() {
            n.props.insert(
                "selected".into(),
                Value::Color(crawrad_ir::Color::rgb(1, 2, 3)),
            );
        }
        if desc.prop("shape").is_some() || desc.prop("shape_kind").is_some() {
            n.props.insert("shape".into(), Value::Enum("Circle".into()));
            n.props
                .insert("shape_kind".into(), Value::Str("Circle".into()));
        }
        if desc.prop("picture").is_some() {
            n.props.insert(
                "picture".into(),
                Value::Picture(crawrad_ir::Picture::from_bytes(vec![
                    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
                ])),
            );
        }
    }
    d.add_child(dono, n).unwrap();

    if com_conteudo && desc.children_area() {
        if classe == "PageControl" {
            // Too many pages for the strip: twelve tabs in 140 points.
            for i in 1..=12 {
                let mut ts = Node::new(format!("pag{i}"), "TabSheet");
                ts.set_name(format!("pag{i}"));
                ts.set_text(format!("Page number {i}"));
                ts.set_rect(Rect::new(
                    0.0,
                    ALTURA_DA_ABA,
                    alvo_r.w,
                    alvo_r.h - ALTURA_DA_ABA,
                ));
                d.add_child("alvo", ts).unwrap();
            }
        } else {
            // Container gets a child that BREACHES the wall: half of it
            // is born outside, and clipping by the parent must show up in the measurement.
            let mut filho = Node::new("transbordo", "Button");
            filho.set_name("transbordo");
            filho.set_text("Child that breaches the wall");
            filho.set_rect(Rect::new(alvo_r.w - 40.0, alvo_r.h - 16.0, 120.0, 32.0));
            d.add_child("alvo", filho).unwrap();
        }
    }
    Cenario {
        doc: d,
        caixa: no_formulario,
    }
}

/// The paint of a frame: the bounding box of what each shape puts on screen, already clipped
/// by the `clip_rect` it was emitted with.
///
/// `Shape::Vec` is flattened manually instead of measured whole: the union box
/// of a composite widget would swallow the relevant detail, and the subtraction
/// against the frame without the component would be coarse.
fn tinta(rt: &mut EguiRuntime, modo: Mode) -> Vec<ERect> {
    rt.set_mode(modo);
    let ctx = rt.ctx.clone();
    let mut colhido = Vec::new();
    // Two frames: egui's first frame has not measured text yet, and an unmeasured
    // galley comes out with the wrong bounding box — we would be measuring the provisional layout.
    for _ in 0..2 {
        let entrada = egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        rt.frame();
        let mut saida = ctx.end_pass();
        // egui panics if the texture delta is dropped unused:
        // it is its check warning that nothing was painted.
        saida.textures_delta.clear();
        colhido.clear();
        for c in &saida.shapes {
            achatar(&c.shape, c.clip_rect, &mut colhido);
        }
    }
    colhido
}

fn achatar(s: &egui::Shape, corte: ERect, saida: &mut Vec<ERect>) {
    let mut pinceladas = Vec::new();
    achatar_pinceladas(s, corte, &mut pinceladas);
    saida.extend(pinceladas.into_iter().map(|p| p.visivel));
}

/// A painted shape, measured on **both sides** of the clip.
///
/// `cru` is the bounding box the shape would have without any clipping; `visivel` is what
/// remains after it. The outer sweep only needed the second — and it was
/// precisely this missing half that let the `GroupBox` frame regression slip through:
/// clipping was **eating the border inward**, and anyone measuring only what escapes
/// cannot see paint that ceased to exist. It is the same loophole as
/// *"painting nothing also leaks nothing"*, on the other side of the wall.
#[derive(Debug, Clone, Copy)]
struct Pincelada {
    cru: ERect,
    visivel: ERect,
}

impl Pincelada {
    /// How much the clip cut off from this shape, in points, on the worst of the four sides.
    fn comido(&self) -> f32 {
        if !self.visivel.is_positive() {
            // Shape that was completely eliminated by clipping. What was lost is the entire shape —
            // and this is the case the owner saw: a frame line that does not disappear
            // halfway, it disappears entirely.
            return self.cru.width().max(self.cru.height());
        }
        [
            self.visivel.min.x - self.cru.min.x,
            self.visivel.min.y - self.cru.min.y,
            self.cru.max.x - self.visivel.max.x,
            self.cru.max.y - self.visivel.max.y,
        ]
        .into_iter()
        .fold(0.0, f32::max)
    }
}

/// Keeps the shape even when clipping left nothing of it.
///
/// Filtering by *anything left* would leave the frame test blind in the exact
/// worst case: a line that vanished entirely would also vanish from the measurement, and the
/// guard would conclude that the component does not draw a border.
fn achatar_pinceladas(s: &egui::Shape, corte: ERect, saida: &mut Vec<Pincelada>) {
    if let egui::Shape::Vec(v) = s {
        for s in v {
            achatar_pinceladas(s, corte, saida);
        }
        return;
    }
    let cru = s.visual_bounding_rect();
    if !cru.is_positive() || !cru.min.x.is_finite() || !cru.max.x.is_finite() {
        return;
    }
    saida.push(Pincelada {
        cru,
        visivel: cru.intersect(corte),
    });
}

/// The subtraction key. Half-point tolerance, which is more than any
/// rounding noise and less than any leak that matters.
fn chave(r: ERect) -> (i32, i32, i32, i32) {
    let q = |v: f32| (v * 2.0).round() as i32;
    (q(r.min.x), q(r.min.y), q(r.max.x), q(r.max.y))
}

/// A runtime opened on a document, with the surface at the origin.
fn runtime_de(doc: &Document) -> EguiRuntime {
    let mut rt = EguiRuntime::new(egui::Context::default());
    rt.set_viewport(
        ViewportPx {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        Scale(1.0),
    );
    rt.open(doc).unwrap();
    rt
}

/// The paint that **the content** of this component puts on screen, and the bounding box in which
/// it should fit — both in screen points.
///
/// It is the frame with content minus the frame without: the paper, the ruler, the frame of
/// the component itself, and everything else that remained unchanged cancel out, leaving
/// content paint and nothing else.
fn tinta_do_conteudo(
    reg: &crawrad_registry::Registry,
    classe: &str,
    modo: Mode,
) -> (Vec<ERect>, ERect) {
    let vazio = cenario(reg, classe, false);
    let cheio = cenario(reg, classe, true);
    let fundo = tinta(&mut runtime_de(&vazio.doc), modo);
    let com = tinta(&mut runtime_de(&cheio.doc), modo);

    let mut sobra: std::collections::HashMap<(i32, i32, i32, i32), usize> =
        std::collections::HashMap::new();
    for r in &fundo {
        *sobra.entry(chave(*r)).or_default() += 1;
    }
    let so_do_conteudo: Vec<ERect> = com
        .into_iter()
        .filter(|r| match sobra.get_mut(&chave(*r)) {
            Some(n) if *n > 0 => {
                *n -= 1;
                false
            }
            _ => true,
        })
        .collect();

    (so_do_conteudo, caixa_na_tela(modo, cheio.caixa))
}

/// **The frame**: the strokes the component paints over its own border,
/// measured before and after clipping.
///
/// Comes from the scenario WITHOUT content intentionally. Without text and without items, what the
/// component paints is background and border — so there is no need to separate frame from
/// content; the frame is already separated. And it is the opposite of `tinta_do_conteudo`, which
/// subtracts this exact frame: the frame cancels out there, which is why it
/// was invisible there.
///
/// The filter is geometric and nothing more, and **whatever draws a border identifies
/// itself through it** — there is no list of classes anywhere in this file. A shape passes
/// if it (a) fits within the component rectangle with border slack and
/// (b) **touches the border**, that is, does not fit within the rectangle shrunk by one
/// stroke thickness. The paper and the ruler are too large for (a); the grid dot,
/// which sits in the middle of the box, does not pass (b). Frame, background, and highlight pass
/// both.
fn moldura_de(
    reg: &crawrad_registry::Registry,
    classe: &str,
    modo: Mode,
) -> (Vec<Pincelada>, ERect) {
    let c = cenario(reg, classe, false);
    let mut rt = runtime_de(&c.doc);
    rt.set_mode(modo);
    let ctx = rt.ctx.clone();
    let mut pinceladas = Vec::new();
    for _ in 0..2 {
        ctx.begin_pass(egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            ..Default::default()
        });
        rt.frame();
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        pinceladas.clear();
        for s in &saida.shapes {
            achatar_pinceladas(&s.shape, s.clip_rect, &mut pinceladas);
        }
    }
    let alvo = caixa_na_tela(modo, c.caixa);
    let dentro = alvo.expand(META_DA_BORDA);
    let miolo = alvo.shrink(ESPESSURA_DA_BORDA);
    pinceladas.retain(|p| dentro.contains_rect(p.cru) && !miolo.contains_rect(p.cru));
    (pinceladas, alvo)
}

/// The worst leak of this component, in this mode, and how many shapes the content
/// painted.
///
/// The count is included because zero with zero shapes means "painted nothing" —
/// which is not the same news as "painted inside", and that is how a sweep
/// turns green without proving anything.
fn vazamento_de(reg: &crawrad_registry::Registry, classe: &str, modo: Mode) -> (f32, usize) {
    let (tinta, alvo) = tinta_do_conteudo(reg, classe, modo);
    let pior = tinta
        .iter()
        .map(|t| vazamento(*t, alvo))
        .fold(0.0, f32::max);
    (pior, tinta.len())
}

/// Where the component's bounding box falls on the SCREEN, calculated directly here.
///
/// Intentionally does not call the runtime's `Transformacao`: a test that
/// asked the code itself where it draws would measure the same function on
/// both sides and pass even if it were wrong. In design mode the origin is one ruler
/// away and zoom is 1; in live mode there is no ruler or zoom.
fn caixa_na_tela(modo: Mode, r: Rect) -> ERect {
    let o = if modo.is_live() { 0.0 } else { REGUA };
    ERect::from_min_size(Pos2::new(o + r.x, o + r.y), Vec2::new(r.w, r.h))
}

/// How much, in points, this paint leaked out of the box — the worst of the four sides.
fn vazamento(tinta: ERect, caixa: ERect) -> f32 {
    let fora = [
        caixa.min.x - tinta.min.x,
        caixa.min.y - tinta.min.y,
        tinta.max.x - caixa.max.x,
        tinta.max.y - caixa.max.y,
    ];
    fora.into_iter().fold(0.0, f32::max)
}

/// Every class declaring the `egui` target, in catalog order.
///
/// From `todos()`, and **not** from `palette()`: `TabSheet` has no entry in the
/// palette — it is born with `PageControl` — yet it is still drawn. Sweeping
/// the palette would leave out the page itself.
///
/// `Form` is excluded because it is the surface: its rectangle is the boundary for
/// the others, not a boundary of its own.
fn do_catalogo(reg: &crawrad_registry::Registry) -> Vec<&'static str> {
    let mut v: Vec<&'static str> = reg
        .all()
        .filter(|d| d.supports(ALVO) && d.class != "Form")
        .map(|d| d.class)
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

/// The sweep slack **is** the outer half of the border stroke, and nothing more.
///
/// The first version said a full point with a comment claiming that *"a
/// full point is already paint on the neighbor's screen"* — the comment contradicted the
/// number, and the reviewer caught it. Now there is no stray number: the slack is
/// [`META_DA_BORDA`], which is exactly where the border legitimately resides, and
/// measured in the same place from which clipping reads it. Half a point more than that is
/// paint on the neighbor's screen, and fails.
const FOLGA: f32 = META_DA_BORDA;

/// **The other wall.** A component's clipping must not cut off its frame.
///
/// This guard was missing, and that absence caused a regression seen on screen:
/// the `GroupBox` with its right side and bottom line missing. *"Someone touched
/// GroupBox, it wasn't clipping borders like this."*
///
/// The cause is simple arithmetic and applies to any toolkit: **a stroke of
/// thickness 1 drawn on the border axis has half of it outside the border.** Clipping
/// to the exact rectangle kills this half. `GroupBox` was the one that surfaced because it
/// draws the frame using `line_segment` centered on the border; the others use
/// `rect_stroke` with `StrokeKind::Inside`, whose bounding box `epaint` does not expand
/// (`shapes/rect_shape.rs:189-193`), and therefore survived measurement — but
/// still lost the antialiasing gradient, which is half a point of
/// paint that this measurement does not reach and the screen shows.
///
/// The sister sweep measures what **escapes**; this one measures what was **eaten**. Without
/// both, the gate has the same loophole as *"painting nothing also leaks
/// nothing"*, turned inward: an increasingly tight clip makes the
/// outer sweep increasingly green.
///
/// **Whatever draws a border identifies itself** — see the geometric filter
/// in [`moldura_de`]. There is no list of classes here, intentionally:
/// component 17 enters with its frame enforced.
#[test]
fn toda_moldura_sobrevive_ao_proprio_recorte() {
    let reg = crawrad_registry::Registry::builtin();
    let mut faltas: Vec<String> = Vec::new();
    let mut com_contorno = 0;
    for classe in do_catalogo(&reg) {
        for modo in [Mode::Design, Mode::Live] {
            let (m, alvo) = moldura_de(&reg, classe, modo);
            if !m.is_empty() {
                com_contorno += 1;
            }
            for p in m {
                let comido = p.comido();
                if comido > 0.0 {
                    faltas.push(format!(
                        "`{classe}` in {modo:?} mode: clip cut off {comido:.2} pt of \
                         shape {:?} (component box {alvo:?})",
                        p.cru
                    ));
                }
            }
        }
    }
    assert!(
        faltas.is_empty(),
        "clipping cuts off the component's own frame: {}",
        faltas.join("; ")
    );
    // The half that prevents false greens: if the filter stops finding
    // any frame — because someone changed the drawing, or because the filter
    // became too tight —, this test would pass without measuring anything.
    assert!(
        com_contorno >= 20,
        "filter found outline in only {com_contorno} class/mode pairs; \
         it stopped seeing the frame and the guard became decorative"
    );
}

/// **The guard.** No visual component from the catalog paints outside its own
/// rectangle, in either mode.
///
/// A sweep, not a list of cases: whatever enters the catalog with the `egui` target
/// enters here without anyone writing a line. A list would answer today's
/// question — *"do other components with items do this?"* — and
/// would allow component 17 to be born with the same defect.
///
/// Non-visual components are excluded and have their OWN guard right below, because
/// their expected behavior is different and is measured there.
#[test]
fn nenhum_componente_pinta_fora_da_propria_caixa() {
    let reg = crawrad_registry::Registry::builtin();
    let mut faltas: Vec<String> = Vec::new();
    let mut mudos: Vec<String> = Vec::new();
    for classe in do_catalogo(&reg) {
        if !reg.get(classe).is_some_and(|d| d.visual) {
            continue;
        }
        for modo in [Mode::Design, Mode::Live] {
            let (pior, formas) = vazamento_de(&reg, classe, modo);
            if pior > FOLGA {
                faltas.push(format!("`{classe}` in {modo:?} mode leaks {pior:.1} pt"));
            }
            // Without this half, the sweep turns green the day someone
            // stops drawing a component: painting nothing also leaks
            // nothing. Every visual class loaded with content must put
            // SOME additional paint on screen.
            if formas == 0 {
                mudos.push(format!("`{classe}` in {modo:?} mode"));
            }
        }
    }
    assert!(
        faltas.is_empty(),
        "component painting outside its own box: {}",
        faltas.join("; ")
    );
    assert!(
        mudos.is_empty(),
        "component with large content that painted NOTHING extra: {}",
        mudos.join("; ")
    );
}

/// **The caption gap does not invert the top line of `GroupBox`.**
///
/// The frame of `TGroupBox` has a notch where the caption passes through, and the notch is
/// sized by the width of the text. A caption that does not fit placed the **start** of the
/// right segment after its **end**, and egui's `line_segment` obeys:
/// it draws the line backwards, entirely outside the component.
/// Measured, in design mode, with a 46-character caption in a 140-wide box:
/// `(372,84) -> (198,00)` — 174.84 points of line over the neighbor.
///
/// **This test measures the ENDPOINTS of the segment, before any clipping**, and
/// this choice is what makes it independent of `META_DA_BORDA`: it fails the
/// incorrect drawing even in a world without any clipping. The two sister sweeps
/// did not catch this and it is worth knowing why — the outer one measures already clipped paint, and
/// clipping hid the line; the frame sweep filters by *shape contained in the
/// box*, and the outside line did not pass the filter. Clipping had traded
/// a line on the neighbor for a missing piece of frame, which was what the
/// owner saw in the screenshot.
///
/// The sweep is by **caption width**, and the four cover the three
/// regimes: loose, tight, and overflowing. Having all three is necessary, because the defect only
/// appears when the text overflows the box — and the design font (14) and live font
/// (12) overflow on different captions.
#[test]
fn o_vao_da_legenda_nao_desenha_a_moldura_fora_do_groupbox() {
    let mut estouraram = 0;
    for legenda in [
        "G",
        "Group",
        "A group with a name",
        "A caption that exceeds the entire group box",
    ] {
        for modo in [Mode::Design, Mode::Live] {
            let alvo = caixa_na_tela(modo, caixa());
            let dentro = alvo.expand(META_DA_BORDA);
            let d = moldura_do_groupbox_com(legenda, modo);
            assert!(
                d.segmentos.len() >= 5,
                "{modo:?} mode, caption {legenda:?}: only {} frame segments; \
                 `GroupBox` draws five — two top and three sides",
                d.segmentos.len()
            );
            // The gap starts at `+8` and leaves `+4` on the other side: exceeding that,
            // the top-right segment has nowhere to exist. This is the geometric
            // condition of the defect, read from the caption that was PAINTED — not from
            // how the fix was written.
            if d.largura_da_legenda > alvo.width() - 12.0 {
                estouraram += 1;
            }
            for (a, b) in d.segmentos {
                assert!(
                    dentro.contains(a) && dentro.contains(b),
                    "{modo:?} mode, caption {legenda:?}: frame was drawn from \
                     ({:.2},{:.2}) to ({:.2},{:.2}), outside box {alvo:?} — caption \
                     gap inverted segment and line drew over neighbor",
                    a.x,
                    a.y,
                    b.x,
                    b.y
                );
            }
        }
    }
    // Without a case that actually OVERFLOWS the box, the loop above is a no-op:
    // a short caption passes in any version of the code. Here we ensure that the
    // chosen widths reach the limit — and there are two because the design
    // font (14) and live font (12) overflow on different captions.
    assert!(
        estouraram >= 2,
        "no test caption exceeded usable box width \
         ({estouraram} exceeded); widths were too short and defect \
         could not appear"
    );
}

/// What a `GroupBox` with this caption painted: the frame segments and the
/// width the caption actually occupied.
struct MolduraPintada {
    /// The endpoints of each segment, in the order requested.
    segmentos: Vec<(Pos2, Pos2)>,
    /// The width of the caption galley, in screen points.
    largura_da_legenda: f32,
}

/// Measures the frame of a `GroupBox` with this caption, in this mode.
///
/// Returns the **endpoints** of the segments, not their bounding box: for the question
/// *"did the segment invert?"* the bounding box is useless — it normalizes, and a segment
/// drawn backwards comes out looking identical to a forward one.
///
/// The filter removes the design ruler, which is also made of `line_segment`:
/// the component's segment lies in its vertical band and to the right of its
/// left edge; the ruler ticks lie in the first 18 points.
fn moldura_do_groupbox_com(legenda: &str, modo: Mode) -> MolduraPintada {
    let alvo = caixa_na_tela(modo, caixa());
    let mut d = Document::new("form", "Form");
    {
        let raiz = d.node_mut("form").unwrap();
        raiz.set_rect(moldura());
        raiz.set_name("form");
    }
    let mut n = Node::new("alvo", "GroupBox");
    n.set_name("alvo");
    n.set_rect(caixa());
    n.set_text(legenda);
    d.add_child("form", n).unwrap();

    let mut rt = runtime_de(&d);
    rt.set_mode(modo);
    let ctx = rt.ctx.clone();
    let mut seg = Vec::new();
    let mut largura_da_legenda = 0.0;
    for _ in 0..2 {
        ctx.begin_pass(egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            ..Default::default()
        });
        rt.frame();
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        seg.clear();
        largura_da_legenda = 0.0;
        for c in &saida.shapes {
            colher_segmentos(&c.shape, &mut seg);
            colher_largura(&c.shape, legenda, &mut largura_da_legenda);
        }
    }
    let na_faixa = |q: Pos2| q.y >= alvo.min.y - 2.0 && q.y <= alvo.max.y + 2.0;
    seg.retain(|(a, b)| {
        na_faixa(*a) && na_faixa(*b) && (a.x >= alvo.min.x - 1.0 || b.x >= alvo.min.x - 1.0)
    });
    MolduraPintada {
        segmentos: seg,
        largura_da_legenda,
    }
}

/// The width of the text galley that matches exactly `qual`.
///
/// `galley.size()` and not `mesh_bounds`: what determines the gap is the layout box
/// that `moldura_do_groupbox` measures with `layout_no_wrap`, and that is what must
/// be compared with the usable width of the box.
fn colher_largura(s: &egui::Shape, qual: &str, maior: &mut f32) {
    match s {
        egui::Shape::Vec(v) => {
            for s in v {
                colher_largura(s, qual, maior);
            }
        }
        egui::Shape::Text(t) if t.galley.job.text == qual => {
            *maior = maior.max(t.galley.size().x);
        }
        _ => {}
    }
}

fn colher_segmentos(s: &egui::Shape, saida: &mut Vec<(Pos2, Pos2)>) {
    match s {
        egui::Shape::Vec(v) => {
            for s in v {
                colher_segmentos(s, saida);
            }
        }
        egui::Shape::LineSegment { points, .. } => saida.push((points[0], points[1])),
        _ => {}
    }
}

/// The non-visual badge escapes the box — and only **downward**.
///
/// It is the exception of a classic RAD IDE, and it is intentional: `TActionList` appears on the
/// form as an icon with the **name written underneath**, and the name is wider
/// than the icon. Without the name, two `ActionList` components on the same form are the
/// same drawing twice.
///
/// An intentional exception, not a forgotten exception: what this test checks is the SHAPE
/// of the escape. The icon stays inside the box, and everything that escapes stays below it
/// — if someone places the name next to the icon, or makes the icon itself grow
/// beyond the badge, this fails.
#[test]
fn o_selo_do_nao_visual_so_escapa_para_baixo() {
    let reg = crawrad_registry::Registry::builtin();
    for classe in do_catalogo(&reg) {
        let Some(desc) = reg.get(classe) else {
            continue;
        };
        if desc.visual {
            continue;
        }
        let (tinta, alvo) = tinta_do_conteudo(&reg, classe, Mode::Design);
        for t in tinta {
            let dentro = vazamento(t, alvo) <= FOLGA;
            let abaixo = t.min.y >= alvo.max.y - FOLGA;
            assert!(
                dentro || abaixo,
                "`{classe}` badge painted {t:?} — outside badge {alvo:?} and \
                 not below it; what escapes the badge is the NAME, and name goes \
                 under the icon",
            );
        }
        // And in the live application, the non-visual does not exist: neither icon nor name.
        let (tinta, _) = tinta_do_conteudo(&reg, classe, Mode::Live);
        assert!(
            tinta.is_empty(),
            "`{classe}` is non-visual and painted {} shapes on live form",
            tinta.len()
        );
    }
}

/// **The original defect, specifically identified.**
///
/// The owner typed twenty lines into a `Memo` 90 points high and the text overflowed:
/// over the `ListBox` next to it and outside the form. He
/// circled it in red on the screen.
///
/// The sweep above already covers this — and yet this test stays. The
/// sweep addresses the catalog and generic leaks; this one addresses the case the
/// owner saw, with the number in the message. This is what someone will read when the
/// defect recurs.
#[test]
fn o_memo_de_trinta_linhas_nao_pinta_abaixo_da_propria_borda() {
    let reg = crawrad_registry::Registry::builtin();
    for modo in [Mode::Design, Mode::Live] {
        let (tinta, alvo) = tinta_do_conteudo(&reg, "Memo", modo);
        assert!(
            !tinta.is_empty(),
            "{modo:?} mode: Memo with thirty lines painted nothing"
        );
        let abaixo = tinta
            .iter()
            .map(|t| t.max.y - alvo.max.y)
            .fold(0.0f32, f32::max);
        assert!(
            abaixo <= FOLGA,
            "{modo:?} mode: 90h Memo with thirty lines painted {abaixo:.1} pt \
             BELOW its own border (y+h = {}), over whatever is below it",
            alvo.max.y
        );
    }
}

/// **And clipping must not turn into "showing too little".**
///
/// The counterpart to the test above, and it exists because the lazy way out of
/// leaking would be a fixed line limit — which was, in fact, what was
/// written: `take(12)`. Twelve lines is too many in a 90-high box and too few in a
/// 300-high box. What dictates it is the HEIGHT, and that is what is measured here.
#[test]
fn o_memo_alto_mostra_as_linhas_que_cabem_nele() {
    let mut d = Document::new("form", "Form");
    {
        let raiz = d.node_mut("form").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 400.0));
        raiz.set_name("form");
    }
    let mut n = Node::new("alvo", "Memo");
    n.set_name("alvo");
    n.set_rect(Rect::new(40.0, 20.0, 140.0, 300.0));
    n.set_text(texto_de_trinta_linhas());
    d.add_child("form", n).unwrap();

    let mut rt = runtime_de(&d);
    rt.set_mode(Mode::Design);
    let linhas = linhas_pintadas(&mut rt);
    // A height of 300 points with line pitch 15 holds twenty lines. The floor is
    // thirteen: anything above `take(12)` proves the hardcoded limit was removed, and an
    // exact number would fail if line spacing changed by half a point.
    assert!(
        linhas > 12,
        "300h Memo with thirty lines painted only {linhas} — drawing \
         stopped at fixed number of lines instead of end of box"
    );
}

/// How many lines of `Memo` text were actually painted in this frame.
fn linhas_pintadas(rt: &mut EguiRuntime) -> usize {
    let ctx = rt.ctx.clone();
    let mut quantas = 0;
    for _ in 0..2 {
        ctx.begin_pass(egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            ..Default::default()
        });
        rt.frame();
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        quantas = 0;
        for c in &saida.shapes {
            contar_linhas(&c.shape, &mut quantas);
        }
    }
    quantas
}

fn contar_linhas(s: &egui::Shape, quantas: &mut usize) {
    match s {
        egui::Shape::Vec(v) => {
            for s in v {
                contar_linhas(s, quantas);
            }
        }
        egui::Shape::Text(t) if t.galley.job.text.starts_with("Line ") => *quantas += 1,
        _ => {}
    }
}

/// **Divergence D3 from the deep-dive study**, resolved.
///
/// `ListBox` showed EIGHT items in the designer and FOUR in the application, in the same
/// 90-point box: line pitch of 16 on one side and 18 on the other. The four excess
/// items in the designer ended up over the neighbor.
///
/// The verification is the COUNT of what was painted in both modes, not the constant's
/// value: a test comparing `LINHA_DA_LISTA` to itself would pass
/// with both sides wrong.
#[test]
fn o_listbox_mostra_o_mesmo_tanto_de_itens_no_designer_e_no_aplicativo() {
    let reg = crawrad_registry::Registry::builtin();
    let (design, _) = tinta_do_conteudo(&reg, "ListBox", Mode::Design);
    let (vivo, _) = tinta_do_conteudo(&reg, "ListBox", Mode::Live);
    assert!(
        !design.is_empty() && !vivo.is_empty(),
        "ListBox with thirty items painted nothing: design {}, live {}",
        design.len(),
        vivo.len()
    );
    assert_eq!(
        design.len(),
        vivo.len(),
        "in a 90-point box designer paints {} item shapes and app \
         paints {} — this is divergence D3, lying to user \
         about how many items fit",
        design.len(),
        vivo.len()
    );
}

/// **The other half of the request, and where the two modes diverge by design.**
///
/// The owner requested clipping *without a scrollbar*, and that applies to the DESIGNER:
/// `TMemo` in the classic RAD IDE designer shows what fits and does not scroll, because there the
/// interaction belongs to the designer, not the form user. In the application it is the
/// opposite — an edit control that swallows typed text without allowing scrolling
/// back would be worse than one that overflows.
///
/// The verification is the POSITION of the painted text after a mouse wheel scroll. In live mode
/// it scrolls up; in design mode it stays put. Without `ScrollArea`, live mode remained
/// static — clipping held back the paint, and no geometric test noticed
/// that lines 6 through 30 had become unreachable.
#[test]
fn o_memo_rola_no_aplicativo_e_nao_rola_no_designer() {
    let reg = crawrad_registry::Registry::builtin();
    for (modo, deve_rolar) in [(Mode::Live, true), (Mode::Design, false)] {
        let c = cenario(&reg, "Memo", true);
        let dentro = caixa_na_tela(modo, c.caixa).center();
        // A runtime for each measurement, not the same one twice: egui's clock
        // does not move backward (it panics with a message saying so), and resetting
        // to time zero for the second measurement would crash the test for a reason
        // unrelated to scrolling.
        let medir = |roda: f32| {
            let mut rt = runtime_de(&c.doc);
            rt.set_mode(modo);
            topo_do_texto(&mut rt, dentro, roda)
        };
        let parado = medir(0.0);
        let rolado = medir(-60.0);
        if deve_rolar {
            assert!(
                rolado < parado - 1.0,
                "{modo:?} mode: mouse wheel did not move Memo text — \
                 it was at {parado:.1} and stayed at {rolado:.1}; overflowing \
                 lines became unreachable"
            );
        } else {
            assert!(
                (rolado - parado).abs() < 1.0,
                "{modo:?} mode: mouse wheel scrolled DESIGNER Memo — from \
                 {parado:.1} to {rolado:.1}; in design gesture belongs to designer"
            );
        }
    }
}

/// Where the topmost text painted by this runtime begins vertically, after
/// `voltas` mouse wheel turns over `ponteiro`.
///
/// Eight frames, with the wheel engaged only from the fourth frame onward. Both reasons were
/// measured: egui scrolling is **smoothed over time** (enters as
/// `smooth_scroll_delta` and spreads over several frames), and `ScrollArea` only
/// accepts scrolling down once it knows the content exceeds the window
/// — and it only knows this **after** drawing the content once. Spinning
/// the wheel on the first frame scrolled nothing, making the test fail for a
/// reason unrelated to the defect.
fn topo_do_texto(rt: &mut EguiRuntime, ponteiro: Pos2, roda: f32) -> f32 {
    let ctx = rt.ctx.clone();
    let mut topo = f32::MAX;
    for passe in 0..8 {
        let mut eventos = vec![egui::Event::PointerMoved(ponteiro)];
        if roda != 0.0 && (3..5).contains(&passe) {
            eventos.push(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: Vec2::new(0.0, roda),
                // `Move` is what egui specifies to use when the phase is
                // unknown, and a mouse wheel has no phase.
                phase: egui::TouchPhase::Move,
                modifiers: egui::Modifiers::default(),
            });
        }
        ctx.begin_pass(egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            time: Some(passe as f64 * 0.25),
            events: eventos,
            ..Default::default()
        });
        rt.frame();
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        topo = f32::MAX;
        for c in &saida.shapes {
            achatar_texto(&c.shape, &mut topo);
        }
    }
    topo
}

fn achatar_texto(s: &egui::Shape, topo: &mut f32) {
    match s {
        egui::Shape::Vec(v) => {
            for s in v {
                achatar_texto(s, topo);
            }
        }
        egui::Shape::Text(t) if t.galley.rows.len() > 5 => *topo = topo.min(t.pos.y),
        _ => {}
    }
}

/// And the 18 for the list line **is not ours**.
///
/// The test above proves both sides agree; this one proves they
/// agree on the RIGHT number. Without it, modifying [`LINHA_DA_LISTA`] changed the
/// designer and Preview together while the test suite stayed green — while the
/// emitted application, which has 18 hardcoded in `rxp.rs`, fell behind.
/// Measured: with the constant at 16 no tests failed.
///
/// The anchor is `interact_size.y` from the paper style, which is the height at which
/// egui's `selectable_label` draws a line. That is where 18 came from, and that is
/// what governs changes to it — if egui changes, this line warns, and
/// `rxp.rs` must be updated in lockstep.
#[test]
fn a_linha_da_lista_e_a_altura_com_que_o_egui_desenha_uma_linha() {
    let alvo = estilo_do_papel().spacing.interact_size.y;
    assert_eq!(
        LINHA_DA_LISTA, alvo,
        "ListBox line is at {LINHA_DA_LISTA} and egui draws its line \
         at {alvo}; `rxp.rs` hardcodes the number and will fall behind"
    );
}

/// The FRAME report: how much clipping cut off from each class's border.
/// `cargo test -p crawrad-runtime-egui --lib -- --ignored --nocapture molduras`
#[test]
#[ignore = "report, not a guard"]
fn molduras() {
    let reg = crawrad_registry::Registry::builtin();
    println!("| component | mode | shapes | worst eaten | bleeding shape |");
    for classe in do_catalogo(&reg) {
        for modo in [Mode::Design, Mode::Live] {
            let (m, alvo) = moldura_de(&reg, classe, modo);
            let pior = m.iter().map(Pincelada::comido).fold(0.0, f32::max);
            let quem = m
                .iter()
                .filter(|p| p.comido() > 0.0)
                .map(|p| format!("{:?}", p.cru))
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "| {classe:12} | {modo:?} | {:2} | {pior:4.2} | {quem} (target {alvo:?})",
                m.len()
            );
        }
    }
}

/// The sweep report, for anyone who wants numbers per component.
/// `cargo test -p crawrad-runtime-egui --lib -- --ignored --nocapture tabela`
#[test]
#[ignore = "report, not a guard"]
fn tabela() {
    let reg = crawrad_registry::Registry::builtin();
    println!("| component | design | live |");
    for classe in do_catalogo(&reg) {
        let mut col = Vec::new();
        for modo in [Mode::Design, Mode::Live] {
            let (pior, formas) = vazamento_de(&reg, classe, modo);
            col.push(format!("{pior:6.1} pt ({formas} shapes)"));
        }
        println!("| {classe:12} | {} | {} |", col[0], col[1]);
    }
}
