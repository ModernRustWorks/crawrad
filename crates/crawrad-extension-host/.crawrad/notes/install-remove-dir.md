---
id: install-remove-dir
kind: doc
target_file: src/install.rs
line_hint: 58
author: 
created: 
title: "remove_dir"
tags: []
signature: "fn remove_dir (dir : & Path) -> std :: io :: Result < () >"
---

Removes a directory with multiple attempts.

A folder removed even while Windows still holds a file the stopped
process had open: a few tries, a little apart.
