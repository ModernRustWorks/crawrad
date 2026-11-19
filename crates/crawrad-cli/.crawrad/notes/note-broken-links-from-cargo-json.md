---
id: note-broken-links-from-cargo-json
kind: doc
target_file: src/note.rs
line_hint: 1347
author: 
created: 
title: "broken_links_from_cargo_json"
tags: []
signature: "pub fn broken_links_from_cargo_json (stdout : & str , root : & Path) -> Result < Vec < BrokenLink > , String >"
---

Parses cargo doc json messages into broken intra-doc link diagnostics.

Parses `cargo doc --message-format=json` stdout into broken intra-doc link diagnostics.

Filters for compiler messages with `rustdoc::broken_intra_doc_links` or
`rustdoc::private_intra_doc_links`, maps primary spans to note anchors, collapses
duplicates, and sorts the result by file and line.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::broken_links_from_cargo_json;

let res = broken_links_from_cargo_json("", Path::new("."));
assert!(res.unwrap().is_empty());
```

# Errors

Returns an error if source files cannot be read to collect note anchors.
