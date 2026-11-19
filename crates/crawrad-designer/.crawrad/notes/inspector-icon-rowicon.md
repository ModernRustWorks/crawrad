---
id: inspector-icon-rowicon
kind: doc
target_file: src/inspector_icon.rs
line_hint: 22
author: 
created: 
title: "RowIcon"
tags: []
signature: "pub type RowIcon = & 'static str"
---

One drawing of the Codicons font — the character, and nothing else.

# Examples

```rust
use crawrad_designer::inspector_icon::{RowIcon, FOLD_CLOSED};

let icon: RowIcon = FOLD_CLOSED;
assert!(!icon.is_empty());
```
