---
id: editor-tabs-editortabs-open
kind: doc
target_file: src/editor_tabs.rs
line_hint: 100
author: 
created: 
title: "EditorTabs::open"
tags: []
signature: "pub fn open (& mut self , path : impl Into < PathBuf >) -> usize"
---

Opens a file or brings it to the front if already open.

**Open a file, or bring it to the front if it is already open.**

Reopening focuses instead of duplicating, which is what every editor
does and what the alternative argues against by itself: two tabs over
one file are two buffers to keep in step, and the second one is always
the stale one.

# Examples

```rust
use crawrad_host::editor_tabs::EditorTabs;

let mut tabs = EditorTabs::default();
let idx = tabs.open("src/lib.rs");
assert_eq!(idx, 0);
assert_eq!(tabs.active_index(), Some(0));
```
