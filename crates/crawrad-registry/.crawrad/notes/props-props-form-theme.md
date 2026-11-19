---
id: props-props-form-theme
kind: doc
target_file: src/props.rs
line_hint: 1202
author: 
created: 
title: "Props::form_theme"
tags: []
signature: "pub fn form_theme (default : & 'static str) -> PropDesc"
---

The application's theme (`Theme`): system default, light, or dark.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::form_theme("thSystem");
assert_eq!(p.key, "theme");
```
