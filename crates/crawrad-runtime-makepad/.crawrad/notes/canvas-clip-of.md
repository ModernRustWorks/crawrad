---
id: canvas-clip-of
kind: doc
target_file: src/canvas.rs
line_hint: 547
author: 
created: 
title: "clip_of"
tags: []
signature: "pub fn clip_of (c : & MakepadCtl , form : RectIr) -> RectIr"
---

The rectangle that clips a control: **its parent's**.

# The contract is the classic RAD, and the owner measured it

He shrank the form over the components in the classic RAD IDE and what happened
was: the component keeps its size, the parent's edge passes over it, and
what falls outside is simply not painted. A `TListBox` cut in half is still
a whole `TListBox`. Nothing snaps, nothing shrinks — *"a classic RAD IDE overlays part of the comp"*. On Win32 this came for free with `WS_CLIPCHILDREN`; here
the adapter does it.

# Why the form is the fallback

`ControleJanelaBase::recorte` (`crawrad-runtime/src/base.rs:154-157`) is the
parent's rectangle **only when the parent is not the document root** — a
control dropped straight on the form gets `None`. But the form is a
container like any other and it clips exactly the same way, which is what
the shrink-the-form measurement shows. So `None` means *the form*, and the
rule stays a single sentence: a control is clipped by its parent.

Note what this deliberately does NOT do: it stops at one level, as the RXP
does (`crawrad-runtime-egui/src/lib.rs:2330`, `recortar_pelo_pai`). A control
inside a `Panel` that itself overflows a `ScrollBox` is clipped by the
`Panel` and not by the `ScrollBox`. Both runtimes have the same limit on
purpose: two adapters that disagree about clipping would show the owner two
different CrawRADs.

# Examples

```rust,no_run
// MakepadCtl instances are managed by MakepadRuntime.
use crawrad_ir::Rect as RectIr;
use crawrad_runtime_makepad::canvas::clip_of;
use crawrad_runtime_makepad::MakepadCtl;

fn example(c: &MakepadCtl) {
    let clip = clip_of(c, RectIr::new(0.0, 0.0, 100.0, 100.0));
    assert_eq!(clip, c.clip.unwrap_or(RectIr::new(0.0, 0.0, 100.0, 100.0)));
}
```
