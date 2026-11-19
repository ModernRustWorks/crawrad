---
id: creation-new-form-action
kind: doc
target_file: src/creation.rs
title: "NEW_FORM_ACTION"
tags: []
signature: "pub const NEW_FORM_ACTION : & str = \"file.new_form\""
---

The action the Toolbox's empty-state button runs: File > New Form.

The shell never writes a command id itself; it takes this one.

# Examples

```rust
use crawrad_project::creation::NEW_FORM_ACTION;

assert_eq!(NEW_FORM_ACTION, "file.new_form");
```
