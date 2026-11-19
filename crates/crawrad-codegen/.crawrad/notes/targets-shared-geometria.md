---
id: targets-shared-geometria
kind: doc
target_file: src/targets/shared.rs
line_hint: 35
author: 
created: 
title: "GEOMETRIA"
tags: []
signature: "pub const GEOMETRIA : & str = include_str ! (\"../../../crawrad-component/src/fraction.rs\")"
---

The shared arithmetic, as text.

**THE SHARED ARITHMETIC**, as text — and it comes FROM THE BOX.

`fraction` lived in `shared_geometry.rs` of the Makepad runtime for a day. It
moved to `crawrad-component` — the box — when the owner asked *"where is
our box for people to create components?"*: the one who most needs this
calculation is the one who WRITES a component, and that person reaches only what the box
offers.

It cannot be resolved during generation like other measurements: it answers
*how full it is RIGHT NOW*, and right now only exists while the user's
application runs. That is why it crosses as TEXT, through the same section snippet as
`desenho_comum.rs`.

**Four readers, one source:** the designer canvas (calls),
`crawrad-codegen` (emits this text), the generated application (receives), and whoever
writes a component (`cargo add crawrad-component`).

# Examples

```rust
use crawrad_codegen::targets::shared::GEOMETRIA;

assert!(GEOMETRIA.contains("fraction"));
```
