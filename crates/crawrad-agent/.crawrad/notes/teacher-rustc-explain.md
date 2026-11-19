---
id: teacher-rustc-explain
kind: doc
target_file: src/teacher.rs
line_hint: 58
author: 
created: 
title: "rustc_explain"
tags: []
signature: "pub fn rustc_explain (code : & str) -> Option < String >"
---

Runs `rustc --explain <code>` and returns trimmed output.

Runs `rustc --explain <code>` and returns trimmed output truncated to 4000 bytes.
Refuses (returns `None`) any code that is not `E` + 4 ASCII digits.

# Examples

```rust,no_run
// Spawns external rustc process.
use crawrad_agent::teacher::rustc_explain;

let explanation = rustc_explain("E0382");
assert!(explanation.is_some());
```
