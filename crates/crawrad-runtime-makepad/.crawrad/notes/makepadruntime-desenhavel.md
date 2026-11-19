---
id: makepadruntime-desenhavel
kind: doc
target_file: src/lib.rs
line_hint: 341
author: 
created: 
title: "MakepadRuntime::desenhavel"
tags: []
signature: "pub fn desenhavel (& self , id : & str) -> bool"
---

Indicates whether the specified node is being drawn in the current frame.

Is the node being drawn in this frame?

`ctls` holds **everything** the document has — including what lives on a
closed page, because that page's tab needs its caption.
`ordem` is what actually goes to the screen.

This exists because separating the two lists opens a hole, and it was already
opened twice: selection and badges sweep `ctls` and start painting
handles for a component that is not on screen. Eight handles floating over the
open page, with nothing underneath, and `Designer::hit_handle` refusing the
drag — **the interface lying**. The RXP closed the same hole in
`crawrad-runtime-egui/src/lib.rs:1828` (`fn desenhavel`).

The form comes in from outside: it is the surface, was never in
`ordem`, and its three handles always exist.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let rt = MakepadRuntime::new();
assert!(!rt.desenhavel("btn1"));
```
