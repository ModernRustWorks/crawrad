---
id: adapter-portadb-bridge-portadbbridge-open-table
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 76
author: 
created: 
title: "PortaDbBridge::open_table"
tags: []
signature: "pub fn open_table (url : & str , table : & str) -> Result < DatasetAdapter , String >"
---

Opens a database table synchronously on a private current-thread runtime.

Connects to `url` via [`ConnectOptions`][portadb::ConnectOptions] and opens `table`
via [`Dataset::open`]. Returns an error if `url` or `table` is empty.

# Engine Lifetime

The underlying `portadb::Dataset` retains an `Engine` clone within its internal
`Arc<Mutex<DatasetInner>>`, ensuring the database connection and driver remain active
for the entire lifecycle of the returned `DatasetAdapter`.

# Examples

```rust
use crawrad_binding::PortaDbBridge;

assert!(PortaDbBridge::open_table("", "").is_err());
```

# Errors

Returns an error if `url` or `table` is empty, or if connecting to the database fails.
