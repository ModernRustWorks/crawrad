---
id: components-declarado-fracao
kind: doc
target_file: src/components/declarado.rs
line_hint: 158
author: 
created: 
title: "fracao"
tags: []
signature: "fn fracao (v : f32 , min : f32 , max : f32) -> f32"
---

**The fraction a value represents within the range** — not computed here.

It lives in [`crawrad_component::fraction`] — **the crate**. It is the same file that
`crawrad-codegen` splices into the generated application, and the same one reached by whoever
writes a component via `cargo add crawrad-component`.

It lived here, private, for one day. The price was written at the doorstep of the
file itself: declared drawing worked on the canvas and did not cross over
to the target, and the very first thing the target needed was this calculation. Two
copies of it would mean a gauge reading 65% in the designer and something else in
the application — the family of defects this house has been tracking since `LIST_ROW_HEIGHT`.
