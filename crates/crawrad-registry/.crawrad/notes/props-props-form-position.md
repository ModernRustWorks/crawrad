---
id: props-props-form-position
kind: doc
target_file: src/props.rs
line_hint: 1178
author: 
created: 
title: "Props::form_position"
tags: []
signature: "pub fn form_position (default : & 'static str) -> PropDesc"
---

Where the window opens on screen (`Position`).

Read-only mode (`ReadOnly`).
Where the window opens (`Position`): where it was designed, or in the
middle of the screen.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::form_position("poDesigned");
assert_eq!(p.key, "position");
```
