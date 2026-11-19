---
id: runtime-set-zoom
kind: doc
target_file: src/lib.rs
line_hint: 202
author: 
created: 
title: "Runtime::set_zoom"
tags: []
signature: "fn set_zoom (& mut self , zoom : f32)"
---

Canvas zoom: how many screen points one design unit is worth.

The value is held by the IDE — it is state of what is being edited, not of the
form: `.crawform` and `.designer.rs` come out identical at any
zoom (ADR 0019).
What **uses** it is the adapter, inside its transformation, and no
caller of the conversion learns that it exists.

Default with no effect, like `set_grid` beside it: an adapter without an artboard
has nothing to zoom, and forcing all three to implement this would invite
dead code in two of them.
