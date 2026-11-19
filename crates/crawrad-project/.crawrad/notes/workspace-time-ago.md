---
id: workspace-time-ago
kind: doc
target_file: src/workspace.rs
line_hint: 438
author: 
created: 
title: "time_ago"
tags: []
signature: "pub fn time_ago (now : std :: time :: SystemTime , then : std :: time :: SystemTime) -> String"
---

Age in words: `today`, `yesterday`, `3 days ago`, `2 months ago`.

Pure, with "now" ENTERING as a parameter — and not read internally. It is the
same reason as the predicate in [`prune`]: without this, proving the "yesterday"
boundary would require waiting for midnight to pass.

In English because the interface is in English, and without a date library because
what is needed is simple division: a calendar dependency to say "3 days ago"
costs more than these ten lines.

# Examples

```rust
use std::time::SystemTime;
use crawrad_project::workspace::time_ago;

let now = SystemTime::now();
assert_eq!(time_ago(now, now), "today");
```
