---
id: targets-makepad-components-codegen-makepadcomponentcodegen-class
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 8
author: 
created: 
title: "MakepadComponentCodegen::class"
tags: []
signature: "fn class (& self) -> & 'static str"
---

The name of the class in the catalog — `"Button"`, `"ScrollBar"`.

It was a closed `enum` of sixteen variants, and ADR 0031 measured the
cost: **until a class entered it, no target could
register a handler for it**. With installable components (ADR 0033) this
became a blocker — class 27 comes from a file.

It is the only time the class appears written in this crate.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::button::ButtonCodegen;
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;

let codegen = ButtonCodegen;
assert_eq!(codegen.class(), "Button");
```
