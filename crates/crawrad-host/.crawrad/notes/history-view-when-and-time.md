---
id: history-view-when-and-time
kind: doc
target_file: src/history_view.rs
line_hint: 213
author: 
created: 
title: "when_and_time"
tags: []
signature: "pub fn when_and_time (time : u64 , now : u64 , offset : i64) -> (String , String)"
---

Formats a timestamp into a time string and a relative day descriptor.

Formats an entry timestamp into a local time string (`"HH:MM:SS"`) and calendar descriptor
(`"today"`, `"yesterday"`, `"dd/mm"`, or `"dd/mm/yyyy"`).

`now` and `offset` (seconds east of UTC) are parameters so tests are deterministic.

# Examples

```rust
use crawrad_host::history_view::when_and_time;

let (time, when) = when_and_time(1000, 1000, 0);
assert_eq!(when, "today");
```
