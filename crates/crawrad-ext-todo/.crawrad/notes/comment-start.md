---
id: comment-start
kind: doc
target_file: src/lib.rs
line_hint: 40
author: 
created: 
title: "comment_start"
tags: []
signature: "fn comment_start (line : & str) -> Option < usize >"
---

Finds where a comment starts on the line, if present.

Where the comment starts on `line`: after `//` or `/*`, or a block
comment's `*` continuation line.
