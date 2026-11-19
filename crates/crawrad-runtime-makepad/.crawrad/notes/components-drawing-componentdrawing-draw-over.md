---
id: components-drawing-componentdrawing-draw-over
kind: doc
target_file: src/components/drawing.rs
line_hint: 110
author: 
created: 
title: "ComponentDrawing::draw_over"
tags: []
signature: "fn draw_over (& self , _p : & mut Brush , _c : & MakepadCtl , _r : Rect , _scene : & Scene)"
---

What this control paints **after** its children, and above them.

# Why this exists, and what the owner saw

> *"The `ScrollBox` clipped the components inside it well, but
> there is a detail: the components are overlapping the scroll bar. In
> a classic RAD IDE the scroll bar still stays on top."*

A container used to paint itself whole — surface **and** scroll bars —
in one go, and every child drawn afterwards passed over the bars. The
bar is chrome of the container, not scenery behind it.

# The framework agrees, and that is what settles it

`makepad-widgets-1.0.0/src/scroll_bars.rs:245-250`: `begin()` opens the
turtle with **no trough reserved**, and `end()` calls
`draw_scroll_bars(cx)` **before** `end_turtle_with_area` — that is, after
all the content. In Makepad the bar is **overlaid**, not non-client.

So this house does not carve a client area out of the model. Doing so
would make the designer disagree with the generated app, which uses the
framework's own `ScrollBars` — the broken-WYSIWYG family of defect. What
a classic RAD IDE and Makepad *do* agree on is the part the owner pointed at: **the
bar stays on top**. That is what this method restores, and it is all it
restores.

`r` is the control's own rectangle on screen, the same one
[`Self::draw`] received.
