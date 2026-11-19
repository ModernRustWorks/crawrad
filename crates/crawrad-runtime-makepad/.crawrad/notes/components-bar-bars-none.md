---
id: components-bar-bars-none
kind: doc
target_file: src/components/bar.rs
line_hint: 218
author: 
created: 
title: "Bars::NONE"
tags: []
signature: "pub const NONE : Bars = Bars"
---

None at all.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::{Bars, ScrollBarState};

assert_eq!(Bars::NONE.vertical, ScrollBarState::Hidden);
assert_eq!(Bars::NONE.horizontal, ScrollBarState::Hidden);
```
