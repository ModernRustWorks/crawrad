---
id: keys-rewrite
kind: doc
target_file: src/keys.rs
line_hint: 55
author: 
created: 
title: "rewrite"
tags: []
signature: "pub fn rewrite (key : & KeyEvent , focus : Focus) -> Option < KeyEvent >"
---

Rewrites a Return key event for the given text box focus type.

The rule itself, with no window: the Enter `focus` should receive in place
of `key`, or `None` when `key` already does the right thing.

# Examples

```rust
use makepad_widgets::{KeyCode, KeyEvent, KeyModifiers};
use crawrad_runtime_makepad::keys::{rewrite, Focus};

let key = KeyEvent {
    key_code: KeyCode::ReturnKey,
    is_repeat: false,
    modifiers: KeyModifiers::default(),
    time: 0.0,
};
let rewritten = rewrite(&key, Focus::Memo);
assert!(rewritten.is_some());
```
