---
id: handlerjump
kind: doc
target_file: src/lib.rs
line_hint: 1
author:
created:
title: "HandlerJump"
tags: []
signature: "pub struct HandlerJump"
---

Where an action asks the IDE to land in the code.

A handler, in this form's unit or in its base form's.

# Examples

```
use crawrad_actions::HandlerJump;

let jump = HandlerJump { name: "btn_save_click".into(), inherited: true };
assert!(jump.inherited);
```
