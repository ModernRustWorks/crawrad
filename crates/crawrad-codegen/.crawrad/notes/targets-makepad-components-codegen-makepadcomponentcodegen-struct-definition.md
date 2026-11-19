---
id: targets-makepad-components-codegen-makepadcomponentcodegen-struct-definition
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 31
author: 
created: 
title: "MakepadComponentCodegen::struct_definition"
tags: []
signature: "fn struct_definition (& self) -> String"
---

Emits the Rust struct definition (matching `crawrad-registry` properties).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;

let codegen = ButtonCodegen;
let def = codegen.struct_definition();
assert!(def.contains("pub struct Button"));
```
