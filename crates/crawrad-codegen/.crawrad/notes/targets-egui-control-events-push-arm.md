---
id: targets-egui-control-events-push-arm
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 156
author: 
created: 
title: "push_arm"
tags: []
signature: "fn push_arm (arms : & mut Vec < (String , String) > , pattern : String , call : String)"
---

Adds a dispatch call to an existing or new pattern arm.

Adds `call` to the arm for `pattern`, or a new arm: two arms with the same
pattern would leave the second unreachable.
