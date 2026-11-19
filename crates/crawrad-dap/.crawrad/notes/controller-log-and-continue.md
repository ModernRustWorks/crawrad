---
id: controller-log-and-continue
kind: doc
target_file: src/controller.rs
line_hint: 392
author: 
created: 
title: "log_and_continue"
tags: []
signature: "fn log_and_continue (session : & mut DebugSession , thread_id : i64 , presenter : & dyn ValuePresenter , updates : & UpdateSink < '_ > ,) -> bool"
---

Handles a stop caused by a logpoint, emitting its evaluated message.

A stop on a LOGPOINT: its message, with each `{expr}` written by the
presenter as `{:?}` would, goes out as output, and the program goes on.
`false` when the stop is not on a logpoint — then it is an ordinary stop.
