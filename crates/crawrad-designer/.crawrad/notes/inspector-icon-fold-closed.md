---
id: inspector-icon-fold-closed
kind: doc
target_file: src/inspector_icon.rs
line_hint: 25
author: 
created: 
title: "FOLD_CLOSED"
tags: []
signature: "pub const FOLD_CLOSED : RowIcon = \"\\u{eb70}\""
---

The mark of a property whose sub-rows are folded away.

The mark of a property whose sub-rows are FOLDED AWAY — it points at what
opening it shows. Codicons `triangle-right`.

# It is a TRIANGLE, and Explorer continues with chevron

It is the day that this module's doc already foresaw: *"the two panels are free to
disagree on the day one of them changes its mind"*. It changed, and the reason is that the
two references are different.

Explorer follows VS Code, which draws chevrons in the file tree. The
Object Inspector follows the screenshot of the egui shell — what the owner approved —, and there
the fold mark is the filled triangle of `egui::CollapsingHeader`. He
compared the two side by side and pointed out the difference.

Same font, same set: the four triangles of Codicons live in
`U+EB6E`..`U+EB71` (down, left, right, up), and the existence of the two used
here was MEASURED in the `cmap` of `assets/fontes/codicon.ttf` before writing.

# Examples

```rust
use crawrad_designer::inspector_icon::FOLD_CLOSED;

assert_eq!(FOLD_CLOSED, "\u{eb70}");
```
