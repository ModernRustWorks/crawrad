---
id: inherited-event-action
kind: doc
target_file: src/lib.rs
line_hint: 1
author:
created:
title: "inherited_event_action"
tags: []
signature: "fn inherited_event_action (id : & 'static str , label : & 'static str , hint : & 'static str , execute : fn (& Context) -> Effect) -> Action"
---

One of the four actions on an inherited event (#851).

They differ only in what they do: the same group, the same `node` and
`event` arguments, the Design view and an open document.
