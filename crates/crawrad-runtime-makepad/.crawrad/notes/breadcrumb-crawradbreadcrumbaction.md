---
id: breadcrumb-crawradbreadcrumbaction
kind: doc
target_file: src/breadcrumb.rs
line_hint: 89
author: 
created: 
title: "CrawradBreadcrumbAction"
tags: []
signature: "pub enum CrawradBreadcrumbAction"
---

What a [`CrawradBreadcrumb`] reports.

# Examples

```rust
use crawrad_runtime_makepad::breadcrumb::CrawradBreadcrumbAction;

let action = CrawradBreadcrumbAction::Clicked(2);
assert!(matches!(action, CrawradBreadcrumbAction::Clicked(2)));
```
