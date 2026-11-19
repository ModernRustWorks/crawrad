---
id: contract-propkey
kind: doc
target_file: src/contract.rs
line_hint: 157
author: 
created: 
title: "PropKey"
tags: []
signature: "pub enum PropKey"
---

A classified property key.

# Examples

```rust
use crawrad_ir::PropKey;

let key = PropKey::parse("text").unwrap();
assert_eq!(key, PropKey::Common("text".to_string()));
```
