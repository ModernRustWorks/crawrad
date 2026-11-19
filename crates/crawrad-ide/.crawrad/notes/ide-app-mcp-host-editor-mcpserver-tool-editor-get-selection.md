---
id: ide-app-mcp-host-editor-mcpserver-tool-editor-get-selection
kind: doc
target_file: src/ide_app/mcp_host/editor.rs
line_hint: 188
author: 
created: 
title: "McpServer::tool_editor_get_selection"
tags: []
signature: "pub (super) fn tool_editor_get_selection (& mut self , host : & HostModel , (selected , range) : (Option < String > , Option < Value >) ,) -> ToolCallResult"
---

Handles the `editor.get_selection` MCP tool request.

`selected`: the code editor's selected text, read the way Copy takes
it (#257); before, this tool said it returned the selection and only
returned the file's length. `range`: where it is (#339).
