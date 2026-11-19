---
id: inspector-model-editors-integer-integer-parse
kind: doc
target_file: src/inspector_model/editors/integer.rs
line_hint: 36
author: 
created: 
title: "Integer::parse"
tags: []
signature: "pub fn parse (text : & str , min : i64 , max : i64) -> Result < i64 , String >"
---

The value a typed text becomes, or the sentence that refuses it.

Public because the four quarters of a [`super::rect`] are whole numbers
under another name, and two copies of "what is a number" would be two
copies to disagree.

# Errors

Returns an error string if the text is not a valid whole number.

# Examples

```rust
use crawrad_actions::inspector_model::editors::integer::Integer;

assert_eq!(Integer::parse("42", 0, 100), Ok(42));
assert!(Integer::parse("not_a_number", 0, 100).is_err());
```
