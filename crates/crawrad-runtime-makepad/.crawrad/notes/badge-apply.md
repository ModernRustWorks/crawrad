---
id: badge-apply
kind: doc
target_file: src/badge.rs
line_hint: 33
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , badge : & WidgetRef , kind : & str , outline : bool , enabled : bool)"
---

Applies kind colors, outline style, and pill corner radius to a Badge widget.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::*;
use crawrad_runtime_makepad::badge::apply;

fn example(cx: &mut Cx, badge: &WidgetRef) {
    apply(cx, badge, "bkPrimary", false, true);
}
```
