---
id: targets-makepad-components-codegen-makepadcomponentcodegen-focus-path
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 69
author: 
created: 
title: "MakepadComponentCodegen::focus_path"
tags: []
signature: "fn focus_path (& self , _doc : & Document , _node : & Node , _field : & str) -> Option < String >"
---

Widget expression that takes keyboard focus in the Tab sequence.

The widget expression that takes the keyboard focus in the Tab sequence,
or `None` when this component cannot take the focus itself (such as a
container whose view does not take keys).
