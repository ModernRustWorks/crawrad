---
id: components-memo-scroll-bars-of
kind: doc
target_file: src/components/memo.rs
line_hint: 63
author: 
created: 
title: "scroll_bars_of"
tags: []
signature: "fn scroll_bars_of (p : & mut Brush , c : & MakepadCtl , lines : & [ContentLine] , frame : Rect , overflow_y : bool ,) -> Bars"
---

The scroll bars of this `Memo` — **and the state of each one** (issue #122).

The `Memo` is the only control in this adapter whose scroll bars **the
developer** turns on, through the document's `ScrollBars`, and it is
therefore the only one whose bar stays on screen with nothing to scroll:
greyed out, no sliding block. Deciding that is [`bars_from_model`]'s job;
what this function adds is the measurement only the `Memo` can make —
**whether any line runs past the window in width**.

It measures sparingly, and that matters: `Brush::measure` is one text layout
per line, and the canvas redraws on every pointer move (the ruler mark
follows the hand). So the width is only measured when it can change
something — the model is asked assuming it overflows and, if not even then
is there a horizontal bar (`ssNone`, `ssVertical`, or a silent document),
there is nothing to measure. With the default `ScrollBars`, the cost is
zero.
