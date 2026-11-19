---
id: badge-badge-text
kind: doc
target_file: src/badge.rs
line_hint: 6
author: 
created: 
title: "badge_text"
tags: []
signature: "pub fn badge_text (caption : & str , count : i64 , max_count : i64) -> Option < String >"
---

Computes the badge text (#500).

When `count > 0`, returns the count string or `"{max_count}+"` when `count > max_count`.
When `count == 0` and `caption` is empty, returns `None` (hidden).
Otherwise returns `Some(caption.to_string())`.

# Examples

```rust
use crawrad_runtime_makepad::badge::badge_text;

assert_eq!(badge_text("", 5, 99), Some("5".to_string()));
assert_eq!(badge_text("", 105, 99), Some("99+".to_string()));
assert_eq!(badge_text("", 0, 99), None);
```
