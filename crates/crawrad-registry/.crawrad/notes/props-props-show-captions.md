---
id: props-props-show-captions
kind: doc
target_file: src/props.rs
line_hint: 1620
author: 
created: 
title: "Props::show_captions"
tags: []
signature: "pub fn show_captions (default : bool) -> PropDesc"
---

Shows captions on toolbar buttons (`ShowCaptions`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::show_captions(true);
assert_eq!(p.key, "show_captions");
```
