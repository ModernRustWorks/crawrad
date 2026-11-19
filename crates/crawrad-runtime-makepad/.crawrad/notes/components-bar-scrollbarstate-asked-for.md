---
id: components-bar-scrollbarstate-asked-for
kind: doc
target_file: src/components/bar.rs
line_hint: 150
author: 
created: 
title: "ScrollBarState::asked_for"
tags: []
signature: "pub fn asked_for (needed : bool) -> Self"
---

Returns active or disabled for a manually configured bar.

[`Self::Active`] or [`Self::Disabled`] — the regime of a bar the
developer turned on by hand, which never disappears.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;

assert_eq!(ScrollBarState::asked_for(true), ScrollBarState::Active);
assert_eq!(ScrollBarState::asked_for(false), ScrollBarState::Disabled);
```
