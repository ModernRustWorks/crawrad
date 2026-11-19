---
id: capture-tool-encode-png
kind: doc
target_file: src/capture_tool.rs
line_hint: 84
author: 
created: 
title: "encode_png"
tags: []
signature: "pub fn encode_png (capture : & crawrad_os :: Capture) -> Vec < u8 >"
---

Encodes raw RGBA capture pixels as PNG image bytes.

# Examples

```
use crawrad_mcp::capture_tool::encode_png;
use crawrad_os::Capture;

let capture = Capture { width: 1, height: 1, rgba: vec![255, 0, 0, 255] };
let png_bytes = encode_png(&capture);
assert!(png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
```
