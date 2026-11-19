---
id: designer-is-visual
kind: doc
target_file: src/lib.rs
line_hint: 387
author: 
created: 
title: "Designer::is_visual"
tags: []
signature: "fn is_visual (& self , doc : & Document , id : & str) -> bool"
---

Returns whether the node is a visual control according to catalog.

Is the node a visual control, by the catalog? Unknown classes count
as visual, as in [`Designer::on_canvas`].
