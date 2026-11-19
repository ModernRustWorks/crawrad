---
id: inspector-model-editors-integer-integer
kind: doc
target_file: src/inspector_model/editors/integer.rs
line_hint: 23
author: 
created: 
title: "Integer"
tags: []
signature: "pub struct Integer"
---

A whole number in a text field.

# Examples

```rust
use crawrad_actions::inspector_model::editors::integer::Integer;
use crawrad_actions::inspector_model::editor::PropEditor;

let int = Integer::new(5, 0, 10);
assert_eq!(int.template(), "RowText");
```
