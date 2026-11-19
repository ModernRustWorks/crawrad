---
id: crash-report-box-timeout
kind: doc
target_file: src/crash_report.rs
line_hint: 53
author: 
created: 
title: "BOX_TIMEOUT"
tags: []
signature: "const BOX_TIMEOUT : std :: time :: Duration = std :: time :: Duration :: from_secs (30)"
---

How long to wait for the box before giving up on it.

The box is modal and blocking, and blocking in a broken environment means
**forever**. The report is already on disk before the box opens, so giving
up on it loses nothing.
