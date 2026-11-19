---
id: ide-app-arrangement-ideapp-escrever-x-dos-paineis
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 388
author: 
created: 
title: "IdeApp::escrever_x_dos_paineis"
tags: []
signature: "pub (super) fn escrever_x_dos_paineis (& mut self , cx : & mut Cx)"
---

Writes the X glyph in the header of every closable panel that currently
exists.

The `btn_panel_close` has the SAME id across the five headers, intentionally: the
search descends through the panel (`WidgetRef::widget` searches by name in the
descendants), so adding a panel does not require a new path written
here.

## Inside `escrever_x_dos_paineis`: `self.dock()`

The collection `+`, in the Structure header. Same path and same
moment: it is an icon glyph, and an icon glyph does not go into the DSL (see
`icon.rs` — `live_design!`'s tokenizer does not know `\u{}`).
