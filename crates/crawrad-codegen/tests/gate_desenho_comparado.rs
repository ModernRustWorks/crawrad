//! **The rendering gate: the same document, painted by all three sides.**
//!
//! `gate_fase3` proves that the generated project compiles and that clicking runs the
//! handler. This gate enforces the golden rule that remains after that: **what the
//! designer draws must be what the application shows.**
//!
//! It exists because that question was already answered with "no" seven times. The
//! study measured seven divergences between the three arms that
//! render the same component, and none of them failed any test: the
//! generator unit tests verify the emitted TEXT, and correct text over
//! differing constants is precisely the bug. The `Label` in the common document
//! was 42 pt out of place with all tests passing.
//!
//! ## The method, and why it is the only measurement that counts
//!
//! All three sides paint the SAME [`documento`], and what is compared are the
//! **painted shapes** — not code, not emitted text:
//!
//! | side | renderer | where it runs |
//! |---|---|---|
//! | **DESIGN** | `crawrad-runtime-egui`, the canvas hand painting (`Modo::Design`) | in this process |
//! | **LIVE** | `crawrad-runtime-egui`, the real widgets of the Preview (`Modo::Vivo`) | in this process |
//! | **APPLICATION** | the `src/form_main.designer.rs` just emitted by `crawrad-codegen` | inside a `cargo test` in the generated project |
//!
//! The third is what gives the gate its value, and it is what costs time: the generated
//! project **cannot depend on our crates** (there is a test guaranteeing this), so
//! there is no way to load it in-process. It is written to disk, compiled by real
//! `cargo`, and run headless; what returns is the shape dump from
//! stdout, tagged line by line. **Emitted text is compiled
//! byte-for-byte as it came out of the generator.**
//!
//! ## Exceptions are declared, not tolerated
//!
//! Seven live divergences cannot become seven `#[ignore]` attributes, much less a
//! gate that only looks at what is already correct — that would be a false green. They are
//! listed in [`DIVERGENCIAS_DECLARADAS`], **each with its number measured across the three
//! sides and the issue that closes it**, and the gate enforces the list in both directions,
//! matching the mechanics of `EVENTOS_NAO_DISPARADOS`:
//!
//! - divergence that appears and is **not** declared → red;
//! - declared divergence whose number **changed** → red, reporting previous
//!   and current values (partial fixes do not pass by accident);
//! - declared divergence that **disappeared** → red, requiring line deletion.
//!
//! The last rule prevents the list from becoming obsolete: fixing an issue without
//! deleting the exception fails the test.
//!
//! ## What this gate does NOT measure
//!
//! **Input gestures**. Click, keystroke, and focus belong to `gate_fase3`; here only
//! painted shapes are inspected. The `TreeView` accordion not collapsing does not
//! appear as any shape and therefore is not in the list.
//!
//! And it does not measure **anchoring**, which is resizing geometry: that has its
//! own lightweight gate, `a_regra_da_ancora_emitida_e_a_do_designer` in
//! `src/tests.rs`, which tests the `crawrad-designer` rule against emitted text
//! without invoking `cargo`. What this file adds is the PAINTED proof of the
//! same thing, in [`a_ancora_pintada_pelo_aplicativo_e_a_que_a_ide_calcula`].
//!
//! ## Cost, and why it is marked `#[ignore]`
//!
//! Measured on this machine on 2026-09-09, with `cargo test -p crawrad-codegen --test
//! gate_desenho_comparado -- --include-ignored`:
//!
//! | run | time |
//! |---|---|
//! | **cold** (with `target/gate-desenho` deleted: `cargo new`, `cargo add`, compile generated project) | **103 s** |
//! | **warm** (with `target/` of the generated project preserved) | **5.2 s** |
//! | the three fast tests here, standalone | milliseconds |
//!
//! For comparison, in the same run: `gate_fase3` took **181 s** and `gate_sem_avisos`
//! took **96 s**. This gate sits right between them, which is why it shares their
//! placement: `#[ignore]`. The criterion is not just the clock — it is that
//! it **invokes `cargo` from within `cargo`** and may require the crates.io index,
//! which a standard test suite cannot demand.
//!
//! What stays in the normal suite are the three tests guarding the gate itself:
//! the dump parser, the consistency of the exception list, and the tag sync
//! with [`ARQUIVO_DO_USUARIO`]. They invoke neither `cargo` nor disk, and exist
//! so that an expensive gate not run every day has a check against rot.
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_desenho_comparado -- --ignored --nocapture
//! ```

mod comum;

use comum::manifesto_intocado;
use crawrad_codegen::Gerador as _;
use crawrad_designer::layout::Anchoring;
use crawrad_ir::{Document, Item, Node, Rect, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

// ===========================================================================
// THE COMMON DOCUMENT
// ===========================================================================

/// **The same document for all three sides.** A single one, written once.
///
/// It is not an attractive form: it is a sample showcasing every
/// component whose drawing has diverged, plus those that agree — because
/// agreement is also an outcome, without which the gate could only report
/// bugs.
///
/// `btn_livre` is here for a different reason than the others: it is the only one
/// **without anchors on any side**, and it is what
/// [`a_ancora_pintada_pelo_aplicativo_e_a_que_a_ide_calcula`] tracks.
fn documento() -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the newly created root");
        raiz.set_rect(Rect::new(0.0, 0.0, LARGURA, ALTURA));
        raiz.set_name("form_main");
        raiz.set_text("Cadastro");
    }

    let mut n = |pai: &str, id: &str, classe: &str, r: Rect, texto: &str| {
        let mut x = Node::new(id, classe);
        x.set_name(id);
        if !texto.is_empty() {
            x.set_text(texto);
        }
        x.set_rect(r);
        d.add_child(pai, x).expect("the child enters");
    };

    n(
        "form_main",
        "lbl_a",
        "Label",
        Rect::new(10.0, 10.0, 120.0, 20.0),
        "&Nome",
    );
    n(
        "form_main",
        "btn_a",
        "Button",
        Rect::new(10.0, 40.0, 96.0, 32.0),
        "&OK",
    );
    n(
        "form_main",
        "edt_a",
        "Edit",
        Rect::new(10.0, 80.0, 200.0, 24.0),
        "designer",
    );
    n(
        "form_main",
        "mem_a",
        "Memo",
        Rect::new(10.0, 110.0, 200.0, 60.0),
        "L1\nL2",
    );
    n(
        "form_main",
        "chk_a",
        "CheckBox",
        Rect::new(10.0, 180.0, 140.0, 20.0),
        "&Ativo",
    );
    n(
        "form_main",
        "rad_a",
        "RadioButton",
        Rect::new(10.0, 205.0, 140.0, 20.0),
        "&Um",
    );
    n(
        "form_main",
        "pnl_a",
        "Panel",
        Rect::new(240.0, 10.0, 180.0, 60.0),
        "Panel",
    );
    n(
        "form_main",
        "grp_a",
        "GroupBox",
        Rect::new(240.0, 80.0, 180.0, 80.0),
        "&Grupo",
    );
    n(
        "form_main",
        "lst_a",
        "ListBox",
        Rect::new(240.0, 170.0, 180.0, 80.0),
        "",
    );
    n(
        "form_main",
        "cbo_a",
        "ComboBox",
        Rect::new(440.0, 10.0, 150.0, 24.0),
        "",
    );
    n(
        "form_main",
        "trv_a",
        "TreeView",
        Rect::new(440.0, 50.0, 180.0, 120.0),
        "",
    );
    // THE SECOND tree, and it is a sibling of the first — neither inside the other.
    // This is the case that `Ui::id()` does not distinguish, and no probe measures: it
    // exists for the test's `duas_arvores_irmas_nao_dividem_o_alvo_do_clique`,
    // which runs inside the compiled application.
    //
    // **The corner was chosen deliberately, not randomly:** `(500, 190)` lies outside
    // the region of every probe in this gate — `trv_a` ends at y=178,
    // `lst_a` at x=480 (it uses 60 padding), and `btn_livre` starts at
    // y=392. Moving this box into any of them would alter probe numbers
    // without touching any drawing code.
    n(
        "form_main",
        "trv_b",
        "TreeView",
        Rect::new(500.0, 190.0, 130.0, 100.0),
        "",
    );
    n(
        "form_main",
        "pgc_a",
        "PageControl",
        Rect::new(10.0, 240.0, 400.0, 120.0),
        "",
    );
    n(
        "pgc_a",
        "ts_a",
        "TabSheet",
        Rect::new(0.0, 24.0, 400.0, 96.0),
        "Um",
    );
    n(
        "pgc_a",
        "ts_b",
        "TabSheet",
        Rect::new(0.0, 24.0, 400.0, 96.0),
        "Dois",
    );
    n(
        "form_main",
        "btn_livre",
        "Button",
        Rect::new(500.0, 400.0, 80.0, 24.0),
        "Livre",
    );

    let arvore = Item::list_to(&[
        Item::with_children("Alpha", vec![Item::new("Alpha 1"), Item::new("Alpha 2")]),
        Item::new("Beta"),
        Item::new("Gama"),
    ]);
    let planos = Item::list_to(&[
        Item::new("SP"),
        Item::new("RJ"),
        Item::new("MG"),
        Item::new("RS"),
        Item::new("BA"),
        Item::new("PR"),
        Item::new("CE"),
        Item::new("GO"),
    ]);
    d.node_mut("trv_a")
        .expect("the tree")
        .props
        .insert("items".into(), arvore);
    // A single item, with text that exists nowhere else: if it appears
    // within another probe's region, the box was moved.
    d.node_mut("trv_b")
        .expect("the sibling tree")
        .props
        .insert("items".into(), Item::list_to(&[Item::new("Delta")]));
    let lst = d.node_mut("lst_a").expect("the list");
    lst.props.insert("items".into(), planos.clone());
    lst.props.insert("item_index".into(), Value::Int(1));
    let cbo = d.node_mut("cbo_a").expect("the box");
    cbo.props.insert("items".into(), planos);
    cbo.props.insert("item_index".into(), Value::Int(0));

    // Without anchors on any side — the case of divergence D6, fixed
    // on 2026-09-09, which this document continues guarding against
    // regression.
    let sem_ancora = Value::Map(
        [
            ("left".to_string(), Value::Bool(false)),
            ("top".to_string(), Value::Bool(false)),
            ("right".to_string(), Value::Bool(false)),
            ("bottom".to_string(), Value::Bool(false)),
        ]
        .into_iter()
        .collect(),
    );
    d.node_mut("btn_livre")
        .expect("the loose button")
        .props
        .insert("anchors".into(), sem_ancora);

    d.validate().expect("the common document is valid");
    d
}

/// The form dimensions. All three sides draw inside it.
const LARGURA: f32 = 640.0;
const ALTURA: f32 = 480.0;

/// How many frames to run before harvesting.
///
/// **Not slack: mandatory.** egui 0.36's `Area` starts with
/// `fade_in: true`, and without advancing the clock everything it paints has alpha
/// 95 of 255 — every measured color would be multiplied by 0.375, and `#F0F0F0` on
/// paper would appear as `#5A5A5A`. Controlling the clock over half a dozen
/// frames ensures the harvested color matches the painted color.
const QUADROS: u32 = 8;

// ===========================================================================
// THE SHAPES, AND THE LINE FORMAT PASSING ACROSS PROCESSES
// ===========================================================================

/// A painted shape, reduced to what is relevant for comparison.
///
/// It is always read from a LINE of text — including the two sides running
/// in-process. Round-tripping through text intentionally guarantees that
/// all three sides undergo identical rounding and parsing: the application
/// side has no choice (it comes from another process), and aligning the
/// other two with it costs nothing while eliminating an entire class of false positives.
#[derive(Debug, Clone, PartialEq)]
enum Forma {
    Ret {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        raio: u8,
        fill: String,
        traco_l: f32,
        traco_cor: String,
    },
    Txt {
        x: f32,
        y: f32,
        larg: f32,
        alt: f32,
        texto: String,
    },
    /// Circle, path, segment — what this gate does not probe.
    Outra,
}

impl Forma {
    /// The origin of the shape, used by the region filter.
    fn origem(&self) -> Option<(f32, f32)> {
        match self {
            Forma::Ret { x, y, .. } | Forma::Txt { x, y, .. } => Some((*x, *y)),
            Forma::Outra => None,
        }
    }
}

/// Reads the dump from one side. Any line not starting with a known label is
/// ignored — ensuring `cargo test` noise does not become a shape.
fn ler(linhas: &[String]) -> Vec<Forma> {
    linhas.iter().filter_map(|l| ler_uma(l)).collect()
}

fn ler_uma(linha: &str) -> Option<Forma> {
    let mut p = linha.split(' ');
    match p.next()? {
        "RET" => Some(Forma::Ret {
            x: p.next()?.parse().ok()?,
            y: p.next()?.parse().ok()?,
            w: p.next()?.parse().ok()?,
            h: p.next()?.parse().ok()?,
            raio: p.next()?.parse().ok()?,
            fill: p.next()?.to_string(),
            traco_l: p.next()?.parse().ok()?,
            traco_cor: p.next()?.to_string(),
        }),
        "TXT" => {
            let x = p.next()?.parse().ok()?;
            let y = p.next()?.parse().ok()?;
            let larg = p.next()?.parse().ok()?;
            let alt = p.next()?.parse().ok()?;
            // The text is the rest of the line: it may contain spaces
            // ("Alpha 1"), and the `Memo` line break is escaped.
            let texto = p.collect::<Vec<_>>().join(" ");
            Some(Forma::Txt {
                x,
                y,
                larg,
                alt,
                texto,
            })
        }
        "OUT" => Some(Forma::Outra),
        _ => None,
    }
}

/// Two decimal places, the precision where all three sides agree without floating-point
/// noise. `-0` becomes `0`.
fn n2(v: f32) -> String {
    let a = (v * 100.0).round() / 100.0;
    format!("{}", a + 0.0)
}

/// The lines from the two sides running IN-PROCESS.
///
/// The twin of this function lives in text inside [`ARQUIVO_DO_USUARIO`], representing the
/// only duplication in this gate. It is mandatory: the generated project cannot
/// depend on our crates, so the other side requires its own copy.
/// Both are short and their format is validated by the single parser.
fn linhas_da_saida(saida: &egui::FullOutput) -> Vec<String> {
    let mut fora = Vec::new();
    for c in &saida.shapes {
        empilhar(&c.shape, &mut fora);
    }
    fora
}

fn empilhar(s: &egui::Shape, fora: &mut Vec<String>) {
    match s {
        egui::Shape::Rect(r) => {
            let c = r.fill.to_array();
            let t = r.stroke.color.to_array();
            fora.push(format!(
                "RET {} {} {} {} {} {:02X}{:02X}{:02X} {} {:02X}{:02X}{:02X}",
                n2(r.rect.min.x),
                n2(r.rect.min.y),
                n2(r.rect.width()),
                n2(r.rect.height()),
                r.corner_radius.nw,
                c[0],
                c[1],
                c[2],
                n2(r.stroke.width),
                t[0],
                t[1],
                t[2]
            ));
        }
        egui::Shape::Text(t) => fora.push(format!(
            "TXT {} {} {} {} {}",
            n2(t.pos.x),
            n2(t.pos.y),
            n2(t.galley.size().x),
            n2(t.galley.size().y),
            t.galley.text().replace('\n', "\\n")
        )),
        egui::Shape::Vec(v) => {
            for x in v {
                empilhar(x, fora);
            }
        }
        _ => fora.push("OUT".to_string()),
    }
}

// ===========================================================================
// THE THREE SIDES
// ===========================================================================

/// A windowless egui frame, with manual clock control.
fn entrada(w: f32, h: f32, quadro: u32) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(w, h),
        )),
        time: Some(quadro as f64),
        ..Default::default()
    }
}

/// The `crawrad-runtime-egui` arm, in its two supported modes.
///
/// DESIGN sits behind an 18 pt ruler, whereas LIVE does not. Instead of hardcoding
/// an internal private number here, the origin is **measured**:
/// it is the corner of the form-sized rectangle painted by that side.
/// Adjusting the ruler therefore never causes this gate to misreport.
fn lado_do_runtime(modo: crawrad_runtime::Mode) -> Vec<Forma> {
    use crawrad_runtime::{Runtime, Scale, ViewportPx};
    // The design viewport must fit the form AFTER the ruler.
    let (w, h) = match modo {
        crawrad_runtime::Mode::Design => (LARGURA + 60.0, ALTURA + 60.0),
        crawrad_runtime::Mode::Live => (LARGURA, ALTURA),
    };
    let ctx = egui::Context::default();
    let mut rt = crawrad_runtime_egui::EguiRuntime::new(ctx.clone());
    rt.set_viewport(
        ViewportPx {
            x: 0,
            y: 0,
            w: w as i32,
            h: h as i32,
        },
        Scale(1.0),
    );
    rt.open(&documento())
        .expect("the runtime opens the common document");
    rt.set_mode(modo);

    let mut linhas = Vec::new();
    for q in 0..QUADROS {
        ctx.begin_pass(entrada(w, h, q));
        rt.frame();
        let mut saida = ctx.end_pass();
        // Without a GPU nothing consumes the font atlas; we acknowledge and
        // discard it deliberately, otherwise egui warns that nothing was painted.
        saida.textures_delta.clear();
        linhas = linhas_da_saida(&saida);
    }

    let formas = ler(&linhas);
    let (ox, oy) = origem_do_formulario(&formas).unwrap_or_else(|| {
        panic!("side {modo:?} did not paint the {LARGURA}x{ALTURA} form background; without it its zero cannot be known")
    });
    deslocar(formas, ox, oy)
}

/// The corner of the form-sized rectangle — the origin for that side.
fn origem_do_formulario(formas: &[Forma]) -> Option<(f32, f32)> {
    formas.iter().find_map(|f| match f {
        Forma::Ret { x, y, w, h, .. }
            if (w - LARGURA).abs() < 0.01 && (h - ALTURA).abs() < 0.01 =>
        {
            Some((*x, *y))
        }
        _ => None,
    })
}

fn deslocar(formas: Vec<Forma>, ox: f32, oy: f32) -> Vec<Forma> {
    formas
        .into_iter()
        .map(|f| match f {
            Forma::Ret {
                x,
                y,
                w,
                h,
                raio,
                fill,
                traco_l,
                traco_cor,
            } => Forma::Ret {
                x: x - ox,
                y: y - oy,
                w,
                h,
                raio,
                fill,
                traco_l,
                traco_cor,
            },
            Forma::Txt {
                x,
                y,
                larg,
                alt,
                texto,
            } => Forma::Txt {
                x: x - ox,
                y: y - oy,
                larg,
                alt,
                texto,
            },
            Forma::Outra => Forma::Outra,
        })
        .collect()
}

// ===========================================================================
// THE APPLICATION SIDE — THE COST AND THE VALUE
// ===========================================================================

/// The marker with which the generated application tags each shape line.
///
/// Without a marker, `cargo test` output (compilation logs, test names, and any other
/// cargo messages) would enter the dump.
const MARCA: &str = "@@F ";

/// The second marker: the same form in an enlarged `Ui`, for anchoring.
const MARCA_CRESCIDO: &str = "@@G ";

/// How much the form grows in the anchoring test. Intentionally asymmetric:
/// an axis swap would not go unnoticed.
const CRESCE_X: f32 = 100.0;
const CRESCE_Y: f32 = 60.0;

/// **The user file of the generated project**, containing the test dumping
/// shapes to standard output.
///
/// It is `Dono::Usuario`: the generator never overwrites it, which is why a
/// `#[cfg(test)]` can be placed inside without altering product code. The
/// `empilhar` function here is the textual twin of the one running in the test process —
/// the only duplication in this gate, made mandatory because the generated project
/// cannot depend on our crates.
const ARQUIVO_DO_USUARIO: &str = r##"//! Arquivo do usuário — escrito à mão; o CrawRAD nunca o sobrescreve.

use crate::form_main_designer::FormMain;

impl FormMain {}

#[cfg(test)]
mod prova {
    use super::*;
    use eframe::egui;

    /// Duas casas, e um `-0` vira `0`. Gêmea da do portão.
    fn n2(v: f32) -> String {
        let a = (v * 100.0).round() / 100.0;
        format!("{}", a + 0.0)
    }

    fn empilhar(s: &egui::Shape, fora: &mut Vec<String>) {
        match s {
            egui::Shape::Rect(r) => {
                let c = r.fill.to_array();
                let t = r.stroke.color.to_array();
                fora.push(format!(
                    "RET {} {} {} {} {} {:02X}{:02X}{:02X} {} {:02X}{:02X}{:02X}",
                    n2(r.rect.min.x),
                    n2(r.rect.min.y),
                    n2(r.rect.width()),
                    n2(r.rect.height()),
                    r.corner_radius.nw,
                    c[0],
                    c[1],
                    c[2],
                    n2(r.stroke.width),
                    t[0],
                    t[1],
                    t[2]
                ));
            }
            egui::Shape::Text(t) => fora.push(format!(
                "TXT {} {} {} {} {}",
                n2(t.pos.x),
                n2(t.pos.y),
                n2(t.galley.size().x),
                n2(t.galley.size().y),
                t.galley.text().replace('\n', "\\n")
            )),
            egui::Shape::Vec(v) => {
                for x in v {
                    empilhar(x, fora);
                }
            }
            _ => fora.push("OUT".to_string()),
        }
    }

    /// O formulário desenhado num `Ui` de `SIZE + folga`, sem janela.
    ///
    /// A origem cai em zero, então a coordenada do código gerado é a
    /// coordenada medida — e o portão do outro lado não precisa deslocar nada.
    fn despejar(marca: &str, folga: egui::Vec2) {
        let ctx = egui::Context::default();
        let mut form = FormMain::new();
        let mut linhas: Vec<String> = Vec::new();
        for q in 0..8u32 {
            let entrada = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(FormMain::SIZE[0] + 200.0, FormMain::SIZE[1] + 200.0),
                )),
                // O relógio na mão: sem ele o fade-in da `Area` deixa tudo com
                // alpha 95 e a cor medida não é a cor pintada.
                time: Some(q as f64),
                ..Default::default()
            };
            ctx.begin_pass(entrada);
            let alvo = egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(FormMain::SIZE[0], FormMain::SIZE[1]) + folga,
            );
            egui::Area::new(egui::Id::new("despejo"))
                .fixed_pos(egui::Pos2::ZERO)
                .show(&ctx, |ui| {
                    let mut filho = ui.new_child(egui::UiBuilder::new().max_rect(alvo));
                    form.show(&mut filho);
                });
            let mut saida = ctx.end_pass();
            saida.textures_delta.clear();
            linhas.clear();
            for c in &saida.shapes {
                empilhar(&c.shape, &mut linhas);
            }
        }
        // A LINHA EM BRANCO NÃO É ENFEITE. Com `--nocapture` o libtest escreve
        // `test nome::do::teste ... ` SEM quebrar a linha e só depois deixa o
        // teste falar, então a PRIMEIRA linha do despejo sai grudada nela:
        //
        //     test prova::despejo_do_desenho ... @@F TXT 52.44 12.5 35.13 15 Nome
        //
        // O portão do outro lado corta por prefixo, aquela linha não começa
        // com a marca, e a primeira forma do formulário — que é o `Label` —
        // some do despejo. Foi medido: as duas sondas do `Label` voltaram
        // `(nada)` e só elas.
        println!();
        for l in linhas {
            println!("{marca}{l}");
        }
    }

    #[test]
    fn despejo_do_desenho() {
        despejar("@@F ", egui::Vec2::ZERO);
    }

    #[test]
    fn despejo_do_formulario_crescido() {
        despejar("@@G ", egui::vec2(100.0, 60.0));
    }

    /// Um quadro do formulário, com os eventos que o usuário mandou.
    ///
    /// Igual ao da `despejar`, menos o despejo: aqui o que interessa é o
    /// ESTADO que o gesto deixou, e não a forma que ele pintou.
    ///
    /// **O passo do relógio é 50 ms, e não um segundo.** Foi medido: com um
    /// segundo por quadro, apertar num quadro e soltar no seguinte passa dos
    /// 600 ms do `max_click_duration` do egui, e o gesto vira pressão longa —
    /// nenhum `clicked()` acontece, em árvore nenhuma. O `despejar` aí em cima
    /// pode andar de segundo em segundo porque ele não clica: o relógio dele
    /// existe só para vencer o `fade_in` da `Area`.
    fn quadro(ctx: &egui::Context, form: &mut FormMain, eventos: Vec<egui::Event>, q: u32) {
        let entrada = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(FormMain::SIZE[0] + 200.0, FormMain::SIZE[1] + 200.0),
            )),
            time: Some(q as f64 * 0.05),
            events: eventos,
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        let alvo = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(FormMain::SIZE[0], FormMain::SIZE[1]),
        );
        egui::Area::new(egui::Id::new("gesto"))
            .fixed_pos(egui::Pos2::ZERO)
            .show(ctx, |ui| {
                let mut filho = ui.new_child(egui::UiBuilder::new().max_rect(alvo));
                form.show(&mut filho);
            });
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
    }

    fn botao(p: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: p,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// **O clique cai numa árvore e a IRMÃ não pode reagir.**
    ///
    /// No aplicativo gerado o formulário INTEIRO desenha num `Ui` só, então
    /// `ui.id()` é literalmente o mesmo valor para todo componente. Sem o nome
    /// da instância no `Id` da linha, as duas árvores pediriam o mesmo alvo
    /// para a linha 0 e um clique responderia pelas duas.
    ///
    /// Este é o teste que faltava: o caso ANINHADO já tinha portão, e o caso
    /// IRMÃO — que é o que o `Ui::id()` não distingue — não tinha nenhum, nem
    /// aqui nem no Preview.
    #[test]
    fn duas_arvores_irmas_nao_dividem_o_alvo_do_clique() {
        let ctx = egui::Context::default();
        let mut form = FormMain::new();
        // O clique é resolvido contra os alvos que o quadro ANTERIOR
        // registrou, então o gesto não pode ser o primeiro quadro.
        for q in 0..3u32 {
            quadro(&ctx, &mut form, Vec::new(), q);
        }
        // A primeira linha da `trv_b`, que mora em (500, 190). O `+2` e a meia
        // linha saem do `tree`: a linha 0 começa em `r.min.y + 2` e tem 18 pt.
        let alvo = egui::pos2(560.0, 201.0);
        quadro(&ctx, &mut form, vec![egui::Event::PointerMoved(alvo)], 3);
        quadro(
            &ctx,
            &mut form,
            vec![egui::Event::PointerMoved(alvo), botao(alvo, true)],
            4,
        );
        quadro(&ctx, &mut form, vec![botao(alvo, false)], 5);
        quadro(&ctx, &mut form, Vec::new(), 6);

        assert_eq!(
            form.trv_b.selection,
            vec![0usize],
            "the click fell on tree B and it should have selected"
        );
        assert!(
            form.trv_a.selection.is_empty(),
            "tree A reacted to the click on tree B: the two share the target, \
             and `ui.id()` is the same for every form component"
        );
    }
}
"##;

/// **The application side, assembled and measured ONCE.**
///
/// Both gates in this file require it, and `cargo test` runs binary tests
/// **in parallel**. Without this `OnceLock`, both would assemble the
/// same project in the same directory concurrently: two `cargo new` calls, two
/// writes overwriting each other, and two `cargo test` runs competing for `target/`.
/// This is the same issue as fixtures deleting one another,
/// which stays invisible when writing the test and surfaces months
/// later in CI once every ten runs.
///
/// `get_or_init` solves both problems at once: whichever finishes first
/// builds, and the subsequent caller **waits** and receives the same measurement. And the gate
/// avoids paying for inner `cargo test` twice.
fn aplicativo() -> &'static (Vec<Forma>, Vec<Forma>) {
    static UMA_VEZ: std::sync::OnceLock<(Vec<Forma>, Vec<Forma>)> = std::sync::OnceLock::new();
    UMA_VEZ.get_or_init(|| {
        let repositorio: PathBuf =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-desenho");
        let destino = repositorio.join("grupo").join("app");
        preparar_projeto(&repositorio, &destino);
        lado_do_aplicativo(&destino)
    })
}

/// Runs `cargo test` on the generated project and returns both dumps.
fn lado_do_aplicativo(destino: &Path) -> (Vec<Forma>, Vec<Forma>) {
    let arqs = crawrad_codegen::GeradorRxp
        .gerar(
            &documento(),
            "gate_desenho",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generate the project");
    crawrad_codegen::escrever(destino, &arqs).expect("write the project");
    std::fs::write(destino.join("src/form_main.rs"), ARQUIVO_DO_USUARIO)
        .expect("write the user file");

    // `--nocapture`: without it `cargo test` swallows the test `println!`, and
    // the dump returns empty — the gate would compare nothing against nothing.
    //
    // `--test-threads=1`: both tests write to the SAME standard output, and
    // interleaved lines would create an unparseable shape that the parser drops in
    // silence. Running them one at a time costs two frames and eliminates doubt.
    let (ok, texto) = cargo(destino, &["test", "--", "--nocapture", "--test-threads=1"]);
    assert!(
        ok,
        "the generated project should compile and run the dump:\n{texto}"
    );

    let colher = |marca: &str| -> Vec<Forma> {
        let linhas: Vec<String> = texto
            .lines()
            .filter_map(|l| l.strip_prefix(marca).map(str::to_string))
            .collect();
        assert!(
            !linhas.is_empty(),
            "the generated application dumped no shapes with marker `{}`. \
             Either the test did not run, or `--nocapture` was lost:\n{texto}",
            marca.trim()
        );
        ler(&linhas)
    };
    (colher(MARCA), colher(MARCA_CRESCIDO))
}

// ===========================================================================
// THE PROBES
// ===========================================================================

/// A named measurement taken across all three sides.
///
/// It does not inspect the entire dump: it examines the screen area where ONE component
/// lives. Comparing raw dumps would be useless — the design side paints the
/// grid, and its dump exceeds four hundred thousand characters.
struct Sonda {
    /// The name appearing in the exception list. It serves as the key and cannot change
    /// without updating the list.
    nome: &'static str,
    /// The document node whose rectangle bounds where the probe searches.
    no: &'static str,
    /// How much the node rectangle is inflated before filtering.
    ///
    /// Eight points suffice for almost everything. `ListBox` requires more **because the
    /// bug is specifically painting outside its own box**: with a
    /// tight margin, overflowing items would not be counted and the gate would report
    /// that both sides agree.
    folga: f32,
    medir: fn(&[Forma]) -> String,
}

/// All probes, with each associated component.
fn sondas() -> Vec<Sonda> {
    vec![
        Sonda {
            nome: "label/x do texto",
            no: "lbl_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "Nome"),
        },
        Sonda {
            nome: "label/altura do texto",
            no: "lbl_a",
            folga: 8.0,
            medir: |f| alt_do_texto(f, "Nome"),
        },
        Sonda {
            nome: "checkbox/x do texto",
            no: "chk_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "Ativo"),
        },
        Sonda {
            nome: "radiobutton/x do texto",
            no: "rad_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "Um"),
        },
        Sonda {
            nome: "edit/x do texto",
            no: "edt_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "designer"),
        },
        Sonda {
            nome: "memo/altura da moldura",
            no: "mem_a",
            folga: 8.0,
            medir: |f| altura_da_face(f, 200.0),
        },
        Sonda {
            nome: "button/face",
            no: "btn_a",
            folga: 8.0,
            medir: |f| face(f, 96.0, 32.0),
        },
        Sonda {
            nome: "button/raio da face",
            no: "btn_a",
            folga: 8.0,
            medir: |f| raio_da_face(f, 96.0, 32.0),
        },
        Sonda {
            nome: "combobox/face",
            no: "cbo_a",
            folga: 8.0,
            medir: |f| face(f, 150.0, 24.0),
        },
        Sonda {
            nome: "panel/cor do fundo",
            no: "pnl_a",
            folga: 8.0,
            medir: |f| fundo(f, 180.0, 60.0),
        },
        Sonda {
            nome: "listbox/itens pintados",
            no: "lst_a",
            folga: 60.0,
            medir: itens_pintados,
        },
        Sonda {
            nome: "listbox/passo da linha",
            no: "lst_a",
            folga: 60.0,
            medir: |f| passo(f, "SP", "RJ"),
        },
        Sonda {
            nome: "listbox/contorno da moldura",
            no: "lst_a",
            folga: 60.0,
            medir: |f| contorno(f, 180.0, 80.0),
        },
        Sonda {
            nome: "treeview/x do texto raiz",
            no: "trv_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "Alpha"),
        },
        Sonda {
            nome: "treeview/passo da linha",
            no: "trv_a",
            folga: 8.0,
            medir: |f| passo(f, "Alpha", "Alpha 1"),
        },
        Sonda {
            nome: "treeview/marcador de expandir",
            no: "trv_a",
            folga: 8.0,
            medir: marcador,
        },
        // The three `PageControl` probes originated with the shared drawing
        // file: `page_control` became a SINGLE function, compiled by
        // Preview and emitted in the application, with no probe measuring
        // whether it continued painting as expected. The first two measure
        // painted `TAB_H` and `TAB_W` — exactly what the refactoring
        // consolidated from three copies into one.
        //
        // **And `92.0, 24.0` here is a FOURTH copy of those dimensions** — the
        // test copy. It cannot be read from `desenho_comum.rs`: the gate intentionally
        // lives outside both crates, and reading the product constant
        // would make the probe track whatever the product painted, measuring
        // itself. The trade-off is silent failure: altering
        // `TAB_W` in the product does not update these numbers, causing both probes
        // to find `(nada)` across all three sides. The BLIND PROBE warning
        // below catches that, rather than the comparison itself.
        Sonda {
            nome: "pagecontrol/face da primeira aba",
            no: "pgc_a",
            folga: 8.0,
            medir: |f| face(f, 92.0, 24.0),
        },
        Sonda {
            nome: "pagecontrol/canto de cada aba",
            no: "pgc_a",
            folga: 8.0,
            medir: |f| abas_pintadas(f, 92.0, 24.0),
        },
        // And this is not geometry: it is the caption, measuring the font. Found in
        // this run and declared below.
        Sonda {
            nome: "pagecontrol/x da legenda da aba",
            no: "pgc_a",
            folga: 8.0,
            medir: |f| x_do_texto(f, "Um"),
        },
    ]
}

/// Returned when the probe found nothing and the region contained no text. A string
/// rather than a `panic!`: a side failing to paint something is a RESULT,
/// and it must appear in the comparison rather than aborting the run before the
/// other two are measured.
const NADA: &str = "(nada)";

/// **Every non-measurement response starts with `(`.** This parenthesis allows the
/// gate to distinguish a number from an absence, and is what the BLIND PROBE
/// warning looks for — no longer testing equality with [`NADA`], which only caught ONE of
/// the two absences and allowed the other to pass as agreement.
const AUSENTE: char = '(';

fn texto<'a>(fs: &'a [Forma], t: &str) -> Option<&'a Forma> {
    fs.iter()
        .find(|f| matches!(f, Forma::Txt { texto, .. } if texto == t))
}

/// **The probe searched for text and found nothing — and reports which of two things
/// happened.**
///
/// `(nada)` is read as *"this side painted nothing here"*, an interpretation that
/// previously led an investigator down the wrong path: the application had indeed
/// painted, under a different name. It combined indent, marker, and text into one string,
/// and `"- Alpha"` never matches `"Alpha"` — three probes blinded at once, with two
/// reporting `(nada)`, asserting the opposite of reality.
///
/// An empty region still yields `(nada)`, while a region that HAS text without a match
/// returns the text present there. Anyone reading the warning sees the new name immediately
/// without having to dump the output manually.
fn sem_casar(fs: &[Forma], procurados: &[&str]) -> String {
    let existentes: Vec<String> = fs
        .iter()
        .filter_map(|f| match f {
            Forma::Txt { texto, .. } => Some(format!("`{texto}`")),
            _ => None,
        })
        .collect();
    if existentes.is_empty() {
        return NADA.to_string();
    }
    let alvo = procurados
        .iter()
        .map(|t| format!("`{t}`"))
        .collect::<Vec<_>>()
        .join(" e ");
    format!("(sem {alvo}; a região pinta {})", existentes.join(", "))
}

fn x_do_texto(fs: &[Forma], t: &str) -> String {
    match texto(fs, t) {
        Some(Forma::Txt { x, .. }) => n2(*x),
        _ => sem_casar(fs, &[t]),
    }
}

fn alt_do_texto(fs: &[Forma], t: &str) -> String {
    match texto(fs, t) {
        Some(Forma::Txt { alt, .. }) => n2(*alt),
        _ => sem_casar(fs, &[t]),
    }
}

fn passo(fs: &[Forma], de: &str, ate: &str) -> String {
    match (texto(fs, de), texto(fs, ate)) {
        (Some(Forma::Txt { y: a, .. }), Some(Forma::Txt { y: b, .. })) => n2(b - a),
        _ => sem_casar(fs, &[de, ate]),
    }
}

/// Where EACH rectangle of that size begins, from left to right.
///
/// This is the tab strip probe, pure geometry — touching no
/// font metrics. It answers both questions governed by `TAB_W`:
/// how many tabs were painted and at what distance from each other.
///
/// **How it reacts to changing `TAB_W`, measured — which differs from what was
/// written here.** The former note claimed changing `TAB_W` in
/// `desenho_comum.rs` *"changes this line across ALL THREE sides"*. It does not: it **erases** it.
/// The search looks for a `92 × 24` rectangle — hardcoded in
/// [`sondas`], which is its fourth copy —, so with `TAB_W` at 100 all three
/// sides return `(nada)` together, and `(nada) == (nada) == (nada)` passed the
/// comparison as consensus. What turns it red is the **BLIND PROBE** check,
/// written after this measurement specifically because of it.
///
/// The property gained by the shared file remains valid and
/// measured — through another probe: `pagecontrol/x da legenda da aba`, which
/// searches for no fixed size and therefore CHANGES value instead of vanishing. With
/// `TAB_W` at 100 it moved from `45.2 / 46.73 / 46.73` to `49.2 / 50.73 /
/// 50.73`: all three sides shifted +4 together.
fn abas_pintadas(fs: &[Forma], w: f32, h: f32) -> String {
    let mut xs: Vec<f32> = fs
        .iter()
        .filter_map(|f| match f {
            Forma::Ret {
                x, w: rw, h: rh, ..
            } if (rw - w).abs() < 0.01 && (rh - h).abs() < 0.01 => Some(*x),
            _ => None,
        })
        .collect();
    xs.sort_by(|a, b| a.total_cmp(b));
    xs.dedup_by(|a, b| (*a - *b).abs() < 0.01);
    if xs.is_empty() {
        return NADA.to_string();
    }
    xs.iter().map(|x| n2(*x)).collect::<Vec<_>>().join(",")
}

/// The sample items that appear painted. The clipping probe.
fn itens_pintados(fs: &[Forma]) -> String {
    let amostra = ["SP", "RJ", "MG", "RS", "BA", "PR", "CE", "GO"];
    let n = amostra.iter().filter(|t| texto(fs, t).is_some()).count();
    n.to_string()
}

/// The `+`/`−` for nodes with children, painted AS ITS OWN SHAPE.
///
/// `−` is U+2212, and requiring this glyph is not pedantry: the application once
/// painted an ASCII hyphen attached to the label (`"- Alpha"`) and this probe returned
/// `no` — the right answer for the wrong reason, because what it measures is whether the
/// marker is distinct from the item text, not whether a dash exists anywhere
/// on the line.
///
/// When missing, it returns what the region paints rather than a bare `no`:
/// a marker with a changed glyph versus a missing marker require different
/// fixes, and the error message must state which occurred.
fn marcador(fs: &[Forma]) -> String {
    let tem = texto(fs, "\u{2212}").is_some() || texto(fs, "+").is_some();
    if tem {
        "sim".to_string()
    } else {
        sem_casar(fs, &["\u{2212}", "+"])
    }
}

/// The first rectangle of that size — the component face.
fn ret_de(fs: &[Forma], w: f32, h: f32) -> Option<&Forma> {
    fs.iter().find(
        |f| matches!(f, Forma::Ret { w: rw, h: rh, .. } if (rw - w).abs() < 0.01 && (rh - h).abs() < 0.01),
    )
}

fn face(fs: &[Forma], w: f32, h: f32) -> String {
    match ret_de(fs, w, h) {
        Some(Forma::Ret { x, y, w, h, .. }) => {
            format!("{},{},{},{}", n2(*x), n2(*y), n2(*w), n2(*h))
        }
        _ => NADA.to_string(),
    }
}

fn raio_da_face(fs: &[Forma], w: f32, h: f32) -> String {
    match ret_de(fs, w, h) {
        Some(Forma::Ret { raio, .. }) => raio.to_string(),
        _ => NADA.to_string(),
    }
}

/// The height of a rectangle of that width, whatever it may be. The probe for
/// components that determine their own height instead of honoring the design.
fn altura_da_face(fs: &[Forma], w: f32) -> String {
    match fs
        .iter()
        .find(|f| matches!(f, Forma::Ret { w: rw, .. } if (rw - w).abs() < 0.01))
    {
        Some(Forma::Ret { h, .. }) => n2(*h),
        _ => NADA.to_string(),
    }
}

/// The face fill color — the rectangle that PAINTS, not the outline.
/// Both share the same size, with the outline having a transparent fill.
fn fundo(fs: &[Forma], w: f32, h: f32) -> String {
    match ret_de(fs, w, h) {
        Some(Forma::Ret { fill, .. }) => format!("#{fill}"),
        _ => NADA.to_string(),
    }
}

/// The thickest stroke among rectangles of that size: the frame.
fn contorno(fs: &[Forma], w: f32, h: f32) -> String {
    let mut melhor: Option<(f32, &str)> = None;
    for f in fs {
        if let Forma::Ret {
            w: rw,
            h: rh,
            traco_l,
            traco_cor,
            ..
        } = f
            && (rw - w).abs() < 0.01
            && (rh - h).abs() < 0.01
        {
            let cand = (*traco_l, traco_cor.as_str());
            if melhor.is_none_or(|(l, _)| cand.0 > l) {
                melhor = Some(cand);
            }
        }
    }
    match melhor {
        Some((l, cor)) => format!("{}@#{cor}", n2(l)),
        None => NADA.to_string(),
    }
}

/// Shapes falling within the component rectangle, including padding.
fn na_regiao(formas: &[Forma], r: Rect, folga: f32) -> Vec<Forma> {
    formas
        .iter()
        .filter(|f| match f.origem() {
            Some((x, y)) => {
                x >= r.x - folga
                    && x <= r.x + r.w + folga
                    && y >= r.y - folga
                    && y <= r.y + r.h + folga
            }
            None => false,
        })
        .cloned()
        .collect()
}

// ===========================================================================
// THE DECLARED EXCEPTIONS
// ===========================================================================

/// A divergence CURRENTLY present in the product, with its measured value and closing issue.
struct Declarada {
    /// The probe name, matching [`sondas`] exactly.
    sonda: &'static str,
    design: &'static str,
    vivo: &'static str,
    aplicativo: &'static str,
    /// Why it remains here, and where it gets resolved.
    porque: &'static str,
}

/// **Current product divergences, one line each, with measured values.**
///
/// This list is the opposite of tolerance: it records debt with exact
/// numbers. The gate validates it bidirectionally — undeclared divergences
/// fail, as do lingering declarations.
///
/// **D6**, anchoring, is not here: it was fixed on 2026-09-09, guarded by
/// `a_regra_da_ancora_emitida_e_a_do_designer` and
/// [`a_ancora_pintada_pelo_aplicativo_e_a_que_a_ide_calcula`] below.
///
/// **D5** — the `TreeView` accordion selecting instead of collapsing — is
/// omitted because it is a gesture rather than a painted shape.
const DIVERGENCIAS_DECLARADAS: &[Declarada] = &[
    // D1 (the Label text 42,44 pt to the right on egui) was fixed;
    // the app and the live preview both lay the Label out in a `Ui` of
    // its own, top-down from the left.
    Declarada {
        sonda: "label/altura do texto",
        design: "16",
        vivo: "15",
        aplicativo: "15",
        porque: "The hand-drawn font is 14 pt and widgets are 12 pt \
                 (issue #74). One point in height, and it shifts the `GroupBox` \
                 frame gap by 5.5 pt.",
    },
    // ---------------------------------------------------------------- D2
    // Was 42,88 and 47,97 pt (the same `add_sized` centring as D1); since the
    // fix that closed D1 the app and the live preview lay them out in a `Ui` of
    // their own, and 3 pt are left.
    Declarada {
        sonda: "checkbox/x do texto",
        design: "31",
        vivo: "28",
        aplicativo: "28",
        porque: "D2, issue #78 — 3 pt. egui draws the box flush with the edge \
                 and keeps its own icon spacing; the designer puts the box 1 pt \
                 in, with its own gap to the text.",
    },
    Declarada {
        sonda: "radiobutton/x do texto",
        design: "31",
        vivo: "28",
        aplicativo: "28",
        porque: "D2, issue #78 — 3 pt. The same as the CheckBox.",
    }, // ---------------------------------------------------------------- D3
    //
    // D3 from the study was closed (the block below explains how), and `ListBox`
    // diverged again from ANOTHER cause on 2026-09-13 — the half-visible line.
    // `passo da linha` remains 18/18/18, proving D3 did not regress.
    Declarada {
        sonda: "listbox/itens pintados",
        design: "4",
        vivo: "5",
        aplicativo: "5",
        porque: "issue #74 — and NOT D3 returning: the line pitch remains 18 on all \
                 three sides, and the designer no longer paints over its neighbor. \
                 What diverges now is the line that fits HALFWAY. The 80 pt \
                 box accommodates four full lines plus 7 pt; the \
                 widget — `ScrollArea` with `Button::selectable`, in Preview and \
                 in the application — draws the fifth and lets clipping cut it, \
                 while the design hand drawing stops at the fourth \
                 (`crawrad-runtime-egui/src/lib.rs`, the DESIGN branch of \
                 `ListBox`: `if topo + alt > r.max.y { break }`). What has to \
                 yield is DESIGN: the list scrolls, and a half line is what \
                 Windows `TListBox` shows — hiding the fifth makes the design \
                 surface promise an emptier box than the application delivers. \
                 Both widget sides became `ScrollArea` in `da22dd8`, and only \
                 DESIGN lagged behind.",
    },
    // ------------------------------------------------- D3 from study: CLOSED
    //
    // The two `ListBox` exceptions lived here, removed because the defect was
    // eliminated from the product: `itens pintados` declared 8/4/4 and `passo da
    // linha` declared 16/18/18. Design rendering adopted the same 18 pt line height
    // as live forms and emitted applications, stopping at the first line that does not
    // fit fully — probes now read **4/4/4 and 18/18/18** across all three sides.
    //
    // This comment remains as a milestone: the **gate** mandated removal
    // with the message *"A EXCEÇÃO SOBROU — os três lados concordam agora, e a lista ainda declara…"*.
    // The probes remain active, ensuring regressions trigger an undeclared
    // divergence failure rather than silent drift.
    //
    // First of the seven to close.
    // ---------------------------------------------------------------- D4
    // Resolved in text x and line pitch constants were aligned
    // across design, live, and application.
    // The third D4 exception, `treeview/marcador de expandir`, was removed on
    // 2026-09-16, mandated by the GATE — *"A EXCEÇÃO SOBROU"*.
    // It declared `sim/sim/não`: the application painted no marker, and after
    // `da22dd8` painted an ASCII hyphen attached to the label (`"- Alpha"`),
    // which was worse — hiding from the probe while corrupting the item text.
    // Today all three sides paint `−` U+2212 as its own shape.

    // ---------------------------------------------------------------- D7
    Declarada {
        sonda: "panel/cor do fundo",
        design: "#E2E2E6",
        vivo: "#0A0A0A",
        aplicativo: "#0A0A0A",
        porque: "D7, issue #76 — closed. The paper stays light on the canvas and the \
                 application follows the system theme. The canvas is canvas by owner decision (09/09/2026, #76): the pair that \
                 must match is Preview and application, and since 03/11/2026 it does — \
                 Preview draws with the form theme (`Theme`), and only the paper stays light.",
    },
    Declarada {
        sonda: "button/raio da face",
        design: "4",
        vivo: "2",
        aplicativo: "2",
        porque: "D7, issue #76 — radius 4 is from Windows 11 and comes from the paper style; 2 is \
                 the egui default that the application uses. The canvas is canvas by owner decision (09/09/2026, #76): the pair that \
                 must match is Preview and application, and since 03/11/2026 it does — \
                 Preview draws with the form theme (`Theme`), and only the paper stays light.",
    },
    Declarada {
        sonda: "listbox/contorno da moldura",
        design: "1@#888888",
        vivo: "0@#000000",
        aplicativo: "0@#000000",
        porque: "D7, issue #76 — egui sends stroke 0 in inactive state, and the list \
                 has no frame in the application nor, now, in Preview. The canvas draws it \
                 with paper style. The canvas is canvas by owner decision (09/09/2026, #76): the pair that \
                 must match is Preview and application, and since 03/11/2026 it does — \
                 Preview draws with the form theme (`Theme`), and only the paper stays light.",
    },
    // ------------------------------------------------------- outside the seven
    Declarada {
        sonda: "memo/altura da moldura",
        design: "60",
        vivo: "64",
        aplicativo: "64",
        porque: "Found by this gate (issue #74): the 60 pt `Memo` in the drawing comes out as \
                 64 in the application — the inner `TextEdit` measures 59.88 and its own margin \
                 adds the 4. With the application theme (#76, 03/11/2026) Preview now \
                 measures the same 64: the difference was stylistic, and only the canvas keeps it.",
    },
    Declarada {
        sonda: "pagecontrol/x da legenda da aba",
        design: "45.2",
        vivo: "46.73",
        aplicativo: "46.73",
        porque: "Found when the tab strip became a section of `desenho_comum.rs` \
                 and gained a probe. **Not tab geometry** — that agrees \
                 on all three sides, and the two probes above measure it: the tab is \
                 `10,240,92,24` and the corners are `10,102` everywhere. What \
                 diverges is the caption FONT: 14 pt in hand drawing versus \
                 12 pt in widgets, the same 1.53 pt of half galley that \
                 `label/altura do texto` already declares — issue #74.",
    },
];

// ===========================================================================
// THE GATE
// ===========================================================================

/// **The gate.** The three renderings of the same document may only differ in what
/// is declared.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn o_desenho_dos_tres_lados_so_diverge_no_que_esta_declarado() {
    let design = lado_do_runtime(crawrad_runtime::Mode::Design);
    let vivo = lado_do_runtime(crawrad_runtime::Mode::Live);
    let aplicativo = &aplicativo().0;

    let doc = documento();
    let mut diverge: Vec<(&'static str, [String; 3])> = Vec::new();
    let mut concorda: Vec<&'static str> = Vec::new();
    // Probes that "agreed" because ALL THREE sides found nothing, reporting
    // what all three returned — indicating whether the region is empty or
    // the searched text changed names.
    let mut cegas: Vec<(&'static str, String)> = Vec::new();

    for s in sondas() {
        let r = doc
            .rect_in_form(s.no)
            .unwrap_or_else(|| panic!("node `{}` has no rectangle", s.no));
        let medir = |lado: &[Forma]| (s.medir)(&na_regiao(lado, r, s.folga));
        let tres = [medir(&design), medir(&vivo), medir(aplicativo)];
        println!(
            "{:<32} design={:<10} vivo={:<10} app={}",
            s.nome, tres[0], tres[1], tres[2]
        );
        if tres[0] == tres[1] && tres[1] == tres[2] {
            // **Agreement on NOTHING is not agreement.** Three `(nada)` match
            // one another and the probe would pass while measuring emptiness — the
            // "blind probe", which nearly slipped through: when
            // `TAB_W` changed from 92 to 100 in mutation testing, the two
            // `PageControl` geometry probes did not CHANGE numbers,
            // they DISAPPEARED, and that equality counted as a pass. What
            // saved it was the third probe and two registry tests; a
            // gate must not rely on luck.
            //
            // The check uses the [`AUSENTE`] parenthesis rather than equality with
            // `(nada)`: since probes now ALSO report what text exists in the
            // region, two distinct absences exist, and the second easily occurs across
            // all three sides simultaneously — simply by renaming an item in the common document.
            if tres[0].starts_with(AUSENTE) {
                cegas.push((s.nome, tres[0].clone()));
            }
            concorda.push(s.nome);
        } else {
            diverge.push((s.nome, tres));
        }
    }

    let achar = |nome: &str| DIVERGENCIAS_DECLARADAS.iter().find(|d| d.sonda == nome);
    let mut queixas: Vec<String> = Vec::new();

    // 1. Undeclared divergence appeared. This is the failure this
    //    gate exists to produce: someone just pushed the two renderings apart.
    for (nome, tres) in &diverge {
        match achar(nome) {
            None => queixas.push(format!(
                "NEW divergence, and undeclared — `{nome}`\n  \
                 design = {}\n  live   = {}\n  app    = {}\n  \
                 The drawing and the application started showing \
                 different things. Fix it, or declare in `DIVERGENCIAS_DECLARADAS` \
                 with the measured number and the closing issue.",
                tres[0], tres[1], tres[2]
            )),
            // 2. Declared, but the number changed. Partial fixes do not pass by
            //    accident, nor do regressions within known divergences.
            Some(d) if [d.design, d.vivo, d.aplicativo] != *tres => queixas.push(format!(
                "DIVERGENCE NUMBER CHANGED — `{nome}`\n  \
                 declared: design = {}, live = {}, app = {}\n  \
                 measured: design = {}, live = {}, app = {}\n  \
                 {}",
                d.design, d.vivo, d.aplicativo, tres[0], tres[1], tres[2], d.porque
            )),
            Some(_) => {}
        }
    }

    // 3. Obsolete declaration. Without this, the list would become obsolete, and
    //    the gate would accept both fixes and defects indifferently.
    for nome in &concorda {
        if let Some(d) = achar(nome) {
            queixas.push(format!(
                "UNUSED EXCEPTION — `{nome}`\n  \
                 all three sides agree now, and the list still declares \
                 design = {}, live = {}, app = {}.\n  \
                 Delete the line from `DIVERGENCIAS_DECLARADAS` and close the issue: \
                 {}",
                d.design, d.vivo, d.aplicativo, d.porque
            ));
        }
    }

    // 4. Declaration for a nonexistent probe — renaming a probe without
    //    updating the list would leave the exception active but blind.
    let nomes: Vec<&str> = sondas().iter().map(|s| s.nome).collect();
    for d in DIVERGENCIAS_DECLARADAS {
        if !nomes.contains(&d.sonda) {
            queixas.push(format!(
                "ORPHAN EXCEPTION — `{}` is not a probe. Someone renamed the \
                 probe and left the declaration behind; it no longer measures anything.",
                d.sonda
            ));
        }
    }

    // 5. BLIND probe: all three sides returned `(nada)` and equality counted
    //    it as a pass. That is not agreement, but an absence of measurement —
    //    occurring precisely when someone alters what the probe searches for,
    //    when visibility is most needed.
    for (nome, o_que) in &cegas {
        queixas.push(format!(
            "BLIND PROBE — `{nome}` measured nothing on all THREE sides: {o_que}\n  \
             Equality counted this as agreement, and it is not: the probe failed \
             to find what it searches for. Either the drawing stopped painting that \
             on all three sides — which is divergence from the past, not agreement —, \
             or the probe is searching for the wrong thing."
        ));
    }

    assert!(
        queixas.is_empty(),
        "\n{}\n\n\
         ({} probes agreed on all three sides, {} diverged, {} blind.)",
        queixas.join("\n\n"),
        concorda.len(),
        diverge.len(),
        cegas.len()
    );
}

/// **Anchoring, proven by the pixel.**
///
/// `a_regra_da_ancora_emitida_e_a_do_designer` (in `src/tests.rs`) compares the
/// `crawrad-designer` rule with emitted TEXT, running for free. This test
/// compares the same rule with the rectangle the application **actually painted**
/// after compilation — closing the only loophole the other has: correct text
/// that `rustc` interprets differently.
///
/// `btn_livre` has no anchors on any side. The form expands by 100 × 60, and
/// both sides must show it stayed in place. Until 2026-09-09 the
/// application painted it at (550, 430): **50 pt and 30 pt** off.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn a_ancora_pintada_pelo_aplicativo_e_a_que_a_ide_calcula() {
    let crescido = &aplicativo().1;

    let doc = documento();
    let n = doc.node("btn_livre").expect("the loose button");
    let r = doc.rect_in_form("btn_livre").expect("its rectangle");
    let a = crawrad_designer::layout::Anchors::read(n.props.get("anchors"));
    let ide = a.adjust(r, CRESCE_X, CRESCE_Y);

    let pintado = face(crescido, r.w, r.h);
    let esperado = format!("{},{},{},{}", n2(ide.x), n2(ide.y), n2(ide.w), n2(ide.h));
    assert_eq!(
        pintado, esperado,
        "\nTHE PAINTED ANCHOR IS NOT WHAT THE IDE CALCULATES.\n\
         anchors       : {a:?}\n\
         drawing       : {r:?}\n\
         form grows    : +{CRESCE_X} x +{CRESCE_Y}\n\
         IDE  (`Anchors::adjust`)  → {esperado}\n\
         APPLICATION (painted)     → {pintado}\n\
         Resizing the window must not move what the designer left in place."
    );
}

// ===========================================================================
// PROJECT ASSEMBLY — identical to the other two expensive gates
// ===========================================================================

/// A `cargo` invocation inside the generated project.
///
/// `env_remove` removes outer cargo environment variables that would direct the
/// inner compiler to the wrong `target/` directory with different flags.
fn cargo(destino: &Path, args: &[&str]) -> (bool, String) {
    let saida = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(destino)
        .output()
        .expect("call cargo inside the generated project");
    let texto = format!(
        "{}{}",
        String::from_utf8_lossy(&saida.stdout),
        String::from_utf8_lossy(&saida.stderr)
    );
    (saida.status.success(), texto)
}

/// The pre-existing workspace, causing the gate's `cargo` to stay within it
/// instead of ascending to OUR `Cargo.toml`. Detailed rationale is in
/// `gate_fase3.rs`.
const REPOSITORIO_DE_MENTIRA: &str = "# Um workspace que já existe, para o `cargo` do portão cair dentro dele.\n\
     [workspace]\n\
     members = []\n\
     resolver = \"2\"\n";

/// The GROUP root, written prior to `cargo new` — as the IDE does (ADR 0018).
const RAIZ_DO_GRUPO: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// Is the draft intact, or does only a fragment remain?
///
/// Checks the entire SET written by assembly, not just a single file: the
/// draft lives in `target/`, CI caches may return it pruned, and a guard looking
/// only at the manifest concludes "no project exists" while `cargo new` concludes
/// "it already exists". Both sibling gates share this function, and the "is this
/// manifest mine?" check is unified in `comum/mod.rs`.
fn rascunho_intacto(repositorio: &Path, grupo: &Path, destino: &Path) -> bool {
    manifesto_intocado(repositorio, REPOSITORIO_DE_MENTIRA)
        && manifesto_intocado(grupo, RAIZ_DO_GRUPO)
        && destino.join("src").is_dir()
        && std::fs::read_to_string(destino.join("Cargo.toml"))
            .is_ok_and(|m| m.contains("eframe") && m.contains("crawrad-runtime"))
}

/// What the IDE does prior to generation: group root, `cargo new`, `cargo add`.
fn preparar_projeto(repositorio: &Path, destino: &Path) {
    let grupo = destino
        .parent()
        .expect("the destination is a member of a group");
    if rascunho_intacto(repositorio, grupo, destino) {
        return;
    }
    if destino.exists() {
        std::fs::remove_dir_all(destino).expect("delete the half draft");
    }
    std::fs::create_dir_all(grupo).expect("create the group root");
    std::fs::write(repositorio.join("Cargo.toml"), REPOSITORIO_DE_MENTIRA)
        .expect("the mock workspace");
    std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).expect("the group root");

    let nome = destino
        .file_name()
        .and_then(|n| n.to_str())
        .expect("the name");
    let (ok, texto) = cargo(
        grupo,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_desenho",
            nome,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{texto}");

    let (ok, offline) = cargo(destino, &["add", "eframe@0.36", "egui@0.36", "--offline"]);
    if !ok {
        let (ok, online) = cargo(destino, &["add", "eframe@0.36", "egui@0.36"]);
        assert!(
            ok,
            "`cargo add` failed offline AND online — without the crates.io index \
             this gate cannot set up the project.\n--- offline ---\n{offline}\
             \n--- online ---\n{online}"
        );
    }

    // Same dependency the IDE writes for an egui form (`dependencias_do_projeto`):
    // the generated `main` reports a startup failure through it.
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crawrad-runtime");
    let runtime = runtime.to_str().expect("the runtime path");
    let (ok, texto) = cargo(destino, &["add", "--path", runtime, "--offline"]);
    assert!(ok, "the `cargo add` of crawrad-runtime failed:\n{texto}");
}

// ===========================================================================
// GATE OF THE GATE — runs in the standard test suite, without cargo or disk
// ===========================================================================

/// The exception list **is** this file's contract, written by
/// hand. This test takes microseconds and catches three ways to corrupt it
/// silently: nonexistent probe names, duplicate probe declarations,
/// and `porque` fields missing an issue reference.
///
/// The third check distinguishes "declared debt" from "excuses": an entry without
/// an issue number is a divergence no one will close.
#[test]
fn toda_excecao_declarada_aponta_para_uma_sonda_e_para_um_issue() {
    let nomes: Vec<&str> = sondas().iter().map(|s| s.nome).collect();
    let mut queixas = Vec::new();

    for d in DIVERGENCIAS_DECLARADAS {
        if !nomes.contains(&d.sonda) {
            queixas.push(format!("`{}` is not a probe", d.sonda));
        }
        if !d.porque.contains("issue #") {
            queixas.push(format!(
                "`{}` is declared without an issue — debt without a number is never closed",
                d.sonda
            ));
        }
        let quantas = DIVERGENCIAS_DECLARADAS
            .iter()
            .filter(|o| o.sonda == d.sonda)
            .count();
        if quantas > 1 {
            queixas.push(format!(
                "`{}` is declared {quantas} times — the first wins and the \
                 other is never read",
                d.sonda
            ));
        }
    }

    for s in sondas() {
        let repetida = sondas().iter().filter(|o| o.nome == s.nome).count();
        if repetida > 1 {
            queixas.push(format!(
                "probe `{}` appears {repetida} times; the name is the key of \
                 the exception list and cannot collide",
                s.nome
            ));
        }
    }

    assert!(
        queixas.is_empty(),
        "the exception list is malformed:\n{queixas:#?}"
    );
}

/// The dump parser bridges both processes, and can fail silently:
/// malformed lines produce `None`, the dump shrinks, and probes
/// report `(nada)` across all three sides — which **agrees**, leaving the gate
/// green without measuring anything.
///
/// It therefore has its own test verifying round-tripping: shapes become lines,
/// lines become shapes, and both must match.
#[test]
fn a_linha_do_despejo_volta_a_ser_a_mesma_forma() {
    let casos = [
        "RET 10 40 96 32 4 FDFDFD 1 888888",
        "TXT 52.44 12.5 35.13 15 Nome",
        // Text containing spaces — the tree has "Alpha 1".
        "TXT 463 72 41.88 14 Alpha 1",
        // And the escaped line break of `Memo`.
        "TXT 14 112 13.84 30 L1\\nL2",
        "OUT",
    ];
    for c in casos {
        let f = ler_uma(c).unwrap_or_else(|| panic!("`{c}` was not read"));
        match (&f, c) {
            (Forma::Ret { x, raio, fill, .. }, "RET 10 40 96 32 4 FDFDFD 1 888888") => {
                assert_eq!((*x, *raio, fill.as_str()), (10.0, 4, "FDFDFD"));
            }
            (Forma::Txt { texto, .. }, "TXT 463 72 41.88 14 Alpha 1") => {
                assert_eq!(texto, "Alpha 1", "the space inside the text was lost");
            }
            (Forma::Txt { texto, .. }, "TXT 14 112 13.84 30 L1\\nL2") => {
                assert_eq!(texto, "L1\\nL2", "the escaped line break was lost");
            }
            _ => {}
        }
    }
    // And `cargo test` noise does not become a shape.
    assert!(ler_uma("   Compiling gate_desenho v0.1.0").is_none());
    assert!(ler_uma("test prova::despejo_do_desenho ... ok").is_none());
    assert!(ler_uma("").is_none());
}

/// Both markers and form growth are written **twice**:
/// here as constants, and inside [`ARQUIVO_DO_USUARIO`] as text. This duplication
/// is unavoidable — that file is compiled in another process by a project
/// that cannot depend on our crates.
///
/// What we can do is prevent silent drift, which is what this
/// test enforces. Drift is insidious: changing the marker here
/// but not there causes the dump to return EMPTY, and the asserting check
/// only runs in the expensive gate under `--ignored`.
#[test]
fn a_marca_e_o_crescimento_do_despejo_batem_com_os_do_arquivo_do_usuario() {
    for (o_que, esperado) in [
        ("the drawing marker", format!("despejar(\"{MARCA}\"")),
        (
            "the grown form marker",
            format!("despejar(\"{MARCA_CRESCIDO}\""),
        ),
        (
            "the growth",
            format!("egui::vec2({CRESCE_X:?}, {CRESCE_Y:?})"),
        ),
    ] {
        assert!(
            ARQUIVO_DO_USUARIO.contains(&esperado),
            "{o_que} diverged: the gate says `{esperado}` and the user \
             file lacks that line. With both out of sync the dump \
             returns empty and the gate compares nothing with nothing."
        );
    }
}
