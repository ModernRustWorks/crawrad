---
id: ide-app-explorer-actions-ideapp-linhas-do-explorer
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 105
author: 
created: 
title: "IdeApp::linhas_do_explorer"
tags: []
signature: "pub (super) fn linhas_do_explorer (& mut self) -> Vec < explorer :: Row >"
---

The Explorer rows, assembled from the document on each draw.

The panel stores nothing beyond folds: it recomputes from the document
and from disk, which is why it never gets out of date. It is the same
decision as the egui shell, with the same rationale — `gerar()` costs
microseconds for a form, and a cache that no one invalidates is how a
panel ends up showing the project from before.
