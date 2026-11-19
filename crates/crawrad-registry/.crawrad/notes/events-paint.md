---
id: events-paint
kind: doc
target_file: src/lib.rs
line_hint: 391
author: 
created: 
title: "Events::PAINT"
tags: []
signature: "pub const PAINT : EventDesc = EventDesc :: new (\"on_paint\" , \"OnPaint\" , \"fn(&mut self, canvas: &mut Canvas)\")"
---

Fired when a control needs custom painting on a canvas.

#459: the handler draws on a `Canvas` — the PaintBox's, or the
form's (under its controls) — as in the classic RAD IDE.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::PAINT.name, "on_paint");
```
