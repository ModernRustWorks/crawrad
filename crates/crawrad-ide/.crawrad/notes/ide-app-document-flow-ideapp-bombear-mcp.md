---
id: ide-app-document-flow-ideapp-bombear-mcp
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 140
author: 
created: 
title: "IdeApp::bombear_mcp"
tags: []
signature: "pub fn bombear_mcp (& mut self , cx : & mut Cx)"
---

Pumps MCP requests received by the IPC listener.

The gesture loop: collects from the canvas, hands over to `crawrad-designer`, applies
to the document, and returns the result to the canvas.

It is the entire role of the shell in ADR 0027. No hit-testing, no
handle arithmetic, and no snapping live here: there are 6,800 tested lines
on the other side of this boundary.
Pumps local MCP requests received by the background IPC listener.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.mcp_rx.is_none());
```
