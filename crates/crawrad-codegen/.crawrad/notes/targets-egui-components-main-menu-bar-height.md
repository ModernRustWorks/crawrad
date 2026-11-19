---
id: targets-egui-components-main-menu-bar-height
kind: doc
target_file: src/targets/egui/components/main_menu.rs
line_hint: 8
author: 
created: 
title: "BAR_HEIGHT"
tags: []
signature: "pub const BAR_HEIGHT : f32 = 22.0"
---

The height of the MainMenu bar along the top of the form, in points.

The height of the MainMenu's bar along the top of the form, in points
(#455) — the same strip the designer draws.

# Examples

```rust
use crawrad_codegen::targets::egui::components::main_menu::BAR_HEIGHT;

assert!(BAR_HEIGHT > 0.0);
```
