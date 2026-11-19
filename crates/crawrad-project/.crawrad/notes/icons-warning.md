---
id: icons-warning
kind: doc
target_file: src/icons.rs
line_hint: 1664
author: 
created: 
title: "WARNING"
tags: []
signature: "pub const WARNING : & str = \"\\u{ea6c}\""
---

The WARNING SIGN for dialogs — Codicons `warning`.

Free-standing like the two chevrons above, and for the same reason: it is not a chrome
role (no one clicks it) nor a component drawing; it is the drawing of ONE phrase,
that of `Unsaved changes`.

**Measured in font `cmap`** before entering: `0xea6c` is in the range
`0xea60..0xea88` of this repo's `codicon.ttf`. A glyph chosen via third-party table
and unverified is an empty box on screen, and this project has already paid for that.

# Examples

```rust
use crawrad_project::icons::WARNING;

assert_eq!(WARNING, "\u{ea6c}");
```
