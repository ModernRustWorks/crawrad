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

// `Ctrl+` names the primary key: `Cmd` (logo) on Apple, `Ctrl` elsewhere.
let apple = cfg!(target_vendor = "apple");
let m = KeyModifiers { control: !apple, logo: apple, ..Default::default() };
assert!(matches_shortcut("Ctrl+S", KeyCode::KeyS, &m));
```
