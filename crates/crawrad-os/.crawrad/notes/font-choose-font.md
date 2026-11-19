---
id: font-choose-font
kind: doc
target_file: src/font.rs
line_hint: 140
author: 
created: 
title: "choose_font"
tags: []
signature: "pub fn choose_font (initial : & FontChoice) -> Option < FontChoice >"
---

Opens the native font dialog initialized with `initial`.

Opens the native font dialog initialized with `initial`: `None` when it is
cancelled, and always `None` where there is no native dialog (see the module).

# Examples

```rust,no_run
// Opens an interactive system modal dialog that requires user input.
use crawrad_os::font::{choose_font, FontChoice};

let choice = choose_font(&FontChoice::default());
assert!(choice.is_none() || choice.is_some());
```
