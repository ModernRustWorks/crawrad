---
id: history-localhistory-file-dir
kind: doc
target_file: src/history.rs
line_hint: 186
author: 
created: 
title: "LocalHistory::file_dir"
tags: []
signature: "fn file_dir (& self , file : & Path) -> Option < (String , PathBuf) >"
---

Resolves relative path and history directory for `file`.

The file's path relative to root (`/` separators) and its history dir,
named by that path's hash; `None` for a file outside root.
