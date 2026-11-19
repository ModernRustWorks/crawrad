---
id: targets-egui-control-events-dispatch
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 164
author: 
created: 
title: "dispatch"
tags: []
signature: "fn dispatch (scrutinee : & str , arms : & [(String , String)]) -> String"
---

Emits dispatch code over a scrutinee expression.

A dispatch over `scrutinee`: an `if let` for one arm (a one-armed `match`
is clippy's `single_match`), a `match` for more.
