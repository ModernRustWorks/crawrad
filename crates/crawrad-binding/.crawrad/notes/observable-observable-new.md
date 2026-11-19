---
id: observable-observable-new
kind: doc
target_file: src/observable.rs
line_hint: 57
author: 
created: 
title: "Observable::new"
tags: []
signature: "pub fn new (value : T) -> Self"
---

Create a new observable initialized with `value`.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new("value");
assert_eq!(obs.get(), "value");
```
