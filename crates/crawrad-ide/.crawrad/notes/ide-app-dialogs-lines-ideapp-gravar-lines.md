---
id: ide-app-dialogs-lines-ideapp-gravar-lines
kind: doc
target_file: src/ide_app/dialogs/lines.rs
line_hint: 97
author: 
created: 
title: "IdeApp::gravar_lines"
tags: []
signature: "pub (in super :: super) fn gravar_lines (& mut self , cx : & mut Cx)"
---

Writes the edited text back to the document.

Takes the text to the document — **via the catalog action**, and passing through the
row editor.

`PropEditor::window_value` is what reads the text back, as in the earlier pad:
converting here would place the forward conversion in one file and the return in another.
