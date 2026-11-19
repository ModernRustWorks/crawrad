---
id: inspector-model-editors-choice-choice-new
kind: doc
target_file: src/inspector_model/editors/choice.rs
line_hint: 24
author: 
created: 
title: "Choice::new"
tags: []
signature: "pub fn new < S : AsRef < str > > (options : & [S] , value : & str) -> Self"
---

Creates a Choice editor with the selected option resolved.

`current` is resolved here, and `None` when the document holds a value
that is not in the set.

A value outside the set does NOT become the first option: the drop-down
would sit there saying `None` over a document that says something else.
Without an index it stays where it is and the row does not lie.

# Examples

```rust
use crawrad_actions::inspector_model::editors::choice::Choice;

let ch = Choice::new(&["Left", "Right"], "Left");
assert_eq!(format!("{ch:?}"), "Choice { options: [\"Left\", \"Right\"], current: Some(0) }");
```
