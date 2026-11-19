---
id: registry-category
kind: doc
title: "Inspector Category"
tags: [inspector, registry]
---

Where the property appears in the inspector.

The order of the variants is the order of the groups in the inspector.

# Examples

```rust
use crawrad_registry::Category;

assert!(Category::Identity < Category::Advanced);
```
