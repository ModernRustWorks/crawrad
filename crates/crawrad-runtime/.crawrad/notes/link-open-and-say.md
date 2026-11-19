---
id: link-open-and-say
kind: doc
target_file: src/link.rs
line_hint: 7
author: 
created: 
title: "open_and_say"
tags: []
signature: "pub fn open_and_say (url : & str) -> String"
---

Opens `url` via [`open_url`] and returns the status line describing the result.

# Examples

```rust
use crawrad_runtime::link::open_and_say;

let msg = open_and_say("ftp://example.com");
assert!(msg.starts_with("Not opened"));
```
