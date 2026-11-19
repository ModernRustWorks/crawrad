---
id: history-origin
kind: doc
target_file: src/history.rs
line_hint: 11
author: 
created: 
title: "Origin"
tags: []
signature: "pub enum Origin"
---

Origin of a file history revision.

# Examples

```rust
use crawrad_host::history::Origin;

let origin = Origin::Save;
assert_eq!(origin.as_str(), "save");
```
