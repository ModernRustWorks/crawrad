---
id: workspace-refusal
kind: doc
target_file: src/workspace.rs
line_hint: 1682
author: 
created: 
title: "Refusal"
tags: []
signature: "pub struct Refusal"
---

Why creation did not happen, and what the shell can **offer** instead.

It is not just a phrase because one of the reasons has a way out, and the way out is a button:
the chosen folder already has a project inside. Whoever points `New Project…` at
such a folder wanted that project — that was what the owner wanted in his two attempts —,
and telling him "rename the folder" sends him away from what he sought.

**The protection does not loosen.** `open_instead` is an invitation to OPEN; nothing
is overwritten in either case, and the `Nothing was written` of both phrases remains
true — sustained by `create_project`'s rollback.

# Examples

```rust
use crawrad_project::workspace::Refusal;

let r = Refusal::simple("failed".into());
assert!(!r.open_instead);
```
