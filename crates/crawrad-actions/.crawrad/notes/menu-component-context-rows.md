---
id: menu-component-context-rows
kind: doc
target_file: src/menu.rs
line_hint: 1
author:
created:
title: "component_context_rows"
tags: []
signature: "pub fn component_context_rows (name : & str , base_form : Option < & str > , ctx : & Context) -> Vec < Row >"
---

The right-click menu of a component on the canvas (#851).

A component of the base form (`base_form` is its name) gets a title, `Go to
base form`, and greyed `Delete`, `Rename` and `Move to another parent` with a
notice saying why: the classic IDE shows what cannot be done, it does not hide
it. A component of this form gets `Edit events`, `Delete` and `Rename`.

# Examples

```
use crawrad_actions::menu::{Tone, component_context_rows};
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
let rows = component_context_rows("btn_save", Some("form_parent"), &ctx);
assert_eq!(rows[0].tone, Tone::Title);
assert_eq!(rows.last().map(|r| r.tone), Some(Tone::Notice));
assert_eq!(component_context_rows("edit_name", None, &ctx).len(), 4);
```
