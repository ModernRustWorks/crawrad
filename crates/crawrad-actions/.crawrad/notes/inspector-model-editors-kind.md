---
id: inspector-model-editors-kind
kind: doc
target_file: src/inspector_model/editors.rs
line_hint: 125
author: 
created: 
title: "Kind"
tags: []
signature: "pub struct Kind"
---

One kind of editor, as this folder publishes it.

Two function pointers and not a trait with associated functions, because
the thing being listed is the **file**, not a type: `edges` produces two
different structs (the folding row and the side under it) and `mute`
produces one that three different callers build directly.

# Examples

```rust
use crawrad_actions::inspector_model::editors::{Kind, KINDS};

let kind: &Kind = &KINDS[0];
let sample = (kind.sample)();
assert!(!sample.template().is_empty());
```
