---
id: ide-app-link-ideapp-abrir-handler-da-linha
kind: doc
target_file: src/ide_app/link.rs
line_hint: 21
author: 
created: 
title: "IdeApp::abrir_handler_da_linha"
tags: []
signature: "pub (super) fn abrir_handler_da_linha (& mut self , cx : & mut Cx , i : usize)"
---

**The link**: double clicking a line in the `Events` tab leads to its handler.

It is the classic Object Inspector gesture, and the cycle is the same that the
egui shell already performs in `Ide::abrir_evento_no_codigo`
(`crates/crawrad-ide/src/main.rs`):

1. **names it**, if the event does not point to anything yet. The name is
   [`crawrad_codegen::handler_padrao`] — the same one the generator would use, and the
   same one that the pruning of ADR 0025 recognizes. A second naming written
   here would be the ruler that pruning does not know;
2. **binds** via the catalogue action `event.bind` — no `Transaction`
   is born in this shell, and a `Ctrl+Z` undoes the naming just like it undoes
   anything else;
3. **takes to code** ([`IdeApp::abrir_no_codigo`]).

THE LIST IS RE-READ here, and the index is the only thing crossing
[`IdeApp::rects_dos_eventos`]: the rectangle cache belongs to the previous
frame, and storing the event inside it would store a selection that
may have changed since then.
