---
id: components-declarado-pintar
kind: doc
target_file: src/components/declarado.rs
line_hint: 67
author: 
created: 
title: "pintar"
tags: []
signature: "pub fn pintar (p : & mut Brush , c : & MakepadCtl , r : Rect , verbos : & [Verb]) -> bool"
---

Paints the declared drawing of a class.

**Paints the declared drawing of a class.**

Returns `false` when the class declares no drawing at all — in which case the caller
proceeds to the usual dashed outline, which remains the right answer for
*"this class did not declare how to paint itself"*.

# Examples

```rust,no_run
// MakepadCtl and Brush require the Makepad canvas runtime.
use makepad_widgets::Rect;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::canvas::Brush;
use crawrad_runtime_makepad::components::declarado::pintar;

fn example(brush: &mut Brush, ctl: &MakepadCtl, rect: Rect) {
    let painted = pintar(brush, ctl, rect, &[]);
    assert!(!painted);
}
```
