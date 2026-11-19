---
id: base-componentbase
kind: doc
target_file: src/base.rs
line_hint: 14
author: 
created: 
title: "ComponentBase"
tags: []
signature: "pub struct ComponentBase"
---

Top level of the component hierarchy: basic identity.

Conceptual equivalent of `TComponent` in the classic RAD IDE. Every component —
visual or non-visual — has a unique identity in the document, a name, and a class.

# Examples

```rust
use crawrad_runtime::ComponentBase;

let comp = ComponentBase::new("btn1", "Button1", "Button");
assert_eq!(comp.name, "Button1");
```
