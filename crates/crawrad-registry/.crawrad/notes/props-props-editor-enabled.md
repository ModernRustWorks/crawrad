---
id: props-props-editor-enabled
kind: doc
target_file: src/props.rs
line_hint: 770
author: 
created: 
title: "Props::editor_enabled"
tags: []
signature: "pub fn editor_enabled (default : bool) -> PropDesc"
---

Whether a `SpinEdit`'s number can be typed directly.

Whether a `SpinEdit`'s number can be typed (`EditorEnabled`); off, only
the buttons change it.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::editor_enabled(true);
assert_eq!(p.key, "editor_enabled");
```
