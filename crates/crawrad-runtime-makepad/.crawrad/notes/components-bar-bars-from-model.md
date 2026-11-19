---
id: components-bar-bars-from-model
kind: doc
target_file: src/components/bar.rs
line_hint: 225
author: 
created: 
title: "bars_from_model"
tags: []
signature: "pub fn bars_from_model (scroll_bars : & str , overflow_y : bool , overflow_x : bool) -> Bars"
---

What the MODEL says about this container's bars.

`scroll_bars` is the document property (`ssNone`, `ssHorizontal`,
`ssVertical`, `ssBoth`), read just like the RXP does
(`crawrad-runtime-egui/src/lib.rs:655`). When the document says nothing — which
is the case of the `ListBox` and of the `TreeView`, whose catalogue has no
such property — the live RXP's `VisibleWhenNeeded`
(`crawrad-runtime-egui/src/lib.rs:1325`) holds: the bar shows up when the
content **overflows**, and that is when it tells you something.

**This function answers two questions, not one** (issue #122): *whether* the
bar is drawn and *how*. Whoever has the property written in the document is
in the developer's regime ([`ScrollBarState::asked_for`]) and the bar never
goes away — when everything fits, it goes grey. Whoever does not have it is
in the automatic regime ([`ScrollBarState::when_needed`]) and the bar simply
does not appear.

`overflow_y` and `overflow_x` say whether the content runs past the window on
each axis. They belong to the CALLER because the caller is the one who knows
the content: how many lines there are, how wide the widest one measures.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::{bars_from_model, ScrollBarState};

let bars = bars_from_model("ssBoth", true, false);
assert_eq!(bars.vertical, ScrollBarState::Active);
assert_eq!(bars.horizontal, ScrollBarState::Disabled);
```
