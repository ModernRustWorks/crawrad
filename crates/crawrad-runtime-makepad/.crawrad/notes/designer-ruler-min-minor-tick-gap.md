---
id: designer-ruler-min-minor-tick-gap
kind: doc
target_file: src/designer/ruler.rs
line_hint: 61
author: 
created: 
title: "MIN_MINOR_TICK_GAP"
tags: []
signature: "const MIN_MINOR_TICK_GAP : f64 = 5.0"
---

The minor stroke only shows up once it has at least this much screen.

Five strokes across two points become a solid line, which measures nothing
and hides the numbered one — this is the RXP's note (`lib.rs:1985-1987`).
