---
id: props-date-time-picker-kinds
kind: doc
target_file: src/props.rs
line_hint: 91
author: 
created: 
title: "DATE_TIME_PICKER_KINDS"
tags: []
signature: "pub const DATE_TIME_PICKER_KINDS : & [& str] = & [\"dtkDate\" , \"dtkTime\"]"
---

The modes of a `DateTimePicker` (#21).

# Examples

```rust
use crawrad_registry::DATE_TIME_PICKER_KINDS;

assert!(DATE_TIME_PICKER_KINDS.contains(&"dtkDate"));
```
