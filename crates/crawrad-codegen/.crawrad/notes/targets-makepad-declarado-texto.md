---
id: targets-makepad-declarado-texto
kind: doc
target_file: src/targets/makepad/declarado.rs
line_hint: 237
author: 
created: 
title: "texto"
tags: []
signature: "fn texto (field : & str , desc : & ComponentDesc , de : & TextSource , color : Ink , centered : bool , id : & str ,) -> Option < (& 'static str , String , String) >"
---

The verb that writes — a `<Label>` inside the area's `<View>`.

The label comes wrapped for the same reason as this target's native `Label`:
Makepad's `Label::draw_walk` consumes the walk twice, and a label with
its own margin does not paint a single pixel. Here the margin belongs to the area; the label is born
with zero.
