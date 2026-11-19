---
id: mask-editmask-fill
kind: doc
target_file: src/mask.rs
line_hint: 200
author: 
created: 
title: "EditMask::fill"
tags: []
signature: "fn fill (& self , typed : & str) -> Vec < Option < char > >"
---

The typed characters each input position takes, in order.

The typed characters each input position takes, in order: a rejected
character is dropped, and the next one is tried in the same position.
