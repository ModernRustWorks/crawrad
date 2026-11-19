---
id: targets-makepad-components-codegen-makepadcomponentcodegen-text-path
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 21
author: 
created: 
title: "MakepadComponentCodegen::text_path"
tags: []
signature: "fn text_path (& self , _node : & Node , field : & str) -> String"
---

Path to the widget holding keyboard focus and text under field.

Where the widget that holds the keyboard and the text lives, under the
control's `field`: the control itself, unless it wraps that widget in a
view of its own (#139: a Memo with scroll bars).
