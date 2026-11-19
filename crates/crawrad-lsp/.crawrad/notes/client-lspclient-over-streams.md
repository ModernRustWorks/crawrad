---
id: client-lspclient-over-streams
kind: doc
target_file: src/client.rs
line_hint: 176
author: 
created: 
title: "LspClient::over_streams"
tags: []
signature: "pub (crate) fn over_streams (child : Option < Child > , stdin : impl std :: io :: Write + Send + 'static , stdout : impl std :: io :: Read + Send + 'static , stderr : Option < impl std :: io :: Read + Send + 'static > , source : crate :: locate :: ServerSource ,) -> Result < Self , LspError >"
---

Creates a client over any pair of streams without a handshake.

A client over any pair of streams — the server's process in
[`Self::spawn`], an in-process fake server in the tests. No handshake.
