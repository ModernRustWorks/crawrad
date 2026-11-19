---
id: document-referrers
kind: doc
target_file: src/lib.rs
line_hint: 420
author: 
created: 
title: "Document::referrers"
tags: []
signature: "pub fn referrers (& self , target : & str) -> Vec < NodeId >"
---

Who points to this node — components whose reference property
holds this id.

This is how an action change reaches
the buttons wired to it.

# Examples

```rust
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
assert!(doc.referrers("Action1").is_empty());
```
