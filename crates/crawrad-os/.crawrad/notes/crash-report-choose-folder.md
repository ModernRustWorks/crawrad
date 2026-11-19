---
id: crash-report-choose-folder
kind: doc
target_file: src/crash_report.rs
line_hint: 83
author: 
created: 
title: "choose_folder"
tags: []
signature: "fn choose_folder (configuration : Option < PathBuf >) -> Option < PathBuf >"
---

The folder choice, with the fall back to the temporary directory.

The configuration folder comes in as a parameter because otherwise the
**fall back** cannot be measured: on this machine `APPDATA` exists, the
first option always answers, and a test that only called `report_folder`
would stay green with the fall back ripped out.
