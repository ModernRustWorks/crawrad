---
id: check-align
kind: doc
target_file: src/check.rs
line_hint: 11
author: 
created: 
title: "align"
tags: []
signature: "pub fn align (cx : & mut Cx , lead : & LabelRef , check : & CheckBoxRef , text : & str , alignment : & str)"
---

Aligns caption and check box based on alignment (#445).

**The caption and its side** (#445). The check box is a row: a `lead`
label and the `check` itself. With the caption at the left
(`taLeftJustify`) the label carries it and fills the row, so the box sits
at the right edge; otherwise the box carries it, as Makepad draws it. Acts
only on a change: the sync runs on every pass.

# Examples

```rust,no_run
// Makepad Cx and widget refs require the Makepad application runtime.
use makepad_widgets::*;
use crawrad_runtime_makepad::check::align;

fn example(cx: &mut Cx, lead: &LabelRef, check: &CheckBoxRef) {
    align(cx, lead, check, "Option", "taLeftJustify");
}
```
