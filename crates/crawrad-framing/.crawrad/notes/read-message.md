---
id: read-message
kind: doc
target_file: src/lib.rs
line_hint: 36
author: 
created: 
title: "read_message"
tags: []
signature: "pub fn read_message < R : BufRead > (reader : & mut R) -> Result < String , TransportError >"
---

Reads a single message from a reader by parsing headers and payload.

Reads a single JSON-RPC message from a buffered reader by parsing headers and reading the exact payload.

# Examples

```rust
use std::io::Cursor;
use crawrad_framing::read_message;

let mut input = Cursor::new(b"Content-Length: 2\r\n\r\n{}");
let msg = read_message(&mut input).unwrap();
assert_eq!(msg, "{}");
```

# Errors

Returns [`TransportError`](crate::TransportError) on I/O failure, unexpected EOF, or invalid headers.
