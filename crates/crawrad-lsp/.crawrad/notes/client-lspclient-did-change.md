---
id: client-lspclient-did-change
kind: doc
target_file: src/client.rs
line_hint: 785
author: 
created: 
title: "LspClient::did_change"
tags: []
signature: "pub fn did_change (& self , file_path : & Path , version : i32 , text : & str) -> Result < () , LspError >"
---

Notifies the language server that a document's content has changed.

`text` is the whole new text (the editor gives no edits). When the
server takes `Incremental` and the document was opened, only the
part that changed goes (LSP `TextDocumentContentChangeEvent` with a
`range`); otherwise the whole text, as before.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.did_change(Path::new("src/lib.rs"), 2, "fn main() {}").is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if sending the notification fails.
