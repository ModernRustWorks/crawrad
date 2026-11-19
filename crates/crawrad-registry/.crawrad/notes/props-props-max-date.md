---
id: props-props-max-date
kind: doc
target_file: src/props.rs
line_hint: 2186
author: 
created: 
title: "Props::max_date"
tags: []
signature: "pub fn max_date (default : & str) -> PropDesc"
---

Maximum permitted date or time in ISO format (`MaxDate`, #21).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max_date("2100-12-31");
assert_eq!(p.key, "max_date");
```
