---
id: targets-egui-form-has-hint
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 757
author: 
created: 
title: "has_hint"
tags: []
signature: "fn has_hint (reg : & crawrad_registry :: Registry , class : & str) -> bool"
---

Checks whether a component class carries a tooltip hint.

Whether `class` carries the `Hint` this target shows as a tooltip (#432):
every control that draws itself, but not a `TabSheet` (a page, not a
widget of its own — the same line the Makepad target draws).
