---
id: events-mouse-click
kind: doc
target_file: src/lib.rs
line_hint: 322
author: 
created: 
title: "Events::MOUSE_CLICK"
tags: []
signature: "pub const MOUSE_CLICK : EventDesc = EventDesc :: new (\"on_click\" , \"OnClick\" , \"fn(&mut self)\")"
---

Standard mouse click event (`on_click`/`OnClick`).

The mouse events keep the `MOUSE_` prefix so the family reads as one in
the catalogue, next to `MOUSE_DOWN`, `MOUSE_UP` and `MOUSE_MOVE`.

The `name` and `display` stay `on_click`/`OnClick`: they are what the
Object Inspector shows and what a `.crawform` already on disk carries.
Renaming the constant is a source change; renaming these two would be a
format change, and a form saved yesterday would lose its handler.
Owner's call, 15/09/2026.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::MOUSE_CLICK.display, "OnClick");
```
