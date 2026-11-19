---
id: markdown-parse-chat-markdown
kind: doc
target_file: src/markdown.rs
line_hint: 62
author: 
created: 
title: "parse_chat_markdown"
tags: []
signature: "pub fn parse_chat_markdown (raw : & str) -> Vec < ChatContentBlock >"
---

Parses an answer string and extracts text blocks and code blocks.

# Examples

```rust
use crawrad_agent::parse_chat_markdown;

let blocks = parse_chat_markdown("Hello\n```rust\nfn main() {}\n```");
assert_eq!(blocks.len(), 2);
```
