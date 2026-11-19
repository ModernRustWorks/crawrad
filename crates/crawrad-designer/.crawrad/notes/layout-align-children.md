---
id: layout-align-children
kind: doc
target_file: src/layout.rs
line_hint: 193
author: 
created: 
title: "align_children"
tags: []
signature: "fn align_children (pass : & mut Pass , parent : & str)"
---

Cascades alignment for children of a single parent.

The cascade of a single parent: `alTop`, `alBottom`, `alLeft`, `alRight` and
finally `alClient`, each consuming a slice of what remains.

The order is fixed and is that of the classic IDE — top and bottom take the entire width,
and the sides only receive the height that remained. It is the biggest *catch* of `Align`
there, and it is the behaviour that someone coming from the classic IDE expects.

**Margins enter here, and only here.** Each child is inset by its own
margins within the slice allotted to it, and the consumed slice **includes** the
margins — which is why two `alTop` of margin 10 end up with a gap of 20:
margin is measured against the container area, and that area shrinks with each
aligned child. Applying margin to the rectangle and forgetting the accumulator produces
two abutting controls with margins drawn on top of each other;
applying it only in the axis of `Align` leaves the width without any indent. These are two
different defects and neither appears looking at a single control, which
is why tests check BOTH controls in BOTH axes.
