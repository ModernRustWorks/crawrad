---
id: canvas-inherited-badge-rect
kind: doc
target_file: src/canvas.rs
line_hint: 1
author:
created:
title: "inherited_badge_rect"
tags: []
signature: "pub (crate) fn inherited_badge_rect (r : Rect , scale : f64) -> Rect"
---

Where the inherited badge of a component goes (#851).

A square in the top-right corner, INSIDE the component, so the container that
clips the component clips the badge too. It never grows past the component.
