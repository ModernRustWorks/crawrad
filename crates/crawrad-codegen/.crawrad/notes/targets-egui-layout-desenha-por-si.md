---
id: targets-egui-layout-desenha-por-si
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 33
author: 
created: 
title: "desenha_por_si"
tags: []
signature: "pub fn desenha_por_si (reg : & crawrad_registry :: Registry , class : & str) -> bool"
---

The class draws itself in `show()` (receives a rectangle).

# The question goes to the CATALOG, not to a list written here

It used to be `e_visual(class) && class != "TabSheet"` — two lists of built-in
names, and both blind to anything coming from the outside. A `Connection` installed
via `manifest.json` is non-visual and fell into the visual branch: the generator would write
`let r = anchor(…)` for a component that has no rectangle, and the user's project
opened with `unused variable: r` — a warning that they cannot fix,
because `.designer.rs` is rewritten.

The catalog already answers both questions by descriptor: `visual` says whether it has
screen presence, `geometria_derivada` says the rectangle comes from another. `TabSheet`
ceases to be a name cited in code — it is derived like any third-party page
would be (ADR 0035).

A class that the catalog does not know falls back to the old list: it is the right answer
for *"I don't know what this is"*, and it is the path for tests that assemble documents
by hand with invented classes.

# Examples

```rust
use crawrad_codegen::targets::egui::layout::desenha_por_si;
use crawrad_registry::Registry;

let reg = Registry::builtin();
assert!(desenha_por_si(&reg, "Button"));
```
