---
id: designer-align-to-neighbors
kind: doc
target_file: src/lib.rs
line_hint: 1202
author: 
created: 
title: "Designer::align_to_neighbors"
tags: []
signature: "fn align_to_neighbors (& self , doc : & Document , reg : Option < & Registry > , moved : & [NodeId] , r : Rect ,) -> (f32 , f32 , Vec < Guide >)"
---

Finds alignment with neighbors and returns adjustment plus guidelines.

Looks for alignment with neighbors and returns adjustment to apply plus
guidelines to draw.

Six relations per axis, the same as the classic IDE: edge to edge
(left-left, right-right), center to center, and edge to opposite edge —
which is abutting one component to another.
