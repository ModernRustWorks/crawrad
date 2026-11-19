---
id: targets-egui-form-tooltip-de
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 810
author: 
created: 
title: "tooltip_de"
tags: []
signature: "fn tooltip_de (reg : & crawrad_registry :: Registry , n : & Node , nome : & str) -> String"
---

Emits tooltip handling code for a control.

**The tooltip of a control** (#432), one generic piece after any control
that draws itself: while the pointer is in its box and `ShowHint` is on,
the `Hint` shows at the pointer. It registers no widget of its own, so the
control keeps its hover and its clicks.
