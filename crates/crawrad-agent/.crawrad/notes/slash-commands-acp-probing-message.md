---
id: slash-commands-acp-probing-message
kind: doc
target_file: src/slash_commands.rs
line_hint: 331
author: 
created: 
title: "acp_probing_message"
tags: []
signature: "pub fn acp_probing_message () -> & 'static str"
---

Background probing message for `/acp test`.

# Examples

```rust
use crawrad_agent::slash_commands::acp_probing_message;

assert!(acp_probing_message().contains("Probing"));
```
