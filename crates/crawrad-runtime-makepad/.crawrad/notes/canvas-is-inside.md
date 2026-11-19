---
id: canvas-is-inside
kind: doc
target_file: src/canvas.rs
line_hint: 626
author: 
created: 
title: "is_inside"
tags: []
signature: "fn is_inside (ctls : & std :: collections :: BTreeMap < NodeId , MakepadCtl > , node : & NodeId , ancestor : & NodeId ,) -> bool"
---

Checks whether `node` is a descendant of `ancestor` in the parent chain.

Is `node` a descendant of `ancestor`, walking the parent chain of the model?

Bounded on purpose: the document comes from a file, and a cycle written by
hand must not turn into a hung frame.
