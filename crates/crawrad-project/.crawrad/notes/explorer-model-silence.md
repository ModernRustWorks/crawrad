---
id: explorer-model-silence
kind: doc
target_file: src/explorer_model.rs
line_hint: 235
author: 
created: 
title: "Silence"
tags: []
signature: "pub enum Silence"
---

Why a member does **not** show files with badges.

# Examples

```rust
use crawrad_project::explorer_model::Silence;

let s = Silence::Phrase("no form");
assert_eq!(s, Silence::Phrase("no form"));
```
