---
id: acp-status-statustracker-tool-index
kind: doc
target_file: src/acp_status.rs
line_hint: 93
author: 
created: 
title: "StatusTracker::tool_index"
tags: []
signature: "fn tool_index (& mut self , id : & str , name : & str , by_name : bool) -> usize"
---

Resolves the tool index by id or name, adding when new.

The tool `id` names (or, for an update without an id match, `name`),
added when new.
