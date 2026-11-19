---
id: process-afterbuild
kind: doc
target_file: src/process.rs
line_hint: 182
author: 
created: 
title: "AfterBuild"
tags: []
signature: "pub enum AfterBuild"
---

What `Run` does once the build has answered.

It exists so that **"a green build launches"** is a fact a headless test can
measure. The Makepad shell reaches the launcher through here, and issue #147
was exactly the absence of such a path: the button dispatched, the effect
came back right, and no line of the shell ever mentioned a child process.

# Examples

```rust
use crawrad_os::process::AfterBuild;

let action = AfterBuild::Launch;
assert_eq!(action, AfterBuild::Launch);
```
