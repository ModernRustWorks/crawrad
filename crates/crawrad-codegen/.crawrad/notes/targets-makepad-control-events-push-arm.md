---
id: targets-makepad-control-events-push-arm
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 98
author: 
created: 
title: "push_arm"
tags: []
signature: "fn push_arm (arms : & mut Vec < (String , String) > , pattern : String , call : String)"
---

Adds `call` to the arm for `pattern`, or creates a new arm.

Adds `call` to the arm for `pattern`, or a new arm: two arms with the same
pattern would leave the second unreachable.
