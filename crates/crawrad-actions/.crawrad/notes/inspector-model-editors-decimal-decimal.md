---
id: inspector-model-editors-decimal-decimal
kind: doc
target_file: src/inspector_model/editors/decimal.rs
line_hint: 17
author: 
created: 
title: "Decimal"
tags: []
signature: "pub struct Decimal"
---

A number in a text field.

# Examples

```rust
use crawrad_actions::inspector_model::editors::decimal::Decimal;
use crawrad_actions::inspector_model::editor::PropEditor;

let dec = Decimal::new(1.5, 0.0, 10.0);
assert_eq!(dec.template(), "RowText");
```
