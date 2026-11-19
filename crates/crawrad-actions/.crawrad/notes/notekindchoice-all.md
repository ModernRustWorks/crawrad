---
id: notekindchoice-all
kind: doc
target_file: src/lib.rs
line_hint: 861
author: 
created: 
title: "NoteKindChoice::ALL"
tags: []
signature: "pub const ALL : [Self ; 3] = [Self :: Doc , Self :: Note , Self :: Comment]"
---

In the popup's row order: the chosen row's index picks the kind.

# Examples

```rust
use crawrad_actions::NoteKindChoice;

assert_eq!(NoteKindChoice::ALL.len(), 3);
```
