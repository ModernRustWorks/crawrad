---
id: ide-app-explorer-actions-ideapp-abrir-aba-relativa
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 11
author: 
created: 
title: "IdeApp::abrir_aba_relativa"
tags: []
signature: "pub (super) fn abrir_aba_relativa (& mut self , cx : & mut Cx , rel : & crawrad_actions :: PathInProject)"
---

The member that owns the open form — `<membro>/src/<nome>.crawform`.

Derived from the path, not stored in a field: a "current member" field
alongside the open document would be two states that could disagree, and
the document is what governs (`workspace::membro_do_documento`).
**Opens a file from inside the generated project in a tab.**

The path arrives RELATIVE and validated — `CaminhoNoProjeto` only exists
if it passed through the catalog's check, which is where it counts
(ADR 0005: MCP applies the same `Efeito`). The one that knows where the
project is is this shell, and it is the one that joins the two.

# A PLANNED file is not an error

The Explorer displays the generation plan before it exists on disk
(ADR 0016), so clicking a pending file is a normal gesture. The
correct response is to say what is missing, rather than opening a
tab with the operating system error inside — which was what the egui
shell already learned in its `abrir_no_editor`.
