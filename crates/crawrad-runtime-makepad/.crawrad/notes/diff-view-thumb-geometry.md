---
id: diff-view-thumb-geometry
kind: doc
target_file: src/diff_view.rs
line_hint: 271
author: 
created: 
title: "thumb_geometry"
tags: []
signature: "fn thumb_geometry (inner : f64 , visible : usize , longest : usize , offset : usize) -> Option < (f64 , f64) >"
---

Calculates the thumb position and width in a track.

The thumb's place and width in a track `inner` wide, or `None` when the
whole side already fits.
