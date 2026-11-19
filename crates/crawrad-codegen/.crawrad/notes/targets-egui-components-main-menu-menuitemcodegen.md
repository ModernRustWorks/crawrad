---
id: targets-egui-components-main-menu-menuitemcodegen
kind: doc
target_file: src/targets/egui/components/main_menu.rs
line_hint: 59
author: 
created: 
title: "MenuItemCodegen"
tags: []
signature: "pub struct MenuItemCodegen"
---

Code generation descriptor for MenuItem.

`MenuItem`, one item of a menu (#455): data its menu shows, so its body
only takes the caption of a linked `Action`.

# Examples

```rust
use crawrad_codegen::targets::egui::components::main_menu::MenuItemCodegen;
use crawrad_codegen::targets::egui::components::EguiComponentCodegen;

let codegen = MenuItemCodegen;
assert_eq!(codegen.class(), "MenuItem");
```
