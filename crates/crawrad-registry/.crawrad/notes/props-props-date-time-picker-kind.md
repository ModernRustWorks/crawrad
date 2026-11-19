---
id: props-props-date-time-picker-kind
kind: doc
target_file: src/props.rs
line_hint: 2150
author: 
created: 
title: "Props::date_time_picker_kind"
tags: []
signature: "pub fn date_time_picker_kind (default : & 'static str) -> PropDesc"
---

Mode of a `DateTimePicker`: Date or Time (`Kind`, #21).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::date_time_picker_kind("dtkDate");
assert_eq!(p.key, "kind");
```
