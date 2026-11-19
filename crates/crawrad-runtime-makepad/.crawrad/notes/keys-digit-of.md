---
id: keys-digit-of
kind: doc
target_file: src/keys.rs
line_hint: 234
author: 
created: 
title: "digit_of"
tags: []
signature: "pub fn digit_of (key : KeyCode) -> Option < u8 >"
---

Extracts the bookmark digit 1–9 from a key, if applicable.

# Examples

```rust
use makepad_widgets::KeyCode;
use crawrad_runtime_makepad::keys::digit_of;

assert_eq!(digit_of(KeyCode::Key1), Some(1));
assert_eq!(digit_of(KeyCode::Key0), None);
```
