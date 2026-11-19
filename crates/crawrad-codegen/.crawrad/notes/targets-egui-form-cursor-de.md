---
id: targets-egui-form-cursor-de
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 824
author: 
created: 
title: "cursor_de"
tags: []
signature: "fn cursor_de (reg : & crawrad_registry :: Registry , n : & Node , nome : & str) -> String"
---

Emits mouse cursor override code for a control.

**The pointer over a control** (#667), one generic piece after any
control that draws itself, beside its tooltip: while the pointer is in its
box, the control is visible and enabled and its `Cursor` is not the
default, egui shows that pointer. `Default` leaves the widget its own (the
text beam of an edit), as the classic `crDefault` does.
