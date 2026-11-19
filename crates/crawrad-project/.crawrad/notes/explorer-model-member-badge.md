---
id: explorer-model-member-badge
kind: doc
target_file: src/explorer_model.rs
line_hint: 226
author: 
created: 
title: "member_badge"
tags: []
signature: "pub fn member_badge (m : & MemberNode) -> Option < & 'static str >"
---

What the member is inside the group, in one word — or nothing.

A function and two constants, so that header and legend read the same word.
Written in both places, they would diverge on the first day someone changed one:
the legend would explain a badge not on screen, which is the most silent way for
a legend to lie.

**The active gets the word even if it is also the default**, and that is the decision:
both coincide almost always — the IDE opens the form of `default-members` —, and
writing `active default` on that line would waste column width to say the same thing
twice. `default` only appears when it **disagrees** with active, which is the only
moment it informs anything.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::explorer_model::{member_badge, MemberNode, MARK_ACTIVE};

let node = MemberNode {
    folder: PathBuf::from("app"),
    active: true,
    is_default: false,
};
assert_eq!(member_badge(&node), Some(MARK_ACTIVE));
```
