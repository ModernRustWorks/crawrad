---
id: explorer-model-no-plan-with-design
kind: doc
target_file: src/explorer_model.rs
line_hint: 248
author: 
created: 
title: "NO_PLAN_WITH_DESIGN"
tags: []
signature: "pub const NO_PLAN_WITH_DESIGN : & str = \"Open a form here to see who owns each file.\""
---

Phrase for member that **has** a design and none of them open.

# Examples

```rust
use crawrad_project::explorer_model::NO_PLAN_WITH_DESIGN;

assert!(NO_PLAN_WITH_DESIGN.contains("form"));
```
