---
id: bookmarks-bookmarksync
kind: doc
target_file: src/bookmarks.rs
line_hint: 23
author: 
created: 
title: "BookmarkSync"
tags: []
signature: "pub enum BookmarkSync"
---

What the editor's view needs after [`Bookmarks::sync_view`].

# Examples

```rust
use crawrad_host::bookmarks::BookmarkSync;

let sync = BookmarkSync::Same;
assert_eq!(sync, BookmarkSync::Same);
```
