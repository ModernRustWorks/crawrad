---
id: components-classes
kind: doc
target_file: src/components/mod.rs
line_hint: 166
author: 
created: 
title: "classes"
tags: []
signature: "pub fn classes () -> Vec < & 'static str >"
---

The classes that this crate draws and that become widgets.

The classes that this crate draws **and that become widgets**, in catalog
order.

This is where [`crate::CLASSES`] comes from: the runtime's class list is not
handwritten anywhere.

Non-visual components are left out, as dictated by the `Runtime::classes` contract:
*"non-visuals do not enter … they become an icon in the designer and nothing in
the application"* (`crawrad-runtime/src/lib.rs:166-168`). It is in the catalog
because the canvas DRAWS it; it is omitted from this list only because the application does
not instantiate it.

# Examples

```rust
use crawrad_runtime_makepad::components::classes;

let list = classes();
assert!(list.contains(&"Button"));
```
