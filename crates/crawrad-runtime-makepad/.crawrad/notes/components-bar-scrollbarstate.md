---
id: components-bar-scrollbarstate
kind: doc
target_file: src/components/bar.rs
line_hint: 131
author: 
created: 
title: "ScrollBarState"
tags: []
signature: "pub enum ScrollBarState"
---

How a scroll bar is drawn — **the issue #122, in one type**.

The rule is the owner's, measured on a classic RAD IDE with a print of each case, and
it splits by **who decides that the bar exists**:

| control | who decides | when the content FITS |
|---|---|---|
| `Memo` | the developer, through `ScrollBars` | [`Self::Disabled`] — grey arrows, empty track, **no thumb** |
| `ListBox`, `TreeView`, `ScrollBox` | automatic | [`Self::Hidden`] |

*"Instead of disabling, since the developer is the one who enables it, it deactivates."* A
bar the developer asked for does not vanish behind his back; it stays where
he put it and says, by being grey, that there is nothing to scroll.

And the house rule that decides how [`Self::Disabled`] must LOOK: a dead
bar may not look clickable. Drawing the thumb on a bar that scrolls nothing
is the same sin as the button that lights up and does nothing.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::ScrollBarState;

assert_eq!(ScrollBarState::default(), ScrollBarState::Hidden);
```
