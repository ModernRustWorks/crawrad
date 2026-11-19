---
id: base64-decode
kind: doc
target_file: src/base64.rs
line_hint: 6
author: 
created: 
title: "decode"
tags: []
signature: "pub fn decode (text : & str) -> Option < Vec < u8 > >"
---

Decodes standard base64 into bytes.

Decodes standard base64 (`A-Z a-z 0-9 + /`, `=` padding). `None` when the
text is not valid base64.

# Examples

```rust
use crawrad_dap::base64::decode;

assert_eq!(decode("TWFu").as_deref(), Some(&b"Man"[..]));
assert_eq!(decode("invalid!"), None);
```
