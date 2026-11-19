---
id: explorer-model-mark-default
kind: doc
target_file: src/explorer_model.rs
line_hint: 223
author: 
created: 
title: "MARK_DEFAULT"
tags: []
signature: "pub const MARK_DEFAULT : & str = \"default\""
---

Word for `default-members` in the workspace group.

The word from `default-members`, and it is `cargo`'s deliberately — whoever
wants to change the active project of the group edits that key, and the UI
uses its name to say where.

# Examples

```rust
use crawrad_project::explorer_model::MARK_DEFAULT;

assert_eq!(MARK_DEFAULT, "default");
```
