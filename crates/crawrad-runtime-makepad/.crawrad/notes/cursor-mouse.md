---
id: cursor-mouse
kind: doc
target_file: src/cursor.rs
line_hint: 12
author: 
created: 
title: "mouse"
tags: []
signature: "pub fn mouse (name : & str) -> Option < MouseCursor >"
---

Returns Makepad's mouse cursor for a given cursor name.

Makepad's pointer for a `Cursor` named `name`; `None` for `default` and a
name this does not know.

# Examples

```rust
use makepad_widgets::MouseCursor;
use crawrad_runtime_makepad::cursor::mouse;

assert_eq!(mouse("hand"), Some(MouseCursor::Hand));
assert_eq!(mouse("unknown"), None);
```
