---
id: props-props-want-returns
kind: doc
target_file: src/props.rs
line_hint: 1259
author: 
created: 
title: "Props::want_returns"
tags: []
signature: "pub fn want_returns (default : bool) -> PropDesc"
---

Whether Enter starts a new line in multiline text (`WantReturns`).

Enter starts a new line in the text (`WantReturns`); off, Enter is
left to the form.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::want_returns(true);
assert_eq!(p.key, "want_returns");
```
