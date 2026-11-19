---
id: targets-egui-form-form-hint
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 894
author: 
created: 
title: "form_hint"
tags: []
signature: "fn form_hint (root : & Node , corpo : & str) -> (String , String)"
---

Emits form-level hint handling code.

**The form's own hint** (#667): over the form where the pointer is over no
control, as the classic form shows its `Hint` over its empty body. Returns
what opens the body (the flag the controls set) and what closes it.
