---
id: keys-arrow-direction
kind: doc
target_file: src/keys.rs
line_hint: 223
author: 
created: 
title: "arrow_direction"
tags: []
signature: "pub fn arrow_direction (key : KeyCode) -> Option < (f32 , f32) >"
---

Returns the direction sign (dx, dy) for an arrow key.

# Examples

```rust
use makepad_widgets::KeyCode;
use crawrad_runtime_makepad::keys::arrow_direction;

assert_eq!(arrow_direction(KeyCode::ArrowLeft), Some((-1.0, 0.0)));
assert_eq!(arrow_direction(KeyCode::ReturnKey), None);
```
