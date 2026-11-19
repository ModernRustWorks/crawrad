---
id: link-pick-linkpick-labels
kind: doc
target_file: src/link_pick.rs
line_hint: 115
author: 
created: 
title: "LinkPick::labels"
tags: []
signature: "pub fn labels (& self) -> Vec < String >"
---

The rows the list shows.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;

let pick = LinkPick::default();
assert!(pick.labels().is_empty());
```
