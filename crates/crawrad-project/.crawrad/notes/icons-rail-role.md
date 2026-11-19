---
id: icons-rail-role
kind: doc
target_file: src/icons.rs
line_hint: 50
author: 
created: 
title: "rail_role"
tags: []
signature: "pub fn rail_role (panel : crawrad_actions :: Panel) -> Option < Chrome >"
---

The icon role of a rail panel.

**The glyph does not live here.** It comes from [`Chrome`],
which is the single chrome table of this shell — and it is the SAME choice
the egui shell makes for the same button (`crates/crawrad-ide/src/icones.rs`),
with a test comparing the two tables character by character.

`None` for a panel with no rail icon, which is the same as saying it has no
rail button: `RAIL` and this function answer the same question from two
directions, and the test `todo_painel_do_trilho_tem_icone` charges them
against each other.

# Examples

```rust
use crawrad_actions::Panel;
use crawrad_project::icons::{rail_role, Chrome};

assert_eq!(rail_role(Panel::Explorer), Some(Chrome::Explorer));
```
