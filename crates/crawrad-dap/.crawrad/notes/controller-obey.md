---
id: controller-obey
kind: doc
target_file: src/controller.rs
line_hint: 308
author: 
created: 
title: "obey"
tags: []
signature: "fn obey (session : & mut DebugSession , command : Command , presenter : & dyn ValuePresenter , updates : & UpdateSink < '_ > ,) -> Option < Update >"
---

Carries out one command. `Some(final update)` when the session is over.
