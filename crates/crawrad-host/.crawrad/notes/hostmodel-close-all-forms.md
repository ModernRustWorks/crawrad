---
id: hostmodel-close-all-forms
kind: doc
target_file: src/lib.rs
line_hint: 318
author: 
created: 
title: "HostModel::close_all_forms"
tags: []
signature: "pub fn close_all_forms (& mut self)"
---

Empties the strip, as closing a project does.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;

let mut host = HostModel::new();
host.open_form(Doc::default(), None);
host.close_all_forms();
assert!(host.forms().is_empty());
assert_eq!(host.front(), None);
```
