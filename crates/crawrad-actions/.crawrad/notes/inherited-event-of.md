---
id: inherited-event-of
kind: doc
target_file: src/lib.rs
line_hint: 1
author:
created:
title: "inherited_event_of"
tags: []
signature: "fn inherited_event_of (c : & Context) -> Option < InheritedEvent >"
---

The event the `node` and `event` arguments name, when it is inherited.

`None` when the form inherits from nothing, the component is its own, or the
base leaves that event unbound: none of the four actions on an inherited
event has anything to do then.
