---
id: components-desenhos
kind: doc
target_file: src/components/mod.rs
line_hint: 97
author: 
created: 
title: "DESENHOS"
tags: []
signature: "static DESENHOS : & [& 'static dyn ComponentDrawing] = & [& non_visual :: DECLARED , & action :: DESENHO , & action_list :: DESENHO , & bit_btn :: Desenho , & button :: Desenho , & check_box :: Desenho , & combo_box :: Desenho , & date_time_picker :: Drawing , & edit :: Desenho , & mask_edit :: Drawing , & grid :: Desenho , & grid_column :: Drawing , & style_rule :: Drawing , & group_box :: Desenho , & image :: Drawing , & image_list :: DESENHO , & label :: Desenho , & list_box :: Desenho , & list_view :: Drawing , & list_column :: Drawing , & main_menu :: DESENHO , & menu_item :: DRAWING , & memo :: Desenho , & open_dialog :: DESENHO , & message_dialog :: DESENHO , & page_control :: Desenho , & paint_box :: Desenho , & panel :: Desenho , & popup_menu :: DESENHO , & progress_bar :: Desenho , & radio_button :: Desenho , & radio_group :: Drawing , & save_dialog :: DESENHO , & scroll_bar :: Desenho , & scroll_box :: Desenho , & shape :: Desenho , & speed_button :: Desenho , & spin_edit :: Drawing , & color_box :: Drawing , & link_label :: Drawing , & bevel :: Drawing , & badge :: Drawing , & card :: Drawing , & drawer :: Drawing , & category_panel :: Drawing , & chips :: Drawing , & breadcrumb :: Drawing , & pagination :: Drawing , & multi_select :: Drawing , & check_list_box :: Drawing , & toggle :: Drawing , & splitter :: Desenho , & status_bar :: Desenho , & status_panel :: Drawing , & tab_sheet :: Desenho , & timer :: DESENHO , & tool_bar :: Desenho , & track_bar :: Desenho , & tree_view :: Desenho ,]"
---

The canvas drawing catalog — one line per file, and nothing more.

The order governs two things, and only those: the order of [`crate::CLASSES`] and
which file responds if two declare the same class (the first one).
