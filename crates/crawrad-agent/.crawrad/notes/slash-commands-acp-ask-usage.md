---
id: slash-commands-acp-ask-usage
kind: doc
target_file: src/slash_commands.rs
line_hint: 336
author: 
created: 
title: "acp_ask_usage"
tags: []
signature: "pub fn acp_ask_usage () -> & 'static str"
---

Usage message for `/acp ask`.

# Examples

```rust
use crawrad_agent::slash_commands::acp_ask_usage;

assert!(acp_ask_usage().contains("/acp ask"));
```
