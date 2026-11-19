---
id: ide-app-view-state-ideapp-arm-tool
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 95
author: 
created: 
title: "IdeApp::arm_tool"
tags: []
signature: "pub fn arm_tool (& mut self , cx : & mut Cx , class_name : & str)"
---

Arms a tool in the palette to be dropped on the form.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.arm_tool(cx, "Button");
assert_eq!(app.tool_armed.as_deref(), Some("Button"));
```
