---
id: workspace-refusal-new
kind: doc
target_file: src/workspace.rs
line_hint: 1629
author: 
created: 
title: "refusal_new"
tags: []
signature: "pub fn refusal_new (name : & str , cargo_error : & str) -> Refusal"
---

Explains why `cargo new` refused to create a project.

The phrase that explains why `cargo new` refused — and it had **two**
reasons hidden behind a single one.

Measured live on 08/09/2026: the owner created a project in `Downloads\teste`, the
IDE did not open the new project (the other defect of this batch), he repeated the gesture
in the same folder, and read this:

```text
error: destination `C:\Users\User\Downloads\teste\teste` already exists —
the package name comes from the folder `teste`; rename the folder and try
again. Nothing was written.
```

The phrase speaks of **package name** when what happened was *a project already
exists here*. It instructs to rename the folder, which is the one thing he DID NOT
want to do: he wanted that project, and the way out was opening it.

Today this path is the **fallback**, not the first line of defense: the name is
typed in the dialog ([ADR 0017], Amendment 3), its form is rejected immediately
by `reject_typed_name`, and folder collision is rejected by
[`refusal_occupied_folder`] before `cargo` runs. What remains here is what
only cargo knows: its list of reserved names — `con`, `prn`, `aux`,
`nul`, `com1`–`com9`, `lpt1`–`lpt9`, Rust keywords, `test`, `build` and `deps`.

That list is NOT copied here by design: it belongs to cargo, changes with toolchains,
and a copy of ours would rot in silence just as the `Cargo.toml` template rotted (ADR 0018).


# Examples

```rust
use crawrad_project::workspace::refusal_new;

let r = refusal_new("app", "destination already exists");
assert!(r.open_instead);
```
