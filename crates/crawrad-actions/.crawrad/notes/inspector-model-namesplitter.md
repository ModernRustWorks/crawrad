---
id: inspector-model-namesplitter
kind: doc
target_file: src/inspector_model.rs
line_hint: 630
author: 
created: 
title: "NameSplitter"
tags: []
signature: "pub struct NameSplitter"
---

The splitter state between the inspector's names and values.

The splitter between the Object Inspector's names and values (#554), as
the shell keeps it: what was asked, and what was last drawn.

It lives here and not in the shell because none of it needs a window:
the shell hands it the list's width, and the strip keeps the drag.

# Examples

```rust
use crawrad_actions::inspector_model::NameSplitter;

let splitter = NameSplitter::default();
assert_eq!(splitter.requested, 136.0);
```
