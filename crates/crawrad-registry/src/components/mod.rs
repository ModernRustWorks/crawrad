pub mod action_list;
pub mod badge;
pub mod base;
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
pub mod date_time_picker;
pub mod drawer;
pub mod edit;
pub mod form;
pub mod grid;
pub mod group_box;
pub mod image;
pub mod image_list;
pub mod label;
pub mod link_label;
pub mod list_box;
pub mod list_view;
pub mod main_menu;
pub mod mask_edit;
pub mod memo;
pub mod message_dialog;
pub mod multi_select;
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

pub mod timer;
pub mod toggle;
pub mod tool_bar;
pub mod track_bar;
pub mod tree_view;

pub use crate::{Children, ComponentDesc, EventDesc, Events};
pub use action_list::{action, action_list};
pub use badge::badge;
pub use base::*;
pub use bevel::bevel;
pub use bit_btn::bit_btn;
pub use breadcrumb::breadcrumb;
pub use button::button;
pub use card::card;
pub use category_panel::category_panel;
pub use check_box::check_box;
pub use check_list_box::check_list_box;
pub use chips::chips;
pub use color_box::color_box;
pub use combo_box::combo_box;
use crawrad_note::note;
pub use date_time_picker::date_time_picker;
pub use drawer::drawer;
pub use edit::edit;
pub use form::form;
pub use grid::{grid, grid_column, style_rule};
pub use group_box::group_box;
pub use image::image;
pub use image_list::image_list;
pub use label::label;
pub use link_label::link_label;
pub use list_box::list_box;
pub use list_view::{list_column, list_view};
pub use main_menu::{main_menu, menu_item};
pub use mask_edit::mask_edit;
pub use memo::memo;
pub use message_dialog::message_dialog;
pub use multi_select::multi_select;
pub use open_dialog::open_dialog;
pub use page_control::{
    PLUS_BUTTON_WIDTH, TAB_SCROLL_BUTTON_WIDTH, TAB_STRIP_HEIGHT, TAB_WIDTH, TabWindow,
    page_control, pages_top, tab_scroll_left_x, tab_sheet, tab_strip_top, tab_window,
    tabs_at_bottom,
};
pub use pagination::pagination;
pub use paint_box::paint_box;
pub use panel::panel;
pub use popup_menu::popup_menu;
pub use progress_bar::progress_bar;
pub use radio_button::radio_button;
pub use radio_group::radio_group;
pub use save_dialog::save_dialog;
pub use scroll_bar::scroll_bar;
pub use scroll_box::scroll_box;
pub use shape::shape;
pub use speed_button::speed_button;
pub use spin_edit::spin_edit;
pub use splitter::splitter;
pub use status_bar::{status_bar, status_panel};
pub use timer::timer;
pub use toggle::toggle;
pub use tool_bar::tool_bar;
pub use track_bar::track_bar;
pub use tree_view::tree_view;

#[note("components-all-builtin")]
pub fn all_builtin() -> Vec<ComponentDesc> {
    vec![
        form(),
        label(),
        button(),
        edit(),
        combo_box(),
        color_box(),
        check_box(),
        radio_button(),
        radio_group(),
        list_box(),
        list_view(),
        list_column(),
        memo(),
        panel(),
        group_box(),
        page_control(),
        tab_sheet(),
        tree_view(),
        grid(),
        grid_column(),
        style_rule(),
        scroll_bar(),
        scroll_box(),
        image(),
        action_list(),
        action(),
        image_list(),
        timer(),
        bit_btn(),
        speed_button(),
        spin_edit(),
        toggle(),
        link_label(),
        mask_edit(),
        date_time_picker(),
        bevel(),
        badge(),
        card(),
        drawer(),
        category_panel(),
        chips(),
        breadcrumb(),
        pagination(),
        multi_select(),
        check_list_box(),
        main_menu(),
        menu_item(),
        popup_menu(),
        tool_bar(),
        status_bar(),
        status_panel(),
        progress_bar(),
        track_bar(),
        paint_box(),
        shape(),
        splitter(),
        open_dialog(),
        save_dialog(),
        message_dialog(),
    ]
    .into_iter()
    .map(with_classic_events)
    .collect()
}
