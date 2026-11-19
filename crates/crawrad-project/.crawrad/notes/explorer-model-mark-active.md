---
id: explorer-model-mark-active
kind: doc
target_file: src/explorer_model.rs
line_hint: 220
author: 
created: 
title: "MARK_ACTIVE"
tags: []
signature: "pub const MARK_ACTIVE : & str = \"active\""
---

The word the IDE writes on screen for the active document member.

`active` and not `default`: what `Generate` writes and what `Run` compiles come
from the **open document**, and that is what the word promises.

# Examples

```rust
use crawrad_project::explorer_model::MARK_ACTIVE;

assert_eq!(MARK_ACTIVE, "active");
```
