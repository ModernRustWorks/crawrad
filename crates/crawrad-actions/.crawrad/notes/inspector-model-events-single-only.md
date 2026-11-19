---
id: inspector-model-events-single-only
kind: doc
target_file: src/inspector_model.rs
line_hint: 146
author: 
created: 
title: "EVENTS_SINGLE_ONLY"
tags: []
signature: "pub const EVENTS_SINGLE_ONLY : & str = \"Events are not available for multiple components.\""
---

Notice shown when multiple components are selected on the events tab.

With several selected there is no events tab: binding the same handler on
two components is a decision nobody asked for (`inspector.rs:412`).

# Examples

```rust
use crawrad_actions::inspector_model::EVENTS_SINGLE_ONLY;

assert_eq!(EVENTS_SINGLE_ONLY, "Events are not available for multiple components.");
```
