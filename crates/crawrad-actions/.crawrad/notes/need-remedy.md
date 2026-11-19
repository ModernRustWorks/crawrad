---
id: need-remedy
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need::remedy"
tags: []
signature: "pub fn remedy (self) -> & 'static str"
---

What an agent calls to meet the need before it resends the action.

The MCP refusal is machine-actionable: it says what is missing
([`Need::reason`]) and the tool that fixes it, so the agent corrects itself
instead of retrying the same call.

# Examples

```rust
use crawrad_actions::Need;

assert_eq!(Need::Project.remedy(), "Call 'project.open' and resend.");
```
