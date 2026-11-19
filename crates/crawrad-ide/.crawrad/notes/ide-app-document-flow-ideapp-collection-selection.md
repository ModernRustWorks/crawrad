---
id: ide-app-document-flow-ideapp-collection-selection
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 248
author: 
created: 
title: "IdeApp::collection_selection"
tags: []
signature: "pub (crate) fn collection_selection (& self) -> Option < NodeId >"
---

**The selected node, when it is a COLLECTION** — and `None` when it is not.

A single answer, and two consumers: clicking the `+` in the
Structure header and its dimmed state. Were they two separate calculations, the button would light up
for one node and reject another, which is the family of defects where a button
lies.

What determines whether it is a collection is the **descriptor**, not a list written
here: `classe_do_item()` returns `Some` specifically for whatever declared
`Filhos::NaColecao { item }` (ADR 0037). A third-party component participates
in this without anyone having to touch this function.
