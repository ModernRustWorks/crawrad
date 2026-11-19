---
id: class-property-check
kind: doc
target_file: src/class_property.rs
line_hint: 93
author: 
created: 
title: "check"
tags: []
signature: "pub fn check (doc : & Document , reg : & Registry) -> Result < () , ClassError >"
---

Checks each document property and event against its class.

**Checks each document property and event against the class that carries it.**

See the header for the rule and why it lives here.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_registry::Registry;
use crawrad_registry::class_property::check;

let doc = Document::new("form", "Form");
let reg = Registry::builtin();
assert!(check(&doc, &reg).is_ok());
```

# Errors

Returns a [`ClassError`] if a node uses an unknown class, undeclared property,
or undeclared event.
