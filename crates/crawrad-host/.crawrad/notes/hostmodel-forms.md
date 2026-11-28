---
id: hostmodel-forms
kind: doc
target_file: src/lib.rs
line_hint: 255
author: 
created: 
title: "HostModel::forms"
tags: []
signature: "pub fn forms (& self) -> & [OpenForm]"
---

The forms open in the strip, in the order their tabs are drawn.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.forms().is_empty(), "nothing is open before anything is opened");
```
