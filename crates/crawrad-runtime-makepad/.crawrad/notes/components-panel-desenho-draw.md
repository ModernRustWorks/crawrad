---
id: components-panel-desenho-draw
kind: doc
target_file: src/components/panel.rs
line_hint: 22
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , _scene : & Scene)"
---

Surface, frame and the corner caption — a classic RAD IDE's panel.

The caption had been ignored here, and the RXP draws it
(`crawrad-runtime-egui/src/lib.rs:2536-2544`): without it the owner's named panel
appeared empty.
