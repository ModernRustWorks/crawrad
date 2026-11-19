---
id: project-template-name-or-default
kind: doc
target_file: src/project_template.rs
line_hint: 503
author: 
created: 
title: "name_or_default"
tags: []
signature: "pub fn name_or_default (typed : & str) -> String"
---

The typed name, or the default when the field is empty.

The field shows the default as its placeholder, so empty means *"use that
one"*, not *"not decided"*.

# Examples

```rust
use crawrad_project::project_template::name_or_default;

assert_eq!(name_or_default(""), "my_app");
assert_eq!(name_or_default("custom"), "custom");
```
