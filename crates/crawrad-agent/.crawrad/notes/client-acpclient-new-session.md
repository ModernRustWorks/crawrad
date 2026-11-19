---
id: client-acpclient-new-session
kind: doc
target_file: src/client.rs
line_hint: 1044
author: 
created: 
title: "AcpClient::new_session"
tags: []
signature: "pub fn new_session (& self , cwd : Option < & Path > , mcp_servers : Vec < serde_json :: Value > ,) -> Result < String , String >"
---

Creates a new ACP session with workspace and MCP servers.

Creates a new ACP session (`session/new`), configuring workspace cwd and local MCP servers.

# Errors

Returns an error if the request fails or sessionId is missing from the response.

# Examples

```rust,no_run
// Creates new session with external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let id = client.new_session(None, vec![]).unwrap();
assert!(!id.is_empty());
```
