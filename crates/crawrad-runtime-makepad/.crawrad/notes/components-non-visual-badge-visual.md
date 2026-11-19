---
id: components-non-visual-badge-visual
kind: doc
target_file: src/components/non_visual.rs
line_hint: 106
author: 
created: 
title: "Badge::visual"
tags: []
signature: "fn visual (& self) -> bool"
---

Returns false because non-visual badges do not appear in the live app.

**No.** This is where the answer to `crate::e_visual_no_canvas` comes
from, and by extension `crawrad_runtime::na_superficie`: whatever lives
inside a non-visual does not go to the screen.
