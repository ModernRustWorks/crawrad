---
id: canvas-draw-adornments
kind: doc
target_file: src/canvas.rs
line_hint: 965
author: 
created: 
title: "draw_adornments"
tags: []
signature: "fn draw_adornments (p : & mut Brush , a : & Adornments)"
---

Paints the designer adornments: selection, guides, badges, marquee, handles.

A free-standing function because it does not belong to the widget: it paints what `Runtime` sent
via `set_marquee`, `set_guides`, `set_badges`, and `set_selection`, and does not
consult the model.
