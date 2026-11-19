---
id: package-manager-humanize-date
kind: doc
target_file: src/package_manager.rs
line_hint: 196
author: 
created: 
title: "humanize_date"
tags: []
signature: "pub fn humanize_date (iso : & str) -> String"
---

Converts an ISO 8601 date to humanized relative text.

Converts an ISO 8601 date (`"2026-01-12T20:57:19.255609Z"`) to humanized
relative text (`"8 months ago"`, `"about 5 years ago"`).

# Examples

```rust
use crawrad_project::package_manager::humanize_date;

assert_eq!(humanize_date("invalid"), "invalid");
```
