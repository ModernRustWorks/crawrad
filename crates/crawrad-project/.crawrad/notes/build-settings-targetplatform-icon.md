---
id: build-settings-targetplatform-icon
kind: doc
target_file: src/build_settings.rs
line_hint: 57
author: 
created: 
title: "TargetPlatform::icon"
tags: []
signature: "pub fn icon (self) -> & 'static str"
---

The target's mark in the Build dialog's list.

# Examples

```rust
use crawrad_project::build_settings::TargetPlatform;

assert_eq!(TargetPlatform::Linux64.icon(), "🐧");
```
