---
id: componentdesc-answers-to
kind: doc
target_file: src/lib.rs
line_hint: 611
author: 
created: 
title: "ComponentDesc::answers_to"
tags: []
signature: "pub fn answers_to (& self , capability : & str) -> bool"
---

Returns whether this component satisfies a requested capability name.

**Does this component answer to this name?**

A reference property declares ONE name (`ref_class`), and this is the
question that decides who appears in its list.

The class counts as its own capability — and that is what makes
`Button.action`, which asks for `"Action"`, continue working without
`Action` declaring anything. The new case is the other: five different providers
declaring `provides = ["DbProvider"]`, and an abstraction asking for
`"DbProvider"` accepting all five without knowing any.

# Examples

```rust
use crawrad_registry::button;

let desc = button();
assert!(desc.answers_to("Button"));
assert!(desc.answers_to("Control"));
```
