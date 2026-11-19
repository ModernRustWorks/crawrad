---
id: clipboard-clipboard-from-text
kind: doc
target_file: src/clipboard.rs
line_hint: 30
author: 
created: 
title: "Clipboard::from_text"
tags: []
signature: "pub fn from_text (txt : & str) -> Option < Self >"
---

Reads components from serialized text.

Reads text from the clipboard. Returns `None` when it is not ours
— arbitrary text copied from elsewhere does not become a component.

# Examples

```rust
use crawrad_designer::clipboard::Clipboard;

assert!(Clipboard::from_text("arbitrary text").is_none());
```
