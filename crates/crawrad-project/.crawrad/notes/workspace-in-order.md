---
id: workspace-in-order
kind: doc
target_file: src/workspace.rs
line_hint: 205
author: 
created: 
title: "in_order"
tags: []
signature: "fn in_order (mut documents : Vec < PathBuf >) -> Vec < PathBuf >"
---

The documents of a member in alphabetical order.

A function for this, and it is PURE by design. The `in_this.sort()` that lived
inside the loop could not be proven: on NTFS `read_dir` already returns
names in order because the directory is a B-tree indexed by name — removing the
sorting left all hundred tests green, and was one of two mutations that
survived review. Here the input is chosen by the test, not the filesystem.

The order matters because this list APPEARS ON SCREEN when choosing which
form to open: a list changing order between openings is a list where clicking
cannot be learned.
