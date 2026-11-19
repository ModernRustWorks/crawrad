---
id: targets-makepad-components-for-class
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 131
author: 
created: 
title: "for_class"
tags: []
signature: "pub fn for_class (class : & str) -> Option < & 'static dyn MakepadComponentCodegen >"
---

Returns the codegen lifecycle handler for a component class.

Returns the codegen lifecycle handler for the specified component class, if registered.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::for_class;

let codegen = for_class("Button");
assert!(codegen.is_some());
```
