---
id: targets-makepad-binding-accessors
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 262
author: 
created: 
title: "accessors"
tags: []
signature: "pub fn accessors () -> Vec < & 'static str >"
---

The distinct accessors that the table uses — the real size of the vocabulary.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::accessors;

let accs = accessors();
assert!(accs.contains(&"button"));
```
