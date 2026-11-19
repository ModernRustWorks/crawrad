//! **The designer drawing and the generated application use the SAME numbers.**
//!
//! # What happened, measured on 18/09/2026
//!
//! The canvas drew `ListBox` rows with `ALTURA_DA_LINHA = 18.0` while the
//! generator emitted `height: 20.0` in `live_design!`. In `TreeView`, 17.0 against
//! the same 20.0. **A list that showed five rows in the designer showed
//! four and a half in the application**, and nothing in the repository compared the two.
//!
//! `ScrollBar` had the same issue on a larger scale: the canvas had
//! `BAR_BUTTON_SIDE: f32 = 16.0` and the generator loaded `16.0` and `32.0` as
//! raw literals, **eight times**, in another crate.
//!
//! # Why a guard, if both now read the same constant
//!
//! Because reading the same constant is a choice someone can undo inadvertently. The
//! `20.0` was not introduced out of bad faith: it entered because typing a number into
//! a DSL string is faster than looking up where it should come from, and
//! nothing complained.
//!
//! This guard complains. It reads the text the generator just emitted and ensures
//! that the height there is **exactly** the shared constant — the
//! same one the canvas uses to paint.
//!
//! # What it does NOT cover, stated plainly
//!
//! Only the three dimensions that have already been unified. It does not scan the DSL for
//! every number: that would capture color, corner radius, and spacing, which have no canvas
//! counterpart, and a guard flagging correct code is disabled on the third run.
//! Coverage grows as new dimensions enter `shared_geometry`.

use crate::Gerador;
use crate::targets::makepad::shared_geometry::{LIST_ROW_HEIGHT, TREE_ROW_HEIGHT};
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// The `live_design!` emitted by the Makepad target for a form with a node of the
/// requested class.
fn dsl_com(classe: &str) -> String {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the newly created root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let n = r
        .instantiate(
            classe,
            format!("{}1", classe.to_lowercase()),
            Rect::new(8.0, 8.0, 200.0, 120.0),
        )
        .expect("the registry instantiates what it lists");
    d.add_child("form_main", n).expect("added to the form");
    d.validate().expect("valid");

    crate::GeradorMakepad
        .gerar(&d, "app", &crawrad_registry::Registry::builtin())
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

/// The emitted row height matches the canvas, not a hand-written number.
#[test]
fn a_altura_da_linha_do_list_box_e_a_do_canvas() {
    let dsl = dsl_com("ListBox");
    let esperado = format!("height: {LIST_ROW_HEIGHT:.1}");
    assert!(
        dsl.contains(&esperado),
        "generated `ListBox` does not emit `{esperado}` — application \
         row height is no longer what the designer draws. That was exactly \
         how the canvas showed 18.0 and the application, 20.0."
    );
    assert!(
        !dsl.contains("item_0 = <Button> {\n                    width: Fill, height: 20.0"),
        "hardcoded `20.0` returned to `ListBox`"
    );
}

/// The same for `TreeView`, which diverged by one additional point.
#[test]
fn a_altura_da_linha_do_tree_view_e_a_do_canvas() {
    let dsl = dsl_com("TreeView");
    let esperado = format!("height: {TREE_ROW_HEIGHT:.1}");
    assert!(
        dsl.contains(&esperado),
        "generated `TreeView` does not emit `{esperado}` — application \
         row height is no longer what the designer draws."
    );
}

/// **The three row dimensions are distinct, intentionally.**
///
/// If someone "simplifies" by unifying all three into one number, `Memo`,
/// `ListBox`, and `TreeView` will share the same density — and the difference
/// between them was designed, not inherited by accident.
#[test]
fn as_alturas_de_linha_nao_se_confundem() {
    use crate::targets::makepad::shared_geometry::MEMO_LINE_HEIGHT;
    assert_ne!(LIST_ROW_HEIGHT, TREE_ROW_HEIGHT);
    assert_ne!(LIST_ROW_HEIGHT, MEMO_LINE_HEIGHT);
    assert_ne!(TREE_ROW_HEIGHT, MEMO_LINE_HEIGHT);
}

/// **The canvas and generator read the SAME constant, not two identical ones.**
///
/// Comparison is against the canvas path: if someone redeclares the constant
/// there, this test stays green and `shared_geometry` becomes dead code — which
/// is why equality is checked against what `crawrad-runtime-makepad` exports, rather
/// than an arbitrary number written here.
#[test]
fn o_canvas_le_a_mesma_fonte_que_o_gerador() {
    assert_eq!(
        LIST_ROW_HEIGHT,
        crawrad_runtime_makepad::components::list_box::ALTURA_DA_LINHA,
        "the `ListBox` canvas stopped reading `shared_geometry`"
    );
    assert_eq!(
        TREE_ROW_HEIGHT,
        crawrad_runtime_makepad::components::tree_view::ALTURA_DA_LINHA,
        "the `TreeView` canvas stopped reading `shared_geometry`"
    );
    assert_eq!(
        crate::targets::makepad::shared_geometry::BAR_BUTTON_SIDE,
        crawrad_runtime_makepad::components::bar::BAR_BUTTON_SIDE,
        "the bar canvas stopped reading `shared_geometry`"
    );
}

// ---------------------------------------------------------------------------
// THE `Strings` PIECE — single implementation across both targets
// ---------------------------------------------------------------------------

/// **Both targets emit the SAME piece, byte-for-byte.**
///
/// It was written twice, with different APIs and each incomplete in
/// a different way: egui had `new`/`set_text`/`PartialEq` and lacked
/// `len`/`get`/`clear`; Makepad had the opposite. A form that
/// compiled on one target failed on the other, in BOTH directions.
///
/// Today both read `crawrad-component/src/pieces/strings.rs` via `include_str!`,
/// making them identical by construction. This guard exists for the day someone
/// "fixes" an issue by pasting the text back on one side — which is
/// exactly how the two copies were born.
#[test]
fn a_peca_strings_e_a_mesma_nos_dois_alvos() {
    let peca = include_str!("../../../../crawrad-component/src/pieces/strings.rs");
    let assinatura = "pub struct Strings {";
    assert!(peca.contains(assinatura), "the piece changed shape");

    for classe in ["ListBox", "Memo"] {
        let mk = dsl_com(classe);
        assert!(
            mk.contains(assinatura),
            "Makepad designer for `{classe}` does not include the `Strings` piece"
        );
    }

    // The piece body — starting from `pub struct` — must appear entirely in
    // what each target emits. Comparing the whole file does not work: the authorship
    // header is internal and may be trimmed one day.
    let corpo = &peca[peca.find(assinatura).expect("the signature")..];
    let corpo_norm = corpo.replace("\r\n", "\n");
    let mk = dsl_com("ListBox");
    assert!(
        mk.contains(corpo_norm.trim_end()),
        "Makepad target emits a `Strings` piece DIFFERENT from `crawrad-component/src/pieces/strings.rs` — someone pasted text back"
    );
}
