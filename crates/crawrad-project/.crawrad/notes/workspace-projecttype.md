---
id: workspace-projecttype
kind: doc
target_file: src/workspace.rs
line_hint: 1344
author: 
created: 
title: "ProjectType"
tags: []
signature: "pub enum ProjectType"
---

Member types offered by `New Project…` ([ADR 0018], Amendment 1).

They are not "new project types": they are **things added to the group**
(ADR 0017, Amendment 1). `New Project` creates the group with the first member;
from then on it is the same choice, with the group already existing.

**Three, and they differ by a flag and a dependency.** That is why they are an
`enum` that parameterizes ONE code path, and not three similar functions:
three paths for the same sequence (`cargo new`, root `[workspace]`, opening folder)
diverge on the first fix only one of them receives. It already happened here,
with the `RadioButton` exclusion written in two places.

The bar for growth comes from the ADR and is narrow: a type enters if it changes
only **how `cargo` is invoked**. Web and services change what the *generator*
emits, which is another target with its own ADR (ADR 0004).

**The DLL left this phrase**, and the history is worth noting because both versions
were half right. Amendment 1 of ADR 0018 removed the dynamic library from the
list of expensive items — *"it is a single manifest line"* —, and that is true
from `Cargo.toml`'s side. But measured on 01/09/2026: `cargo new --lib` writes
`pub fn add(left, right)`, and **that is not exported from a `cdylib`**. A
"Dynamic Library" made only with the flag and manifest line delivers a `.dll`
that the classic RAD IDE **cannot call** — whoever tries sees the file in `target/`,
writes `external`, and gets "entry point not found".

That is: `cdylib` and `staticlib` cost the manifest line **plus the smallest
generator in the product** — a `src/lib.rs` with an example `#[unsafe(no_mangle)]
pub extern "C"` function. And writing `src/*.rs` is literally the definition of
generator in ADR 0018. They enter, but not in the cheap batch.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::Gui.flag(), "--bin");
```
