---
id: startup-startup-log-path
kind: doc
target_file: src/startup.rs
line_hint: 31
author: 
created: 
title: "startup_log_path"
tags: []
signature: "fn startup_log_path () -> PathBuf"
---

Returns the log file path beside the executable or in temp.

`<exe>.error.log` beside the executable — where the person who got the
program will find it — or the temp folder when there is no exe path.
