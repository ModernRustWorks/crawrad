---
id: services-hosteditorservice
kind: doc
target_file: src/services.rs
line_hint: 81
author: 
created: 
title: "HostEditorService"
tags: []
signature: "pub struct HostEditorService"
---

Host implementation of `EditorService` managing file buffers.

Host implementation of `EditorService` managing file buffers and active document view.

# Examples

```rust
use crawrad_extension_api::services::EditorService;
use crawrad_extension_host::HostEditorService;

let svc = HostEditorService::new();
assert!(svc.active_file().is_none());
```
