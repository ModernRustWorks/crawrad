---
id: build-settings-refusal-hold
kind: doc
target_file: src/build_settings.rs
line_hint: 169
author: 
created: 
title: "REFUSAL_HOLD"
tags: []
signature: "pub const REFUSAL_HOLD : Duration = Duration :: from_secs (3)"
---

How long a busy refusal message is held on the status bar.

How long that refusal holds the bar: the build clock rewrites the bar every
0.1 s, and the refusal was gone before anyone could read it.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_settings::REFUSAL_HOLD;

assert_eq!(REFUSAL_HOLD, Duration::from_secs(3));
```
