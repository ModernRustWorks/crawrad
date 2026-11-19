---
id: targets-makepad-components-whole-look-widget
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 136
author: 
created: 
title: "whole_look_widget"
tags: []
signature: "pub fn whole_look_widget (class : & str) -> Option < & 'static str >"
---

The accessor of `class`'s own widget when it takes the whole look (#433).

# Examples

```
use crawrad_codegen::targets::makepad::components::whole_look_widget;

assert_eq!(whole_look_widget("ComboBox"), Some("crawrad_combo_box"));
assert_eq!(whole_look_widget("Button"), None);
```
