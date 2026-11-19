---
id: document-resolve-inheritance
kind: doc
target_file: src/lib.rs
line_hint: 213
author: 
created: 
title: "Document::resolve_inheritance"
tags: []
signature: "pub fn resolve_inheritance (& self , base : & Document) -> Result < Document , IrError >"
---

Resolves the visual inheritance of this form from the ancestor form `base`.

Produces a new merged [`Document`] containing all ancestor nodes,
with overwritten properties and new integrated derived nodes.

# Examples

```rust
use crawrad_ir::{Document, Node};

let mut base = Document::new("BaseForm", "Form");
base.add_child("BaseForm", Node::new("btn", "Button")).unwrap();
let derived = Document::new_inherited("DerivedForm", "Form", "BaseForm");
let resolved = derived.resolve_inheritance(&base).unwrap();
assert!(resolved.node("btn").is_some());
```

# Errors

Returns [`IrError::SelfInheritance`] if the form inherits from itself,
[`IrError::InheritanceCycle`] if an inheritance cycle is detected, or
[`IrError`] if document validation fails.
