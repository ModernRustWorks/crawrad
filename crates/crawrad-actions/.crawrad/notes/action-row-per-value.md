---
id: action-row-per-value
kind: doc
target_file: src/lib.rs
line_hint: 535
author: 
created: 
title: "Action::row_per_value"
tags: []
signature: "pub fn row_per_value (& self) -> Option < & 'static Param >"
---

Returns the parameter a menu can supply by drawing a row per value.

The one argument a MENU can supply by drawing a row per value — or
`None` for every action a menu has to draw as a single row, or not at
all.

# The three conditions, and why each one is there

1. **Exactly one required parameter.** Two closed sets are a grid, not
   a list: `layout.move_panel(panel, side)` would be eighteen rows, and
   the door that collects two arguments is the panel context menu,
   which already knows one of them.
2. **It publishes a [`Domain`].** A menu cannot enumerate free text.
3. **The domain says [`row_per_value`](Domain::row_per_value).** The
   reason is written there: a row per value has to BE the whole
   command, not one of four ways to say it.

The shape — a borrowed `Param` and not a `bool` — is what lets the
caller build the argument without naming it: the row passes
`Args::text(p.name, choice.id)`, so the menu never writes `"panel"`.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save").unwrap();
assert!(action.row_per_value().is_none());
```
