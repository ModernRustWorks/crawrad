---
id: explorer-model-no-design
kind: doc
target_file: src/explorer_model.rs
line_hint: 251
author: 
created: 
title: "NO_DESIGN"
tags: []
signature: "pub const NO_DESIGN : & str = \"No form here yet.\""
---

Phrase for a member with no designs at all.

Phrase for member with no designs at all. It does not promise a badge that will
appear: there is no form to open there, and saying "open one" would send looking
for what does not exist.

# Examples

```rust
use crawrad_project::explorer_model::NO_DESIGN;

assert_eq!(NO_DESIGN, "No form here yet.");
```
