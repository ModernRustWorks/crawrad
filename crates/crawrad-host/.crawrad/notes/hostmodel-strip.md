---
id: hostmodel-strip
kind: doc
target_file: src/lib.rs
line_hint: 310
author: 
created: 
title: "HostModel::strip"
tags: []
signature: "pub fn strip (& self, files : & [(String, bool)]) -> Vec < StripRow >"
---

Builds the strip of tabs.

The open forms come first, then the loose files the shell passes in. They are
kept in different lists because they are different things, and they are joined
HERE rather than in the drawing code, so the order and the boundary are one fact
with one owner.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;
use crawrad_ir::Document;

let mut host = HostModel::new();
host.open_form(Doc::new(Document::new("form_main", "Form")), None);

let rows = host.strip(&[("main.rs".to_string(), false)]);
assert_eq!(rows.len(), 2);
assert!(rows[0].is_form);
assert!(rows[0].label.ends_with("form_main"));
assert!(!rows[1].is_form);
assert_eq!(rows[1].label, "main.rs");
```
