---
id: targets-shared-recorte
kind: doc
target_file: src/targets/shared.rs
line_hint: 19
author: 
created: 
title: "recorte"
tags: []
signature: "pub fn recorte (texto : & str , nome : & str) -> String"
---

The `nome` section of `texto`, ready to be emitted.

What comes out starts on the line **after** the start marker and ends on the line
**before** the end one — the markers are internal and do not travel to the user's
project.

The line ending is normalized: this repository's `core.autocrlf` is turned on, and the
shared file arrives in CRLF on the cloner's disk.

# Examples

```rust
use crawrad_codegen::targets::shared::recorte;

let doc = "// [CRAWRAD:INICIO sec]\nfn foo() {}\n// [CRAWRAD:FIM sec]";
assert_eq!(recorte(doc, "sec"), "fn foo() {}\n");
```

# Panics

Panics if the text does not contain the start or end marker of the indicated section.
