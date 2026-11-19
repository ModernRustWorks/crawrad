---
id: designer-ruler-max-ticks
kind: doc
target_file: src/designer/ruler.rs
line_hint: 58
author: 
created: 
title: "MAX_TICKS"
tags: []
signature: "const MAX_TICKS : usize = 8_000"
---

How many ticks the ruler draws at most, under any circumstance.

A 4000-point range with the minor step at the floor of 5 gives 800 ticks;
the ceiling is deliberately generous and exists only so that no combination
of zoom and window size turns into a long loop inside a frame.
