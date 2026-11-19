---
id: ide-app-dialogs-items-ideapp-open-items
kind: doc
target_file: src/ide_app/dialogs/items.rs
line_hint: 12
author: 
created: 
title: "IdeApp::open_items"
tags: []
signature: "pub (in super :: super) fn open_items (& mut self , cx : & mut Cx , no : NodeId)"
---

Opens the collection editor for the node.

The state comes from [`ItemsEditorState::from_node`], which is the SAME
constructor used by the egui shell: it is what reads the items from the document and
asks the catalog whether this collection accepts children. None of that is
answered here.

## Inside `open_items`: `let Some(registro) = self.designer.registry.as_deref() else {`

WITHOUT A CATALOG IT DOES NOT OPEN, and this is not pedantry: it is the catalog that indicates
whether this collection accepts children, and opening without it would result in a dimmed
`New SubItem` in a tree — the opposite of what was promised.

## Inside `open_items`: `self.items_na_tela = None;`

Initializes with nothing on screen, so that the drawing logic writes the text field
of the selected item — see [`IdeApp::items_na_tela`].
