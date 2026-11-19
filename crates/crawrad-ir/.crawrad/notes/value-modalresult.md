---
id: value-modalresult
kind: doc
target_file: src/value.rs
line_hint: 191
author: 
created: 
title: "ModalResult"
tags: []
signature: "pub enum ModalResult"
---

Modal dialog result in the `.crawform` document.

# Examples

```rust
use crawrad_ir::ModalResult;

assert_eq!(ModalResult::default(), ModalResult::None);
assert_eq!(ModalResult::Ok.as_str(), "ok");
```
