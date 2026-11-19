---
id: props-props-date-time-value
kind: doc
target_file: src/props.rs
line_hint: 2162
author: 
created: 
title: "Props::date_time_value"
tags: []
signature: "pub fn date_time_value (default : & str) -> PropDesc"
---

Date and time value in canonical ISO format (`DateTime`, #21).

Default today is NOT allowed in the designer (it would dirty and change
the form file every day): default is fixed to "2000-01-01" or "00:00:00".

# Examples

```rust
use crawrad_registry::Props;

let p = Props::date_time_value("2000-01-01");
assert_eq!(p.key, "date_time");
```
