---
id: desktop-desktop-encode
kind: doc
target_file: src/desktop.rs
line_hint: 97
author: 
created: 
title: "Desktop::encode"
tags: []
signature: "pub fn encode (& self) -> String"
---

The text of the file.

# Examples

```rust
use crawrad_project::desktop::Desktop;

let text = Desktop::default().encode();
assert!(text.starts_with("crawrad-desktop 1"));
```
