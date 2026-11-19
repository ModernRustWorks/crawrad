---
id: inheritedevent
kind: doc
target_file: src/lib.rs
line_hint: 1
author:
created:
title: "InheritedEvent"
tags: []
signature: "struct InheritedEvent"
---

An event of an inherited component that the base form handles.

`current` is this form's handler of it today, `base` the base's: equal while
the event is only inherited, different once it is overridden.
