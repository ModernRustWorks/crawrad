---
id: components-memo-memo
kind: doc
target_file: src/components/memo.rs
line_hint: 6
author: 
created: 
title: "memo"
tags: []
signature: "pub fn memo () -> ComponentDesc"
---

A multiline text field component.

`Memo`: multiline text field. It is `Edit` with `ES_MULTILINE` on Win32 and
`TextEdit::multiline` in egui — the same `text` property on both.

# Examples

```rust
use crawrad_registry::memo;

let desc = memo();
assert_eq!(desc.class, "Memo");
```
