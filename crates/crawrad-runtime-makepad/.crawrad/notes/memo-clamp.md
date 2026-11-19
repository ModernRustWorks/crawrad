---
id: memo-clamp
kind: doc
target_file: src/memo.rs
line_hint: 14
author: 
created: 
title: "clamp"
tags: []
signature: "pub fn clamp (text : & str , max_length : i64) -> & str"
---

`text` cut to `max_length` characters; `0` is no limit.

# Examples

```rust
use crawrad_runtime_makepad::memo::clamp;

assert_eq!(clamp("Hello World", 5), "Hello");
assert_eq!(clamp("Hello", 0), "Hello");
```
