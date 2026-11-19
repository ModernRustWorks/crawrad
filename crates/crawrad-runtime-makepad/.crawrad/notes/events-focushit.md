---
id: events-focushit
kind: doc
target_file: src/events.rs
line_hint: 243
author: 
created: 
title: "FocusHit"
tags: []
signature: "pub struct FocusHit"
---

One focus or keyboard event for one control.

# Examples

```rust
use makepad_widgets::KeyModifiers;
use crawrad_runtime_makepad::events::{Focus, FocusHit};

let hit = FocusHit {
    control: 0,
    kind: Focus::Enter,
    key: None,
    ch: None,
    modifiers: KeyModifiers::default(),
};
assert_eq!(hit.control, 0);
```
