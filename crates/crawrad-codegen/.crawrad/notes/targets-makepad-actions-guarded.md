---
id: targets-makepad-actions-guarded
kind: doc
target_file: src/targets/makepad/actions.rs
line_hint: 32
author: 
created: 
title: "guarded"
tags: []
signature: "pub fn guarded (guard : & str , block : & str) -> String"
---

`block` run only while `guard` holds.

A block that is one `if` takes the guard into its condition: an `if` alone
inside another is `clippy::collapsible_if` in the user's project. An
`if let` joins it as a let chain (`guard && let Some(v) = …`), which the
2024 edition the IDE creates projects in accepts, and which clippy asks
for (#667).

# Examples

```rust
use crawrad_codegen::targets::makepad::actions::guarded;

let g = guarded("self.enabled", "        do_something();\n");
assert!(g.contains("self.enabled"));
```
