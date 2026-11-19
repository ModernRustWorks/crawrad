---
id: edit-review-ask-before-edits-meta
kind: doc
target_file: src/edit_review.rs
line_hint: 12
author: 
created: 
title: "ask_before_edits_meta"
tags: []
signature: "pub fn ask_before_edits_meta() -> serde_json::Value"
---

The `session/new` `_meta` that makes a Claude agent ask before every edit.

A `PreToolUse` hook answers `ask` for its edit tools, so even in `auto` or
`acceptEdits` the edit reaches the IDE as `session/request_permission`.

# Examples

```rust
let meta = crawrad_agent::edit_review::ask_before_edits_meta();
assert!(meta["claudeCode"]["options"]["settings"]["hooks"]["PreToolUse"].is_array());
```
