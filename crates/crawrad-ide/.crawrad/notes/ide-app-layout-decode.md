---
id: ide-app-layout-decode
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 113
author: 
created: 
title: "decode"
tags: []
signature: "pub fn decode (text : & str) -> Option < HashMap < LiveId , DockItem > >"
---

The arrangement a text carries, or `None` when it cannot be trusted whole.

Four ways to get `None`, and every one of them means *use the default*:

1. the version marker is missing or is not [`FORMAT_VERSION`](crawrad_runtime_makepad::dock::FORMAT_VERSION) — an
   arrangement written by another format;
2. the body does not parse — a truncated or hand-edited file;
3. there is no `root` — the dock has nowhere to start drawing, and the
   screen would come up empty;
4. a `Tab` names a panel this build does not know — the arrangement is from
   a build where a panel existed, or was called something else.

The fourth is the migration rule, and it is deliberately blunt. A finer one
— drop the unknown tab, keep the rest — leaves the user with a window
missing a panel and no explanation. Discarding the whole file costs one
rearrangement and says what happened in the status bar.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let arr = decode("");
assert!(arr.is_none());
```
