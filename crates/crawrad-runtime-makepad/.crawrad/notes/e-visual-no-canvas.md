---
id: e-visual-no-canvas
kind: doc
target_file: src/lib.rs
line_hint: 613
author: 
created: 
title: "e_visual_no_canvas"
tags: []
signature: "fn e_visual_no_canvas (class : & str) -> bool"
---

Indicates whether the class represents a visual component drawn on the canvas.

Does the class draw a real component, or is it a non-visual (`ActionList`,
`ImageList`, `Timer`, `Action`)?

The answer belongs to the DRAWING CATALOG, and not to a list here: as long as it was
a `matches!` by class name, it was a hand-written list next to
a catalog that already knew the same thing — and the list said "non-visual"
while dispatch sent the component to the FALLBACK drawing
(`componentes/generico.rs`), which is the dashed border meaning "a file is missing here".

The adapter continues to not know about `crawrad-registry` (it is a `dev-dependency`
in this crate, on purpose — see `Cargo.toml`): what it knows is what it
knows how to DRAW. What prevents the catalog here from lagging behind the registry is
the test `todo_nao_visual_do_registro_e_selo_e_nao_widget`, which compares the
two from inside `#[cfg(test)]`, where the registry exists.
