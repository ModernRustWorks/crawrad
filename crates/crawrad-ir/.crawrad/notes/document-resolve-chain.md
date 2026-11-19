---
id: document-resolve-chain
kind: doc
target_file: src/lib.rs
line_hint: 270
author:
created:
title: "Document::resolve_chain"
tags: []
signature: "pub fn resolve_chain (bases : & [Document]) -> Option < Document >"
---

Folds a chain of base forms, nearest first, into one base.

It is the one form a derived form inherits from: the outermost base is resolved into the next one down, and so
on. `None` for an empty chain.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut a = Document::new("form_a", "Form");
a.add_child("form_a", Node::new("btn_a", "Button")).unwrap();
let mut b = Document::new_inherited("form_b", "Form", "form_a");
b.add_child("form_b", Node::new("btn_b", "Button")).unwrap();
// form_c inherits form_b: the chain is [form_b, form_a]
let base = Document::resolve_chain(&[b, a]).unwrap();
assert!(base.node("btn_a").is_some() && base.node("btn_b").is_some());
```
