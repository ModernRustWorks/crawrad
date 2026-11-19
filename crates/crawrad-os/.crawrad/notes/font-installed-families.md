---
id: font-installed-families
kind: doc
target_file: src/font.rs
line_hint: 61
author: 
created: 
title: "installed_families"
tags: []
signature: "pub fn installed_families () -> & 'static [String]"
---

Returns the sorted list of installed font families on this system.

Returns the sorted, deduplicated list of installed font families on this system.

Enumerating fonts is expensive (~tens of ms); this function caches the result in a
[`OnceLock`] so subsequent calls return immediately without touching system APIs.

# Examples

```rust
use crawrad_os::font::installed_families;

let families = installed_families();
#[cfg(target_os = "windows")]
assert!(!families.is_empty());
```
