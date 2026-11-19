---
id: sqlite-file
kind: doc
target_file: src/lib.rs
line_hint: 86
author: 
created: 
title: "sqlite_file"
tags: []
signature: "fn sqlite_file (url : & str) -> Option < & str >"
---

Returns the file path a SQLite URL names, if any.

The file a SQLite url names, or `None` for an in-memory or `file:` URI one
and for any other database.
