---
id: targets-makepad-components-util-field-selection
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 17
author: 
created: 
title: "FIELD_SELECTION"
tags: []
signature: "pub const FIELD_SELECTION : & str = \"    pub selection: Vec<usize>,\""
---

Every row a `ListBox` marks (#439, `MultiSelect`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::FIELD_SELECTION;

assert!(FIELD_SELECTION.contains("selection"));
```
