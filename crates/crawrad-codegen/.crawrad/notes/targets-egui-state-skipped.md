---
id: targets-egui-state-skipped
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 183
author: 
created: 
title: "SKIPPED"
tags: []
signature: "pub const SKIPPED : & [(& str , & str , & str)] = & [(\"\" , \"action\" , \"Action links are resolved at generation, not stored in state\" ,) , (\"BitBtn\" , \"action\" , \"Action links are resolved at generation, not stored in state\" ,) , (\"BitBtn\" , \"kind\" , \"BitBtn standard kind is resolved at render time, not kept in state\" ,) , (\"Button\" , \"action\" , \"Action links are resolved at generation, not stored in state\" ,) , (\"Grid\" , \"data_source\" , \"Data source binding is resolved at generation, not held in state\" ,) , (\"Grid\" , \"design_rows\" , \"Designer-only: how many real rows the canvas shows (#925); the app reads its own data\" ,) , (\"MainMenu\" , \"images\" , \"ImageList reference for MainMenu is resolved at generation, not kept in state\" ,) , (\"Memo\" , \"scroll_bars\" , \"Egui TextEdit handles scroll bars automatically\" ,) , (\"MenuItem\" , \"action\" , \"Action links are resolved at generation, not stored in state\" ,) , (\"PopupMenu\" , \"images\" , \"ImageList reference for PopupMenu is resolved at generation, not kept in state\" ,) , (\"ScrollBar\" , \"kind\" , \"ScrollBar orientation kind is resolved at layout/render time, not kept in state\" ,) , (\"SpeedButton\" , \"action\" , \"Action links are resolved at generation, not stored in state\" ,) , (\"ToolBar\" , \"button_height\" , \"ToolBar button dimensions are resolved at layout/render time\" ,) , (\"ToolBar\" , \"button_width\" , \"ToolBar button dimensions are resolved at layout/render time\" ,) , (\"TreeView\" , \"images\" , \"ImageList reference for TreeView is not held in egui state\" ,) ,]"
---

Catalogue properties not kept in the egui component state.

# Examples

```rust
use crawrad_codegen::targets::egui::state::SKIPPED;

assert!(!SKIPPED.is_empty());
```
