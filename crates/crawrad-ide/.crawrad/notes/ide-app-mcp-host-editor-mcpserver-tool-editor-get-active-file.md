---
id: ide-app-mcp-host-editor-mcpserver-tool-editor-get-active-file
kind: doc
target_file: src/ide_app/mcp_host/editor.rs
line_hint: 14
author: 
created: 
title: "McpServer::tool_editor_get_active_file"
tags: []
signature: "pub (super) fn tool_editor_get_active_file (& mut self , host : & HostModel , (here , read_only , status) : (Option < crawrad_host :: navigation :: Place > , bool , Option < String >) , breadcrumb : Vec < String > , (disk_changed , deleted_on_disk) : (bool , bool) ,) -> ToolCallResult"
---

Handles the `editor.get_active_file` MCP tool request.

`front`: the caret's place in the tab in front, and whether that
tab refuses typing (#256), and the status bar's sentence (#258).
