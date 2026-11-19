---
id: gate
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Gate"
tags: []
signature: "pub struct Gate"
---

When an action may run: its [`Need`], refined by an optional check.

One field for both, so no action can declare a need its check contradicts:
`Need::Form.only()` when the need alone decides, `Need::Selection.and(two_or_more)`
when it narrows further. [`Gate::allows`] is the need AND the check — an
unmet need always greys, and it is the reason the IDE and the MCP give.

# Examples

```rust
use crawrad_actions::{Need, by_id};

let generate = by_id("project.generate").unwrap();
assert_eq!(generate.enabled.need, Need::FormInProject);
```
