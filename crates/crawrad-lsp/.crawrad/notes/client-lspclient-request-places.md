---
id: client-lspclient-request-places
kind: doc
target_file: src/client.rs
line_hint: 1009
author: 
created: 
title: "LspClient::request_places"
tags: []
signature: "fn request_places (& self , method : & str , file_path : & Path , at : Position , context : Option < serde_json :: Value > , timeout : Duration ,) -> Result < Vec < Location > , LspError >"
---

A request at a position whose answer is a list of places.
