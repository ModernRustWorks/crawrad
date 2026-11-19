---
id: components-page-control-page-control
kind: doc
target_file: src/components/page_control.rs
line_hint: 83
author: 
created: 
title: "page_control"
tags: []
signature: "pub fn page_control () -> ComponentDesc"
---

A multi-page tabbed container component.

`PageControl`: the page container. The classic `TPageControl`.

Brings two rules that [ADR 0012] gained by amendment: the page has
**derived geometry** (fills the control below the tabs, and does not move)
and **conditional visibility** (only the active one is on screen).

`ActivePageIndex` is a DOCUMENT property, not IDE state: in the classic RAD IDE
it goes to the form definition file, and the open page when saving is the one that opens afterwards.


# Examples

```rust
use crawrad_registry::page_control;

let desc = page_control();
assert_eq!(desc.class, "PageControl");
```
