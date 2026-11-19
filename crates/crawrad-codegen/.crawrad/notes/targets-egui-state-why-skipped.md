---
id: targets-egui-state-why-skipped
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 433
author: 
created: 
title: "why_skipped"
tags: []
signature: "pub fn why_skipped (class : & str , key : & str) -> Option < & 'static str >"
---

Checks whether a catalogue property is skipped for `class`.

# Examples

```rust
use crawrad_codegen::targets::egui::state::why_skipped;

assert!(why_skipped("Button", "action").is_some());
```
