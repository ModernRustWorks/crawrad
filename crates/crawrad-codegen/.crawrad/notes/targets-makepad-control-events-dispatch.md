---
id: targets-makepad-control-events-dispatch
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 109
author: 
created: 
title: "dispatch"
tags: []
signature: "fn dispatch (scrutinee : & str , arms : & [(String , String)]) -> String"
---

Emits a dispatch over `scrutinee` using `if let` or `match`.

A dispatch over `scrutinee`: an `if let` for one arm (a one-armed `match`
is clippy's `single_match`), a `match` for more.
