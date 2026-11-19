---
id: workspace-executables
kind: doc
target_file: src/workspace.rs
line_hint: 1476
author: 
created: 
title: "executables"
tags: []
signature: "pub fn executables (json : & str) -> Result < std :: collections :: BTreeMap < String , bool > , String >"
---

Each package of the group, and whether it produces an **executable**.

Pure over the text, like `crate::diagnostics_model::analyze`, and that is why
it exists separated from [`group_executables`]: testing the format does not
need a subprocess, and testing the subprocess does not need a hand-written JSON.

**Map, not set of those that run**, and the difference is a decision: with a
set, a missing name could be *"this is the library"* or *"I don't know who this package is"*,
and the shell could not tell. It needs to: [`crate::generator::package_name_in`]
falls back to a fallback `"app"` when it cannot read the member's manifest, and
disabling `Run` because of a fallback name would be the IDE inventing a verdict
on a project it could not identify.

`bin` and not `crate_types`: what `cargo run` requires is a **binary target**,
and that is the exact word it uses when refusing (*"a bin target must be available"*).
A `cdylib` produces a `.dll` and does not run; an `rlib` does not either.

# Examples

```rust
use crawrad_project::workspace::executables;

let json = r#"{"packages": []}"#;
assert_eq!(executables(json).unwrap().len(), 0);
```

# Errors

Returns an error when `json` cannot be parsed as cargo metadata.
