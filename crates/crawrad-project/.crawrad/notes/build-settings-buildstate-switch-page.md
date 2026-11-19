---
id: build-settings-buildstate-switch-page
kind: doc
target_file: src/build_settings.rs
line_hint: 276
author: 
created: 
title: "BuildState::switch_page"
tags: []
signature: "pub fn switch_page (& mut self , page : BuildPage)"
---

Sets the active page and records the timestamp of the switch for animation.

# Examples

```rust
use crawrad_project::build_settings::{BuildPage, BuildState};

let mut state = BuildState::default();
state.switch_page(BuildPage::Options);
assert_eq!(state.page, BuildPage::Options);
```
