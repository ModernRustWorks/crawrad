---
id: hostmodel-strip-click
kind: doc
target_file: src/lib.rs
line_hint: 165
author: 
created: 
title: "StripClick"
tags: []
signature: "pub enum StripClick"
---

What a click on a strip row means.

The shell asks rather than subtracting: the boundary between the forms and the
loose files is a fact about how many forms are open, and that fact lives here.

# Examples

```rust
use crawrad_host::StripClick;

// Each kind carries its own index, counted from zero within its own list.
assert_eq!(StripClick::Form(0), StripClick::Form(0));
assert_ne!(StripClick::Form(0), StripClick::File(0));
```
