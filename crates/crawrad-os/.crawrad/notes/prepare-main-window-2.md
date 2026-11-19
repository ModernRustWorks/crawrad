---
id: prepare-main-window-2
kind: doc
target_file: src/lib.rs
line_hint: 318
author: 
created: 
title: "prepare_main_window"
tags: []
signature: "pub fn prepare_main_window () -> bool"
---

Readies the main window: nothing to do outside Windows.

The taskbar overlap and the rounded corners are Windows behaviours, so on the
other platforms this returns `false` and changes nothing.

# Examples

```rust
use crawrad_os::prepare_main_window;

let prepared = prepare_main_window();
assert!(prepared || !prepared);
```
