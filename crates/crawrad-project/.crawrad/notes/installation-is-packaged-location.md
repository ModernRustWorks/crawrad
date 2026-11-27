---
id: installation-is-packaged-location
kind: doc
target_file: src/installation.rs
line_hint: 253
author: 
created: 
title: "is_packaged_location"
tags: []
signature: "pub fn is_packaged_location (exe : & Path) -> bool"
---

Tells whether a path lies inside a Microsoft Store (MSIX) install.

**A STORE INSTALL IS READ-ONLY.**

Store packages unpack under `WindowsApps`, which no program may write to. Any
path component named `WindowsApps` (case-insensitive) marks the location as
packaged.

# Examples

```rust
use std::path::Path;
use crawrad_project::installation::is_packaged_location;

// Forward slash on purpose: the backslash only separates a path on Windows,
// and this example runs on every platform.
assert!(is_packaged_location(Path::new("/Program Files/WindowsApps/CrawRAD")));
assert!(!is_packaged_location(Path::new("/Program Files/CrawRAD")));
```
