---
id: rename-owner-refusal
kind: doc
target_file: src/rename.rs
line_hint: 37
author: 
created: 
title: "Owner::refusal"
tags: []
signature: "pub fn refusal (& self) -> String"
---

Why the rename is refused, and the road that renames it right.

# Examples

```rust
use crawrad_host::rename::Owner;

let owner = Owner::Component { form: "MainForm".into(), name: "btn".into() };
assert!(owner.refusal().contains("Object Inspector"));
```
