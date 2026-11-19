---
id: startup-report-startup-failure
kind: doc
target_file: src/startup.rs
line_hint: 18
author: 
created: 
title: "report_startup_failure"
tags: []
signature: "pub fn report_startup_failure (app : & str , error : & dyn Display)"
---

Displays a message box and logs startup failure.

Tells the person the app could not start, and why. Never panics: it is the
last thing the program does, and failing here would hide the first failure.

# Examples

```no_run
// Displays a blocking GUI message box, which cannot run in automated tests.
use crawrad_runtime::report_startup_failure;

report_startup_failure("DemoApp", &"Initialization failed");
```
