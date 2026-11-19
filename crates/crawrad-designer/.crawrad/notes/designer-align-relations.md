---
id: designer-align-relations
kind: doc
target_file: src/lib.rs
line_hint: 1042
author: 
created: 
title: "Designer::align_relations"
tags: []
signature: "fn align_relations (& self , doc : & Document , reg : Option < & Registry > , moved_id : Option < & NodeId > , r : Rect , other : & NodeId , o : Rect , ext : GuideSpan ,) -> Vec < (f32 , Guide) >"
---

Computes all alignment relations between two rectangles.

All alignment relations between two rectangles, with the distance
remaining for each to hold.

This is the **only** implementation of the rule: drag and Ctrl call this
function and only change what they do with the result — drag chooses the closest
per axis and snaps; Ctrl keeps those already exact. Previously
they were two similar blocks, and similar is how rules diverge.
