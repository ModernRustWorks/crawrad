---
id: targets-egui-components-envolve-filhos
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 193
author: 
created: 
title: "envolve_filhos"
tags: []
signature: "pub fn envolve_filhos (classe : & str) -> bool"
---

Checks whether the class wraps its child nodes.

Does this class wrap its children?

It queries the emitter catalog, rather than a list written here: a class
that starts wrapping tomorrow answers on its own.

# Examples

```rust
use crawrad_codegen::targets::egui::components::envolve_filhos;

assert!(!envolve_filhos("Button"));
```
