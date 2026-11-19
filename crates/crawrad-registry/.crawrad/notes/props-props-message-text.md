---
id: props-props-message-text
kind: doc
target_file: src/props.rs
line_hint: 1924
author: 
created: 
title: "Props::message_text"
tags: []
signature: "pub fn message_text (default : impl Into < String >) -> PropDesc"
---

Text/body message for a message dialog (`Text`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::message_text("Are you sure?");
assert_eq!(p.key, "text");
```
