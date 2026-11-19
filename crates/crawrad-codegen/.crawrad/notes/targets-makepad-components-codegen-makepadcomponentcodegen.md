---
id: targets-makepad-components-codegen-makepadcomponentcodegen
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 6
author: 
created: 
title: "MakepadComponentCodegen"
tags: []
signature: "pub trait MakepadComponentCodegen : Send + Sync"
---

Contract for code generation lifecycle of a visual Makepad component.

Contract encapsulating the full code generation lifecycle of a visual component in Makepad.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;

let codegen = ButtonCodegen;
assert_eq!(codegen.class(), "Button");
```
