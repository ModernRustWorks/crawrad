---
id: targets-makepad-components-main-menu-menuitemcodegen
kind: doc
target_file: src/targets/makepad/components/main_menu.rs
line_hint: 83
author: 
created: 
title: "MenuItemCodegen"
tags: []
signature: "pub struct MenuItemCodegen"
---

Code generation descriptor for MenuItem in Makepad.

`MenuItem`, one item of a menu (#455): a flat button in the MainMenu's bar
or in a drop-down list (`crate::targets::makepad::menus`), whose state the
sync hands to it.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::codegen::MakepadComponentCodegen;
use crawrad_codegen::targets::makepad::components::main_menu::MenuItemCodegen;

let codegen = MenuItemCodegen;
assert_eq!(codegen.class(), "MenuItem");
```
