---
id: targets-makepad-action-link-actionlinks
kind: doc
target_file: src/targets/makepad/action_link.rs
line_hint: 23
author: 
created: 
title: "ActionLinks"
tags: []
signature: "pub struct ActionLinks"
---

Action link dispatches and pre-sync lines added to a Makepad form.

What the links add to the form: the dispatch in `handle_actions` and the
lines that run at the top of `sync_ui`.

# Examples

```rust
use crawrad_codegen::targets::makepad::action_link::ActionLinks;

let links = ActionLinks::default();
assert!(links.dispatch.is_empty());
```
