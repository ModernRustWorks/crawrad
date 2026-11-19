---
id: targets-makepad-components-has-enabled-field
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 156
author: 
created: 
title: "has_enabled_field"
tags: []
signature: "pub fn has_enabled_field (class : & str) -> bool"
---

Checks whether the generated struct for a class has an enabled field.

Whether the struct this target writes for `class` has an `enabled` field
(#431: the sync hands it to the widget, the actions skip a disabled one).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::has_enabled_field;

assert!(has_enabled_field("Button"));
```
