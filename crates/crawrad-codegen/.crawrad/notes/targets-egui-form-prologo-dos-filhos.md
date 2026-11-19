---
id: targets-egui-form-prologo-dos-filhos
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 621
author: 
created: 
title: "PROLOGO_DOS_FILHOS"
tags: []
signature: "const PROLOGO_DOS_FILHOS : & str = \"\\\n\\x20                               // **THE CHILDREN GO IN HERE**, and that is what makes\\n\\\n\\x20                               // this container contain. `origin` is the CONTENT\\n\\\n\\x20                               // origin — it moves as the bar scrolls — and the\\n\\\n\\x20                               // children are placed relative to it, exactly as they\\n\\\n\\x20                               // are stored.\\n\\\n\\x20                               //\\n\\\n\\x20                               // `grown` is zero: what grows with the window is the\\n\\\n\\x20                               // BOX, through its own anchors. A child in here\\n\\\n\\x20                               // anchors to the box, and the box already took it\\n\\\n\\x20                               // along.\\n\\\n\\x20                               let origin = ui.min_rect().min;\\n\\\n\\x20                               let grown = egui::Vec2::ZERO;\\n\""
---

What opens the children's space inside a wrapping container.

Redefines `origin` and `grown` for the container's content: the body of each
child is the SAME text that would appear in the form, and what changes is where it
measures from. Without this, each child would need an "inside container" variant,
and two variants of the same design diverge.
