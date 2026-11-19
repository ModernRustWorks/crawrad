---
id: history-origin-parse
kind: doc
target_file: src/history.rs
line_hint: 29
author: 
created: 
title: "Origin::parse"
tags: []
signature: "pub fn parse (s : & str) -> Option < Self >"
---

Parses an origin from its serialized string representation.

# Examples

```rust
use crawrad_host::history::Origin;

assert_eq!(Origin::parse("save"), Some(Origin::Save));
assert_eq!(Origin::parse("other"), None);
```
