---
id: targets-makepad-layout-posicao
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 7
author: 
created: 
title: "posicao"
tags: []
signature: "pub fn posicao (x : f32 , y : f32) -> String"
---

Where a component sits on the form, as one line of Makepad DSL.

**Where a component sits on the form**, as one line of Makepad DSL.

# The defect this function closed, and it took the whole target with it

Every component used to write `margin: {left: x, top: y}` — fifteen copies
of the same decision, one per emitter — and the generated application
**drew the background and nothing else**. Not one component appeared.
Measured on 17/09/2026 (issue #156) with a form of three `Label` and zero
events: an empty window.

Three runs isolated it, and the third is the one that decides:

1. painting the body red fills the window — the root `View` exists, has a
   size and draws;
2. a `Label` with `margin` does not appear, not even with its colour forced;
3. the same `Label` with `abs_pos` appears at exactly the point asked for.
   Two of them, at `(100, 40)` and `(200, 80)`, landed on those very
   pixels.

# The bug is `Label`'s, and only `Label`'s

`Label::draw_walk` (`makepad-widgets-1.0.0/src/label.rs:228-238`) spends
the walk TWICE: `cx.begin_turtle(walk, …)` opens a turtle already displaced
by the margin, and then `self.draw_text.draw_walk(cx, walk, …)` is handed
**the same walk** inside it. A margin is RELATIVE, so the second use
displaces it again, and the text lands outside the widget's own turtle,
which `end_turtle_with_area` clips away.

`Button` does it right — `draw_bg.begin(cx, walk, layout)` once, and the
icon and the text walk with their OWN (`button.rs:721-725`) — and so does
`View`. **So `margin` works for everything except `Label`**, and the
`Label` is wrapped in a `View` that carries the margin for it
(`components/label.rs`).

# Why NOT `abs_pos`, which was tried and reverted

`abs_pos` also draws, and it was here for a day. It is ABSOLUTE
(`makepad-draw-1.0.0/src/turtle.rs:534`, `Rect {pos, size}` with no turtle
origin added) — and that is exactly why it had to go: **a component
positioned by `abs_pos` does not scroll.** The owner found it in one
glance, with a `ScrollBox` whose bar moved and whose second button never
came into view: *"it's there, but it doesn't navigate like in egui to view the 2nd
button at the bottom"*.

The scroll offset lives in the turtle, and a margin is measured from it. An
absolute position ignores it, so the box scrolled and the content stayed.

# And why this is ONE function

Fifteen copies of a positioning decision are fifteen places to fix it, and
the day one of them is missed the form has one component in the wrong
place — the hardest kind to notice, because the other fourteen are right.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::posicao;

assert_eq!(posicao(10.0, 20.0), "margin: {left: 10, top: 20}");
```
