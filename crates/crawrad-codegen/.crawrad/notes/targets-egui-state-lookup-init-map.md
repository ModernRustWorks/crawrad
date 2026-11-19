---
id: targets-egui-state-lookup-init-map
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 715
author: 
created: 
title: "lookup_init_map"
tags: []
signature: "pub fn lookup_init_map (class : & str , field : & str) -> Option < InitFn >"
---

Finds a computed initialiser for `class` and `field`, if any.

# Examples

```rust
use crawrad_codegen::targets::egui::state::lookup_init_map;

let init = lookup_init_map("Button", "text");
assert!(init.is_some());
```
