---
id: inspector-model-tab
kind: doc
target_file: src/inspector_model.rs
line_hint: 92
author: 
created: 
title: "Tab"
tags: []
signature: "pub enum Tab"
---

The tabs of the Object Inspector: Properties and Events.

The tabs of the Object Inspector, **identical to the classic RAD one**: `Properties`
and `Events`. It is the same pair as the egui shell's
(`crates/crawrad-ide/src/inspector.rs:18-23`).

The events are not an extra: without them the panel shows half the
component, and the missing half is the one that ties the form to the code.
The list comes out of the **catalogue** (`crawrad_registry::EventDesc`), like
the properties — no `on_click` is written in this shell.

# Examples

```rust
use crawrad_actions::inspector_model::Tab;

let tab = Tab::default();
assert_eq!(tab, Tab::Properties);
```
