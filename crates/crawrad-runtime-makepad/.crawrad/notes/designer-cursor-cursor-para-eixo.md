---
id: designer-cursor-cursor-para-eixo
kind: doc
target_file: src/designer/cursor.rs
line_hint: 7
author: 
created: 
title: "cursor_para_eixo"
tags: []
signature: "pub fn cursor_para_eixo (eixo : Axis) -> MouseCursor"
---

Returns the appropriate mouse cursor icon for each designer resize handle axis.

# Examples

```rust
use makepad_widgets::MouseCursor;
use crawrad_runtime::Axis;
use crawrad_runtime_makepad::designer::cursor_para_eixo;

assert_eq!(cursor_para_eixo(Axis::Ns), MouseCursor::NsResize);
```
