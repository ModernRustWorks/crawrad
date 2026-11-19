---
id: build-busy-should-open-on-finish
kind: doc
target_file: src/build_busy.rs
line_hint: 90
author: 
created: 
title: "should_open_on_finish"
tags: []
signature: "pub fn should_open_on_finish (mode : BuildMode , progress : & BuildProgress , window_open : bool) -> bool"
---

Determines whether the build window should open on finish.

Whether the window must open by itself at the end: a failed (not
cancelled) run that finished while the window was closed.

# Examples

```rust
use crawrad_project::build_busy::should_open_on_finish;
use crawrad_project::build_progress::BuildProgress;
use crawrad_project::build_settings::BuildMode;

let progress = BuildProgress::new("Building", "my_pkg", "cargo build");
assert!(!should_open_on_finish(BuildMode::Build, &progress, true));
// A Run never opens the Build window: its errors go to the Problems panel.
assert!(!should_open_on_finish(BuildMode::Run, &progress, false));
```
