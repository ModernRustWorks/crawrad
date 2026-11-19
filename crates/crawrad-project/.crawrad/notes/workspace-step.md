---
id: workspace-step
kind: doc
target_file: src/workspace.rs
line_hint: 1410
author: 
created: 
title: "Step"
tags: []
signature: "pub enum Step"
---

A step in project creation, and what the progress dialog shows.

Exists because creating a project **takes time**, and what takes time is a single
step: measured on this machine, `cargo new` costs 151 ms and `cargo add eframe@0.36`
costs 6.8 s — 97% of the time. An indeterminate progress bar would say "wait" for
seven seconds without saying for what; naming the step, the user knows the
delay is the network, not a frozen IDE.

**The list is a single fact with two readers** — the dialog, which draws it, and
`create_project`, which steps through it. Both come from [`steps_for`], and that
prevents the drawing from promising a step that never runs (or hiding one that runs):
the exact flaw ADR 0011 identified when a shortcut lived in a label and `match`
simultaneously.

**There used to be five, now three** ([ADR 0017], Amendment 3). The group root and
`git init` were removed because group creation performed them, and creation no
longer creates groups: today `cargo new` writes `.git` and `.gitignore` on its
own, verified. Two steps the IDE no longer executes have no reason to appear
in a list promising what is about to happen.


# Examples

```rust
use crawrad_project::workspace::{Step, ProjectType};

assert!(Step::Package.label(ProjectType::Gui).contains("cargo new"));
```
