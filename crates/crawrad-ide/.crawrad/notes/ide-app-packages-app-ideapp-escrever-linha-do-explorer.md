---
id: ide-app-packages-app-ideapp-escrever-linha-do-explorer
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 120
author: 
created: 
title: "IdeApp::escrever_linha_do_explorer"
tags: []
signature: "pub (super) fn escrever_linha_do_explorer (cx : & mut Cx , item : & WidgetRef , linha : & explorer :: Row)"
---

Writes an Explorer line into the widget created by the template.

**FIVE CALLS, and no `match`** — and that is what makes a new line
type cost one file and nothing more. Every template of the panel has `face`, and
inside it the four names below, of which only `nome` is mandatory; whatever
a template does not declare returns an empty `WidgetRef`, and a `set_text` on
an empty widget does nothing. The painter does not know what type of line it is
drawing, and has no way of being wrong.

INDENTATION is pixels, not whitespace in text — the same decision (and the
same number) as the Structure tree and the egui shell tree
(`tree.rs:127-140`). It goes on `face`, not on the row: it is what holds
the content, and the click target retains the full width.
