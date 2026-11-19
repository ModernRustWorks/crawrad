---
id: targets-makepad-menus-close-lists
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 91
author: 
created: 
title: "close_lists"
tags: []
signature: "pub fn close_lists (lists : & [List] , keep : & [String] , pad : usize) -> String"
---

The lines that close every list but `keep`, `pad` spaces in.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::close_lists;

let code = close_lists(&[], &[], 4);
assert!(code.is_empty());
```
