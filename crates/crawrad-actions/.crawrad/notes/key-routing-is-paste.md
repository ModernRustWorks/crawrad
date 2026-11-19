---
id: key-routing-is-paste
kind: doc
target_file: src/key_routing.rs
line_hint: 118
author: 
created: 
title: "is_paste"
tags: []
signature: "pub fn is_paste (p : & Press) -> bool"
---

Whether the key fires the catalog's Paste action in the active scheme.

The chat uses it to paste a picture with the same chord that pastes text.

# Examples

```rust,ignore
if crawrad_actions::key_routing::is_paste(&press) {
    println!("paste");
}
```
