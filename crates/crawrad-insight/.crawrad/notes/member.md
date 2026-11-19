---
id: member
kind: doc
target_file: src/lib.rs
line_hint: 40
author: 
created: 
title: "Member"
tags: []
signature: "pub struct Member"
---

What the completion list shows for one member.

# Examples

```rust
use crawrad_insight::Member;

let m = Member { label: "width".into(), detail: "f32".into(), documentation: None };
assert_eq!(m.label, "width");
```
