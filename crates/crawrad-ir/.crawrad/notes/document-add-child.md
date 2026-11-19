---
id: document-add-child
kind: doc
target_file: src/lib.rs
line_hint: 351
author: 
created: 
title: "Document::add_child"
tags: []
signature: "pub fn add_child (& mut self , parent : & str , node : Node) -> Result < () , IrError >"
---

Inserts `node` as a child of `parent`, at the end of the order.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut doc = Document::new("MainForm", "Form");
let child = Node::new("btn", "Button");
assert!(doc.add_child("MainForm", child).is_ok());
```

# Errors

Returns [`IrError::DuplicateId`] if a node with `node.id` already exists,
or [`IrError::MissingNode`] if `parent` does not exist in the document.

# Panics

Panics if the `parent` node cannot be found immediately after verifying its existence.
