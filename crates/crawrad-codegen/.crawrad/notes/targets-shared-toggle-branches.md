---
id: targets-shared-toggle-branches
kind: doc
target_file: src/targets/shared.rs
line_hint: 460
author: 
created: 
title: "toggle_branches"
tags: []
signature: "pub fn toggle_branches (cond : & str , when_true : & str , when_false : & str , indent : & str) -> String"
---

Generates branching code for toggle event callbacks.

The two-way dispatch after a toggle: `when_true` runs if `cond` holds,
`when_false` otherwise. Each body is already-indented code (or empty).

Only the branches that carry a call are emitted: with no handler the
result is empty, and with one the `if` stands alone. An empty
`if x {} else {}` fails the generated project's own clippy
(`if_same_then_else`, `needless_else`) — the clean-gate caught it on
the CategoryPanel's `OnCollapsed` / `OnExpanded`.

# Examples

```rust
use crawrad_codegen::targets::shared::toggle_branches;

let code = toggle_branches("flag", "    foo();\n", "", "");
assert!(code.contains("if flag"));
```
