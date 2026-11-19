---
id: components-action-desenho
kind: doc
target_file: src/components/action.rs
line_hint: 11
author: 
created: 
title: "DESENHO"
tags: []
signature: "pub static DESENHO : Badge = Badge :: for_class (\"Action\")"
---

O desenho da `Action`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::action::DESENHO;

assert_eq!(DESENHO.class_name(), "Action");
```
