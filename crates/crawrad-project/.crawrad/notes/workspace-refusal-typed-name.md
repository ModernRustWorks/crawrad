---
id: workspace-refusal-typed-name
kind: doc
target_file: src/workspace.rs
line_hint: 1533
author: 
created: 
title: "refusal_typed_name"
tags: []
signature: "pub fn refusal_typed_name (name : & str) -> Option < String >"
---

Checks if the typed name is acceptable as a package name.

Is the typed name acceptable as a package name? `Some` is the phrase explaining
what is wrong, **in English**.

It exists because the name stopped being deduced from the folder and became
**typed** ([ADR 0017], Amendment 3). Previously [`suggested_name`] sanitized
the name and nothing could go wrong; now it can, and the place to say so is
the dialog — refusing immediately with the reason is better than letting
`cargo` fail seven seconds later.

**What it DOES NOT do is replicate cargo's word list.** `con`, `test`, `build`,
`deps`, Rust keywords — that list belongs to cargo, changes with toolchains, and
a copy here would rot in silence as the `Cargo.toml` template rotted (ADR 0018).
What is measured here is **form**, which is stable and documented: a package name
is a sequence of ASCII letters, digits, `_` and `-`, and does not start with a digit.
Whoever refuses `con` remains cargo, and its refusal turns into a phrase via [`refusal_new`].


# Examples

```rust
use crawrad_project::workspace::refusal_typed_name;

assert!(refusal_typed_name("").is_some());
assert!(refusal_typed_name("valid_name").is_none());
```
