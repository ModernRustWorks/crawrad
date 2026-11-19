---
id: creation-new-form-hint
kind: doc
target_file: src/creation.rs
title: "new_form_hint"
tags: []
signature: "pub fn new_form_hint (framework_missing: bool) -> & 'static str"
---

What `New Form…` adds, written under the Toolbox's empty state.

A project with no UI framework also gets its dependency, on Generate.

# Examples

```rust
use crawrad_project::creation::new_form_hint;
assert!(new_form_hint(true).ends_with("the framework dependency"));
assert_eq!(new_form_hint(false), "Adds src/form_main.crawform");
```
