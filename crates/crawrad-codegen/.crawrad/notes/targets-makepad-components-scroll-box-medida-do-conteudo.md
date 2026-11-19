---
id: targets-makepad-components-scroll-box-medida-do-conteudo
kind: doc
target_file: src/targets/makepad/components/scroll_box.rs
line_hint: 185
author: 
created: 
title: "medida_do_conteudo"
tags: []
signature: "fn medida_do_conteudo (doc : & Document , node : & Node , w : f32 , h : f32) -> String"
---

The size of the content, intentionally told to the container.

**The size of the content, intentionally told to the container.**

# Why it does not discover it on its own

In `flow: Overlay`, Makepad measures content by the SIZE of each child and
**ignores their margin**: the Overlay branch calls
`turtle.update_height_max(turtle.pos.y, size.y)`
(`makepad-draw-1.0.0/src/turtle.rs:611`), and `turtle.pos` does not advance in
Overlay. A 32-point button at a margin of 200 reports 32, never 232 — and the
box never knows that it overflows.

Measured on screen on 17/09/2026: without this line, `ScrollBox` shows no scrollbar
and does not scroll, even with a child far beyond the bottom.

# Why not use `abs_pos`, which would measure correctly

Because `abs_pos` measures correctly and **does not scroll** — the other arm of the same `match`
uses the absolute position, which ignores turtle scrolling. That was the bug
the owner saw earlier: a scrollbar that moves and content that stands still. The entire rationale
is in [`super::super::layout::posicao`].

# And the spacer draws nothing

It is a `<View>` without `show_bg`: it exists to OCCUPY, and occupying is what the
container's calculation reads. The size calculation is the neutral one, the same one the
designer uses to decide whether to draw the scrollbar — because they are the same, both
sides cannot disagree.
