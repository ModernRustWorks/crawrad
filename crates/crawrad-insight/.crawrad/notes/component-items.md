---
id: component-items
kind: doc
target_file: src/lib.rs
line_hint: 142
author: 
created: 
title: "component_items"
tags: []
signature: "pub fn component_items (form : Option < & Document > , designer : Option < & str > , base : & str ,) -> Vec < item :: ItemSugestao >"
---

Generates completion items for fields accessible on a form component.

The completion rows after `base.` in a form's code: the class of `base`
from the live `form`, else from its `designer` source, and its fields as
generated, each with its type beside it.

# Examples

```rust
use crawrad_insight::component_items;

let items = component_items(None, None, "unknown");
assert!(items.is_empty());
```
