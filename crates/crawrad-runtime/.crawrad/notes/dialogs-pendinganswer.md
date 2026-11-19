---
id: dialogs-pendinganswer
kind: doc
target_file: src/dialogs.rs
line_hint: 155
author: 
created: 
title: "PendingAnswer"
tags: []
signature: "pub struct PendingAnswer (std :: sync :: Arc < std :: sync :: Mutex < Option < ModalResult > > >)"
---

The answer to a dialog opened by a component, read when it arrives.

# Examples

```rust
use crawrad_runtime::dialogs::PendingAnswer;

let pending = PendingAnswer::default();
assert_eq!(pending.take(), None);
```
