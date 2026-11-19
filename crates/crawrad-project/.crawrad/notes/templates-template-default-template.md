---
id: templates-template-default-template
kind: doc
target_file: src/templates.rs
line_hint: 190
author: 
created: 
title: "Template::default_template"
tags: []
signature: "pub fn default_template () -> & 'static Template"
---

The blank template — what `New Project…` uses when none was chosen.

The first in the list, and not a fourth copy of the empty form written
separately: the path of "create without choosing a template" must go
through the same place as choosing one, otherwise the two diverge on the
first fix that only one receives.

# Examples

```rust
use crawrad_project::templates::Template;

let t = Template::default_template();
assert_eq!(t.name, "Blank form");
```
