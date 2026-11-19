---
id: vieweffect
kind: doc
target_file: src/lib.rs
line_hint: 795
author: 
created: 
title: "ViewEffect"
tags: []
signature: "pub enum ViewEffect"
---

What the action wants to see in the center of the window.

It is not `FileEffect`: switching tabs neither reads nor writes anything, and mixing the two
families would make an agent requesting `preview` of an `Effect` have to guess
which of the variants touches disk.

# Examples

```rust
use crawrad_actions::ViewEffect;

let effect = ViewEffect::ToggleDesignCode;
assert_eq!(effect, ViewEffect::ToggleDesignCode);
```
