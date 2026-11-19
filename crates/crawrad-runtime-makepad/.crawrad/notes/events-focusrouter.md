---
id: events-focusrouter
kind: doc
target_file: src/events.rs
line_hint: 255
author: 
created: 
title: "FocusRouter"
tags: []
signature: "pub struct FocusRouter"
---

Tracks and routes keyboard focus among form controls.

**The focus of the form's controls** — which one has it, and so which one
the keyboard goes to.

The same idea as [`PointerRouter`], and the same rules as a classic RAD
form: a press on a control that CAN take the focus gives it the focus (the
one that had it gets `Exit`, the new one `Enter`); a press anywhere else
leaves the focus where it is; the keys go to the control that has it. Which
classes can take the focus is data — the generator passes them in.

The keyboard focus moving into such a control moves it too: Tab and
Shift+Tab (#667) fire `Exit` and `Enter` as a press does.

# Examples

```rust
use crawrad_runtime_makepad::events::FocusRouter;

let router = FocusRouter::default();
assert_eq!(router.focused(), None);
```
