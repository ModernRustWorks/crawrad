---
id: multi-select-multi-select-summary
kind: doc
target_file: src/multi_select.rs
line_hint: 110
author: 
created: 
title: "multi_select_summary"
tags: []
signature: "pub fn multi_select_summary (items : & [String] , selected : & [String] , hint : & str , mut fits : impl FnMut (& str) -> bool ,) -> String"
---

Pure function computing the summary string for a MultiSelect control (#500).

# Examples

```rust
use crawrad_runtime_makepad::multi_select::multi_select_summary;

let items = vec!["A".to_string(), "B".to_string()];
let sel = vec!["A".to_string()];
assert_eq!(multi_select_summary(&items, &sel, "Select...", |_| true), "A");
```
