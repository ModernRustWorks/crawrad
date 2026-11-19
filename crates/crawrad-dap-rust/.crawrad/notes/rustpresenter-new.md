---
id: rustpresenter-new
kind: doc
target_file: src/lib.rs
line_hint: 46
author: 
created: 
title: "RustPresenter::new"
tags: []
signature: "pub fn new () -> Self"
---

A presenter that reads only the adapter's tree.

# Examples

```rust
use crawrad_dap_rust::RustPresenter;

let presenter = RustPresenter::new();
assert!(!presenter.has_layouts());
```
