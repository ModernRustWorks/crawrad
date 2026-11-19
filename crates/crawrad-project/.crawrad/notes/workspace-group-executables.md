---
id: workspace-group-executables
kind: doc
target_file: src/workspace.rs
line_hint: 1488
author: 
created: 
title: "group_executables"
tags: []
signature: "pub fn group_executables (root : & Path) -> Result < std :: collections :: BTreeMap < String , bool > , String >"
---

Asks `cargo` who, in the group, has an executable target.

**Asks `cargo`, and not our own manifest reader** ([ADR 0018]). Only cargo
sees all three ways a binary target can arise — declared `[[bin]]`, discovered
`src/main.rs`, and each `src/bin/*.rs` — and this file already has two `Cargo.toml`
scanners ([`workspace_list`] and [`has_package`]); a third, for a question
`cargo` answers for free, would be the reimplementation the ADR condemns.

**`--no-deps`**, and that is what makes the call cheap: without it `cargo`
resolves the entire tree and may hit the network. Measured on 01/09/2026, with
`cargo 1.95`: **82 to 92 ms**, and the number **does not change** with group
size — 85 ms in a single-member group, 88 ms in the ten-crate workspace of
this repository. It is the cost of launching the process, not reading the project.

It is this number that decides **when** to ask, not author preference: 85 ms
vanishes inside a `Run`, which takes seconds — but who asks the question is
the unlit toolbar button, and it asks **every frame**. Sixty times per second,
85 ms is impossible. Therefore the answer is cached by the shell.


# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace::group_executables;

// Avoid spawning cargo processes in doctests.
let _ = group_executables(Path::new("."));
```

# Errors

Returns an error when executing `cargo metadata` fails or returns a non-zero exit code.
