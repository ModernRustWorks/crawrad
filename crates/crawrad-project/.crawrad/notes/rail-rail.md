---
id: rail-rail
kind: doc
target_file: src/rail.rs
line_hint: 55
author: 
created: 
title: "RAIL"
tags: []
signature: "pub const RAIL : & [Panel] = & [Panel :: Explorer , Panel :: Components , Panel :: Inspector , Panel :: Packages , Panel :: Tests , Panel :: Debug , Panel :: Extensions ,]"
---

The rail, top to bottom.

**The order is the owner's, numbered on a screenshot** (08/09/2026):
Explorer, Components, Inspector — the same as `ItemRail::DO_TOPO` of the
egui shell (`main.rs:313`). `Packages` comes after the three, which is
where its button sits on the egui rail (`main.rs:5734`): below the panels
and above the gear.

# Examples

```rust
use crawrad_actions::Panel;
use crawrad_project::rail::RAIL;

assert_eq!(RAIL[0], Panel::Explorer);
```
