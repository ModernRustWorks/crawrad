---
id: targets-makepad-binding-binding
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 62
author: 
created: 
title: "Binding"
tags: []
signature: "pub struct Binding"
---

How this target binds a catalog event to the framework.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::{Binding, Shape};

let b = Binding { accessor: "button", part: "", query: "clicked", shape: Shape::Plain, writeback: None };
assert_eq!(b.accessor, "button");
```
