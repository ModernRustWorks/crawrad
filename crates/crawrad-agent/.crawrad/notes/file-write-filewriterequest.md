---
id: file-write-filewriterequest
kind: doc
target_file: src/file_write.rs
line_hint: 13
author: 
created: 
title: "FileWriteRequest"
tags: []
signature: "pub struct FileWriteRequest"
---

An agent's file write, held until the IDE decides it.

The answer to the agent's `fs/write_text_file` waits for the user's review;
dropping the request unanswered tells the agent the edit was not applied.

# Examples

```rust
use crawrad_agent::FileWriteRequest;

let req = FileWriteRequest::new("a.rs".into(), "fn main() {}".into(), |_| {});
assert_eq!(req.content, "fn main() {}");
```
