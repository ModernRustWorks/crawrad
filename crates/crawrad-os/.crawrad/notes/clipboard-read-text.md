---
id: clipboard-read-text
kind: doc
target_file: src/clipboard.rs
line_hint: 9
author: 
created: 
title: "read_text"
tags: []
signature: "pub fn read_text () -> Option < String >"
---

Reads the clipboard's text, or returns `None`.

The clipboard's text, or `None` when it holds no text (empty, a picture,
files) or cannot be opened.

# Examples

```rust
use crawrad_os::clipboard::{read_text, write_text};

if write_text("CrawRAD") {
    assert_eq!(read_text().as_deref(), Some("CrawRAD"));
}
```
