---
id: client-acpclient-set-config-option
kind: doc
target_file: src/client.rs
line_hint: 1215
author: 
created: 
title: "AcpClient::set_config_option"
tags: []
signature: "pub fn set_config_option (& self , config_id : & str , value : & str ,) -> Result < Vec < serde_json :: Value > , String >"
---

Sets a configuration option for the active session.

Sets a configuration option for the active session (`session/set_config_option`).

# Errors

Returns an error if there is no active session or setting the option fails.

# Examples

```rust,no_run
// Sets config option on external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.set_config_option("opt", "val").is_err());
```
