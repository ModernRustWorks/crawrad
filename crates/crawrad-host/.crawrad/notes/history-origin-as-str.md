---
id: history-origin-as-str
kind: doc
target_file: src/history.rs
line_hint: 21
author: 
created: 
title: "Origin::as_str"
tags: []
signature: "pub fn as_str (& self) -> & 'static str"
---

Returns the serialized identifier of this origin ("save" or "external").

# Examples

```rust
use crawrad_host::history::Origin;

assert_eq!(Origin::Save.as_str(), "save");
assert_eq!(Origin::External.as_str(), "external");
```
