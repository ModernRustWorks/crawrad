---
id: package-rows-package-stats
kind: doc
target_file: src/package_rows.rs
line_hint: 91
author: 
created: 
title: "package_stats"
tags: []
signature: "pub fn package_stats (item : & model :: PackageItem) -> Vec < String >"
---

The right-hand column of a search result, one line per fact.

**The right-hand column of a search result, one line per fact.**

# Why three lines and not one string

It was one string — `29,449,600 downloads · 8 months ago` — under the
description, and the owner asked for the two columns back: *"o Manager
regrediu, tinha duas colunas"*. The egui window writes these same three
facts stacked and right-aligned (`package_manager.rs:286-313`), which is
what a narrow column can hold and a wide line cannot.

# What is NOT here

The arrows. The egui labels carry `⬇` and `↻`, and neither is in this
shell's text face — they would come out as empty boxes. ADR 0026 is the
rule: a drawing comes from a drawn set, not from a loose character. The
words carry the meaning on their own.

Absent facts produce **no line at all**, rather than a line with a blank on
one side: a crate the API answered nothing about gets a card with the button
and nothing above it, which is the truth.

# Examples

```rust
use crawrad_project::package_manager::PackageItem;
use crawrad_project::package_rows::package_stats;

let item = PackageItem {
    name: "serde".into(),
    version: "1.0.0".into(),
    description: String::new(),
    downloads: Some(100),
    recent_downloads: None,
    updated_at: None,
    documentation: None,
    repository: None,
    homepage: None,
};
assert_eq!(package_stats(&item), vec!["All-time: 100"]);
```
