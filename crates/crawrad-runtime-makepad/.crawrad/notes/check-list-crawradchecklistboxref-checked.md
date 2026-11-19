---
id: check-list-crawradchecklistboxref-checked
kind: doc
target_file: src/check_list.rs
line_hint: 221
author: 
created: 
title: "CrawradCheckListBoxRef::checked"
tags: []
signature: "pub fn checked (& self) -> Vec < bool >"
---

Which rows are checked now.

# Examples

```rust
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

let list = CrawradCheckListBoxRef::default();
assert!(list.checked().is_empty());
```
