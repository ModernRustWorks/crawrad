---
id: components-group-box-desenho-draw
kind: doc
target_file: src/components/group_box.rs
line_hint: 28
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , _scene : & Scene)"
---

The Windows frame, with the caption gap — and **no background**.

The owner circled this component on screen: it turned into an *"empty rectangle"* and
could not be distinguished from a `Panel`. The contract is in the RXP
(`crawrad-runtime-egui/src/lib.rs:3019-3060`) and is that of `TGroupBox`: a four-sided
frame, the top line **interrupted** for the caption to fit, and
the caption **straddling** that line — the top line passes through the middle of the text's
height, like in Windows.

**No background, and on purpose:** *"the parent shows through it"*. Drawing
it this way, instead of painting a rectangle and erasing a piece of the line, avoids
having to guess the color behind it — inside a `Panel` it is not that of
the paper, and the only right answer is not needing the answer.
