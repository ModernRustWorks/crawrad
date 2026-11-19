---
id: desktop-desktop-decode
kind: doc
target_file: src/desktop.rs
line_hint: 114
author: 
created: 
title: "Desktop::decode"
tags: []
signature: "pub fn decode (text : & str) -> Option < Self >"
---

Back from the text; `None` when the header is not this format's.

# Examples

```rust
use crawrad_project::desktop::Desktop;

let d = Desktop::decode("crawrad-desktop 1\ndock\n");
assert!(d.is_some());
```
