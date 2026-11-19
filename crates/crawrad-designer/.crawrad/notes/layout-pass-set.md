---
id: layout-pass-set
kind: doc
target_file: src/layout.rs
line_hint: 107
author: 
created: 
title: "Pass::set"
tags: []
signature: "fn set (& mut self , id : & str , new_rect : Rect)"
---

Emits a rectangle change in this pass when different.

Only emits when changing: an empty transaction on each frame would fill
undo with nothing.
