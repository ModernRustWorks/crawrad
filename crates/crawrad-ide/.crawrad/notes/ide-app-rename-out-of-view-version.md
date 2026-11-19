---
id: ide-app-rename-out-of-view-version
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 18
author: 
created: 
title: "OUT_OF_VIEW_VERSION"
tags: []
signature: "static OUT_OF_VIEW_VERSION : AtomicI32 = AtomicI32 :: new (1_000_000)"
---

Monotonic document version counter for modified background files.

Versions for files a rename changes while they are not in front: above
any the editor's own counter reaches, and only growing.
