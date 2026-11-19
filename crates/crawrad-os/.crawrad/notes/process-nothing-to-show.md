---
id: process-nothing-to-show
kind: doc
target_file: src/process.rs
line_hint: 179
author: 
created: 
title: "NOTHING_TO_SHOW"
tags: []
signature: "pub const NOTHING_TO_SHOW : & str = \"build failed, with nothing to show\""
---

What the bar says about a build that failed **and said nothing at all**.

A `const` and not a literal at the call site because there are two callers
now — the egui shell, which shows it only when the diagnostics list came
back empty, and [`after_build`], which is the whole answer for a shell with
no panel to show. Two literals is how a sentence starts drifting.

# Examples

```rust
use crawrad_os::process::NOTHING_TO_SHOW;

assert_eq!(NOTHING_TO_SHOW, "build failed, with nothing to show");
```
