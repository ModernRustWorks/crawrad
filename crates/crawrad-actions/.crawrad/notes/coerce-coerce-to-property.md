---
id: coerce-coerce-to-property
kind: doc
target_file: src/coerce.rs
line_hint: 10
author: 
created: 
title: "coerce_to_property"
tags: []
signature: "pub fn coerce_to_property (value : & Value , desc : & PropDesc , ir : & crawrad_ir :: Document , registry : & Registry ,) -> Result < Value , String >"
---

Coerces an incoming dynamic [`Value`] into the shape declared by [`PropDesc`].

# Errors

Returns an error string when the value cannot be coerced or violates constraints.

# Examples

```rust
use crawrad_actions::coerce_to_property;
use crawrad_ir::{Document, Value};
use crawrad_registry::{Category, PropDesc, Registry};

let desc = PropDesc::new("caption", "Caption", Value::Str(String::new()), Category::Identity);
let doc = Document::new("form", "Form");
let reg = Registry::builtin();
let val = Value::Str("Hello".to_string());
let coerced = coerce_to_property(&val, &desc, &doc, &reg).unwrap();
assert_eq!(coerced, Value::Str("Hello".to_string()));
```
