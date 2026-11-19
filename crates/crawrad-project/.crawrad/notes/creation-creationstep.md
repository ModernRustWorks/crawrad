---
id: creation-creationstep
kind: doc
target_file: src/creation.rs
line_hint: 33
author: 
created: 
title: "CreationStep"
tags: []
signature: "pub enum CreationStep"
---

What the creation thread reports **while** working.

The channel used to be for a single value — the verdict at the end — and that
does not sustain a progress dialog: the two fields that matter (which step, and
how many of how many) only exist if whoever is working speaks during the work.
It is the same structural consequence that [ADR 0023] wrote for the compile
dialog, in the easy case: one-way flow, and nothing to write back.

**`Done` does not live here**, and the separation belongs to the compiler:
[`create_project`] receives a `&dyn Fn(CreationStep)` and therefore **has no way**
to announce an outcome. Whoever decides that it ended is the thread, with the
return value — a "done" step reported by mistake mid-way would close the dialog
with work in progress.


# Examples

```rust
use crawrad_project::creation::CreationStep;

let step = CreationStep::Plan(vec![]);
match step {
    CreationStep::Plan(p) => assert!(p.is_empty()),
    _ => unreachable!(),
}
```
