---
id: dialogs-pendinganswer-take
kind: doc
target_file: src/dialogs.rs
line_hint: 181
author: 
created: 
title: "PendingAnswer::take"
tags: []
signature: "pub fn take (& self) -> Option < ModalResult >"
---

The answer, once: `Some` the first time after the user answered, then `None`.

# Examples

```rust
use crawrad_runtime::dialogs::PendingAnswer;

let pending = PendingAnswer::default();
assert_eq!(pending.take(), None);
```
