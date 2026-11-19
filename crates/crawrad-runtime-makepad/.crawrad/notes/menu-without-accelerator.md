---
id: menu-without-accelerator
kind: doc
target_file: src/menu.rs
line_hint: 38
author: 
created: 
title: "without_accelerator"
tags: []
signature: "pub fn without_accelerator (text : & str) -> String"
---

Strips accelerator ampersands from a menu caption.

The caption without the classic RAD accelerator mark: `&File` is `File`,
and `&&` stands for one `&`.

# Examples

```rust
use crawrad_runtime_makepad::menu::without_accelerator;

assert_eq!(without_accelerator("&Open..."), "Open...");
assert_eq!(without_accelerator("A && B"), "A & B");
```
