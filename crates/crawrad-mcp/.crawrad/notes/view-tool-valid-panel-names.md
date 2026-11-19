---
id: view-tool-valid-panel-names
kind: doc
target_file: src/view_tool.rs
line_hint: 75
author: 
created: 
title: "VALID_PANEL_NAMES"
tags: []
signature: "pub const VALID_PANEL_NAMES : & [& str] = & [\"Explorer\" , \"Inspector\" , \"Palette\" , \"Structure\" , \"Components\" , \"Packages\" , \"Debug\" , \"Extensions\" , \"Chat\" , \"Terminal\" , \"References\" , \"Problems\" , \"Design\" , \"Notes\" ,]"
---

Valid panel names advertised in the catalogue and error messages.

# Examples

```rust
use crawrad_mcp::view_tool::VALID_PANEL_NAMES;

assert!(VALID_PANEL_NAMES.contains(&"Explorer"));
```
