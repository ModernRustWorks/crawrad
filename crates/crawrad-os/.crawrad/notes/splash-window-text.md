---
id: splash-window-text
kind: doc
target_file: src/splash_window.rs
line_hint: 23
author: 
created: 
title: "Text"
tags: []
signature: "pub struct Text"
---

One line of text, vertically centred in `area`.

# Examples

```rust
use crawrad_os::splash_window::Text;

let text = Text {
    text: "Loading...".to_string(),
    area: [0.0, 0.0, 100.0, 20.0],
    color: 0x00FFFFFF,
    size: 12.0,
    weight: 400,
    align_right: false,
};
assert_eq!(text.text, "Loading...");
```
