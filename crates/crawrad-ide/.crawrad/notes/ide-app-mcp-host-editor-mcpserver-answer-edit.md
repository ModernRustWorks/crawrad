---
id: ide-app-mcp-host-editor-mcpserver-answer-edit
kind: doc
target_file: src/ide_app/mcp_host/editor.rs
line_hint: 150
author: 
created: 
title: "McpServer::answer_edit"
tags: []
signature: "pub (super) fn answer_edit (& mut self , tool : & str , args : & Value , (path_str , after) : (& str , & str) , landed : Result < Option < crawrad_agent :: diff_review :: DiffReview > , String > , mut answer : Value ,) -> ToolCallResult"
---

Formats the edit tool answer for diff reviews and applied edits.

The answer both edit tools share, once the window landed the edit:
waiting for review (`Ok(Some)`), or applied with its new hash
remembered for the next edit. `answer` gets the hash and the status.
