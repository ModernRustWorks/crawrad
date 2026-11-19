---
id: package-rows-max-stat-lines
kind: doc
target_file: src/package_rows.rs
line_hint: 88
author: 
created: 
title: "MAX_STAT_LINES"
tags: []
signature: "pub const MAX_STAT_LINES : usize = 3"
---

How many lines the right-hand column of a card can show.

The DSL has that many fixed slots (`CrawradPackageRow`, `stat_1..stat_3`), and
fixed is deliberate: the `live_design!` is static and there is no public API
to inject DSL at runtime (ADR 0027). A fourth statistic would vanish in
silence, so [`package_stats`] never builds one and a gate charges it.

# Examples

```rust
use crawrad_project::package_rows::MAX_STAT_LINES;

assert_eq!(MAX_STAT_LINES, 3);
```
