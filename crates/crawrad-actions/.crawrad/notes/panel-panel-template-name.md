---
id: panel-panel-template-name
kind: doc
target_file: src/panel.rs
line_hint: 87
author: 
created: 
title: "Panel::template_name"
tags: []
signature: "pub fn template_name (self) -> String"
---

The panel's template in the dock's DSL: `Panel` + its id, capitalised.

# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::Explorer.template_name(), "PanelExplorer");
```
