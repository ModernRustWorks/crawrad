---
id: font-fontchoice
kind: doc
target_file: src/font.rs
line_hint: 33
author: 
created: 
title: "FontChoice"
tags: []
signature: "pub struct FontChoice"
---

A standalone typography choice decoupled from document IR structures.

# Examples

```rust
use crawrad_os::font::FontChoice;

let font = FontChoice::default();
assert_eq!(font.name, "Segoe UI");
```
