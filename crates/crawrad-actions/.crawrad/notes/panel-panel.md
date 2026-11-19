---
id: panel-panel
kind: doc
target_file: src/panel.rs
line_hint: 37
author: 
created: 
title: "Panel"
tags: []
signature: "pub enum Panel"
---

One dockable panel of the IDE.

The variants are the panels that **exist**, not the panels that are
planned. `Problems`, `Build` and `Terminal` are named by Amendment 9 and are
deliberately absent: [ADR 0014] names drawing an empty region before there
is something to put in it as the classic mistake, and a panel in this enum
is a panel an agent can ask for by name — publishing one that answers
nothing would be the catalogue lying about what the IDE can do.


# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::Explorer.id(), "explorer");
```
