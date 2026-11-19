---
id: components-non-visual-badge-clips-to-own-box
kind: doc
target_file: src/components/non_visual.rs
line_hint: 111
author: 
created: 
title: "Badge::clips_to_own_box"
tags: []
signature: "fn clips_to_own_box (& self) -> bool"
---

Returns false as the non-visual badge writes its name below its rectangle.

**No — and this is the one exception in the whole catalog.**

The name of the instance is written **below** the badge ([`name_box`]),
which is how a classic RAD IDE shows a `TActionList`: icon above, name under it.
Clipping this badge to its own rectangle would erase the name of every
`ActionList`, `ImageList`, `Timer` and `Action` on the form — and the
name is the only thing that tells two identical badges apart.

The badge is not content overflowing a control. It is a **designer
overlay**: it does not exist in the application, it has no client area
and it has no box to respect. The law of issue #129 is about a control's
content leaving the control's box, and a badge has neither.
