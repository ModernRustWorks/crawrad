---
id: components-bar-scrollbarstate-live
kind: doc
target_file: src/components/bar.rs
line_hint: 160
author: 
created: 
title: "ScrollBarState::live"
tags: []
signature: "pub fn live (self) -> bool"
---

Returns true if there is content to scroll and grab.

Is there anything to scroll — and therefore anything to grab?

**This is the honesty rule of the whole issue, and it is why it lives
on the type instead of inside the painter**: a bar that shows the
sliding block promises a drag. On a bar that scrolls nothing, that
promise is a lie, and it is the same sin as the button that lights up
and does nothing. What the owner's classic RAD print shows for a `Memo` with
`ScrollBars` and content that fits is exactly this: grey arrows, empty
track, nothing to take hold of.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;

assert!(ScrollBarState::Active.live());
assert!(!ScrollBarState::Disabled.live());
```
