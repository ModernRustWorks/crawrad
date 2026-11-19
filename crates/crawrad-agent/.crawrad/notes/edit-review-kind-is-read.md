---
id: edit-review-kind-is-read
kind: doc
target_file: src/edit_review.rs
line_hint: 19
author: 
created: 
title: "kind_is_read"
tags: []
signature: "pub fn kind_is_read(kind: Option<&str>) -> bool"
---

Whether an ACP tool call `kind` may be auto-approved as a read.

Only `read` and `search`, or no kind at all, count; `edit`, `delete` and the
rest never pass as reads, whatever the tool's name.

# Examples

```rust
use crawrad_agent::edit_review::kind_is_read;
assert!(kind_is_read(Some("read")));
assert!(!kind_is_read(Some("edit")));
```
