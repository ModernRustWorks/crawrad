---
id: keys-key-by-name
kind: doc
target_file: src/keys.rs
line_hint: 160
author: 
created: 
title: "key_by_name"
tags: []
signature: "pub fn key_by_name (name : & str) -> Option < KeyCode >"
---

Translates a key name string to a Makepad KeyCode.

# Examples

```rust
use makepad_widgets::KeyCode;
use crawrad_runtime_makepad::keys::key_by_name;

assert_eq!(key_by_name("F1"), Some(KeyCode::F1));
assert_eq!(key_by_name("A"), Some(KeyCode::KeyA));
```
