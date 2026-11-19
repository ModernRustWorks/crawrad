---
id: cargo-tool-compiler-messages
kind: doc
target_file: src/cargo_tool.rs
line_hint: 74
author: 
created: 
title: "compiler_messages"
tags: []
signature: "pub fn compiler_messages (stdout : & [u8]) -> Vec < Value >"
---

The `message` of each `compiler-message` line of `--message-format=json`.

# Examples

```rust
use crawrad_mcp::cargo_tool::compiler_messages;

let msgs = compiler_messages(b"");
assert!(msgs.is_empty());
```
