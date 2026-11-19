---
id: targets-egui-form-form-cursor
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 856
author: 
created: 
title: "form_cursor"
tags: []
signature: "fn form_cursor (root : & Node) -> String"
---

Emits form-level mouse cursor handling code.

The form's own `Cursor` (#667): over the form where no control showed a
pointer of its own. The form keeps no state struct (#462), so the value is
the designer's, written as egui's pointer.
