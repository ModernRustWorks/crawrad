---
id: creation-outcome
kind: doc
target_file: src/creation.rs
line_hint: 129
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What creation produced, and what the dialog does with it.

A type, and not an `if` inside `Ide::harvest_creation`, because **here is where
the bug was** and an `if` in there could only be measured by launching the IDE — which
is how it survived: the suite had 794 green tests and the created project would
not open.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::Outcome;

let out = Outcome::Opened(PathBuf::from("."));
assert!(out.closes_automatically());
```
