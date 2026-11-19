---
id: chat-layout-cli-fallback-notice
kind: doc
target_file: src/chat_layout.rs
line_hint: 42
author: 
created: 
title: "cli_fallback_notice"
tags: []
signature: "pub fn cli_fallback_notice (reason : & str) -> String"
---

Formats the CLI fallback notice when ACP is unavailable.

Returns `"ACP unavailable — answered through the CLI (<reason>)"` where `<reason>`
is the error's first line, trimmed, cut to 120 chars with `…` when longer (counting chars, not bytes).

# Examples

```rust
use crawrad_agent::chat_layout::cli_fallback_notice;

let notice = cli_fallback_notice("timeout");
assert!(notice.contains("timeout"));
```
