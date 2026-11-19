---
id: events-nest-base-events
kind: doc
target_file: src/events.rs
line_hint: 830
author:
created:
title: "nest_base_events"
tags: []
signature: "pub (crate) fn nest_base_events (doc : & Document , base_type : & str , base_events : & [WiredEvent] ,) -> Vec < WiredEvent >"
---

The base's events as this form reports them up, wrapped in its `Base`.

Each is one more level deep (`through` gains the base's type), and its
handler is this form's override when it has one: that is the method a
further derived form calls as `self.base.<handler>()`.
