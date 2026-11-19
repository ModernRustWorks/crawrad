---
id: keys-matches-shortcut
kind: doc
target_file: src/keys.rs
line_hint: 215
author: 
created: 
title: "matches_shortcut"
tags: []
signature: "pub fn matches_shortcut (chord : & str , key : KeyCode , m : & KeyModifiers) -> bool"
---

Checks if a chord string matches the given key and modifiers.

# Examples

```rust
use makepad_widgets::{KeyCode, KeyModifiers};
use crawrad_runtime_makepad::keys::matches_shortcut;

let m = KeyModifiers { control: true, ..Default::default() };
assert!(matches_shortcut("Ctrl+S", KeyCode::KeyS, &m));
```
