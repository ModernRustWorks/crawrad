---
id: components-bar-scrollbarstate-when-needed
kind: doc
target_file: src/components/bar.rs
line_hint: 145
author: 
created: 
title: "ScrollBarState::when_needed"
tags: []
signature: "pub fn when_needed (needed : bool) -> Self"
---

Returns active if needed or hidden in the automatic regime.

[`Self::Active`] or [`Self::Hidden`] — the automatic regime, where a bar
that has nothing to say does not appear.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;

assert_eq!(ScrollBarState::when_needed(true), ScrollBarState::Active);
assert_eq!(ScrollBarState::when_needed(false), ScrollBarState::Hidden);
```
