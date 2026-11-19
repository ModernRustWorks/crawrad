---
id: note-preview-doctestrunner
kind: doc
target_file: src/note_preview.rs
line_hint: 282
author: 
created: 
title: "DoctestRunner"
tags: []
signature: "pub struct DoctestRunner"
---

Background runner for Note dialog doctests.

The Test example runs of the Note dialog, one at a time, off the UI thread.
The task wakes the IDE when it answers (`crawrad_os::tasks::wake`).

# Examples

```rust
use crawrad_project::note_preview::DoctestRunner;

let runner = DoctestRunner::default();
assert!(!runner.is_running());
```
