---
id: targets-makepad-components-edit-struct-of
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 65
author: 
created: 
title: "struct_of"
tags: []
signature: "pub fn struct_of (desc : & crawrad_registry :: ComponentDesc) -> String"
---

State struct of a text box of the Edit family from its catalogue entry.

The state struct of a text box of the `Edit` family, from its catalogue
entry (a `MaskEdit` adds its mask, #71).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::edit::struct_of;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Edit").unwrap();
let def = struct_of(desc);
assert!(def.contains("pub struct Edit"));
```
