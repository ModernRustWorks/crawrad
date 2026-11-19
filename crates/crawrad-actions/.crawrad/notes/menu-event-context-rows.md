---
id: menu-event-context-rows
kind: doc
target_file: src/menu.rs
line_hint: 1
author:
created:
title: "event_context_rows"
tags: []
signature: "pub fn event_context_rows (node : & str , event : & str , overridden : bool , ctx : & Context) -> Vec < Row >"
---

The right-click menu of an inherited event's row in the Object Inspector.

`Go to handler` and `Revert to inherited` need an override to act on, and
stay greyed out without one: the row is still there, so the menu has the
same shape whatever the state of the event.

# Examples

```
use crawrad_actions::menu::event_context_rows;
use crawrad_actions::{Args, Context, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Author;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let clip = Clipboard::default();
let ctx = Context {
    doc: None,
    registry: &reg,
    clipboard: &clip,
    args: Args::none(),
    project: Project::default(),
    author: Author::User,
    editor_dirty: false,
    text: Default::default(),
};
let rows = event_context_rows("btn_save", "on_click", false, &ctx);
assert_eq!(rows.len(), 4);
assert!(!rows[0].enabled);
```
