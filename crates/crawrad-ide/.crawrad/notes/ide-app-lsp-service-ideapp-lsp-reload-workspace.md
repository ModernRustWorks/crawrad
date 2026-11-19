---
id: ide-app-lsp-service-ideapp-lsp-reload-workspace
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 132
author: 
created: 
title: "IdeApp::lsp_reload_workspace"
tags: []
signature: "pub (super) fn lsp_reload_workspace (& self)"
---

Reloads the project manifests in rust-analyzer.

rust-analyzer reads the project's manifests again (a `main.rs` or a
dependency just appeared).
