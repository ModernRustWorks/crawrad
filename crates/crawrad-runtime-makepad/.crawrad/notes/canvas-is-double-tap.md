---
id: canvas-is-double-tap
kind: doc
target_file: src/canvas/brush.rs
line_hint: 143
author: 
created: 
title: "is_double_tap"
tags: []
signature: "pub fn is_double_tap(previous: Option<(DVec2, f64)>, abs: DVec2, time: f64) -> bool"
---

Whether this tap closes a double click with the previous one.

# The framework's ruler, on purpose

Makepad counts taps in `Fingers::process_tap_count`
(`makepad-platform-1.0.0/src/event/finger.rs:461`) and keeps the result in a
`pub(crate)` field — `tap_count` only comes out wrapped in a `Hit::FingerDown`,
and the `Hit` of an Object Inspector row would be a click target over its text
field, which would stop it from being edited.

So the count is redone here — but with ITS constants, imported. Writing `0.5`
and `5.0` here would be a second idea of a double click, the one that disagrees
with the rest of the IDE the day the framework changes its own.

A free function because it is pure: it takes the previous tap and this one, and
a test measures it without opening a window. It moved out of the IDE's shell
(#925).

# Examples

```rust
use makepad_widgets::DVec2;
use crawrad_runtime_makepad::canvas::is_double_tap;

let at = DVec2 { x: 10.0, y: 10.0 };
assert!(!is_double_tap(None, at, 0.0));
assert!(is_double_tap(Some((at, 0.0)), at, 0.1));
assert!(!is_double_tap(Some((at, 0.0)), at, 5.0));
```
