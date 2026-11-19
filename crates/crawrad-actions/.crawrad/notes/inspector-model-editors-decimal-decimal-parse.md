---
id: inspector-model-editors-decimal-decimal-parse
kind: doc
target_file: src/inspector_model/editors/decimal.rs
line_hint: 30
author: 
created: 
title: "Decimal::parse"
tags: []
signature: "pub fn parse (text : & str , min : f64 , max : f64) -> Result < f64 , String >"
---

The value a typed text becomes, or the sentence that refuses it.

Public for the reason [`super::integer::Integer::parse`] gives: the
quarters of a `Rect` and the sides of a `Margins` are numbers under
another name.

# Errors

Returns an error string if the text is not a valid finite float.

# Examples

```rust
use crawrad_actions::inspector_model::editors::decimal::Decimal;

assert_eq!(Decimal::parse("3.5", 0.0, 10.0), Ok(3.5));
assert!(Decimal::parse("abc", 0.0, 10.0).is_err());
```
