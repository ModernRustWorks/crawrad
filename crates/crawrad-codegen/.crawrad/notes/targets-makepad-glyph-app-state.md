---
id: targets-makepad-glyph-app-state
kind: doc
target_file: src/targets/makepad/glyph.rs
line_hint: 47
author: 
created: 
title: "app_state"
tags: []
signature: "pub fn app_state (field : & str) -> AppState"
---

What the glyph keeps in the `App`: what it shows now.

# Examples

```rust
use crawrad_codegen::targets::makepad::glyph::app_state;

let state = app_state("btn1");
assert!(state.field.contains("btn1_glyph_shown"));
```
