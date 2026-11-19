---
id: dock-with-root-restored
kind: doc
target_file: src/dock.rs
line_hint: 28
author: 
created: 
title: "with_root_restored"
tags: []
signature: "pub fn with_root_restored (items : & HashMap < LiveId , DockItem >) -> Option < HashMap < LiveId , DockItem > >"
---

The arrangement with its `root` back, or `None` when it did not lose it (#272).

Emptying a group that is a direct child of `root` makes the `Dock` unsplit
`root` itself: `unsplit_tabs` removes the splitter and re-points its
parent, and `root` has none (`makepad-widgets-1.0.0/src/dock.rs:596-620`).
Drawing starts at `root` (`dock.rs:1092-1095`), so the whole window went
blank. The one container no splitter points at is the surviving sibling,
and it becomes `root`.

# Examples

```rust
use std::collections::HashMap;
use crawrad_runtime_makepad::dock::with_root_restored;

let empty = HashMap::new();
assert!(with_root_restored(&empty).is_none());
```
