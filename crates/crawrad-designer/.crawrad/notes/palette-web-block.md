---
id: palette-web-block
kind: doc
target_file: src/palette.rs
line_hint: 140
author: 
created: 
title: "web_block"
tags: []
signature: "pub fn web_block (class : & str) -> Option < & 'static str >"
---

Why a component cannot work in a web page, if it cannot.

A web project runs as WebAssembly in the browser: no files on the user's
disk, no raw network sockets, no native file dialogs. The SQLite providers,
the PostgreSQL and Redis providers and the Open/Save dialogs answer a reason;
every other class answers `None`.

# Examples

```rust
use crawrad_designer::palette::web_block;

assert!(web_block("SqliteProvider").is_some());
assert_eq!(web_block("Button"), None);
```
