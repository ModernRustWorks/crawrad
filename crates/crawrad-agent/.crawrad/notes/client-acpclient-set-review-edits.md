---
id: client-acpclient-set-review-edits
kind: doc
target_file: src/client.rs
line_hint: 1560
author: 
created: 
title: "AcpClient::set_review_edits"
tags: []
signature: "pub fn set_review_edits(&self, on: bool)"
---

Makes the next sessions' file edits ask the IDE first.

On, `session/new` carries [`crate::edit_review::ask_before_edits_meta`], so the
agent's own mode no longer accepts edits by itself.

# Examples

```rust
fn review(client: &crawrad_agent::client::AcpClient) {
    client.set_review_edits(true);
}
```
