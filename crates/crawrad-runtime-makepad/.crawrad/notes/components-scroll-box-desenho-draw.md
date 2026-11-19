---
id: components-scroll-box-desenho-draw
kind: doc
target_file: src/components/scroll_box.rs
line_hint: 23
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , _c : & MakepadCtl , r : Rect , _scene : & Scene)"
---

The container surface, which is sunken relative to the paper.

The container surface — and **it is SUNKEN**, which is what separates it from a
`Panel`.

The bars used to come out of here, and because of that every child passed over them. They
moved to [`Self::draw_over`], which runs after the children; the whole
reason is there.

# Why it stopped being the same box as `Panel`

It used to be `caixa_de_superficie`, the same as `Panel` and `TabSheet`, and what
distinguished the two on screen was the bar — drawn always, enabled or
not. The bar came to follow the content (this is the rule of a classic RAD IDE, and is what
the generated application does), and an empty `ScrollBox` would look identical to a
`Panel`. That was already rejected once, with the owner circling the
component and writing *"Scroll???"*.

**A classic RAD IDE separates the two by the BORDER, not by a phantom bar:** the
`TScrollBox` is born with `BorderStyle = bsSingle` and a sunken client
area, whereas `TPanel` has a raised bevel. Here sunken is
[`MAKEPAD_WIDGET_INPUT`], the same as `Edit` and `Memo` — a single token, and
not a new color for just one class.

And that is what the egui target already generates: `egui::Frame::canvas`
(`crawrad-codegen/src/targets/egui/components/scroll_box.rs`), which is egui's
sunken frame.
