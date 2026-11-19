---
id: targets-makepad-control-events-click
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 42
author: 
created: 
title: "CLICK"
tags: []
signature: "pub const CLICK : (& str , & str) = (\"on_click\" , \"Click\")"
---

The click event delivered by the router when unhandled by widgets.

The click and the double click, delivered by the router only where the
widget does not report them itself.

# Examples

```rust
use crawrad_codegen::targets::makepad::control_events::CLICK;

assert_eq!(CLICK.0, "on_click");
assert_eq!(CLICK.1, "Click");
```
