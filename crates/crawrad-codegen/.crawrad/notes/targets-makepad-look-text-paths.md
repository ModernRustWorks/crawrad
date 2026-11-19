---
id: targets-makepad-look-text-paths
kind: doc
target_file: src/targets/makepad/look.rs
line_hint: 49
author: 
created: 
title: "text_paths"
tags: []
signature: "fn text_paths (doc : & Document , node : & Node , f : & str , text : & str) -> Vec < String >"
---

Returns text paths of `node` under its field name.

Where each text of `node` lives: one path, or one per tree node or tab
(the same names the class's own generator writes).
