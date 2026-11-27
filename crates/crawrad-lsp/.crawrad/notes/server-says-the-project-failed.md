---
id: server-says-the-project-failed
kind: doc
target_file: src/server_says.rs
line_hint: 130
author:
created:
title: "says_the_project_failed"
tags: []
signature: "pub fn says_the_project_failed (text : & str) -> bool"
---

Whether the text is the server saying it could not load the project.

Matched on the words cargo and rust-analyzer actually use. Deliberately
narrow: a false positive here would hide a real empty answer behind a
scary message.

# Examples

```rust
use crawrad_lsp::server_says::says_the_project_failed;

assert!(says_the_project_failed("failed to load manifest for workspace member"));
assert!(says_the_project_failed("cargo metadata failed"));
assert!(!says_the_project_failed("unresolved import"));
```
