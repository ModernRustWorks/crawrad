---
id: editor-tabs-tail
kind: doc
target_file: src/editor_tabs.rs
line_hint: 232
author: 
created: 
title: "tail"
tags: []
signature: "fn tail (p : & Path , k : usize) -> String"
---

The last `k` folders above the file, with forward slashes.

Forward slashes on every platform, on purpose: this text is read on screen
and a Windows backslash in the middle of a tab reads like an escape. The
path itself is never rebuilt from this.
