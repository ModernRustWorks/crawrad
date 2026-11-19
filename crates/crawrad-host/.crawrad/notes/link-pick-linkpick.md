---
id: link-pick-linkpick
kind: doc
target_file: src/link_pick.rs
line_hint: 56
author: 
created: 
title: "LinkPick"
tags: []
signature: "pub struct LinkPick"
---

The list while it is up: the note's caret, the question and the items.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;

let pick = LinkPick::default();
assert!(!pick.is_open());
```
