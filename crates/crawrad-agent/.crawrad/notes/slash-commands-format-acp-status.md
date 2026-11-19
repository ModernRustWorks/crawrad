---
id: slash-commands-format-acp-status
kind: doc
target_file: src/slash_commands.rs
line_hint: 256
author: 
created: 
title: "format_acp_status"
tags: []
signature: "pub fn format_acp_status (acp : & crate :: acp :: AcpConfig) -> String"
---

Formats the markdown status report for the `/acp` slash command.

# Examples

```rust
use crawrad_agent::acp::AcpConfig;
use crawrad_agent::slash_commands::format_acp_status;

let cfg = AcpConfig::default();
let status = format_acp_status(&cfg);
assert!(status.contains("Agent Client Protocol"));
```
