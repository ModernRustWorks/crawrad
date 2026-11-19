---
id: canvas-brush-brush-as-chrome
kind: doc
target_file: src/canvas/brush.rs
line_hint: 427
author: 
created: 
title: "Brush::as_chrome"
tags: []
signature: "pub (crate) fn as_chrome (& mut self , paint : impl FnOnce (& mut Self))"
---

Directs primitives to container chrome layers during `paint`.

Runs `paint` with every box and arrow going to the **container chrome**
layers ([`Layer::Chrome`] and [`Layer::ChromeArrow`]) instead of the
component ones.

It is what the canvas wraps around
[`crate::componentes::ComponentDrawing::draw_over`], and it is the
whole of the second half of issue #129: the bar a container paints over
its children now sits **above the marquee** as well, because layer beats
emission order and the marquee is a layer of its own.

It restores the flag on the way out, so nesting cannot leak the mode
into the next control.
