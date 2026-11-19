---
id: targets-makepad-components-types-strings-type-def
kind: doc
target_file: src/targets/makepad/components/types.rs
line_hint: 13
author: 
created: 
title: "STRINGS_TYPE_DEF"
tags: []
signature: "pub const STRINGS_TYPE_DEF : & str = include_str ! (\"../../../../../crawrad-component/src/pieces/strings.rs\")"
---

The text of the piece, ready to go into the user's project.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::types::STRINGS_TYPE_DEF;

assert!(STRINGS_TYPE_DEF.contains("struct Strings"));
```
