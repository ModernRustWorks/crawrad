---
id: crash-report-write
kind: doc
target_file: src/crash_report.rs
line_hint: 91
author: 
created: 
title: "write"
tags: []
signature: "fn write (folder : & Path , error : & str , details : & str) -> Option < PathBuf >"
---

Writes the report, and returns the path **only if it was really written**.

No `unwrap` and no `?` that blows up: this is already the failure path, and
panicking inside the reporter would swap a message for a crash with no
message at all, which is exactly the defect this module closes.
