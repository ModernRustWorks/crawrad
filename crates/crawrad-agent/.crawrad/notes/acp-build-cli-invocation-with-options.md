---
id: acp-build-cli-invocation-with-options
kind: doc
target_file: src/acp.rs
line_hint: 50
author: 
created: 
title: "build_cli_invocation_with_options"
tags: []
signature: "pub fn build_cli_invocation_with_options (binary : & str , prompt : & str , model : Option < & str > , session : Option < (& str , bool) > , support : SessionSupport , delivery : PromptDelivery , cwd : Option < PathBuf > ,) -> CliInvocation"
---

Builds a CLI invocation with custom options.

Builds a structured CLI invocation with configurable delivery mechanism and working directory.

# Examples

```rust
use crawrad_agent::{PromptDelivery, SessionSupport, build_cli_invocation_with_options};

let inv = build_cli_invocation_with_options(
    "claude",
    "hello",
    None,
    None,
    SessionSupport::None,
    PromptDelivery::Stdin,
    None,
);
assert_eq!(inv.binary, "claude");
```
