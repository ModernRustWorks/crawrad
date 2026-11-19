---
id: bookmarks-bookmarks-sync-view
kind: doc
target_file: src/bookmarks.rs
line_hint: 63
author: 
created: 
title: "Bookmarks::sync_view"
tags: []
signature: "pub fn sync_view (& mut self , file : & Path , view_owner : Option < & Path > , view : & [(u8 , usize)] ,) -> BookmarkSync"
---

Reconciles the editor's view of `file` with the session.

Reconciles the editor's view of `file` with the session, once a frame.
`view_owner` is the file the view held BEFORE this frame put another
in it; a view of another file gets this file's bookmarks, a view of
this file that differs was changed by the user, and wins.

# Examples

```rust
use std::path::Path;
use crawrad_host::bookmarks::{BookmarkSync, Bookmarks};

let mut bookmarks = Bookmarks::default();
let file = Path::new("src/main.rs");
let sync = bookmarks.sync_view(file, Some(file), &[(1, 5)]);
assert_eq!(sync, BookmarkSync::Pulled);
```
