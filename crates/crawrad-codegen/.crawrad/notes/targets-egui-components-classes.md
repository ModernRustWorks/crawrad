---
id: targets-egui-components-classes
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 168
author: 
created: 
title: "classes"
tags: []
signature: "pub fn classes () -> Vec < & 'static str >"
---

The classes that this target knows how to generate.

**The classes that THIS target knows how to generate** — derived from what it registered.

It used to come from `ComponentClass::ALL_STR`, which was the catalog's `enum`: the
target claimed to generate the sixteen variants of the enum, rather than those it actually
registered. They coincided by luck.

Derived from `REGISTRY`, it becomes true by construction — and a
third-party component that registers code generation appears here without
anyone editing any list.

# Examples

```rust
use crawrad_codegen::targets::egui::components::classes;

let list = classes();
assert!(list.contains(&"Button"));
```
