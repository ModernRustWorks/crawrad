---
id: markdown-chatcontentblock
kind: doc
target_file: src/markdown.rs
line_hint: 9
author: 
created: 
title: "ChatContentBlock"
tags: []
signature: "pub enum ChatContentBlock"
---

A content segment extracted from the Markdown answer.

# Examples

```rust
use crawrad_agent::ChatContentBlock;

let block = ChatContentBlock::Text("hello".to_string());
assert!(matches!(block, ChatContentBlock::Text(_)));
```
