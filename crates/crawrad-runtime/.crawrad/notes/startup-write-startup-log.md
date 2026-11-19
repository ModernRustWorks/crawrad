---
id: startup-write-startup-log
kind: doc
target_file: src/startup.rs
line_hint: 38
author: 
created: 
title: "write_startup_log"
tags: []
signature: "pub (crate) fn write_startup_log (path : & Path , app : & str , error : & dyn Display) -> Option < PathBuf >"
---

Writes the startup failure log to disk with a temp fallback.

Writes the log; falls back to the temp folder when the exe's folder is not
writable (an app installed under `Program Files`). `None` if both fail.
