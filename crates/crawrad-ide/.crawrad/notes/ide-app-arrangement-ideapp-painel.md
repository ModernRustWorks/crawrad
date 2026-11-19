---
id: ide-app-arrangement-ideapp-painel
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 32
author: 
created: 
title: "IdeApp::painel"
tags: []
signature: "pub (super) fn painel (& self , p : Panel) -> WidgetRef"
---

A panel's WIDGET, retrieved from the DOCK — never via the window path.

# The defect that this method exists to make impossible

`self.ui.widget(path)` traverses down the tree and the `View` **caches** the
result (`WidgetCache`). The `Dock` recreates an item's widget whenever
the arrangement changes (`dock.rs:989-993`, `load_state`) and also when
a closed tab returns — and the cached `WidgetRef` continues pointing
to the DEAD instance.

Measured on 2026-09-16, by pointer, with the IDE open:

```text
1ª chamada:   doca=0x2c8a5a9f2b0   janela=0x2c8a5a9f2b0   iguais=true
2ª em diante: doca=0x2c8a5b59560   janela=0x2c8a5a9f2b0   iguais=FALSE
```

**Three faces of the same defect**, and all of them appeared on the owner's screen:

1. the **canvas** drew paper and grid and none of the 19 components —
   `reabrir_no_canvas` opened the document in the dead `DesignCanvas`, and the
   live one drew an empty runtime (`ctls = 0`, default `form_rect`);
2. the **Object Inspector** came up empty with a node selected — the
   draw step's `borrow_mut_if_eq` compares the step's `WidgetRef`
   (live) with what the shell saved (dead), never matches, and the
   list is not populated;
3. both **appeared on the second interaction**: the next frame
   rebuilds the `View` cache, and then the two paths coincide again.
   This was what made the defect look like *"doesn't repaint"* instead of
   *"points to the wrong widget"*.

`DockRef::item` reads `items` on the fly (`dock.rs:1211`), without caching. That is why
it is the only gateway to the contents of a panel.
