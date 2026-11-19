---
id: rename-owner
kind: doc
target_file: src/rename.rs
line_hint: 23
author: 
created: 
title: "Owner"
tags: []
signature: "pub enum Owner"
---

Who owns a name in the form.

# Examples

```rust
use crawrad_host::rename::Owner;

let owner = Owner::Component { form: "MainForm".into(), name: "btn_ok".into() };
assert!(owner.refusal().contains("btn_ok"));
```
