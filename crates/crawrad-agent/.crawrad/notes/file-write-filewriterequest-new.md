---
id: file-write-filewriterequest-new
kind: doc
target_file: src/file_write.rs
line_hint: 20
author: 
created: 
title: "FileWriteRequest::new"
tags: []
signature: "pub fn new (path : PathBuf , content : String , responder : impl FnOnce (Result < () , String >) + Send + 'static ,) -> Self"
---

A write of `content` to `path`, answered once through `responder`.

# Examples

```rust
use crawrad_agent::FileWriteRequest;

let req = FileWriteRequest::new("a.rs".into(), String::new(), |result| println!("{result:?}"));
assert!(req.content.is_empty());
```
