---
id: debug-panel-is-own-source
kind: doc
target_file: src/debug_panel.rs
line_hint: 228
author: 
created: 
title: "is_own_source"
tags: []
signature: "pub fn is_own_source (path : & Path) -> bool"
---

Returns whether a source path belongs to the project itself.

Whether a frame's file is the project's own: a real `.rs` file, not a
crate from the cargo registry or a git checkout, not the toolchain's library
(`rustlib`, the `/rustc/<hash>/` paths of a prebuilt std). Measured on
30/09/2026, `lldb-dap` also names a "source" for code with none — the module
and symbol, `user32.dll`CallWindowProcW`, `meu_app.exe`main` — and the C
runtime's `exe_common.inl`; neither is the project's.

# Examples

```rust
use std::path::Path;
use crawrad_host::debug_panel::is_own_source;

assert!(is_own_source(Path::new("src/main.rs")));
```
