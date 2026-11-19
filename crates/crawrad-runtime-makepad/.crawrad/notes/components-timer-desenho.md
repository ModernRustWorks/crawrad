---
id: components-timer-desenho
kind: doc
target_file: src/components/timer.rs
line_hint: 9
author: 
created: 
title: "DESENHO"
tags: []
signature: "pub static DESENHO : Badge = Badge :: for_class (\"Timer\")"
---

O desenho do `Timer`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::timer::DESENHO;

assert_eq!(DESENHO.class_name(), "Timer");
```
