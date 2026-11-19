---
id: build-busy-card-2
kind: doc
target_file: src/build_busy.rs
line_hint: 51
author: 
created: 
title: "card"
tags: []
signature: "pub fn card (progress : & BuildProgress , since_start : Duration , since_end : Option < Duration > ,) -> Option < Card >"
---

Calculates the corner card content for the build window.

None = hidden. Running → spinner icon, headline, counter (or "" ), clock.
Finished → "✓" (success) / "✕" (failed or cancelled) for CARD_LINGER
(4 s) after the end, then None. `since_end` = time since it finished.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_busy::card;
use crawrad_project::build_progress::BuildProgress;

let progress = BuildProgress::new("Building", "my_pkg", "cargo build");
let current_card = card(&progress, Duration::from_millis(100), None);
assert!(current_card.is_some());
```
