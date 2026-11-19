---
id: viewcenter-toggle-form-source
kind: doc
target_file: src/lib.rs
line_hint: 96
author: 
created: 
title: "ViewCenter::toggle_form_source"
tags: []
signature: "pub fn toggle_form_source (self) -> Self"
---

`Alt+F12`: always to form-source, only from inside it comes back.

# Examples

```rust
use crawrad_host::ViewCenter;

assert_eq!(ViewCenter::Design.toggle_form_source(), ViewCenter::FormSource);
```
