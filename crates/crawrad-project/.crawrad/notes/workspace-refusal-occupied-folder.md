---
id: workspace-refusal-occupied-folder
kind: doc
target_file: src/workspace.rs
line_hint: 1707
author: 
created: 
title: "refusal_occupied_folder"
tags: []
signature: "pub fn refusal_occupied_folder (parent : & Path , name : & str) -> Option < Refusal >"
---

Checks if an existing folder already occupies the chosen project name.

The **project name that an existing folder already occupies**, and the phrase
that says so before `cargo` runs.

`None` when the path is free — and creation proceeds.

Exists because `cargo new`'s refusal arrives late and speaks of what it knows
(the destination), not of what the user did (chose a name). With the typed name
([ADR 0017], Amendment 3) the phrase explains itself: *there is already a folder
named `teste` here*. And it arrives **before** any writing of ours, keeping
`Nothing was written` true without needing any rollback.

`open_instead` only when there is a `Cargo.toml` inside: offering `Open` on a
folder that happened to have that name would lead the person to a place without any project.


# Examples

```rust
use std::path::Path;
use crawrad_project::workspace::refusal_occupied_folder;

let r = refusal_occupied_folder(Path::new("non_existent_dir_xyz"), "demo");
assert!(r.is_none());
```
