---
id: explorer-rows-rows-timeline-entry-timelineentry
kind: doc
target_file: src/explorer_rows/rows/timeline_entry.rs
line_hint: 10
author: 
created: 
title: "TimelineEntry"
tags: []
signature: "pub struct TimelineEntry"
---

One line in the Timeline section, representing a single history copy.

# Examples

```rust
use crawrad_project::explorer_rows::rows::timeline_entry::TimelineEntry;
use crawrad_project::explorer_rows::row::ExplorerRow;

let entry = TimelineEntry::new(0, "save 1");
assert_eq!(entry.text(), "save 1");
```
