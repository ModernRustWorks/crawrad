---
id: locked-warning
kind: doc
target_file: src/lib.rs
line_hint: 34
author: 
created: 
title: "LOCKED_WARNING"
tags: []
signature: "pub const LOCKED_WARNING : & str = \"controls are locked — Edit ▸ Lock controls to unlock\""
---

What the bar says when the gesture hits the lock.

The sentence states the status **and** where to turn it off, because the defect it avoids
is not the refused gesture: it is the conclusion that comes after it. A mute,
global switch causes the person to drag, see nothing happen, and decide that the
IDE broke — and nobody looks for a menu to fix a broken IDE.

# Examples

```rust
use crawrad_designer::LOCKED_WARNING;

assert!(LOCKED_WARNING.contains("Lock controls"));
```
