---
id: ide-app-classe-arrastada
kind: doc
target_file: src/ide_app.rs
line_hint: 1082
author: 
created: 
title: "classe_arrastada"
tags: []
signature: "pub fn classe_arrastada (itens : & [DragItem]) -> Option < String >"
---

The CLASS being dragged from the palette, if it is ours.

`None` for anything else dropped onto the window — a desktop file,
text from another program. The criterion is `internal_id`, and it is not
pedantry: without it, dropping a file onto the form would attempt to create a component
whose class would be the file path.

Standalone function on purpose: it is pure over the drag items, which is why
the test exercises it without opening a window.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let items = vec![];
assert_eq!(classe_arrastada(&items), None);
```
