---
id: clipboard-clipboard
kind: doc
target_file: src/clipboard.rs
line_hint: 12
author: 
created: 
title: "Clipboard"
tags: []
signature: "pub struct Clipboard"
---

What is on the clipboard.

What is on the clipboard. Empty when nobody has copied anything.

# Examples

```rust
use crawrad_designer::clipboard::Clipboard;

let cb = Clipboard::default();
assert!(cb.is_empty());
```
