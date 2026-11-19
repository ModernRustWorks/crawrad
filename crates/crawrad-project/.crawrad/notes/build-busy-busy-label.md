---
id: build-busy-busy-label
kind: doc
target_file: src/build_busy.rs
line_hint: 19
author: 
created: 
title: "busy_label"
tags: []
signature: "pub fn busy_label (mode : BuildMode , timing : & BusyTiming , since_start : Duration , ended : Option < Duration > ,) -> Option < String >"
---

Returns the rail or toolbar label of a running command.

The rail/toolbar label of a command while it runs: "◐ Building" etc.,
or None when the BusyTiming says the indicator is not visible yet
(opacity == 0) — then the button shows its normal text.

# Examples

```rust
use std::time::Duration;
use crawrad_component::pieces::BusyTiming;
use crawrad_project::build_busy::busy_label;
use crawrad_project::build_settings::BuildMode;

let timing = BusyTiming::default();
assert_eq!(
    busy_label(BuildMode::Build, &timing, Duration::from_millis(150), None),
    Some("◓ Building".to_string())
);
```
