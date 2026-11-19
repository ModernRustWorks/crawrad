---
id: arrownudge
kind: doc
target_file: src/lib.rs
line_hint: 140
author: 
created: 
title: "ArrowNudge"
tags: []
signature: "pub enum ArrowNudge"
---

What a keyboard arrow does with the selection.

**What a keyboard arrow does with the selection**, and it is the map of the classic IDE.

# Why this is a type, and not an `f32` calculated in the shell

Because the number depends on the GRID, and the grid belongs to this crate. With the step
resolved in each shell, changing `Grid size` in the options would change the arrow in
one IDE and not the other — and the discrepancy would appear as *"here it moves 8, there it
moves 1"*, far from where it was decided. The shell translates the chord to one
of these four and asks [`Designer::arrow_step`] for the step (ADR 0027).

# The chord of each one, stated by the owner on 2026-09-17

- plain arrow → moves one grid step;
- `Shift+arrow` → **resizes**, width horizontally and height
  vertically;
- `Ctrl+Shift+arrow` → moves fast, three steps.

`Ctrl+arrow` alone — the FINE adjustment, of one unit — was not in the statement
and enters out of necessity: with the plain arrow moving from grid to grid, without
it **there is no way to land off-grid via the keyboard**. That is what the classic IDE does
with the same key.

# Examples

```rust
use crawrad_designer::ArrowNudge;

assert_eq!(ArrowNudge::Move, ArrowNudge::Move);
```
