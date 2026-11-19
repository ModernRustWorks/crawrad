---
id: components-non-visual-declared-class
kind: doc
target_file: src/components/non_visual.rs
line_hint: 80
author: 
created: 
title: "DECLARED_CLASS"
tags: []
signature: "pub const DECLARED_CLASS : & str = \"crawrad::DeclaredNonVisual\""
---

The painted class of a catalogue component declared `visual: false`.

A third-party non-visual brings no Rust drawing; the runtime paints it as
this badge, chosen by the manifest flag and never by the class name. The
`::` keeps it from ever colliding with a real class.

# Examples

```rust
use crawrad_runtime_makepad::components::non_visual::DECLARED_CLASS;

assert!(DECLARED_CLASS.contains("::"));
```
