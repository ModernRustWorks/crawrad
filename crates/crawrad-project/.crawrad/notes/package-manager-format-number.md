---
id: package-manager-format-number
kind: doc
target_file: src/package_manager.rs
line_hint: 183
author: 
created: 
title: "format_number"
tags: []
signature: "pub fn format_number (n : u64) -> String"
---

Formats a number with thousands separator (comma): `29449600` → `"29,449,600"`.

# Examples

```rust
use crawrad_project::package_manager::format_number;

assert_eq!(format_number(29449600), "29,449,600");
```
