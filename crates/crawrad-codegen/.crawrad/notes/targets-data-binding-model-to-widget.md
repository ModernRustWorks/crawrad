---
id: targets-data-binding-model-to-widget
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 269
author: 
created: 
title: "model_to_widget"
tags: []
signature: "fn model_to_widget (controls : & [BoundControl] , form : & str , push : impl Fn (& BoundControl) -> String ,) -> String"
---

Generates model to widget synchronization code for bound controls.

Model -> widget for every binding: when the cell differs from the last
value exchanged, the form's field takes it and `push` shows it in the
widget.

`push(control)` returns the target's extra lines, run with `val` (the cell's
text) and, for a check box, `is_checked` in scope.
