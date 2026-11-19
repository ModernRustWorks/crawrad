---
id: client-track-progress
kind: doc
target_file: src/client.rs
line_hint: 1154
author: 
created: 
title: "track_progress"
tags: []
signature: "pub (crate) fn track_progress (payload : & str , work : & Mutex < std :: collections :: BTreeMap < String , String > > , inlay_refresh : & AtomicBool ,) -> bool"
---

Tracks LSP work done progress notifications from the server.

`$/progress` (LSP 3.15 work done progress): keeps one line per active
token, `title message (percentage%)`, and drops it at `end`. When the LAST
one ends — rust-analyzer finished loading or indexing — the inlay hints are
asked again: measured on 01/10/2026, the server's own
`workspace/inlayHint/refresh` does not always come, and the first answer
(given while indexing) was empty.
