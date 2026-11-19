---
id: properties-unknown-key
kind: doc
target_file: src/properties.rs
line_hint: 11
author: 
created: 
title: "unknown_key"
tags: []
signature: "pub fn unknown_key (class : & str , keys : & [& str] , key : & str) -> Option < String >"
---

Refuses an undeclared property key and lists the valid class keys.

The refusal of a key the class does not declare, naming the keys it has so
an agent corrects itself in one step. `None` when the key is fine to try:
declared, a nested path (`font.size`), or a class with no known keys.

# Examples

```rust
use crawrad_mcp::properties::unknown_key;

let refusal = unknown_key("Button", &["caption", "enabled"], "title");
assert!(refusal.is_some());
assert!(refusal.unwrap().contains("caption, enabled"));
```
