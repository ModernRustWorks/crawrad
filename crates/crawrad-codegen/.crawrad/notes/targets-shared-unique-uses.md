---
id: targets-shared-unique-uses
kind: doc
target_file: src/targets/shared.rs
line_hint: 310
author: 
created: 
title: "unique_uses"
tags: []
signature: "pub fn unique_uses (text : & str) -> String"
---

Deduplicates top-level `use` imports across generated code pieces.

Several pieces in one file (Font, Cursor, Shortcut) each import
`std::fmt`: a top-level `use` goes out once.

# Examples

```rust
use crawrad_codegen::targets::shared::unique_uses;

let code = "use std::fmt;\nuse std::fmt;\n";
assert_eq!(unique_uses(code), "use std::fmt;\n");
```
