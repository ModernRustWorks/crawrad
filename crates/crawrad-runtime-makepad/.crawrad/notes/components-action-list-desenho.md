---
id: components-action-list-desenho
kind: doc
target_file: src/components/action_list.rs
line_hint: 10
author: 
created: 
title: "DESENHO"
tags: []
signature: "pub static DESENHO : Badge = Badge :: for_class (\"ActionList\")"
---

O desenho do `ActionList`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::action_list::DESENHO;

assert_eq!(DESENHO.class_name(), "ActionList");
```
