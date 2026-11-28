---
id: hostmodel-strip-row
kind: doc
target_file: src/lib.rs
line_hint: 150
author: 
created: 
title: "StripRow"
tags: []
signature: "pub struct StripRow"
---

One row of the strip of tabs.

`dirty` is what the tab's X shows. `is_form` says which of the two kinds it is: a
form carries a Designer and a Code page, a loose `.rs` carries only Code.

# Examples

```rust
use crawrad_host::StripRow;

let row = StripRow { label: "main.rs".to_string(), dirty: true, is_form: false };
assert!(!row.is_form);
assert!(row.dirty);
```
