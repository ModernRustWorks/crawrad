---
id: ide-app-host-api-ideapp-projeto-contexto
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 11
author: 
created: 
title: "IdeApp::projeto_contexto"
tags: []
signature: "pub fn projeto_contexto (& self) -> crawrad_actions :: Project"
---

Constructs the current project context for evaluating action guards.

`open` says a Cargo project is open (the Explorer has a root), and `debug`
mirrors the debugger's phase so Step needs a stopped program and Pause a
running one.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
let ctx = app.projeto_contexto();
assert!(ctx.has_binary);
```
