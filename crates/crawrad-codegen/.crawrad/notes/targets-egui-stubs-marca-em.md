---
id: targets-egui-stubs-marca-em
kind: doc
target_file: src/targets/egui/stubs.rs
line_hint: 20
author: 
created: 
title: "marca_em"
tags: []
signature: "pub fn marca_em (texto : & str) -> Option < (usize , bool) >"
---

Where the marker is in the user's file, and whether it arrived in **CRLF**.

# The `\r` that split the `impl` in two

[`MARCAS_ACEITAS`] is written with `\n`, and a raw `contains` only finds the marker
in a Unix line-ending file. The file BELONGS TO THE USER: on Windows it
comes out in CRLF from a `git clone` with `core.autocrlf=true`, from Notepad,
from half of all editors. Without this check, generating into a CRLF file did not find
the anchor and sent the handler to the END — birthing a second
`impl FormMain` underneath the first.

Measured on screen on 16/09/2026, with a double click on `OnMouseDown` of a button
in the sample app: the code came out right, at the wrong anchor. It compiles — and that
is exactly why it went unnoticed.

Returns the BYTE where to insert and the line ending that the file uses, because the
inserter needs both: writing a `\n` block inside a `\r\n` file
leaves the file mixed, and the `git diff` of whoever receives it marks the
entire file.

# Examples

```rust
use crawrad_codegen::targets::egui::stubs::{marca_em, MARCA};

let (pos, crlf) = marca_em(MARCA).unwrap();
assert_eq!(pos, 0);
assert!(!crlf);
```
