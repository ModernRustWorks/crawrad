---
id: components-scroll-box-desenho-draw-over
kind: doc
target_file: src/components/scroll_box.rs
line_hint: 32
author: 
created: 
title: "Desenho::draw_over"
tags: []
signature: "fn draw_over (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , scene : & Scene)"
---

**The bars**, over its children.

What identifies it as a `ScrollBox` is the SUNKEN area of [`Self::draw`] —
the bar says something else: **that there is content beyond the window.** The two
statements were conflated into one here, and that was what led the designer to show
a bar where the application does not show one.

They continue to come from the MODEL first — the `HorzScrollBar` and
`VertScrollBar` of the catalog (`crawrad-registry/src/componentes/scroll_box.rs`),
which are born enabled —, and only then from the content.

The bars stay ON TOP of the area, and are not subtracted from it: the children of
`ScrollBox` are positioned by the document and drawing cannot change
where they land (ADR 0012). It is also what the framework does —
`ScrollBars::begin` reserves no trough at all
(`makepad-widgets-1.0.0/src/scroll_bars.rs:245`) —, and following the framework
is what keeps the designer and the generated application saying the same thing.
