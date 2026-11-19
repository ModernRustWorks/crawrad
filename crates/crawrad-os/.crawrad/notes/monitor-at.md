---
id: monitor-at
kind: doc
target_file: src/lib.rs
line_hint: 109
author: 
created: 
title: "monitor_at"
tags: []
signature: "pub fn monitor_at (point : (f64 , f64)) -> Option < MonitorArea >"
---

The monitor that contains `point`, in physical pixels.

`point` is a desktop coordinate: on a multi-monitor setup the desktop is
one continuous plane and a secondary screen can sit at a negative `x`. The
nearest monitor answers when the point falls in a gap between screens, so
a window dragged half off the desktop still gets a usable answer.

# When it answers `None`

On every platform other than Windows, always — and that is a declared
limit, not an oversight. The defect this crate exists for is specific to
how Windows maximizes a `WS_POPUP` window; macOS and Linux maximize to the
usable area on their own, so there the caller keeps using the framework's
own maximize and never needs this answer. The day a second platform does
need it, it gets its own arm here and its own measurement in the doc.

On Windows it answers `None` only if `GetMonitorInfoW` fails, which is the
system refusing to describe a monitor it just handed us. The caller is
expected to fall back to the framework's maximize rather than guess.

# Examples

```rust
use crawrad_os::monitor_at;

let area = monitor_at((0.0, 0.0));
#[cfg(target_os = "windows")]
assert!(area.is_some());
#[cfg(not(target_os = "windows"))]
assert!(area.is_none());
```
