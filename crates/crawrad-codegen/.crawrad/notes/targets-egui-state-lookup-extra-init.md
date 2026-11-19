---
id: targets-egui-state-lookup-extra-init
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 518
author: 
created: 
title: "lookup_extra_init"
tags: []
signature: "pub fn lookup_extra_init (class : & str , field : & str) -> Option < & 'static str >"
---

Returns the initial literal for an extra field, if defined.

# Examples

```rust
use crawrad_codegen::targets::egui::state::lookup_extra_init;

assert_eq!(lookup_extra_init("Grid", "cells"), Some("Default::default()"));
```
