---
id: tasks-set-waker
kind: doc
target_file: src/tasks.rs
line_hint: 15
author: 
created: 
title: "set_waker"
tags: []
signature: "pub fn set_waker (w : Waker)"
---

Installs the global UI waker callback.

Installed ONCE by the UI shell (Makepad: SignalToUI::set_ui_signal). Global so every manager wakes the same window.

# Examples

```rust
use crawrad_os::tasks::set_waker;

fn my_waker() {}
set_waker(my_waker);
```
