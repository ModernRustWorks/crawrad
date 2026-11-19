---
id: link-elo-do-duplo-clique
kind: doc
target_file: src/link.rs
line_hint: 451
author: 
created: 
title: "elo_do_duplo_clique"
tags: []
signature: "pub fn elo_do_duplo_clique (no : & Node , desc : & ComponentDesc) -> Option < Elo >"
---

Which handler double clicking on a component looks for.

# Examples

```rust
use crawrad_codegen::link::elo_do_duplo_clique;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let node = Node::new("btn", "Button");
let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let elo = elo_do_duplo_clique(&node, desc);
assert!(elo.is_some());
```

The rule, in two lines:

1. **an already connected event wins** — if the user wrote the `OnClick` of a
   `ComboBox`, double clicking navigates to `OnClick`, rather than creating an `OnChange`
   alongside it. Navigating to what exists is always cheaper to undo than
   creating what was not asked for;
2. otherwise, the **first declared event** in the registry.

The first declared one, rather than a table of "default event per class" written
here: the registry is the source of what each component has, and a second list in the
shell would be a second truth about the same component — the one that diverges
when someone adds an event. **Known divergence:** in a classic RAD IDE the
default for `TListBox` is `OnClick` and here it yields `OnChange`, because that is how the
registry declares them. The fix is a marker on the descriptor, on the data side; the
shell is not the place to contradict it.

`None` for anything declaring no events at all — `Label`, `Panel`, `GroupBox`.
Double clicking on them does nothing, as in a classic RAD IDE.
