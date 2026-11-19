---
id: components-page-control-tab-window
kind: doc
target_file: src/components/page_control.rs
line_hint: 28
author: 
created: 
title: "tab_window"
tags: []
signature: "pub fn tab_window (width : f32 , shown : usize , active : usize , plus : bool) -> TabWindow"
---

Computes which tabs of a strip fit on screen (#13).

- `avail = width - (if plus { PLUS_BUTTON_WIDTH } else { 0.0 })`
- If `shown as f32 * TAB_WIDTH <= avail` -> `TabWindow { first: 0, count: shown, arrows: false }`
- Else `avail -= 2.0 * TAB_SCROLL_BUTTON_WIDTH`; `per = max(1, floor(avail / TAB_WIDTH))`
  (at least one tab even in a tiny control); `count = min(per, shown)`; `active = min(active, shown - 1)`;
  `first = active.saturating_sub(count - 1)` (the active tab is the last one of the window
  when it would otherwise be off to the right; when active < count the window starts at 0);
  also clamp `first <= shown - count`. `arrows: true`.
- `shown == 0` -> `{0, 0, false}`.

# Examples

```rust
use crawrad_registry::tab_window;

let win = tab_window(300.0, 3, 0, false);
assert_eq!(win.count, 3);
```
