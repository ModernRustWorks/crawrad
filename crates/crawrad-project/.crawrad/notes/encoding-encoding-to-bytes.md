---
id: encoding-encoding-to-bytes
kind: doc
target_file: src/encoding.rs
line_hint: 87
author: 
created: 
title: "Encoding::to_bytes"
tags: []
signature: "pub fn to_bytes (& self , text : & str) -> Vec < u8 >"
---

Converts text to bytes ready for disk with the encoded mark and endings.

**The text, ready for disk — with the mark and the endings it came in.**

The round trip is the point, and it is why saving cannot be a plain
`fs::write`. What the editor holds is **always** a bare newline and
never a BOM, because that is the only text a cursor can be counted in
(see [`LineEnding`]). Writing that
back as-is would silently convert a CRLF file to LF and drop its mark —
a one-word edit arriving at review as every line of the file changed.

# Examples

```rust
use crawrad_project::encoding::{Encoding, LineEnding};

let enc = Encoding { bom: false, line_ending: LineEnding::Lf };
assert_eq!(enc.to_bytes("hello\n"), b"hello\n");
```
