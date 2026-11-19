---
id: inspector-model-editors-decimal-decimal-show
kind: doc
target_file: src/inspector_model/editors/decimal.rs
line_hint: 38
author: 
created: 
title: "Decimal::show"
tags: []
signature: "pub fn show (value : f64) -> String"
---

How a number is written into a field the user types over.

The shortest text that represents it — `3`, not `3.00` — and never
`-0`, which is a float fact and not a number anybody means.

# Examples

```rust
use crawrad_actions::inspector_model::editors::decimal::Decimal;

assert_eq!(Decimal::show(3.0), "3");
assert_eq!(Decimal::show(-0.0), "0");
```
