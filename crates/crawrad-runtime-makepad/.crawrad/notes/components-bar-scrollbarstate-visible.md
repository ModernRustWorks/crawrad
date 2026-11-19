---
id: components-bar-scrollbarstate-visible
kind: doc
target_file: src/components/bar.rs
line_hint: 155
author: 
created: 
title: "ScrollBarState::visible"
tags: []
signature: "pub fn visible (self) -> bool"
---

Returns true if anything is painted for this bar.

Is anything painted for this bar?

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;

assert!(ScrollBarState::Active.visible());
assert!(!ScrollBarState::Hidden.visible());
```
