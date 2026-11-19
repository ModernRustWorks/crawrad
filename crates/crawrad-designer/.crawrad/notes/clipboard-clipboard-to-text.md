---
id: clipboard-clipboard-to-text
kind: doc
target_file: src/clipboard.rs
line_hint: 25
author: 
created: 
title: "Clipboard::to_text"
tags: []
signature: "pub fn to_text (& self) -> String"
---

Serializes content to text for the system clipboard.

Serializes content to text — this is what goes to the system
clipboard, and is what allows pasting between two CrawRAD windows
(or pasting something written by hand).

# Examples

```rust
use crawrad_designer::clipboard::Clipboard;

let cb = Clipboard::default();
assert!(cb.to_text().contains("crawrad.components"));
```
