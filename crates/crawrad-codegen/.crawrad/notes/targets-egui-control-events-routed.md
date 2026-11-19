---
id: targets-egui-control-events-routed
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 70
author: 
created: 
title: "routed"
tags: []
signature: "fn routed (desc : & crawrad_registry :: ComponentDesc , n : & Node , body : & str , events : & [(& 'static str , & 'static str)] ,) -> Result < Vec < (& 'static str , String) > , Erro >"
---

Finds handlers on a node that are not already emitted in its body.

The `(event, kind)` of `events` that `n` has a handler for, its class
declares, and its own emitted `body` does not call already.
