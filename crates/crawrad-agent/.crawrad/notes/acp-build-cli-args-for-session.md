---
id: acp-build-cli-args-for-session
kind: doc
target_file: src/acp.rs
line_hint: 28
author: 
created: 
title: "build_cli_args_for_session"
tags: []
signature: "pub fn build_cli_args_for_session (binary : & str , prompt : & str , model : Option < & str > , session : Option < (& str , bool) > , support : SessionSupport ,) -> CliInvocation"
---

Builds the CLI command-line arguments and stdin payload for an agent query.

# Examples

```rust
use crawrad_agent::{SessionSupport, build_cli_args_for_session};

let inv = build_cli_args_for_session("claude", "hello", None, None, SessionSupport::None);
assert_eq!(inv.binary, "claude");
```
