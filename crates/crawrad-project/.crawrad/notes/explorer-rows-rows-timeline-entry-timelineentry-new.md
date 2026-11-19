---
id: explorer-rows-rows-timeline-entry-timelineentry-new
kind: doc
target_file: src/explorer_rows/rows/timeline_entry.rs
line_hint: 18
author: 
created: 
title: "TimelineEntry::new"
tags: []
signature: "pub fn new (index : usize , text : impl Into < String >) -> Self"
---

Creates a new timeline entry with its position index and formatted text.

# Examples

```rust
use crawrad_project::explorer_rows::rows::timeline_entry::TimelineEntry;
use crawrad_project::explorer_rows::row::ExplorerRow;

let entry = TimelineEntry::new(1, "entry #1");
assert_eq!(entry.text(), "entry #1");
```
