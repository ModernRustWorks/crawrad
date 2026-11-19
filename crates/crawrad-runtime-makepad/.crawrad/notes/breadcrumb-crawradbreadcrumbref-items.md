---
id: breadcrumb-crawradbreadcrumbref-items
kind: doc
target_file: src/breadcrumb.rs
line_hint: 482
author: 
created: 
title: "CrawradBreadcrumbRef::items"
tags: []
signature: "pub fn items (& self) -> Vec < String >"
---

The items on screen (#604): what a host reports to MCP and a test reads.

# Examples

```rust
use crawrad_runtime_makepad::breadcrumb::CrawradBreadcrumbRef;

let breadcrumb = CrawradBreadcrumbRef::default();
assert!(breadcrumb.items().is_empty());
```
