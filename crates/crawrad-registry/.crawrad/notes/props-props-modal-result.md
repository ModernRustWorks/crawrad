---
id: props-props-modal-result
kind: doc
target_file: src/props.rs
line_hint: 565
author: 
created: 
title: "Props::modal_result"
tags: []
signature: "pub fn modal_result (default : crawrad_ir :: ModalResult) -> PropDesc"
---

Modal dialog result (`ModalResult`).

# Examples

```rust
use crawrad_ir::ModalResult;
use crawrad_registry::Props;

let p = Props::modal_result(ModalResult::None);
assert_eq!(p.key, "modal_result");
```
