---
id: pathinproject
kind: doc
target_file: src/lib.rs
line_hint: 268
author: 
created: 
title: "PathInProject"
tags: []
signature: "pub struct PathInProject (String)"
---

A path **proven inside** the generated project.

The field is private and the only doorway is [`PathInProject::new`], which refuses
everything that escapes the root. This is not type pedantry: `Effect` is `pub`, and
ADR 0005 says that the
catalogue is **also** the MCP surface — on the other side of this parameter
may be an agent, and a second applier of `Effect` (the MCP server, a
test, the next shell) would receive the raw text without ever going through the shell
that validates today. With the newtype, the applier cannot receive a path
that was not validated: it does not exist.

The validation is **pure** — `components()` does not touch disk —, so it fits
here without violating the rule that the action does no I/O.

# Examples

```rust
use crawrad_actions::PathInProject;

let path = PathInProject::new("src/main.rs").unwrap();
// The separator is the platform's own.
assert_eq!(path.as_str().replace('\\', "/"), "src/main.rs");
```
