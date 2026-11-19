---
id: paramkind
kind: doc
target_file: src/lib.rs
line_hint: 325
author: 
created: 
title: "ParamKind"
tags: []
signature: "pub enum ParamKind"
---

The semantic value type of a parameter, defining its shape in JSON Schema.

# Examples

```rust
use crawrad_actions::ParamKind;

let kind = ParamKind::String;
assert_eq!(kind, ParamKind::String);
```
