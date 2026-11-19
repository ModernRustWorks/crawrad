---
id: targets-makepad-components-status-bar-statuspanelcodegen
kind: doc
target_file: src/targets/makepad/components/status_bar.rs
line_hint: 165
author: 
created: 
title: "StatusPanelCodegen"
tags: []
signature: "pub struct StatusPanelCodegen"
---

Makepad code generator for `StatusPanel` components.

`StatusPanel`, one panel of a StatusBar (#458): data the StatusBar's own
sync draws, so it has no widget and no sync of its own.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::status_bar::StatusPanelCodegen;

let codegen = StatusPanelCodegen;
let _ = codegen;
```
