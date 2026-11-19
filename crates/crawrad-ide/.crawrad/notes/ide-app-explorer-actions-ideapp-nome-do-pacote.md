---
id: ide-app-explorer-actions-ideapp-nome-do-pacote
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 52
author: 
created: 
title: "IdeApp::nome_do_pacote"
tags: []
signature: "pub (super) fn nome_do_pacote (& self) -> Option < String >"
---

The package name of the active member, for `cargo` and for the
generator.

`None` when there is no member or its manifest does not provide the
name — and `None` is used: it is what determines if the Package Manager
has a project to write to.
