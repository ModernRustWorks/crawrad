---
id: designer-current-gesture
kind: doc
target_file: src/lib.rs
line_hint: 358
author: 
created: 
title: "Designer::current_gesture"
tags: []
signature: "pub fn current_gesture (& self) -> u64"
---

The number of the current gesture, to stamp its transactions.

All transactions of a drag carry the same, and that is what makes
`Ctrl+Z` undo **the gesture** and not its last frame.

# Examples

```rust
use crawrad_designer::Designer;

let d = Designer::default();
assert_eq!(d.current_gesture(), 0);
```
