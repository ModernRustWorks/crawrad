---
id: inspector-model-bound-to-dataset
kind: doc
target_file: src/inspector_model.rs
line_hint: 152
author: 
created: 
title: "BOUND_TO_DATASET"
tags: []
signature: "pub const BOUND_TO_DATASET : & str = \"bound\""
---

The reason a bound property row is read-only (#788 slice 3a).

ONE word, like `read-only`: the note is drawn at its own width and the value
box takes what is left, so a sentence here pushed `from dst1.name` off the
row — seen in the IDE, not in a test. Where to change it is the Data
category right below.

# Examples

```rust
use crawrad_actions::inspector_model::BOUND_TO_DATASET;

assert_eq!(BOUND_TO_DATASET, "bound");
```
