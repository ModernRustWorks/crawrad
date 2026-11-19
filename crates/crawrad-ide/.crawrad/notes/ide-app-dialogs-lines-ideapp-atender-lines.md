---
id: ide-app-dialogs-lines-ideapp-atender-lines
kind: doc
target_file: src/ide_app/dialogs/lines.rs
line_hint: 66
author: 
created: 
title: "IdeApp::atender_lines"
tags: []
signature: "pub (in super :: super) fn atender_lines (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures for the Lines Editor.

#### At `let aplicar = dlg.button(id!(lines_apply)).clicked(actions);`

`Esc` cancels, as in every window of this shell. `Enter` does NOT
confirm here: in a multiline field it is a line break, which is
what Makepad's `TextInput` does on its own — confirming with it
would make it impossible to write the second line.

## Inside `atender_lines`: `if let Some(texto) = campo.changed(actions) {`

Typing updates the STATE and not the document: what transfers it is `OK` or
`Apply`, as in egui. And the footer count tracks each letter,
because it is derived from the state.
