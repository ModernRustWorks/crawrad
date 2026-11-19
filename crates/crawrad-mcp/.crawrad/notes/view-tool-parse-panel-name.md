---
id: view-tool-parse-panel-name
kind: doc
target_file: src/view_tool.rs
line_hint: 92
author: 
created: 
title: "parse_panel_name"
tags: []
signature: "pub fn parse_panel_name (name : & str) -> Option < Panel >"
---

Parses a panel name or alias into a catalogue Panel enum value.

Parses a panel name (case-insensitive, accepting rail names, aliases, and IDs)
into a catalogue [`Panel`].

# Examples

```rust
use crawrad_mcp::view_tool::parse_panel_name;
use crawrad_actions::Panel;

assert_eq!(parse_panel_name("explorer"), Some(Panel::Explorer));
```
