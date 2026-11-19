---
id: props-props-password-char
kind: doc
target_file: src/props.rs
line_hint: 1167
author: 
created: 
title: "Props::password_char"
tags: []
signature: "pub fn password_char (default : & str) -> PropDesc"
---

Character used to mask passwords (`PasswordChar`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::password_char("*");
assert_eq!(p.key, "password_char");
```
