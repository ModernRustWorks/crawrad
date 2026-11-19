---
id: creation-creation-settle
kind: doc
target_file: src/creation.rs
line_hint: 118
author: 
created: 
title: "Creation::settle"
tags: []
signature: "pub fn settle (& mut self , outcome : Outcome) -> bool"
---

Settles creation with the final verdict and returns whether to keep state.

The verdict arrived. Returns **`false` when this state is no longer useful
for anything** — and then the caller discards `creating` entirely.

Method here, and not an `if` inside `Ide::settle_creation`, because
`Ide` cannot be constructed in tests while `Creation` can. It was precisely
because the decision lived in `Ide` that the 08/09/2026 revision managed to
apply the missing fix **without a single test changing color**.

Two reasons to discard, and the second is the bug it measured:

1. **finished green** — the project is already on screen, and the dialog closes along with it;
2. **the dialog is hidden.** `Hide`, `Esc` and the X set `show` to `false`,
   and it **never returns to `true`** — dialog rendering returns on the first line
   when `!show`. Keeping the outcome here would keep it for no one, and a lingering
   `creating` makes every subsequent `New Project…` respond *"already creating a project —
   wait for it to finish"* regarding a job that already ended, until restarting the IDE.

The phrase is not lost in case 2: it goes to the status bar, which is
precisely what remains for whoever dismissed the window.

# Examples

```rust
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Instant;
use crawrad_project::creation::{Creation, Outcome};
use crawrad_project::workspace::ProjectType;

let (_tx, rx) = channel();
let mut creation = Creation {
    rx,
    project_type: ProjectType::Gui,
    root: PathBuf::from("."),
    plan: Vec::new(),
    completed: 0,
    current: None,
    since: Instant::now(),
    outcome: None,
    show: true,
};
assert!(!creation.settle(Outcome::Opened(PathBuf::from("."))));
```
