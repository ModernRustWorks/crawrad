//! Canvas components — **one file per class**.
//!
//! Each `<class>.rs` handles its entire lifecycle and returns the result to
//! the canvas: where row content comes from and how it paints itself. The contract
//! is in [`drawing::ComponentDrawing`]; below is only the **catalogue**.
//!
//! # Why a catalogue, and not a `match`
//!
//! The project rule (`CONTRIBUTING.md`, *Makepad component rules*) forbids a
//! monolithic `match` block by class, and the reason is the double maintenance
//! point: in a `match`, adding a component requires remembering N places, and
//! forgetting one is silent. The `DESENHOS` list below **has no behavior** —
//! it does not know what any component draws, nor the class name (which comes
//! from the file itself via `classe()`). Adding a component means creating the
//! file and referencing it once here; everything else — drawing, rows, and
//! [`crate::CLASSES`] — comes for free.
//!
//! # Why dispatch does not query `crawrad-registry`
//!
//! Because the adapter **does not depend on the catalogue**, and that is a boundary
//! decision, not an oversight: `crawrad-registry` is a `dev-dependency` in this
//! crate's `Cargo.toml`, exactly as in the RXP (`crawrad-runtime-egui`). What the
//! catalogue defines — tab strip constants, for example — is **mirrored** here and
//! verified by a test (`canvas::tests::a_tira_de_abas_espelha_o_catalogo`).
//! Replacing this with a real dependency would tie drawing to the catalogue
//! at compile time with nothing in return: the class name already appears once,
//! inside the component file.

pub mod action;
pub mod action_list;
pub mod badge;
pub mod bar;
pub mod bevel;
pub mod bit_btn;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod category_panel;
pub mod check_box;
pub mod check_list_box;
pub mod chips;
pub mod color_box;
pub mod combo_box;
pub mod comum;
pub mod date_time_picker;
pub mod declarado;
pub mod drawer;
pub mod drawing;
pub mod edit;
pub mod generic;
pub mod grid;
pub mod grid_column;
pub mod group_box;
pub mod image;
pub mod image_list;
pub mod label;
pub mod link_label;
pub mod list_box;
pub mod list_column;
pub mod list_view;
pub mod main_menu;
pub mod mask_edit;
pub mod memo;
pub mod menu_item;
pub mod message_dialog;
pub mod multi_select;
pub mod non_visual;
pub mod open_dialog;
pub mod page_control;
pub mod pagination;
pub mod paint_box;
pub mod panel;
pub mod popup_menu;
pub mod progress_bar;
pub mod radio_button;
pub mod radio_group;
pub mod save_dialog;
pub mod scroll_bar;
pub mod scroll_box;
pub mod shape;
pub mod speed_button;
pub mod spin_edit;
pub mod splitter;
pub mod status_bar;
pub mod status_panel;
pub mod style_rule;

pub mod tab_sheet;
pub mod timer;
pub mod toggle;
pub mod tool_bar;
pub mod track_bar;
pub mod tree_view;

pub use comum::*;
use crawrad_note::note;
pub use drawing::{ComponentDrawing, Scene, text_color};

#[note("components-desenhos")]
static DESENHOS: &[&'static dyn ComponentDrawing] = &[
    &non_visual::DECLARED,
    &action::DESENHO,
    &action_list::DESENHO,
    &bit_btn::Desenho,
    &button::Desenho,
    &check_box::Desenho,
    &combo_box::Desenho,
    &date_time_picker::Drawing,
    &edit::Desenho,
    &mask_edit::Drawing,
    &grid::Desenho,
    &grid_column::Drawing,
    &style_rule::Drawing,
    &group_box::Desenho,
    &image::Drawing,
    &image_list::DESENHO,
    &label::Desenho,
    &list_box::Desenho,
    &list_view::Drawing,
    &list_column::Drawing,
    &main_menu::DESENHO,
    &menu_item::DRAWING,
    &memo::Desenho,
    &open_dialog::DESENHO,
    &message_dialog::DESENHO,
    &page_control::Desenho,
    &paint_box::Desenho,
    &panel::Desenho,
    &popup_menu::DESENHO,
    &progress_bar::Desenho,
    &radio_button::Desenho,
    &radio_group::Drawing,
    &save_dialog::DESENHO,
    &scroll_bar::Desenho,
    &scroll_box::Desenho,
    &shape::Desenho,
    &speed_button::Desenho,
    &spin_edit::Drawing,
    &color_box::Drawing,
    &link_label::Drawing,
    &bevel::Drawing,
    &badge::Drawing,
    &card::Drawing,
    &drawer::Drawing,
    &category_panel::Drawing,
    &chips::Drawing,
    &breadcrumb::Drawing,
    &pagination::Drawing,
    &multi_select::Drawing,
    &check_list_box::Drawing,
    &toggle::Drawing,
    &splitter::Desenho,
    &status_bar::Desenho,
    &status_panel::Drawing,
    &tab_sheet::Desenho,
    &timer::DESENHO,
    &tool_bar::Desenho,
    &track_bar::Desenho,
    &tree_view::Desenho,
];

#[note("components-desenho-de")]
pub fn desenho_de(classe: &str) -> &'static dyn ComponentDrawing {
    DESENHOS
        .iter()
        .copied()
        .find(|d| d.class_name() == classe)
        .unwrap_or(&generic::Desenho)
}

#[note("components-classes")]
pub fn classes() -> Vec<&'static str> {
    DESENHOS
        .iter()
        .filter(|d| d.visual())
        .map(|d| d.class_name())
        .collect()
}

#[note("components-e-visual")]
pub fn e_visual(classe: &str) -> bool {
    desenho_de(classe).visual()
}
