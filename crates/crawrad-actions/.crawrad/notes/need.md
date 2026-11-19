---
id: need
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need"
tags: []
signature: "pub enum Need"
---

What an action needs before it can run: why it is greyed when it is.

Every action of the catalogue declares one in its [`Gate`]. It is a
NECESSARY condition, never a sufficient one: `align.left` needs a selection
and its gate also wants two components. The gate makes the promise
structural — no action is enabled while its need is unmet — and the IDE and
the MCP say the need when they refuse: *"Generate needs a form in the
project"*, instead of *"not available now"*.

# Examples

```rust
use crawrad_actions::{Need, by_id};

assert_eq!(by_id("project.generate").unwrap().enabled.need, Need::FormInProject);
```
