---
id: targets-egui-state-init-mask-edit-text
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 809
author: 
created: 
title: "init_mask_edit_text"
tags: []
signature: "fn init_mask_edit_text (n : & Node , doc : Option < & Document >) -> String"
---

Initializes MaskEdit text shaped by its edit mask.

A `MaskEdit`'s text (#71): cased as an `Edit`'s, then shaped by the mask
when the app starts — the blank mask for an empty text.
