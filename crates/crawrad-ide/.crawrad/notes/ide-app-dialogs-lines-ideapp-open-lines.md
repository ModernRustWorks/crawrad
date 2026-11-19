---
id: ide-app-dialogs-lines-ideapp-open-lines
kind: doc
target_file: src/ide_app/dialogs/lines.rs
line_hint: 11
author: 
created: 
title: "IdeApp::open_lines"
tags: []
signature: "pub (in super :: super) fn open_lines (& mut self , cx : & mut Cx , no : NodeId , key : & str)"
---

Opens the multiline text editor for the property.

The state comes from [`LinesEditorState::from_node`], which is the SAME
constructor used by the egui shell: it is what reads the value from the document and falls
back to the DEFAULT value when the node does not have the key — opening the window with
empty text while the panel row says `3 lines` would be the window and the
panel disagreeing.

#### At `let novo = self`

THE DESCRIPTOR is what provides the property label and default value.
Without it — a class or key unknown to the catalog — there is neither title
nor honest content, and the window does not open. It is the same guard as in the egui
shell (`abrir_editor_linhas`).
