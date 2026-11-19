---
id: props-props-want-tabs
kind: doc
target_file: src/props.rs
line_hint: 1270
author: 
created: 
title: "Props::want_tabs"
tags: []
signature: "pub fn want_tabs (default : bool) -> PropDesc"
---

Whether Tab inserts a tab character in text (`WantTabs`).

Tab types a tab character in the text (`WantTabs`); off, Tab is left
to the form.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::want_tabs(true);
assert_eq!(p.key, "want_tabs");
```
