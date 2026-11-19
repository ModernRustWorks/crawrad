---
id: explorer-model-filestate
kind: doc
target_file: src/explorer_model.rs
line_hint: 30
author: 
created: 
title: "FileState"
tags: []
signature: "pub enum FileState"
---

What crossing the plan with the disk discovered about a file.

The states from the ADR 0016 table. An `enum` and not a pair of `bool`s:
`(planned, exists)` has four combinations and one of them — neither planned
nor existing — means nothing.

# Examples

```rust
use crawrad_project::explorer_model::FileState;

assert_eq!(FileState::OnDisk, FileState::OnDisk);
```
