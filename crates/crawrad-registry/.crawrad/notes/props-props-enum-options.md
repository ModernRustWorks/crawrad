---
id: props-props-enum-options
kind: doc
target_file: src/props.rs
line_hint: 1339
author: 
created: 
title: "Props::enum_options"
tags: []
signature: "pub fn enum_options (key : & 'static str , display : & 'static str , default : & str , options : & 'static [& 'static str] , category : Category ,) -> PropDesc"
---

Enumerated property with a closed options list.

# Examples

```rust
use crawrad_registry::{Category, Props};

let p = Props::enum_options("kind", "Kind", "A", &["A", "B"], Category::Behavior);
assert_eq!(p.key, "kind");
```
