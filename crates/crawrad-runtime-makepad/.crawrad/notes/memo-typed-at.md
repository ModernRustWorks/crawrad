---
id: memo-typed-at
kind: doc
target_file: src/memo.rs
line_hint: 25
author: 
created: 
title: "typed_at"
tags: []
signature: "pub fn typed_at (text : & str , start : usize , end : usize , typed : & str , max_length : i64 ,) -> Option < (String , usize) >"
---

Replaces text slice with typed input within `max_length` limit.

`text` with the selection `start..end` (byte indices) replaced by `typed`,
and where the cursor goes; `None` when the limit leaves no room.

# Examples

```rust
use crawrad_runtime_makepad::memo::typed_at;

let (text, pos) = typed_at("hello", 5, 5, "!", 10).unwrap();
assert_eq!(text, "hello!");
assert_eq!(pos, 6);
```
