---
id: client-lspclient-initialize
kind: doc
target_file: src/client.rs
line_hint: 422
author: 
created: 
title: "LspClient::initialize"
tags: []
signature: "fn initialize (& self , workspace_root : & Path) -> Result < () , LspError >"
---

Performs the LSP handshake (`initialize` -> response -> `initialized`).
