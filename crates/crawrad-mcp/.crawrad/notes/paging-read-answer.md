---
id: paging-read-answer
kind: doc
target_file: src/paging.rs
line_hint: 19
author: 
created: 
title: "read_answer"
tags: []
signature: "pub fn read_answer (path : & str , text : & str , hash : & str , args : & serde_json :: Value , predicted : Option < String > ,) -> (serde_json :: Value , String)"
---

Builds the JSON response and message for a paged buffer read.

What a paged read answers (`code.read`, `editor.get_buffer_content`):
the slice `args` asks for (`offset`, `limit`, default 64 KiB), the whole
text's hash and size, and the pending diff review's predicted hash.

# Examples

```rust
use crawrad_mcp::paging::read_answer;
use serde_json::json;

let (ans, msg) = read_answer("src/lib.rs", "hello\n", "abc", &json!({}), None);
assert_eq!(ans["content"], "hello\n");
assert!(msg.contains("Read 6 bytes"));
```
