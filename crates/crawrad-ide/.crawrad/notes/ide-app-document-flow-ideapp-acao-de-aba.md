---
id: ide-app-document-flow-ideapp-acao-de-aba
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 218
author: 
created: 
title: "IdeApp::acao_de_aba"
tags: []
signature: "pub (crate) fn acao_de_aba (& mut self , cx : & mut Cx , acao : TabAction)"
---

Clicking a `PageControl` tab.

Opening the page is `component.set_property` — the open page is
a document property, not IDE state, exactly as the egui
shell already resolves (`main.rs:4372-4386`). Creating a page does not yet have
a catalog path (it is `AddNode` and `SetProp` in the same transaction), and the
bar reports this instead of pretending it was created.

#### At `self.execute_with(cx, "page.new", crawrad_actions::Args::text("control", `

The control goes by ARGUMENT, never by the selection: the
`+` that was clicked belongs to a `PageControl` that may not
be the selected one — and after this, the selection is the
page that was just born, which the action returns in the
effect. Reading the selection here would create the page in
the wrong control whenever two `PageControl` are on screen.

`page.new` already exists in the catalogue
(`crawrad-actions/src/lib.rs`, group `Edit`, hint *"Add a page to
the selected PageControl, and open it"*) and the egui shell
has called it since day one (`main.rs:4305`). This branch
used to answer `new page is not wired in the Makepad shell
yet` — the capability was there, only this shell did not ask
for it.
