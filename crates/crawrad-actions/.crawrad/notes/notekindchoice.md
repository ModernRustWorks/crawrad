---
id: notekindchoice
kind: doc
target_file: src/lib.rs
line_hint: 852
author: 
created: 
title: "NoteKindChoice"
tags: []
signature: "pub enum NoteKindChoice"
---

The kind of note chosen by a direct shortcut or menu action.

# Examples

```rust
use crawrad_actions::NoteKindChoice;

let kind = NoteKindChoice::Doc;
assert_eq!(kind, NoteKindChoice::Doc);
```
