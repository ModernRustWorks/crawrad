---
id: workspace-new-args
kind: doc
target_file: src/workspace.rs
line_hint: 1522
author: 
created: 
title: "new_args"
tags: []
signature: "pub fn new_args (name : & str , project_type : ProjectType) -> Vec < String >"
---

Arguments for `cargo new` that create the project inside the chosen directory.

`new` and not `init`: the project is created in a **subfolder** with the name the
user typed ([ADR 0017], Amendment 3). That is what `cargo new` does in terminal,
and what the owner asked for — *"in Rust, when creating, a project name is given,
and it creates a folder with that name"*.

**`--name` always**, not just when the name looks suspect: it is the only way
to assert the package name rather than letting cargo deduce it from the path.
With the typed name both coincide — and continuing to pass `--name` maintains
the promise that the typed name is the name that enters `Cargo.toml`.

**`--vcs none` LEFT**, and its reason expired with the group. It existed
because *the repository and `.gitignore` belonged to the group*, written by us
at the root; without a group there is no root of ours, and the alternative to
letting cargo decide would be reimplementing its decision — the opposite of [ADR 0018].

Measured on 08/09/2026: `cargo new` in a standalone folder writes `.git` **and**
a `.gitignore` with `/target`, exactly what `root_files` wrote by hand. Inside an
existing repository it also wrote a nested `.git` — and that is cargo's decision,
not ours, just as it would be in the terminal.

One function, not three: this is where ADR 0018 types distinguish themselves, and
having this in one place prevents a bugfix from updating one and missing the others.


# Examples

```rust
use crawrad_project::workspace::{new_args, ProjectType};

let args = new_args("demo", ProjectType::Gui);
assert_eq!(args[0], "new");
assert_eq!(args[1], "demo");
```
