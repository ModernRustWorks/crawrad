---
id: workspace-adopting-workspace
kind: doc
target_file: src/workspace.rs
line_hint: 1777
author: 
created: 
title: "adopting_workspace"
tags: []
signature: "pub fn adopting_workspace (cargo_output : & str) -> Option < String >"
---

Checks if `cargo new` added the package to an existing workspace.

Did `cargo new` just **edit someone else's manifest**? `Some` is the path
of the workspace that adopted the package.

Measured on 08/09/2026, twice, one of them in this repository's own `Cargo.toml`:
running `cargo new` inside a folder that already lives in a workspace, cargo
adds the package to `members` there and prints to terminal:

```text
      Adding `teste` as member of workspace at `E:\...\CrawRAD`
```

With groups, that did not happen: the ordering *group root before `cargo new`*
(ADR 0018, Amendment 2) made cargo find our root. Without groups
([ADR 0017], Amendment 3), it happens — and **we cannot prevent it without
reimplementing cargo's decision**, the opposite of ADR 0018. What we can do is
not stay silent: creation returns this as a warning, and the dialog remains on screen with it.

Reads cargo's output line instead of searching for workspaces up the tree,
for the usual reason: cargo decides, and deriving the answer on our own would be
our idea of its rule — exactly the mistake Amendment 2 of ADR 0018 named.

`None` when the line does not appear, which is the common case: standalone folder,
no workspace above.

**Color is stripped before inspecting the line** ([`without_color`], issue #63).
[`crate::creation::cargo_with_output`] already asks cargo not to colorize, and this
is the second lock: reading tool output cannot depend on callers remembering to turn off color.


# Examples

```rust
use crawrad_project::workspace::adopting_workspace;

let out = "      Adding `app` as member of workspace at `/root`";
assert_eq!(adopting_workspace(out), Some("/root".to_string()));
```
