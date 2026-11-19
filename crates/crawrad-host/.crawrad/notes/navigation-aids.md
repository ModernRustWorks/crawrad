---
id: navigation-aids
kind: doc
target_file: src/navigation.rs
line_hint: 24
author: 
created: 
title: "Aids"
tags: []
signature: "pub struct Aids"
---

Editor helpers waiting for asynchronous language server responses.

The editor's helpers that wait for the language server: the jumps of
Go to Definition and the hover card. One value, so the IDE keeps one field.

# Examples

```rust
use crawrad_host::navigation::Aids;

let aids = Aids::default();
assert!(!aids.choices.is_open());
```
