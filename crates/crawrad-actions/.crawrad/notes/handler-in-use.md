---
id: handler-in-use
kind: doc
target_file: src/lib.rs
line_hint: 1270
author: 
created: 
title: "handler_in_use"
tags: []
signature: "fn handler_in_use (ir : & crawrad_ir :: Document , handler : & str) -> bool"
---

Valid whenever there is an open document.

Replaced an `always` that said `true` without looking at anything. It was right
while a document was guaranteed; with ADR 0017 it ceased to be, and the three
actions using it — `file.save`, `file.reload` and tab order renumbering —
would start appearing lit in an IDE without a form, to do
nothing when clicked.

Actions that are valid EVEN without a document use no predicate: they are
about the folder, and declare `|_| true` instead.
Whether some event of the form is bound to `handler`.
