---
id: debug-panel-preview
kind: doc
target_file: src/debug_panel.rs
line_hint: 218
author: 
created: 
title: "preview"
tags: []
signature: "pub fn preview (value : & str) -> String"
---

Formats a value onto a single line truncated with an ellipsis.

A value on ONE line, as VS Code's Variables shows it: the whitespace of a
pretty-printed value folded to single spaces, and the tail cut with `…`.
Measured on 30/09/2026: `self` of a click handler is a whole form, and
written out it took the panel's full height, pushing the stack and the
breakpoints out of sight.

# Examples

```rust
use crawrad_host::debug_panel::preview;

assert_eq!(preview("foo\nbar"), "foo bar");
```
