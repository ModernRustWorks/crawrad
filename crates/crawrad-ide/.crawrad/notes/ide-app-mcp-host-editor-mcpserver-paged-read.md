---
id: ide-app-mcp-host-editor-mcpserver-paged-read
kind: doc
target_file: src/ide_app/mcp_host/editor.rs
line_hint: 60
author: 
created: 
title: "McpServer::paged_read"
tags: []
signature: "pub (super) fn paged_read (& mut self , tool : & str , host : & HostModel , args : & Value , (path_str , path) : (& str , & std :: path :: Path) , predicted : Option < String > ,) -> ToolCallResult"
---

Performs a paged read of buffer or disk text for editor and code tools.

`code.read` and `editor.get_buffer_content`: the live text (or the
disk), its hash kept for the next edit, the page asked for.
