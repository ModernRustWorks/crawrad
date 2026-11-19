---
id: props-props-design-rows
kind: doc
target_file: src/props.rs
line_hint: 1871
author: 
created: 
title: "Props::design_rows"
tags: []
signature: "pub fn design_rows() -> PropDesc"
---
How many real rows the Designer shows in a bound grid (#925).

The generated app never reads it: it reads its own data at run time.

# Examples

```rust
use crawrad_registry::Props;

assert_eq!(Props::design_rows().key, "design_rows");
```
