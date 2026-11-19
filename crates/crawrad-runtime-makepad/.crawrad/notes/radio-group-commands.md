---
id: radio-group-commands
kind: doc
target_file: src/radio_group.rs
line_hint: 92
author: 
created: 
title: "commands"
tags: []
signature: "pub fn commands (state : & GroupState , w : f32 , h : f32 , caption_w : f32) -> Vec < Command >"
---

Generates paint commands to draw the radio group frame and options.

The group's drawing for `state` in a box `w` x `h`; `caption_w` is the
caption's width, for the gap in the frame's top edge.

# Examples

```rust
use crawrad_runtime_makepad::radio_group::{commands, GroupState};

let state = GroupState::default();
let cmds = commands(&state, 200.0, 100.0, 40.0);
assert!(!cmds.is_empty());
```
