---
id: hostmodel-strip-target
kind: doc
target_file: src/lib.rs
line_hint: 360
author: 
created: 
title: "HostModel::strip_target"
tags: []
signature: "pub fn strip_target (& self, i : usize) -> StripClick"
---

Reads a click on a strip row.

Row `i` is either a form or a loose file, decided by the same `forms()` the strip
was drawn from.

# Examples

```rust
use crawrad_host::{HostModel, StripClick};
use crawrad_doc::Doc;
use crawrad_ir::Document;

let mut host = HostModel::new();
host.open_form(Doc::new(Document::new("form_main", "Form")), None);

// The forms come first, so row 0 is the form.
assert_eq!(host.strip_target(0), StripClick::Form(0));
// Row 1 is the first loose file, counted from zero again.
assert_eq!(host.strip_target(1), StripClick::File(0));
```
