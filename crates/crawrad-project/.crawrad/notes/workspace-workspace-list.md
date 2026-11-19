---
id: workspace-workspace-list
kind: doc
target_file: src/workspace.rs
line_hint: 76
author: 
created: 
title: "workspace_list"
tags: []
signature: "pub fn workspace_list (cargo_toml : & str , key : & str) -> Vec < String >"
---

The list of a key under `[workspace]` — `members` or `default-members`.

A twenty-line parser instead of the `toml` crate, for the same reason as
`package_name`: two keys, in one section, always in the same shape.
Accepts the list on a single line and broken across multiple lines, which
are the two forms that `cargo` writes and that a person writes by hand.

After the prefix there must come an `=`, and that line is load-bearing:
without it `members` would match **any key starting with `members`** —
a future TOML's `members-of`, or a key cargo invents — and its values would
enter our list. Not the case for `default-members`, which starts with another
word and would never match anyway.

# Examples

```rust
use crawrad_project::workspace::workspace_list;

let toml = "[workspace]\nmembers = [\"app\"]\n";
assert_eq!(workspace_list(toml, "members"), vec!["app"]);
```
