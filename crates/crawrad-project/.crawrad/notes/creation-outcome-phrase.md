---
id: creation-outcome-phrase
kind: doc
target_file: src/creation.rs
line_hint: 188
author: 
created: 
title: "Outcome::phrase"
tags: []
signature: "pub fn phrase (& self) -> Option < & str >"
---

The phrase to read, when there is one.

Exists because it goes to TWO places — the dialog and the status bar —,
and whoever clicked `Hide` only has the bar. An `if let Outcome::Stay` per
caller is the pair that diverges on the day a third arm appears — and
the third arm did appear: `OpenedWithWarning` debuted without reaching the status bar.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::Outcome;

let out = Outcome::Opened(PathBuf::from("."));
assert_eq!(out.phrase(), None);
```
