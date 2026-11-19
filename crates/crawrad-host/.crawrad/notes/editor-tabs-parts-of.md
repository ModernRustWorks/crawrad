---
id: editor-tabs-parts-of
kind: doc
target_file: src/editor_tabs.rs
line_hint: 216
author: 
created: 
title: "parts_of"
tags: []
signature: "fn parts_of (p : & Path) -> Vec < String >"
---

The pieces of a path, split at BOTH separators on every platform.

Measured on 01/10/2026 in a Linux container: `Path` splits only at `/`
there, so `c:\proj\alpha\lib.rs` was one "file name" with its
backslashes, and the tab showed it whole. The label is text to read, never
parsed back into a path, so splitting the text is exact on every system.
