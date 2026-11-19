---
id: dialogs-table-generated
kind: doc
target_file: src/dialogs_table.rs
line_hint: 313
author: 
created: 
title: "GENERATED"
tags: []
signature: "pub const GENERATED : [DialogSpec ; 23] = [ITEMS , LINES , RENAME , COMPONENT_RENAME , ACTIONS , STANDARD , IMAGELIST , ADDICON , QUIT , RECOVER , PROJECT_OPTIONS , COLOR , CREATING , ICONS , MEMORY , MCP , NOTE , GOTO , QUICK_OPEN , INHERIT_FORM , NEW_COMPONENT , PERMISSION , TEMPLATES ,]"
---

Every generated dialog.

# Examples

```rust
use crawrad_mcp::dialogs_table::GENERATED;

assert!(GENERATED.iter().any(|d| d.name == "inherit_form"));
```
