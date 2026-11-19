---
id: targets-makepad-components-classes
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 161
author: 
created: 
title: "classes"
tags: []
signature: "pub fn classes () -> Vec < & 'static str >"
---

The classes that this target knows how to generate.

**The classes that THIS target knows how to generate** — derived from what it registered.

It used to come from `ComponentClass::ALL_STR`, which was the catalog's `enum`, and not
what this target actually registered. They coincided by luck. Derived from
`REGISTRY`, it becomes true by construction.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::classes;

let list = classes();
assert!(list.contains(&"Button"));
```
