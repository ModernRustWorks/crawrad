---
id: menu-panel-rows
kind: doc
target_file: src/menu.rs
line_hint: 446
author: 
created: 
title: "panel_rows"
tags: []
signature: "pub fn panel_rows (panel : Panel , ctx : & Context) -> Vec < Row >"
---

The rows of the CONTEXT menu of a panel — `Move To`, `Hide`, `Float`.

# Why this menu exists at all

[ADR 0014, Amendment 10] puts the command first and the drag second, and gives
three reasons; the first is the one this function answers:

> *"Accessibility. Dragging is the only gesture of the arrangement with no
> keyboard equivalent. Whoever does not use a mouse precisely is left out of
> a whole function of the product."*

And the platform gives the second: `CxOsOp::StartDragging` has no arm in
`linux_x11.rs`, so on Linux dragging a tab logs an error and does nothing.
**This menu is the door that is open on all three platforms, today.**

# Why the rows are not read out of a group

`rows` filters out every action that requires an argument, because a top
menu has nowhere to type one. These five all require `panel` — and this
menu knows it, because it was opened on a panel. So the rows are built
here, by id, and each carries its own [`Args`].

The ids are written once each, and the sides come from [`DockSide::ALL`] —
so a side added to the catalogue gets a row here without this list being
edited. What it does need is a label, and `move_label` is an exhaustive
`match`: a side with no wording does not compile.


# Examples

```rust
use crawrad_actions::menu::panel_rows;
use crawrad_actions::{Args, Author, Context, EditorText, Panel, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

let registry = Registry::builtin();
let clipboard = Clipboard::default();
let ctx = Context {
    doc: None,
    registry: &registry,
    clipboard: &clipboard,
    args: Args::none(),
    project: Project { has_binary: false, ..Project::default() },
    author: Author::User,
    editor_dirty: false,
    text: EditorText::default(),
};
let rows = panel_rows(Panel::Inspector, &ctx);
assert!(!rows.is_empty());
```
