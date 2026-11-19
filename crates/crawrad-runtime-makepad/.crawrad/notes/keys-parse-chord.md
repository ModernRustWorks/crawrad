---
id: keys-parse-chord
kind: doc
target_file: src/keys.rs
line_hint: 199
author: 
created: 
title: "parse_chord"
tags: []
signature: "pub fn parse_chord (chord : & str) -> Option < (KeyCode , Chord) >"
---

Decomposes a chord string into a key and requested modifiers.

# Examples

```rust
use makepad_widgets::KeyCode;
use crawrad_runtime_makepad::keys::parse_chord;

let (key, chord) = parse_chord("Ctrl+S").unwrap();
assert_eq!(key, KeyCode::KeyS);
assert!(chord.primary);
```
