---
id: controller-update
kind: doc
target_file: src/controller.rs
line_hint: 83
author: 
created: 
title: "Update"
tags: []
signature: "pub enum Update"
---

What the session tells the IDE.

# Examples

```rust
use crawrad_dap::Update;

let update = Update::Running;
assert_eq!(update, Update::Running);
```
