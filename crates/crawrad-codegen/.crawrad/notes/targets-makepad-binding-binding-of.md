---
id: targets-makepad-binding-binding-of
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 254
author: 
created: 
title: "binding_of"
tags: []
signature: "pub fn binding_of (class : & str , event : & str) -> Option < Binding >"
---

The binding that this target has for `(classe, evento)`, if any.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::binding_of;

let b = binding_of("Button", "on_click");
assert!(b.is_some());
```
