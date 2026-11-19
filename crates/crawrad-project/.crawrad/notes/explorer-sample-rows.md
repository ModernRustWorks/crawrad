---
id: explorer-sample-rows
kind: doc
target_file: src/explorer.rs
line_hint: 473
author: 
created: 
title: "sample_rows"
tags: []
signature: "pub fn sample_rows () -> Vec < Row >"
---

One row of every kind, so the DSL gate can instantiate every template.

**The gate covers exactly what the probe instantiates**, so a row component
added without a line here is a template nobody checks — and an unchecked
template is one that can draw nothing at all, in silence
(`portal_list.rs:601` only warns in the log).

It is also what feeds `every_explorer_icon_exists_in_codicons`: a drawing
the font does not have is an empty box on screen, and nothing complains.
Public, not `#[cfg(test)]`: the shell's gates read it from another crate.

# Examples

```rust
use crawrad_project::explorer::sample_rows;

let samples = sample_rows();
assert!(!samples.is_empty());
```
