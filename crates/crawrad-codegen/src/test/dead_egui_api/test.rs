//! **The egui designer never calls an egui API that egui 0.36 removed.**
//!
//! # The defect that gave this file a reason to exist
//!
//! every form carrying a `ScrollBox` produced a project that did
//! **not compile**, because the emitted drawing called `allocate_ui_at_rect` —
//! deprecated in egui 0.32 and gone in 0.36. The file that failed to compile
//! was the `.designer.rs`, which is rewritten whole on every generation, so
//! the user could not even patch it: any fix of his disappears on the next
//! save.
//!
//! # Why a text test and not the compiler
//!
//! The compiler already says this, and says it better — but it only says it
//! inside `gate_sem_avisos`, which is `#[ignore]`, calls `cargo` from inside
//! `cargo`, needs the crates.io index and takes a minute. That gate stayed red
//! for days precisely because nobody ran it. This file asks the cheap half of
//! the same question in microseconds, in the suite everybody runs, so the
//! **next** removed API is caught on the commit that introduces it rather than
//! by a user's bug report.
//!
//! # Where the list comes from
//!
//! It is not invented here: it is every `#[deprecated]` function in the egui
//! **0.32** source, which is exactly the set 0.36 deleted. Extracted with
//!
//! ```text
//! grep -rn -A4 '#\[deprecated' src/ | grep -oE 'pub fn [a-z_0-9]+'
//! ```
//!
//! over `egui-0.32.3/src`.
//!
//! # What it is NOT
//!
//! It proves the emitted text does not *name* a dead function. It cannot prove
//! the live function it names exists, takes those arguments, or draws the same
//! thing — `gate_sem_avisos` (compiles the project with `-D warnings`) and
//! `gate_desenho_comparado` (compares the drawing with the RXP) are what prove
//! that. This one catches a single, recurring, cheap mistake.
//!
//! The sweep is over the **egui** designer only, on purpose: `set_visible` and
//! friends are live API of `makepad-widgets`, so charging the Makepad target
//! with egui's obituary would be a false alarm.

use crate::Gerador;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// Every egui function that carried `#[deprecated]` in egui 0.32 and is gone
/// in 0.36.
///
/// `new` is deliberately absent: `Ui::new` was one of them, but `.new(` is
/// half of any Rust file and the match would be noise rather than a net.
const REMOVED_IN_EGUI_036: &[&str] = &[
    "add_visible_ui",
    "allocate_new_ui",
    "allocate_ui_at_rect",
    "allow_interaction",
    "any_popup_open",
    "child_ui",
    "child_ui_with_id_source",
    "clamp_range",
    "clamp_to_range",
    "close_all_popups",
    "close_menu",
    "close_popup",
    "drag_to_scroll",
    "enable_scrolling",
    "fade_out_to_color",
    "from_id_source",
    "global_dark_light_mode_buttons",
    "global_dark_light_mode_switch",
    "id_source",
    "interact_bg",
    "interact_with_hovered",
    "is_context_menu_open",
    "is_popup_open",
    "keep_popup_open",
    "layer_transforms",
    "menu_image_button",
    "open_popup",
    "open_popup_at",
    "popup_above_or_below_widget",
    "popup_below_widget",
    "popup_position",
    "push_stack_info",
    "round_pos_to_pixel_center",
    "round_pos_to_pixels",
    "round_rect_to_pixels",
    "round_to_pixel",
    "round_vec_to_pixels",
    "rounding",
    "set_enabled",
    "set_fade_to_color",
    "set_sizing_pass",
    "set_visible",
    "show_tooltip",
    "show_tooltip_at",
    "show_tooltip_at_pointer",
    "show_tooltip_for",
    "show_tooltip_text",
    "stroke_ui",
    "toggle_popup",
    "was_tooltip_open_last_frame",
    "with_layer_id",
    "wrap_text",
];

/// The whole egui designer of a document, as the user would receive it.
fn designer(doc: &Document) -> String {
    crate::GeradorRxp
        .gerar(doc, "app", &crawrad_registry::Registry::builtin())
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

/// **One of every class in the catalogue**, plus a `PageControl` holding two
/// real pages — a `TabSheet` only exists inside one, and the branch that draws
/// it would never be emitted otherwise.
///
/// A class added to the catalogue tomorrow is swept tomorrow, with no edit
/// here: the list comes from [`Registry::builtin`].
fn form_with_everything() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the brand new root");
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
            .expect("the catalogue instantiates what it lists");
        d.add_child("form_main", n).expect("it goes on the form");
        y += h + 4.0;
    }

    let pgc = r
        .suggest_id("PageControl", |i| d.node(i).is_some())
        .to_string();
    let n = r
        .instantiate("PageControl", pgc.clone(), Rect::new(8.0, y, 320.0, 200.0))
        .expect("instantiates");
    d.add_child("form_main", n).expect("it goes on the form");
    for _ in 0..2 {
        let id = r.suggest_id("TabSheet", |i| d.node(i).is_some());
        let t = r
            .instantiate("TabSheet", id, Rect::new(0.0, 24.0, 320.0, 176.0))
            .expect("instantiates");
        d.add_child(&pgc, t).expect("the page goes in the control");
    }

    d.validate().expect("valid");
    d
}

/// Every call site of `name` in `source`, as `line number: line`.
fn call_sites(source: &str, name: &str) -> Vec<String> {
    let needle = format!(".{name}(");
    source
        .lines()
        .enumerate()
        .filter(|(_, l)| l.contains(&needle))
        .map(|(i, l)| format!("{}: {}", i + 1, l.trim()))
        .collect()
}

/// **The charge.** No dead egui call in the file the user cannot fix.
#[test]
fn the_designer_calls_no_egui_api_removed_in_036() {
    let source = designer(&form_with_everything());

    let mut found = Vec::new();
    for name in REMOVED_IN_EGUI_036 {
        for site in call_sites(&source, name) {
            found.push(format!("`{name}` in {site}"));
        }
    }

    assert!(
        found.is_empty(),
        "the designer calls egui API that 0.36 removed — the generated project \
         will NOT compile, and the user cannot patch it because `.designer.rs` \
         is rewritten on every generation:\n  {}",
        found.join("\n  ")
    );
}

/// **The guard bites.** The sweep is only worth its line count if it fails on
/// the very text that shipped the defect, so it is fed that text here.
///
/// This is the mutation done honestly: `allocate_ui_at_rect` is what
/// `scroll_box.rs` emitted until 16/09/2026.
#[test]
fn the_sweep_catches_the_call_that_shipped_broken() {
    let shipped = "            ui.add_enabled_ui(is_enabled, |ui| {\n\
                   \x20               ui.allocate_ui_at_rect(r, |ui| {\n";
    let mut found = Vec::new();
    for name in REMOVED_IN_EGUI_036 {
        found.extend(call_sites(shipped, name));
    }
    assert_eq!(
        found.len(),
        1,
        "the sweep has to see `allocate_ui_at_rect` and nothing else here: {found:?}"
    );
}
