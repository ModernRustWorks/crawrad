---
id: targets-egui-form-look-abre
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 771
author: 
created: 
title: "look_abre"
tags: []
signature: "fn look_abre (reg : & crawrad_registry :: Registry , n : & Node , nome : & str) -> String"
---

Emits style setup for font and background before drawing a control.

**The font and background of a control** (#433), before it draws: the
style's font size (the control's own, or the form's under `ParentFont`) and
its background (the same under `ParentColor`). The style is put back by
[`look_fecha`] after the control — and its children — so nothing leaks to
the next control. Size 0 and alpha 0 leave the theme's.
