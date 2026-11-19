---
id: inspector-model-editors-check-check
kind: doc
target_file: src/inspector_model/editors/check.rs
line_hint: 15
author: 
created: 
title: "Check"
tags: []
signature: "pub struct Check"
---

A check box.

# Examples

```rust
use crawrad_actions::inspector_model::editors::check::Check;
use crawrad_actions::inspector_model::editor::PropEditor;

let chk = Check::new(true);
assert_eq!(chk.template(), "RowCheck");
```
