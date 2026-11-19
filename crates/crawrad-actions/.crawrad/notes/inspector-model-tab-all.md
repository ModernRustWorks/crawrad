---
id: inspector-model-tab-all
kind: doc
target_file: src/inspector_model.rs
line_hint: 101
author: 
created: 
title: "Tab::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Properties , Self :: Events]"
---

Both, in the order the strip shows them — the classic RAD IDE's.

# Examples

```rust
use crawrad_actions::inspector_model::Tab;

assert_eq!(Tab::ALL.len(), 2);
```
