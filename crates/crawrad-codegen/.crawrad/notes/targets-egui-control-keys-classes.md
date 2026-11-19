---
id: targets-egui-control-keys-classes
kind: doc
target_file: src/targets/egui/control_keys.rs
line_hint: 20
author: 
created: 
title: "CLASSES"
tags: []
signature: "pub const CLASSES : [& str ; 2] = [\"Edit\" , \"MaskEdit\"]"
---

The classes whose emitter dispatches their own key events.

# Examples

```rust
use crawrad_codegen::targets::egui::control_keys::CLASSES;

assert!(CLASSES.contains(&"Edit"));
```
