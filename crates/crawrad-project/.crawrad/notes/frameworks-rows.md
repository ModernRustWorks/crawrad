---
id: frameworks-rows
kind: doc
target_file: src/frameworks.rs
line_hint: 33
author: 
created: 
title: "ROWS"
tags: []
signature: "pub const ROWS : usize = 6"
---

Maximum number of framework rows drawn by the New Project dialog.

How many rows the dialog draws (`np_fw_0` … `np_fw_5`); the catalogue test
keeps the list within them.

# Examples

```rust
use crawrad_project::frameworks::ROWS;

assert_eq!(ROWS, 6);
```
