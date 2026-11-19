---
id: componentdesc-accepts-child
kind: doc
target_file: src/lib.rs
line_hint: 618
author: 
created: 
title: "ComponentDesc::accepts_child"
tags: []
signature: "pub fn accepts_child (& self , class : & str) -> bool"
---

Returns whether this component accepts a child of the given class.

**Does this component accept a child OF THIS class?**

# The hole it closes

Until 18/09/2026 the answer was "any". `accepts_children`
answered yes for the collection and nobody checked the class —
`Document::add_child` only checks duplicate id and existing parent. Meaning:
**a `Button` could be a child of an `ActionList`**, and the document
accepted it.

With the class declared inside [`Children::Collection`], the collection becomes
the real `TCollection`: it says what items are made of, and whatever
is not that is rejected.

Area continues accepting any control, and that is not careless — a
`Panel` receives whatever the user wants to put in it, which is what a visual
container is.

# Examples

```rust
use crawrad_registry::{action_list, panel};

assert!(panel().accepts_child("Button"));
assert!(action_list().accepts_child("Action"));
assert!(!action_list().accepts_child("Button"));
```
