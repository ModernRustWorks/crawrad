---
id: host-shellextensionhost-view-item-clicked
kind: doc
target_file: src/host.rs
line_hint: 226
author: 
created: 
title: "ShellExtensionHost::view_item_clicked"
tags: []
signature: "pub fn view_item_clicked (& self , extension : & str , view_id : & str , item_id : & str ,) -> Result < () , ExtensionError >"
---

An item of a view was clicked: its extension is told.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.view_item_clicked("unknown", "view1", "item1").is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the extension is unknown or fails to receive the click event.
