---
id: write-message
kind: doc
target_file: src/lib.rs
line_hint: 27
author: 
created: 
title: "write_message"
tags: []
signature: "pub fn write_message < W : Write > (writer : & mut W , payload : & str) -> Result < () , TransportError >"
---

Writes a payload framed with a standard `Content-Length` header.

Formats and writes a JSON-RPC payload with the standard LSP `Content-Length` header.

# Examples

```rust
use crawrad_framing::write_message;

let mut buf = Vec::new();
write_message(&mut buf, "{}").unwrap();
assert_eq!(buf, b"Content-Length: 2\r\n\r\n{}");
```

# Errors

Returns [`TransportError`](crate::TransportError) if writing to the stream fails.
