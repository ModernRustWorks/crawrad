---
id: ide-app-status-bar-slots
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 47
author: 
created: 
title: "SLOTS"
tags: []
signature: "pub (super) const SLOTS : & [& [LiveId]] = & [id ! (bottom_status_bar . cell_problems . lbl_errors) , id ! (bottom_status_bar . cell_problems . lbl_warnings) , id ! (bottom_status_bar . cell_lsp . lbl_lsp) , id ! (bottom_status_bar . cell_lsp_progress . lbl_lsp_progress) , id ! (bottom_status_bar . cell_target . lbl_target) , id ! (bottom_status_bar . cell_project . lbl_project) , id ! (bottom_status_bar . cell_message . opened_file_label) , id ! (bottom_status_bar . cell_selection . status_nodes_label) , id ! (bottom_status_bar . cell_nodes . lbl_nodes) , id ! (bottom_status_bar . cell_rev . lbl_rev) , id ! (bottom_status_bar . cell_zoom . lbl_zoom) , id ! (bottom_status_bar . cell_framework . pill . lbl_framework) ,]"
---

The label of every cell, in the order [`IdeApp::celulas_do_rodape`] fills
them.

It is the single statement of the pairing between a value and the place it
is drawn. A cell added here without a widget of that name draws nowhere and
says nothing — which is why the DSL gate reads this list.
