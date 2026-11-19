---
id: items-editor-editor-title
kind: doc
target_file: src/items_editor.rs
line_hint: 117
author: 
created: 
title: "editor_title"
tags: []
signature: "pub fn editor_title (class_name : & str) -> String"
---

Computes the items editor window title for a component class.

The window title: `ListBox Items Editor`, `ComboBox Items Editor`,
`TreeView Items Editor`.

It is a function, not an `if` inside `show_viewport_immediate`, because a
named rule reads on its own and an `if` buried in a widget does not. The bug
it fixes is this: the two-arm `if` chose between two literals for three
components, and `ListBox` received the name of `TreeView`.

**The reason written here was false**, and cost the loophole three PRs
declared: it said that "a decision inside the viewport is a decision no test
can reach". It can. `embed_viewports()` is born `true` in a
`Context::default()` (`egui-0.36.1/src/context.rs:739`), and then
`show_viewport_immediate` runs the callback in the same context
(`context.rs:4121`) — the entire dialogue comes out painted in the harness,
title bar included. While the impossibility was held true, nobody tried, and
replacing this call with a literal kept the suite green. Not today: see
`a_janela_aberta_pinta_o_nome_da_classe_no_titulo`.

# Examples

```rust
use crawrad_actions::items_editor::editor_title;

assert_eq!(editor_title("ListBox"), "ListBox Items Editor");
```
