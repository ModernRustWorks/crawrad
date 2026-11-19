---
id: history-view-timeline-lines
kind: doc
target_file: src/history_view.rs
line_hint: 276
author: 
created: 
title: "timeline_lines"
tags: []
signature: "pub fn timeline_lines (entries : & [Entry] , now : u64 , offset : i64) -> Vec < String >"
---

Formats history entries into timeline lines for display, newest first.

Each line has the format `"{origin} · {time} · {when}"`.

# Examples

```rust
use crawrad_host::history_view::timeline_lines;

let lines = timeline_lines(&[], 0, 0);
assert!(lines.is_empty());
```
