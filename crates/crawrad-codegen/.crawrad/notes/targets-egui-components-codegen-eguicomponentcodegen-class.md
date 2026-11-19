---
id: targets-egui-components-codegen-eguicomponentcodegen-class
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 9
author: 
created: 
title: "EguiComponentCodegen::class"
tags: []
signature: "fn class (& self) -> & 'static str"
---

The class name in the catalog — `"Button"`, `"ScrollBar"`.

# It used to be an `enum`, and the `enum` was the bottleneck

Until 09/17/2026 this returned `ComponentClass`, a closed `enum` of
sixteen variants. ADR 0031 measured the consequence: **as long as a
class was not in that enum, no target could register a handler
for it** — and that was why `Form`, `Action`, `ActionList`, and
`ImageList` were left out.

With installable components (ADR 0033), this ceased being an annoyance and
became a blocker: class 27 comes from a file, and no compiled `enum`
can accommodate it.

**It is the ONLY time the class appears written in this crate.** Dispatch
compares against this, and nothing else compares a literal — the same
grammar that `DESENHOS` in the Makepad adapter has already proven.

# Examples

```rust
use crawrad_codegen::targets::egui::components::button::ButtonCodegen;
use crawrad_codegen::targets::egui::components::EguiComponentCodegen;

let codegen = ButtonCodegen;
assert_eq!(codegen.class(), "Button");
```
