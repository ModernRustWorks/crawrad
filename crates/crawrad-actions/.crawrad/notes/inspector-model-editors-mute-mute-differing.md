---
id: inspector-model-editors-mute-mute-differing
kind: doc
target_file: src/inspector_model/editors/mute.rs
line_hint: 63
author: 
created: 
title: "Mute::differing"
tags: []
signature: "pub fn differing () -> Editor"
---

The row of a value several selected components do not share.

# Examples

```rust
use crawrad_actions::inspector_model::editors::mute::Mute;

let editor = Mute::differing();
assert_eq!(editor.template(), "RowMute");
```
