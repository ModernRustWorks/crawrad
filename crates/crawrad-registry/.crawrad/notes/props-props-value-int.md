---
id: props-props-value-int
kind: doc
target_file: src/props.rs
line_hint: 1357
author: 
created: 
title: "Props::value_int"
tags: []
signature: "pub fn value_int (key : & 'static str , display : & 'static str , default : i64 , min : i64 , max : i64 ,) -> PropDesc"
---

Current position / value in scroll bars and spins (`Position` / `Value`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::value_int("pos", "Position", 0, 0, 100);
assert_eq!(p.key, "pos");
```
