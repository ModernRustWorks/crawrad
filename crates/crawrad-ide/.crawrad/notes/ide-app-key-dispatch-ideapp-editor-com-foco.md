---
id: ide-app-key-dispatch-ideapp-editor-com-foco
kind: doc
target_file: src/ide_app/key_dispatch.rs
line_hint: 11
author: 
created: 
title: "IdeApp::editor_com_foco"
tags: []
signature: "pub (crate) fn editor_com_foco (& self , cx : & Cx) -> bool"
---

Checks whether the code view has keyboard focus.

A key that reached the shell, already knowing whether it came from a text
field that rejected it.

All dispatch goes through [`teclado::comandos`], which reads
`crawrad_actions::catalogo()` — **there is not a single chord `match` line
here**, and that is what makes the 18 declared shortcuts light up at once
and the 19th be born connected (ADR 0011).

Redrawing at the end, just once, is intentional: a key can trigger
more than one command, and reopening the canvas for each one would cost a document
scan per command.
**Does the code view have keyboard focus?**

The FOCUS, and not whether the view is visible: with a dialog open on top of the
code, the key belongs to the dialog — and it deserves it.
