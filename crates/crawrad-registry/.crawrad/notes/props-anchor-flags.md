---
id: props-anchor-flags
kind: doc
target_file: src/props.rs
line_hint: 69
author: 
created: 
title: "ANCHOR_FLAGS"
tags: []
signature: "pub const ANCHOR_FLAGS : & [& str] = & [\"left\" , \"top\" , \"right\" , \"bottom\"]"
---

The four flags of `Anchors`, in classic RAD IDE order.

**The four flags of `Anchors`**, in classic RAD IDE order.

They are the payload of [`PropKind::Set`]: it is the piece's label that says which
boxes the editor draws, not a list written inside the editor.

# Examples

```rust
use crawrad_registry::ANCHOR_FLAGS;

assert!(ANCHOR_FLAGS.contains(&"left"));
```
