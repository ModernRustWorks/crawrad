---
id: views-installed
kind: doc
target_file: src/views.rs
line_hint: 101
author: 
created: 
title: "Installed"
tags: []
signature: "pub struct Installed"
---

An installed extension, as the panel's last section lists it.

# Examples

```rust
use crawrad_extension_host::views::Installed;

let inst = Installed { id: "ext1".into(), name: "Ext".into(), version: "0.1".into() };
assert_eq!(inst.id, "ext1");
```
