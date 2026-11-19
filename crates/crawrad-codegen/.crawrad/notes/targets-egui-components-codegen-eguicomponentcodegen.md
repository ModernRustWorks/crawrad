---
id: targets-egui-components-codegen-eguicomponentcodegen
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 7
author: 
created: 
title: "EguiComponentCodegen"
tags: []
signature: "pub trait EguiComponentCodegen : Send + Sync"
---

Contract for code generation of an egui visual or non-visual component.

Contract encapsulating the code generation lifecycle of an egui visual or non-visual component.

# Examples

```rust
use crawrad_codegen::targets::egui::components::button::ButtonCodegen;
use crawrad_codegen::targets::egui::components::EguiComponentCodegen;

let codegen = ButtonCodegen;
assert_eq!(codegen.class(), "Button");
```
